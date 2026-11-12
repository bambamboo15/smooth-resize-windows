use crate::proj;
use crate::sys::{self, Interface};
use std::cell::Cell;

#[allow(dead_code, reason = "keep some objects alive")]
pub struct Context {
    pub compositor_context: proj::CompositorContext,
    target: sys::DesktopWindowTarget,
    visual: sys::SpriteVisual,
    surface: sys::CompositionDrawingSurface,
    surface_interop: sys::ICompositionDrawingSurfaceInterop,
    brush: sys::CompositionSurfaceBrush,
    width: Cell<u32>,
    height: Cell<u32>,
    hwnd: sys::HWND,

    pub resize_sync: sys::HANDLE,
    pub resize_queued: bool,
    pub between_frames: u32,
}

impl Context {
    pub fn new(hwnd: sys::HWND) -> Self {
        unsafe {
            proj::init_winrt_single_threaded();

            // Set up the WinRT compositor architecture.
            let compositor_context = proj::CompositorContext::new(hwnd);
            let desktop_interop: sys::ICompositorDesktopInterop =
                compositor_context.compositor.cast().unwrap();
            let target = desktop_interop
                .CreateDesktopWindowTarget(hwnd, false)
                .unwrap();
            let visual = compositor_context.compositor.CreateSpriteVisual().unwrap();
            let surface = compositor_context
                .graphics_device
                .CreateDrawingSurface(
                    sys::Size {
                        Width: 1.0,
                        Height: 1.0,
                    },
                    sys::DirectXPixelFormat::B8G8R8A8UIntNormalized,
                    sys::DirectXAlphaMode::Premultiplied,
                )
                .unwrap();
            let surface_interop = surface.cast().unwrap();
            let brush = compositor_context.compositor.CreateSurfaceBrush().unwrap();
            brush.SetSurface(&surface).unwrap();
            visual.SetBrush(&brush).unwrap();
            target.SetRoot(&visual).unwrap();

            Self {
                compositor_context,
                brush,
                visual,
                target,
                surface,
                surface_interop,
                width: Cell::new(0),
                height: Cell::new(0),
                hwnd,
                resize_sync: Default::default(),
                resize_queued: false,
                between_frames: 1,
            }
        }
    }

    pub fn update(&self) {
        let mut rect: sys::RECT = Default::default();
        unsafe { sys::GetClientRect(self.hwnd, &mut rect) }.unwrap();
        let width = (rect.right - rect.left) as u32;
        let height = (rect.bottom - rect.top) as u32;

        self.width.set(width);
        self.height.set(height);
        if width > 0 && height > 0 {
            self.surface
                .Resize(sys::SizeInt32 {
                    Width: width as i32,
                    Height: height as i32,
                })
                .unwrap();
            self.visual
                .SetSize(windows_numerics::Vector2 {
                    X: width as f32,
                    Y: height as f32,
                })
                .unwrap();
        }

        self.draw();
    }

    fn draw(&self) {
        unsafe {
            let mut offset = sys::POINT::default();
            let context: sys::ID2D1DeviceContext =
                self.surface_interop.BeginDraw(None, &mut offset).unwrap();

            context.SetTransform(&windows_numerics::Matrix3x2::translation(
                offset.x as f32,
                offset.y as f32,
            ));
            context.Clear(Some(&sys::D2D1_COLOR_F {
                r: 0.0,
                g: 0.2,
                b: 0.4,
                a: 1.0,
            }));
            let brush = context
                .CreateSolidColorBrush(
                    &sys::D2D1_COLOR_F {
                        r: 0.2,
                        g: 0.8,
                        b: 1.0,
                        a: 1.0,
                    },
                    None,
                )
                .unwrap();
            context.FillRoundedRectangle(
                &sys::D2D1_ROUNDED_RECT {
                    rect: sys::D2D_RECT_F {
                        left: 40.0,
                        top: 40.0,
                        right: self.width.get() as f32 - 40.0,
                        bottom: self.height.get() as f32 - 40.0,
                    },
                    radiusX: 16.0,
                    radiusY: 16.0,
                },
                &brush,
            );

            drop(brush);
            drop(context);
            self.surface_interop.EndDraw().unwrap();
        }
    }
}
