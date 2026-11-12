use crate::sys::{self, Interface};
use std::sync::OnceLock;

/// Initialize WinRT as single-threaded.
pub unsafe fn init_winrt_single_threaded() {
    unsafe {
        sys::CoInitializeEx(
            None,
            sys::COINIT_APARTMENTTHREADED | sys::COINIT_DISABLE_OLE1DDE,
        )
        .unwrap();
        sys::RoInitialize(sys::RO_INIT_SINGLETHREADED).unwrap();
    }
}

type InternalApis = (
    /* GetResizeDCompositionSynchronizationObject */
    unsafe extern "system" fn(sys::HWND, *mut sys::HANDLE) -> sys::BOOL,
    /* EnableResizeLayoutSynchronization */
    unsafe extern "system" fn(sys::HWND, sys::BOOL) -> sys::BOOL,
    /* LayoutCompleted */
    unsafe extern "system" fn(sys::HWND) -> sys::BOOL,
);

fn get_apis() -> &'static InternalApis {
    static APIS: OnceLock<InternalApis> = OnceLock::new();
    APIS.get_or_init(|| unsafe {
        let dll = sys::GetModuleHandleW(sys::w!("user32"))
            .or_else(|_| sys::LoadLibraryExW(sys::w!("user32"), None, sys::LOAD_LIBRARY_FLAGS(0)))
            .unwrap();
        let [p1, p2, p3] = [2614, 2615, 2538]
            .map(|ordinal| sys::GetProcAddress(dll, sys::PCSTR(ordinal as *const u8)).unwrap());
        (
            std::mem::transmute(p1),
            std::mem::transmute(p2),
            std::mem::transmute(p3),
        )
    })
}

#[sys::interface("E01EB649-787E-4560-B398-0DE7A2065D8B")]
unsafe trait IDCompositionDesktopDevicePartner6: sys::IUnknown {
    unsafe fn __commit(&self);
    unsafe fn __wait_for_commit_completion(&self);
    unsafe fn __get_frame_statistics(&self);
    unsafe fn __create_visual(&self);
    unsafe fn __create_surface_factory(&self);
    unsafe fn __create_surface(&self);
    unsafe fn __create_virtual_surface(&self);
    unsafe fn __create_translate_transform(&self);
    unsafe fn __create_scale_transform(&self);
    unsafe fn __create_rotate_transform(&self);
    unsafe fn __create_skew_transform(&self);
    unsafe fn __create_matrix_transform(&self);
    unsafe fn __create_transform_group(&self);
    unsafe fn __create_translate_transform3_d(&self);
    unsafe fn __create_scale_transform3_d(&self);
    unsafe fn __create_rotate_transform3_d(&self);
    unsafe fn __create_matrix_transform3_d(&self);
    unsafe fn __create_transform3_dgroup(&self);
    unsafe fn __create_effect_group(&self);
    unsafe fn __create_rectangle_clip(&self);
    unsafe fn __create_animation(&self);
    unsafe fn __create_target_for_hwnd(&self);
    unsafe fn __create_surface_from_handle(&self);
    unsafe fn __create_surface_from_hwnd(&self);
    unsafe fn __create_shared_resource(&self);
    unsafe fn __open_shared_resource_handle(&self);
    unsafe fn __open_shared_resource_1(&self);
    unsafe fn __create_animation_trigger(&self);
    unsafe fn __create_gradient_surface(&self);
    unsafe fn __enable_white_pixel_optimization(&self);
    unsafe fn __offer_surface_resources(&self);
    unsafe fn __reclaim_surface_resources(&self);
    unsafe fn __disable_d2_dstate_preservation(&self);
    unsafe fn __flush(&self);
    unsafe fn __create_surface_from_visual_snapshot(&self);
    unsafe fn __create_interaction(&self);
    unsafe fn __create_touch_interaction_configuration(&self);
    unsafe fn __create_touchpad_interaction_configuration(&self);
    unsafe fn __create_pen_interaction_configuration(&self);
    unsafe fn __create_mouse_interaction_configuration(&self);
    unsafe fn __open_shared_resource_2(&self);
    unsafe fn __create_desktop_tree(&self);
    unsafe fn __get_current_batch_id(&self);
    unsafe fn __get_last_confirmed_batch_id(&self);
    unsafe fn __create_gaussian_blur_effect(&self);
    unsafe fn __create_manipulation_transform(&self);
    unsafe fn __create_brightness_effect(&self);
    unsafe fn __create_color_matrix_effect(&self);
    unsafe fn __create_shadow_effect(&self);
    unsafe fn __create_hue_rotation_effect(&self);
    unsafe fn __create_saturation_effect(&self);
    unsafe fn __create_turbulence_effect(&self);
    unsafe fn __create_linear_transfer_effect(&self);
    unsafe fn __create_table_transfer_effect(&self);
    unsafe fn __register_callback_thread(&self);
    unsafe fn __create_composite_effect(&self);
    unsafe fn __snap_animation_read_back_time(&self);
    unsafe fn __get_max_texture_size(&self);
    unsafe fn __create_blend_effect(&self);
    unsafe fn __create_arithmetic_composite_effect(&self);
    unsafe fn __create_affine_transform2_deffect(&self);
    unsafe fn __set_composition_primitive_group_renderer_enabled(&self);
    unsafe fn __create_render_target(&self);
    unsafe fn __create_ddisplay_render_target(&self);
    unsafe fn __create_cursor_visual(&self);
    unsafe fn __create_mousewheel_interaction_configuration(&self);
    pub unsafe fn synchronized_commit(&self, hobject: sys::HANDLE) -> sys::HRESULT;
    unsafe fn __create_visual_reference_controller(&self);
    unsafe fn __create_window_target(&self);
}

#[sys::interface("22118ADF-23F1-4801-BCFA-66CBF48CC51B")]
unsafe trait IInteropCompositorFactoryPartner: sys::IUnknown {
    // Manually provide the `sys::IInspectable` vtable as `sys::interface` does not support it.
    unsafe fn get_iids(&self, count: *mut u32, values: *mut *mut sys::GUID) -> sys::HRESULT;
    unsafe fn get_runtime_class_name(&self, value: *mut *mut std::ffi::c_void) -> sys::HRESULT;
    unsafe fn get_trust_level(&self, value: *mut i32) -> sys::HRESULT;

    unsafe fn create_interop_compositor(
        &self,
        arg1: *mut std::ffi::c_void,
        arg2: *mut std::ffi::c_void,
        riid: *const sys::GUID,
        result: *mut *mut std::ffi::c_void,
    ) -> sys::HRESULT;
}

pub struct CompositorContext {
    pub compositor: sys::Compositor,
    #[allow(unused)]
    dispatcher_queue: sys::DispatcherQueueController,
    #[allow(unused)]
    d2d_device: sys::ID2D1Device,
    pub graphics_device: sys::CompositionGraphicsDevice,
    partner: IDCompositionDesktopDevicePartner6,
    hwnd: sys::HWND,
}

impl CompositorContext {
    pub unsafe fn new(hwnd: sys::HWND) -> Self {
        let child = unsafe {
            sys::CreateWindowExW(
                sys::WINDOW_EX_STYLE(0),
                sys::w!("STATIC"),
                sys::w!(""),
                sys::WS_CHILD,
                0,
                0,
                0,
                0,
                Some(hwnd),
                None,
                None,
                None,
            )
        }
        .unwrap();

        let (_, __enable_resize_layout_synchronization, _) = get_apis();
        unsafe { __enable_resize_layout_synchronization(hwnd, sys::TRUE) }.unwrap();
        unsafe { __enable_resize_layout_synchronization(child, sys::TRUE) }.unwrap();

        let dispatcher_queue = unsafe {
            sys::CreateDispatcherQueueController(sys::DispatcherQueueOptions {
                dwSize: std::mem::size_of::<sys::DispatcherQueueOptions>() as u32,
                threadType: sys::DQTYPE_THREAD_CURRENT,
                apartmentType: sys::DQTAT_COM_STA,
            })
        }
        .unwrap();

        let factory: IInteropCompositorFactoryPartner =
            unsafe { sys::RoGetActivationFactory(sys::h!("Windows.UI.Composition.Compositor")) }
                .unwrap();

        let mut compositor = std::ptr::null_mut();
        unsafe {
            factory.create_interop_compositor(
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &sys::Compositor::IID,
                &mut compositor,
            )
        }
        .unwrap();
        let compositor = unsafe { sys::Compositor::from_raw(compositor) };

        let mut device: Option<sys::ID3D11Device> = None;
        unsafe {
            sys::D3D11CreateDevice(
                None,
                sys::D3D_DRIVER_TYPE_HARDWARE,
                Default::default(),
                sys::D3D11_CREATE_DEVICE_BGRA_SUPPORT,
                None,
                sys::D3D11_SDK_VERSION,
                Some(&mut device),
                None,
                None,
            )
        }
        .unwrap();
        let dxgi_device: sys::IDXGIDevice = device.unwrap().cast().unwrap();
        let d2d_factory: sys::ID2D1Factory1 =
            unsafe { sys::D2D1CreateFactory(sys::D2D1_FACTORY_TYPE_SINGLE_THREADED, None) }
                .unwrap();
        let d2d_device: sys::ID2D1Device =
            unsafe { d2d_factory.CreateDevice(&dxgi_device) }.unwrap();
        let interop: sys::ICompositorInterop = compositor.cast().unwrap();
        let graphics_device = unsafe { interop.CreateGraphicsDevice(&d2d_device) }.unwrap();

        let partner: IDCompositionDesktopDevicePartner6 = compositor.cast().unwrap();

        Self {
            compositor,
            dispatcher_queue,
            d2d_device,
            graphics_device,
            partner,
            hwnd,
        }
    }

    /// Obtains the synchronization handle for this window.
    pub unsafe fn get_sync_handle(&self) -> sys::HANDLE {
        let mut handle: sys::HANDLE = Default::default();
        let (__get_resize_dcomposition_synchronization_object, _, _) = get_apis();
        unsafe { __get_resize_dcomposition_synchronization_object(self.hwnd, &mut handle) }.unwrap();
        handle
    }

    /// Signal to the compositor that resize has been completed.
    pub unsafe fn layout_completed(&self) {
        let (_, _, __layout_completed) = get_apis();
        unsafe { __layout_completed(self.hwnd) }.unwrap();
    }

    /// Perform a synchronized commit.
    pub unsafe fn synchronized_commit(&self, handle: sys::HANDLE) {
        unsafe { self.partner.synchronized_commit(handle) }.unwrap();
    }
}
