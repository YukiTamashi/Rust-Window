

#[cfg(windows)]
pub mod win32 {
    use windows::Win32::{UI::WindowsAndMessaging::*, System::LibraryLoader::*, Foundation::*, Graphics::Gdi::*};

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
        name: Vec<u16>,
        h_instance: *mut c_void,
        hCursor: HCursor,
        procedure: fn(HWnd, UInt, WParam, LParam) -> LResult,
    ) -> WNDClassW {
        let wc = WNDClassW {
            lpfnWndProc: Some(procedure),
            hInstance: h_instance,
            lpszClassName: name.as_ptr(),
            hCursor,
            ..Default::default()
        };
        let atom = unsafe { RegisterClassW(&wc) };
        if atom == 0 {
            let last_err = unsafe { GetLastError() };
            panic!("Could not register the window, error code: {}", last_err);
        }
        wc
    }

    fn make_window(name: Vec<u16>, h_instance: *mut c_void) -> HWnd {
        let lparam: *mut i32 = Box::leak(Box::new(5_i32));
        let hwnd = unsafe {
            CreateWindowExW(
                0,
                name.as_ptr(),
                name.as_ptr(),
                WS_OVERLAPPEDWINDOW,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                null_mut(),
                null_mut(),
                unsafe { h_instance },
                lparam.cast(),
            )
        };
        if hwnd.is_null() {
            panic!("Failed to create a window");
        }
        hwnd
    }

    //Handles entire creation of the window.
    //Returns window handle.
    pub fn create_window(name: &str, hCursor: HCursor, procedure: fn(HWnd, UInt, WParam, LParam) -> LResult) -> HWnd {
        let hinstance = get_handle();
        register_window(wide_str(name), hinstance, hCursor, procedure);
        let hwnd = make_window(wide_str(name), hinstance);
        unsafe { ShowWindow(hwnd, SW_SHOW) };
        unsafe{ SetWindowTextW(hwnd, wide_str(name).as_ptr()) };
        hwnd
    }

    //Returns cursor handle based on type called.
    pub fn cursor(name: IDCursor) -> Result<HCursor, CursorError> {
        let cursor = unsafe { LoadCursorW(null_mut(), unsafe { MakeIntResourceW(name as Word) }) };
        if cursor.is_null() {
            Err(CursorError)
        } else {
            Ok(cursor)
        }
    }

    //Converts into UTF-8 string.
    fn wide_str(str: &str) -> Vec<u16> {
        str.encode_utf16().chain(Some(0)).collect()
    }

    /// Resolve runtime updates to window. Should be enclosed in a loop
    /// Returns true if close command was called.
    pub fn update(msg: &mut Msg) -> bool {
        let message = unsafe { GetMessageW(msg, null_mut(), 0, 0) };
        if message == 0 {
            return true;
        } else if message == -1 {
            panic!();
        } else {
            unsafe {
                TranslateMessage(msg);
                DispatchMessageW(msg);
            }
        }
        false
    }
    pub fn procedure() -> fn(HWnd, UInt, WParam, LParam) -> LResult{
        |x, y, z, a|unsafe{window_procedure(x, y, z, a)} 
    }
    unsafe extern "system" fn window_procedure(
        hWnd: HWnd,
        Msg: UInt,
        wParam: WParam,
        lParam: LParam,
    ) -> LResult {
        match Msg {
            WM_NCCREATE => {
                let createstruct: *mut CreateStructW = lParam as *mut _;
                if createstruct.is_null() {
                    return 0;
                }
                let boxed_i32_ptr: *mut i32 = (*createstruct).lpCreateParams().cast();
                SetWindowLongPtrW(hWnd, GWLP_USERDATA, boxed_i32_ptr as LongPtr);
                return 1;
            }
            WM_CLOSE => {
                let message = "Do you really want to exit?";
                let title = "Quit";
                let result = MessageBoxW(hWnd, message.into(), title.into(), MB_OKCANCEL);
                if result == IDOK {
                    DestroyWindow(hWnd);
                } else {
                    return 0;
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
                FillRect(hdc, &ps.rcPaint, (COLOR_WINDOW.into() + 5) as HBRUSH);
                EndPaint(hWnd, &ps);
            }
            _ => return DefWindowProcW(hWnd, Msg, wParam, lParam),
        }
        0
    }
}
