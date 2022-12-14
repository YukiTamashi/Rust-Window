#[cfg(windows)]
pub mod win32 {
    use windows::Win32::{UI::WindowsAndMessaging::*, Foundation::*, Graphics::Gdi::*, System::LibraryLoader::GetModuleHandleW};

    #[derive(Debug)]
    pub struct CursorError;

    /// Cursor types, required for [cursor]
    pub enum IDCursor {
        /// Standard arrow and small hourglass
        AppStarting = 32650,
        /// Standard arrow
        Arrow = 32512,
        /// Crosshair
        Cross = 32515,
        /// Hand
        Hand = 32649,
        /// Arrow and question mark
        Help = 32651,
        /// I-beam
        IBeam = 32513,
        /// Slashed circle
        No = 32648,
        /// Four-pointed arrow pointing north, south, east, and west
        SizeAll = 32646,
        /// Double-pointed arrow pointing northeast and southwest
        SizeNeSw = 32643,
        /// Double-pointed arrow pointing north and south
        SizeNS = 32645,
        /// Double-pointed arrow pointing northwest and southeast
        SizeNwSe = 32642,
        /// Double-pointed arrow pointing west and east
        SizeWE = 32644,
        /// Vertical arrow
        UpArrow = 32516,
        /// Hourglass
        Wait = 32514,
    }

    fn register_window(
        name: &str,
        h_instance: HINSTANCE,
        hCursor: HCURSOR,
    ) -> WNDCLASSW {
        let wc = WNDCLASSW {
            lpfnWndProc: Some(window_procedure),
            hInstance: h_instance,
            lpszClassName: (&windows::core::HSTRING::from(name)).into(),
            hCursor,
            ..Default::default()
        };
        let atom = unsafe { RegisterClassW(&wc) };
        if atom == 0 {
            let last_err = unsafe { GetLastError() };
            panic!("Could not register the window, error code: {:?}", last_err);
        }
        wc
    }

    fn make_window(name: &str, h_instance: HINSTANCE) -> HWND {
        let hwnd = unsafe {
            CreateWindowExW(
                WINDOW_EX_STYLE(0),
                windows::core::PCWSTR::from(&windows::core::HSTRING::from(name)),
                windows::core::PCWSTR::from(&windows::core::HSTRING::from(name)),
                WS_OVERLAPPEDWINDOW,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                HWND::default(),
                HMENU::default(),
                h_instance,
                None,
            )
        };
        hwnd
    }

    
    fn get_handle() ->  HINSTANCE{
        unsafe { GetModuleHandleW(windows::core::PCWSTR::null()).unwrap_or_default() }
    }

    //Handles entire creation of the window.
    //Returns window handle.
    pub fn create_window(name: &str, hCursor: HCURSOR) -> HWND {
        let hinstance = get_handle();
        register_window(name, hinstance, hCursor);
        let hwnd = make_window(name, hinstance);
        unsafe { ShowWindow(hwnd, SW_SHOW) };
        unsafe{ SetWindowTextW::<HWND, windows::core::PCWSTR>(hwnd, (&windows::core::HSTRING::from(name)).into()) };
        hwnd
    }

    //Returns cursor handle based on type called.
    pub fn cursor(name: IDCursor) -> Result<HCURSOR, windows::core::Error> {
        let cursor = unsafe { LoadCursorW(HINSTANCE::default(), windows::core::PCWSTR::from_raw((name as u16) as *const u16))};
        cursor
    }

    /// Resolve runtime updates to window. Should be enclosed in a loop
    /// Returns true if close command was called.
    pub fn update(msg: &mut MSG) -> bool {
        let message = unsafe { GetMessageW(msg, HWND::default(), 0, 0) };
        if message.0 == 0 {
            return true;
        } else if message.0 == -1 {
            panic!();
        } else {
            unsafe {
                TranslateMessage(msg);
                DispatchMessageW(msg);
            }
        }
        false
    }
    unsafe extern "system" fn window_procedure(
        hWnd: HWND,
        Msg: u32,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> LRESULT {
        match Msg {
            WM_NCCREATE => {
                SetWindowLongPtrW(hWnd, GWLP_USERDATA, LPARAM as isize);
                return LRESULT(1);
            }
            WM_CLOSE => {
                let message = "Do you really want to exit?";
                let title = "Quit";
                let result = MessageBoxW(hWnd, &windows::core::HSTRING::from(message), &windows::core::HSTRING::from(title), MB_OKCANCEL);
                if result == IDOK {
                    DestroyWindow(hWnd);
                } else {
                    return LRESULT(0);
                }
            }
            WM_DESTROY => {
                let ptr = GetWindowLongPtrW(hWnd, GWLP_USERDATA) as *mut i32;
                drop(Box::from_raw(ptr));
                PostQuitMessage(0);
            }
            WM_PAINT => {
                let mut ps = PAINTSTRUCT::default();
                let hdc = BeginPaint(hWnd, &mut ps);
                FillRect(hdc, &ps.rcPaint, HBRUSH((COLOR_WINDOW.0 + 5) as isize));
                EndPaint(hWnd, &ps);
            }
            _ => return DefWindowProcW(hWnd, Msg, wParam, lParam),
        }
        LRESULT(0)
    }
}
