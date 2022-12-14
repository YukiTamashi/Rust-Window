#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use triangle::win32::*;
use windows::Win32::UI::WindowsAndMessaging::MSG;

fn main() {
    let name = "window";
    let cursor = cursor(IDCursor::Arrow).unwrap();
    let hwnd = create_window(name, cursor);
    let mut msg = MSG::default();
    loop {
        let is_done = update(&mut msg);
        if is_done {
            break;
        }
    }
}

