use crate::sys::{self, Interface};
use crate::winrt;
use std::cell::{Cell, RefCell};

#[allow(dead_code, reason = "we just have to keep `dxgi_factory` and `target` alive")]
pub struct Context {
    device: sys::ID3D11Device,
    dxgi_factory: sys::IDXGIFactory2,
    surface: RefCell<sys::IDCompositionSurface>,
    device_context: sys::ID3D11DeviceContext,
    width: Cell<u32>,
    height: Cell<u32>,

    dcomp: sys::IDCompositionDevice,
    target: sys::IDCompositionTarget,
    visual: sys::IDCompositionVisual,
    pub partner: winrt::IDCompositionDesktopDevicePartner6,
    hwnd: sys::HWND,
}

impl Context {
    pub fn new(hwnd: sys::HWND) -> Self {
        unsafe {
            winrt::init_com_single_threaded();
            winrt::make_window_eligible_for_dcomp_resize_sync(hwnd);

            let dxgi_factory: sys::IDXGIFactory2 = sys::CreateDXGIFactory1().unwrap();

            let mut adapters = Vec::new();
            let mut i = 0;
            while let Ok(adapter) = dxgi_factory.EnumAdapters(i) {
                i += 1;
                adapters.push(adapter);
            }

            let mut flags = sys::D3D11_CREATE_DEVICE_BGRA_SUPPORT;
            #[cfg(debug_assertions)]
            {
                flags |= sys::D3D11_CREATE_DEVICE_DEBUG;
            }

            let mut device: Option<sys::ID3D11Device> = None;
            let mut device_context: Option<sys::ID3D11DeviceContext> = None;
            sys::D3D11CreateDevice(
                None,
                sys::D3D_DRIVER_TYPE_HARDWARE,
                Default::default(),
                flags,
                None,
                sys::D3D11_SDK_VERSION,
                Some(&mut device),
                None,
                Some(&mut device_context),
            )
            .unwrap();
            let device = device.unwrap();
            let device_context = device_context.unwrap();

            let dxgi_device: sys::IDXGIDevice = device.cast().unwrap();
            let dcomp: sys::IDCompositionDevice =
                sys::DCompositionCreateDevice(&dxgi_device).unwrap();
            let partner = dcomp
                .cast::<winrt::IDCompositionDesktopDevicePartner6>()
                .unwrap();
            let target: sys::IDCompositionTarget = dcomp.CreateTargetForHwnd(hwnd, false).unwrap();
            let visual: sys::IDCompositionVisual = dcomp.CreateVisual().unwrap();
            target.SetRoot(&visual).unwrap();

            let (mut left, mut top, mut right, mut bottom) = (0, 0, 0, 0);
            for adapter in &adapters {
                let mut i = 0;
                while let Ok(output) = adapter.EnumOutputs(i) {
                    let output_desc = output.GetDesc().unwrap();
                    left = std::cmp::min(left, output_desc.DesktopCoordinates.left);
                    top = std::cmp::min(top, output_desc.DesktopCoordinates.top);
                    right = std::cmp::max(right, output_desc.DesktopCoordinates.right);
                    bottom = std::cmp::max(bottom, output_desc.DesktopCoordinates.bottom);
                    i += 1;
                }
            }
            let surface = dcomp
                .CreateSurface(
                    (right - left) as u32,
                    (bottom - top) as u32,
                    sys::DXGI_FORMAT_B8G8R8A8_UNORM,
                    sys::DXGI_ALPHA_MODE_PREMULTIPLIED,
                )
                .unwrap();

            visual.SetContent(&surface).unwrap();
            dcomp.Commit().unwrap();

            Self {
                device,
                dxgi_factory,
                device_context,
                width: Cell::new(0),
                height: Cell::new(0),
                surface: RefCell::new(surface),
                dcomp,
                target,
                visual,
                partner,
                hwnd,
            }
        }
    }

    pub fn resize(&self, width: u32, height: u32) {
        self.width.set(width);
        self.height.set(height);

        unsafe {
            let handle = winrt::get_resize_dcomp_sync_object(self.hwnd);

            if !(width == 0 || height == 0) {
                let new_surface = self
                    .dcomp
                    .CreateSurface(
                        width as u32,
                        height as u32,
                        sys::DXGI_FORMAT_B8G8R8A8_UNORM,
                        sys::DXGI_ALPHA_MODE_PREMULTIPLIED,
                    )
                    .unwrap();
                self.visual.SetContent(&new_surface).unwrap();
                *self.surface.borrow_mut() = new_surface;
                self.draw();
            }

            self.partner.synchronized_commit(handle).unwrap();
            winrt::signal_dcomp_resize_completion();
        }
    }

    pub fn draw(&self) {
        #[repr(C)]
        struct SimpleVertex {
            position: [f32; 3],
            color: [f32; 3],
        }

        let vertices = [
            SimpleVertex { position: [0.0, 0.5, 0.0],  color: [1.0, 0.0, 0.0] },
            SimpleVertex { position: [0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] },
            SimpleVertex { position: [-0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0] },
        ];

        let shader_code = b"
            struct VS_INPUT { 
                float3 Pos : POSITION; 
                float3 Color : COLOR; 
            };
            struct PS_INPUT { 
                float4 Pos : SV_POSITION; 
                float3 Color : COLOR; 
            };

            PS_INPUT VSMain(VS_INPUT input) {
                PS_INPUT output;
                output.Pos = float4(input.Pos, 1.0f);
                output.Color = input.Color;
                return output;
            }

            float4 PSMain(PS_INPUT input) : SV_Target {
                return float4(input.Color, 1.0f);
            }
        ";

        unsafe {
            let mut vs_blob = None;
            let mut ps_blob = None;

            sys::D3DCompile2(
                shader_code.as_ptr() as *const _,
                shader_code.len(),
                None,
                None,
                None,
                sys::s!("VSMain"),
                sys::s!("vs_4_0"),
                sys::D3DCOMPILE_DEBUG,
                0,
                0,
                None,
                0,
                &mut vs_blob,
                None,
            )
            .unwrap();
            sys::D3DCompile2(
                shader_code.as_ptr() as *const _,
                shader_code.len(),
                None,
                None,
                None,
                sys::s!("PSMain"),
                sys::s!("ps_4_0"),
                sys::D3DCOMPILE_DEBUG,
                0,
                0,
                None,
                0,
                &mut ps_blob,
                None,
            )
            .unwrap();

            let vs_blob = vs_blob.unwrap();
            let vs_data = std::slice::from_raw_parts(
                vs_blob.GetBufferPointer() as *const _,
                vs_blob.GetBufferSize(),
            );
            let ps_blob = ps_blob.unwrap();
            let ps_data = std::slice::from_raw_parts(
                ps_blob.GetBufferPointer() as *const _,
                ps_blob.GetBufferSize(),
            );

            let mut vertex_shader = None;
            let mut pixel_shader = None;
            let mut input_layout = None;
            let mut vertex_buffer = None;

            self.device
                .CreateVertexShader(vs_data, None, Some(&mut vertex_shader))
                .unwrap();
            self.device
                .CreatePixelShader(ps_data, None, Some(&mut pixel_shader))
                .unwrap();

            let element_desc = [
                sys::D3D11_INPUT_ELEMENT_DESC {
                    SemanticName: sys::s!("POSITION"),
                    SemanticIndex: 0,
                    Format: sys::DXGI_FORMAT_R32G32B32_FLOAT,
                    InputSlot: 0,
                    AlignedByteOffset: 0,
                    InputSlotClass: sys::D3D11_INPUT_PER_VERTEX_DATA,
                    InstanceDataStepRate: 0,
                },
                sys::D3D11_INPUT_ELEMENT_DESC {
                    SemanticName: sys::s!("COLOR"),
                    SemanticIndex: 0,
                    Format: sys::DXGI_FORMAT_R32G32B32_FLOAT,
                    InputSlot: 0,
                    AlignedByteOffset: 12,
                    InputSlotClass: sys::D3D11_INPUT_PER_VERTEX_DATA,
                    InstanceDataStepRate: 0,
                },
            ];
            self.device
                .CreateInputLayout(&element_desc, vs_data, Some(&mut input_layout))
                .unwrap();

            let buffer_desc = sys::D3D11_BUFFER_DESC {
                ByteWidth: std::mem::size_of_val(&vertices) as u32,
                Usage: sys::D3D11_USAGE_DEFAULT,
                BindFlags: sys::D3D11_BIND_VERTEX_BUFFER.0 as u32,
                CPUAccessFlags: 0,
                MiscFlags: 0,
                StructureByteStride: std::mem::size_of::<SimpleVertex>() as u32,
            };
            let subresource_data = sys::D3D11_SUBRESOURCE_DATA {
                pSysMem: vertices.as_ptr() as *const _,
                SysMemPitch: 0,
                SysMemSlicePitch: 0,
            };
            self.device
                .CreateBuffer(
                    &buffer_desc,
                    Some(&subresource_data),
                    Some(&mut vertex_buffer),
                )
                .unwrap();

            let stride = std::mem::size_of::<SimpleVertex>() as u32;
            let offset = 0;
            self.device_context.IASetVertexBuffers(
                0,
                1,
                Some(&vertex_buffer),
                Some(&stride),
                Some(&offset),
            );
            self.device_context.IASetInputLayout(input_layout.as_ref());
            self.device_context
                .VSSetShader(vertex_shader.as_ref(), None);
            self.device_context.PSSetShader(pixel_shader.as_ref(), None);

            let surface_ref = self.surface.borrow();
            let mut offset = sys::POINT::default();
            let back_buffer: sys::ID3D11Texture2D =
                surface_ref.BeginDraw(None, &mut offset).unwrap();

            let viewport = sys::D3D11_VIEWPORT {
                TopLeftX: offset.x as f32,
                TopLeftY: offset.y as f32,
                Width: self.width.get() as f32,
                Height: self.height.get() as f32,
                MinDepth: 0.0,
                MaxDepth: 1.0,
            };
            self.device_context.RSSetViewports(Some(&[viewport]));

            let mut rtv = None;
            self.device
                .CreateRenderTargetView(&back_buffer, None, Some(&mut rtv))
                .unwrap();

            let clear_color = [0.0, 0.2, 0.4, 1.0];
            self.device_context
                .ClearRenderTargetView(rtv.as_ref(), &clear_color);
            self.device_context.OMSetRenderTargets(Some(&[rtv]), None);

            self.device_context
                .IASetPrimitiveTopology(sys::D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
            self.device_context.Draw(3, 0);

            surface_ref.EndDraw().unwrap();
        }
    }
}
