use std::ffi::{c_void};
use std::ptr::{null, null_mut};

macro_rules! unsafe_impl_default_zeroed {
    ($t:ty) => {
        impl Default for $t {
            #[inline]
            #[must_use]
            fn default() -> Self {
                unsafe {core::mem::zeroed()}
            }
        }
    };
}

const WS_OVERLAPPED: u32 = 0x00000000;
const WS_CAPTION: u32 = 0x00C00000;
const WS_SYSMENU: u32 = 0x00080000;
const WS_THICKFRAME: u32 = 0x00040000;
const WS_MINIMIZEBOX: u32 = 0x00020000;
const WS_MAXIMIZEBOX: u32 = 0x00010000;
const WS_OVERLAPPEDWINDOW: u32 = WS_OVERLAPPED
  | WS_CAPTION
  | WS_SYSMENU
  | WS_THICKFRAME
  | WS_MINIMIZEBOX
  | WS_MAXIMIZEBOX;
const CW_USEDEFAULT: c_int = 0x80000000_u32 as c_int;
const SW_SHOW: c_int = 5;

type ATOM = WORD;
type BOOL = c_int;
type c_int = i32;
type c_uint = u32;
type DWORD = u32;
type HANDLE = *mut c_void;
type HBRUSH = HANDLE;
type HCURSOR = HICON;
type HICON = HANDLE;
type HINSTANCE = HANDLE;
type HMENU = HANDLE;
type HMODULE = HINSTANCE;
type HWND = HANDLE;
type int = c_int;
type LONG = i32;
type LONG_PTR = isize;
type LPARAM = LONG_PTR;
type LPMSG = *mut MSG;
type LPVOID = *mut c_void;
type LPCWSTR = *const wchar_t;
type LRESULT = LONG_PTR;
type UINT = c_uint;
type UINT_PTR = usize;
type wchar_t = u16;
type WPARAM = UINT_PTR;
type WORD = u16;

type WNDPROC = Option<
    unsafe extern "system" fn(
        hwnd: HWND,
        uMsg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> LRESULT,
>;


#[repr(C)]
pub struct WNDCLASSW {
  style: UINT,
  lpfnWndProc: WNDPROC,
  cbClsExtra: int,
  cbWndExtra: int,
  hInstance: HINSTANCE,
  hIcon: HICON,
  hCursor: HCURSOR,
  hbrBackground: HBRUSH,
  lpszMenuName: LPCWSTR,
  lpszClassName: LPCWSTR,
}

pub struct MSG {
    hwnd: HWND,
    message: UINT,
    wParam: WPARAM,
    lParam: LPARAM,
    time: DWORD,
    pt: POINT,
    lPrivate: DWORD,
}

pub struct POINT {
    x: LONG,
    y: LONG,
}

unsafe_impl_default_zeroed!(WNDCLASSW);
unsafe_impl_default_zeroed!(MSG);


#[link(name = "Kernel32")]
extern "system" {
    pub fn GetModuleHandleW (lpModuleName: LPCWSTR) -> HMODULE;
    pub fn GetLastError() -> DWORD;
    pub fn CreateWindowExW(
        dwExStyle: DWORD,
        lpClassName: LPCWSTR,
         lpWindowName: LPCWSTR,
         dwStyle: DWORD,
          X: int,
          Y: int,
          nWidth: int,
         nHeight: int,
         hWndParent: HWND,
         hMenu: HMENU,
         hInstance: HINSTANCE,
         lpParam: LPVOID
    ) -> HWND;
}

#[link(name = "User32")]
extern "system" {
    pub fn ShowWindow(
        hWnd: HWND,
        nCmdShow: int
    ) -> BOOL;
}

#[link(name = "User32")]
extern "system" {
    pub fn RegisterClassW(lpWndClass: *const WNDCLASSW) -> ATOM;
    pub fn DefWindowProcW(
      hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM,
    ) -> LRESULT;
    pub fn GetMessageW(
      lpMsg: LPMSG,
      hWnd: HWND,
      wMsgFilterMin: UINT,
      wMsgFilterMax: UINT
    ) -> BOOL;
    pub fn TranslateMessage(
        lpMsg: *const MSG
    ) -> BOOL;
    pub fn DispatchMessageW(
        lpMsg: *const MSG
    ) -> LRESULT;
}

#[link(name = "Kernel32")]
extern "system" {
}


fn main() {
    let name = wide_str("window");
    let hInstance = unsafe{GetModuleHandleW(core::ptr::null())};
    let mut wc = register_window(name.clone(), hInstance);
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

pub fn wide_str(str: &str) -> Vec<u16> {
    str.encode_utf16().chain(Some(0)).collect()
}

pub fn make_window(name: Vec<u16>,hInstance: *mut c_void) -> HWND{
    let hwnd = unsafe{CreateWindowExW(
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
        unsafe{hInstance},   
        null_mut(),
        )};
    if hwnd.is_null(){
        panic!("Failed to create a window");
    }
    hwnd
}

pub fn register_window(name: Vec<u16>,hInstance: *mut c_void) -> WNDCLASSW{
    let mut wc = WNDCLASSW::default();
    wc.lpfnWndProc = Some(DefWindowProcW);
    wc.hInstance = hInstance;
    wc.lpszClassName = name.as_ptr();
    let atom = unsafe {RegisterClassW(&wc)};
    if atom == 0{
        let last_err = unsafe {GetLastError()};
        panic!("Could not register the window, error code: {}", last_err);
    }
    wc
}