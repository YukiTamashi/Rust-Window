pub mod definitions;
use definitions::*;
use std::ptr::{null, null_mut};

fn main() {
    let name = wide_str("window");
    let hInstance = unsafe{GetModuleHandleW(core::ptr::null())};
    let mut wc = register_window(name.clone(), hInstance.clone());
    let hwnd = make_window(name, hInstance);
    let _win = unsafe {ShowWindow(hwnd, SW_SHOW)};
    let mut msg = MSG::default();
    loop{
        let message = unsafe{GetMessageW(&mut msg, null_mut(), 0, 0)};
        if message == 0{
            break;
        }
        else if message == -1{
            panic!();
        }
        else {
            unsafe{TranslateMessage(&msg); 
            DispatchMessageW(&msg);}
        }
    }
}
