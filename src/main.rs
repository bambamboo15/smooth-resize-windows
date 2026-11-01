mod context;
mod winrt;
mod sys {
    pub use windows::{
        UI::Core::*, Win32::Foundation::*, Win32::Graphics::Direct3D::Fxc::*,
        Win32::Graphics::Direct3D::*, Win32::Graphics::Direct3D11::*,
        Win32::Graphics::DirectComposition::*, Win32::Graphics::Dxgi::Common::*,
        Win32::Graphics::Dxgi::*, Win32::System::Com::*, Win32::System::LibraryLoader::*,
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
        match umsg {
            sys::WM_CLOSE => {
                sys::PostQuitMessage(0);
                sys::LRESULT(0)
            }
            sys::WM_SIZE => {
                let width = (lparam.0 as usize & 0xFFFF) as u16;
                let height = ((lparam.0 as usize >> 16) & 0xFFFF) as u16;

                let ptr = sys::GetWindowLongPtrW(hwnd, sys::WINDOW_LONG_PTR_INDEX(0));
                if ptr != 0 {
                    let context: &context::Context = &*(ptr as *const _);
                    context.resize(width as u32, height as u32);
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
            lpszClassName: sys::w!("D3DWindow"),
            cbClsExtra: 0,
            cbWndExtra: std::mem::size_of::<*const ()>() as i32,
            ..Default::default()
        };
        sys::RegisterClassW(&wc);

        let hwnd = sys::CreateWindowExW(
            sys::WS_EX_NOREDIRECTIONBITMAP,
            wc.lpszClassName,
            sys::w!("A Never Flickering DirectX Window"),
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
