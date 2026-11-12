mod context;
mod proj;
mod sys {
    pub use windows::{
        Foundation::*, Graphics::DirectX::*, Graphics::*, System::*,
        UI::Composition::Desktop::DesktopWindowTarget, UI::Composition::*, Win32::Foundation::*,
        Win32::Graphics::Direct2D::Common::*, Win32::Graphics::Direct2D::*,
        Win32::Graphics::Direct3D::*, Win32::Graphics::Direct3D11::*, Win32::Graphics::Dwm::*,
        Win32::Graphics::Dxgi::*, Win32::Graphics::Gdi::*, Win32::System::Com::*,
        Win32::System::LibraryLoader::*, Win32::System::WinRT::Composition::*,
        Win32::System::WinRT::*, Win32::UI::WindowsAndMessaging::*, core::*,
    };
}

unsafe extern "system" fn window_proc(
    hwnd: sys::HWND,
    umsg: u32,
    wparam: sys::WPARAM,
    lparam: sys::LPARAM,
) -> sys::LRESULT {
    unsafe {
        const WM_APP_RESIZE_RENDER: u32 = sys::WM_USER + 1;
        let context_ptr =
            sys::GetWindowLongPtrW(hwnd, sys::WINDOW_LONG_PTR_INDEX(0)) as *mut context::Context;

        match umsg {
            sys::WM_CLOSE => {
                sys::PostQuitMessage(0);
                sys::LRESULT(0)
            }
            sys::WM_WINDOWPOSCHANGED => {
                let pos = &*(lparam.0 as *const sys::WINDOWPOS);
                if pos.flags.0 & sys::SWP_NOSIZE.0 != 0 {
                    return sys::LRESULT(0);
                }

                if let Some(context) = context_ptr.as_mut() {
                    let sync = proj::get_resize_dcomp_sync_object(hwnd);
                    if !context.resize_sync.is_invalid() {
                        sys::CloseHandle(context.resize_sync).unwrap();
                    }
                    context.resize_sync = sync;

                    if !context.resize_queued {
                        context.resize_queued = true;

                        sys::PostMessageW(
                            Some(hwnd),
                            WM_APP_RESIZE_RENDER,
                            sys::WPARAM(0),
                            sys::LPARAM(0),
                        )
                        .unwrap();
                    }
                }
                sys::LRESULT(0)
            }
            sys::WM_SIZE | sys::WM_PAINT => {
                let mut ps = sys::PAINTSTRUCT::default();
                sys::BeginPaint(hwnd, &mut ps);
                if let Some(context) = context_ptr.as_mut() {
                    let between_frames = context.between_frames;
                    context.between_frames = 0;
                    context.update();
                    context.between_frames = between_frames;
                }
                sys::EndPaint(hwnd, &ps).unwrap();
                sys::DwmFlush().unwrap();
                sys::LRESULT(0)
            }
            WM_APP_RESIZE_RENDER => {
                if let Some(context) = context_ptr.as_mut()
                    && context.resize_queued
                {
                    let sync = context.resize_sync;
                    context.resize_sync = Default::default();
                    context.resize_queued = false;

                    if context.between_frames > 0 {
                        context.between_frames = 0;
                        context.update();
                        context.between_frames = 1;
                        sys::ValidateRect(Some(hwnd), None).unwrap();
                    }

                    context.partner.synchronized_commit(sync).unwrap();
                    if !sync.is_invalid() {
                        sys::CloseHandle(sync).unwrap();
                    }
                    proj::layout_completed(hwnd);
                }
                sys::LRESULT(0)
            }

            _ => sys::DefWindowProcW(hwnd, umsg, wparam, lparam),
        }
    }
}

fn main() {
    unsafe {
        let hinstance: sys::HINSTANCE = sys::GetModuleHandleW(None).unwrap().into();
        let wc = sys::WNDCLASSW {
            lpfnWndProc: Some(window_proc),
            hInstance: hinstance,
            hCursor: sys::LoadCursorW(None, sys::IDC_ARROW).unwrap(),
            lpszClassName: sys::w!("Window"),
            cbClsExtra: 0,
            cbWndExtra: std::mem::size_of::<*const ()>() as i32,
            ..Default::default()
        };
        sys::RegisterClassW(&wc);

        let hwnd = sys::CreateWindowExW(
            sys::WS_EX_NOREDIRECTIONBITMAP,
            wc.lpszClassName,
            sys::w!("Smooth resizing on Windows 11"),
            sys::WS_OVERLAPPEDWINDOW,
            sys::CW_USEDEFAULT,
            sys::CW_USEDEFAULT,
            sys::CW_USEDEFAULT,
            sys::CW_USEDEFAULT,
            None,
            None,
            Some(hinstance),
            None,
        )
        .unwrap();

        let context = context::Context::new(hwnd);
        sys::SetWindowLongPtrW(
            hwnd,
            sys::WINDOW_LONG_PTR_INDEX(0),
            &context as *const _ as isize,
        );

        let _ = sys::ShowWindow(hwnd, sys::SW_SHOWNORMAL);

        let mut msg: sys::MSG = Default::default();
        while sys::GetMessageW(&mut msg, None, 0, 0).as_bool() {
            let _ = sys::TranslateMessage(&msg);
            sys::DispatchMessageW(&msg);
        }
    }
}
