#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use triangle::win32::*;
use std::ptr::{null_mut};

fn main() {
    let name = wide_str("window");
    let hinstance = unsafe{GetModuleHandleW(core::ptr::null())};
    register_window(name.clone(), hinstance);
    let hwnd = make_window(name, hinstance);
    unsafe {ShowWindow(hwnd, SW_SHOW)};
    let mut msg = Msg::default();
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
