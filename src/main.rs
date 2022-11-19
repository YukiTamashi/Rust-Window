#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use triangle::win32::*;

fn main() {
    let name = "window";
    let cursor = cursor(IDCursor::Arrow).unwrap();
    let hwnd = create_window(name, cursor, window_procedure);
    let mut msg = Msg::default();
    loop {
        let is_done = update(&mut msg);
        if is_done {
            break;
        }
    }
}

//Custom window procedure.
//Can customize behaviour.
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
            let message = wide_str("Do you really want to exit?");
            let title = wide_str("Quit");
            let result = MessageBoxW(hWnd, message.as_ptr(), title.as_ptr(), MB_OKCANCEL);
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
            let mut ps = PaintStruct::default();
            let hdc = BeginPaint(hWnd, &mut ps);
            FillRect(hdc, ps.rcPaint(), (COLOR_WINDOW + 5) as HBrush);
            EndPaint(hWnd, &ps);
        }
        _ => return DefWindowProcW(hWnd, Msg, wParam, lParam),
    }
    0
}