#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use triangle::win32::*;

fn main() {
    let name = "window";
    let cursor = cursor(IDCursor::Arrow).unwrap();
    let hwnd = create_window(name, cursor, procedure());
    let mut msg = Msg::default();
    loop {
        let is_done = update(&mut msg);
        if is_done {
            break;
        }
    }
}

