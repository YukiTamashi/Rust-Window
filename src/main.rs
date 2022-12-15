#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use triangle::win32::*;
use windows::Win32::UI::WindowsAndMessaging::MSG;

fn main() {
    let name = "window";
    let cursor = cursor(IDCursor::Arrow).unwrap();
    create_window(name, cursor);
    let mut msg = MSG::default();
    loop {
        if update(&mut msg) {
            break;
        }
    }
}

