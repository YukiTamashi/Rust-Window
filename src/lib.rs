#[cfg(windows)]
pub mod win32{

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
pub const WS_THICKFRAME: u32 = 0x00040000;
pub const WS_MINIMIZEBOX: u32 = 0x00020000;
pub const WS_MAXIMIZEBOX: u32 = 0x00010000;
pub const WS_OVERLAPPEDWINDOW: u32 = WS_OVERLAPPED
  | WS_CAPTION
  | WS_SYSMENU
  | WS_THICKFRAME
  | WS_MINIMIZEBOX
  | WS_MAXIMIZEBOX;
pub const CW_USEDEFAULT: c_int = 0x80000000_u32 as c_int;
pub const SW_SHOW: c_int = 5;
pub const IDC_ARROW: LPCWSTR = MAKEINTRESOURCEW(32512);
pub const WM_CLOSE: u32 = 0x0010;
pub const WM_DESTROY: u32 = 0x0002;
const WM_CREATE: u32 = 0x0001;

const WM_NCCREATE: u32 = 0x0081;

const WM_PAINT: u32 = 0x000F;
const COLOR_WINDOW:u32 = 5;
const MB_OKCANCEL: u32 = 0x00000001;
const IDOK: c_int = 1;
const GWLP_USERDATA: i32 = -21;
const WM_SETCURSOR: u32 = 0x0020;


type ATOM = WORD;
type BOOL = c_int;
type c_int = i32;
type c_uint = u32;
type DWORD = u32;
type HANDLE = *mut c_void;
type HBRUSH = HANDLE;
type HCURSOR = HICON;
type HDC = HANDLE;
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
type LPWSTR = *mut WCHAR;
type LPCWSTR = *const wchar_t;
type LRESULT = LONG_PTR;
type UINT = c_uint;
type UINT_PTR = usize;
type ULONG_PTR = usize;
type wchar_t = u16;
type WCHAR = wchar_t;
type WPARAM = UINT_PTR;
type WORD = u16;

type WNDPROC = Option<
    unsafe extern "system" fn(
        hwnd: HWND,
        Msg: UINT,
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

#[repr(C)]
pub struct MSG {
    hwnd: HWND,
    message: UINT,
    wParam: WPARAM,
    lParam: LPARAM,
    time: DWORD,
    pt: POINT,
    lPrivate: DWORD,
}

#[repr(C)]
pub struct POINT {
    x: LONG,
    y: LONG,
}

#[repr(C)]
pub struct PAINTSTRUCT{
    hdc: HDC,
    fErase: BOOL,
    rcPaint: RECT,
    fRestore: BOOL,
    fIncUpdate: BOOL,
    rgbReserved: [u8; 32],
}

#[repr(C)]
pub struct RECT{
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}

#[repr(C)]
pub struct CREATESTRUCTW{
    lpCreateParams: LPVOID,
    hInstance: HINSTANCE,
    hMenu: HMENU,
    hwndParent: HWND,
    cy: c_int,
    cx: c_int,
    y: c_int,
    x: c_int,
    style: LONG,
    lpszName: LPCWSTR,
    lpszClass: LPCWSTR,
    dwExStyle: DWORD,
  }

unsafe_impl_default_zeroed!(WNDCLASSW);
unsafe_impl_default_zeroed!(MSG);
unsafe_impl_default_zeroed!(RECT);
unsafe_impl_default_zeroed!(PAINTSTRUCT);


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
    pub fn DestroyWindow(
        hWnd: HWND
    ) -> BOOL;
    pub fn PostQuitMessage(
        nExitCode: c_int
    );
    pub fn LoadCursorW(
        hInstance: HINSTANCE,
        lpCursorName: LPCWSTR
    ) -> HCURSOR;
    pub fn BeginPaint(
        hWnd: HWND,
        lpPaint: *mut PAINTSTRUCT,
    ) -> HDC;
    pub fn EndPaint(
        hWnd: HWND,
        lpPaint: *const PAINTSTRUCT
    ) -> BOOL;
    pub fn FillRect(
        hDC: HDC,
        lprc: *const RECT,
        hbr: HBRUSH
    ) -> int;
    pub fn MessageBoxW(
        hWnd: HWND,
        lpText: LPCWSTR,
        lpCaption: LPCWSTR,
        uType: UINT
    ) -> c_int;
    pub fn SetWindowLongPtrW(
        hWnd: HWND,
        nIndex: int,
        dwNewLong: LONG_PTR
    ) -> LONG_PTR;
    pub fn GetWindowLongPtrW(
        hWnd: HWND,
        nIndex: c_int
    ) -> LONG_PTR;
    pub fn SetCursor(
        hCursor: HCURSOR
    ) -> HCURSOR;
}


pub const fn MAKEINTRESOURCEW(i: WORD) -> LPWSTR {
    i as ULONG_PTR as LPWSTR
}

pub unsafe extern "system" fn window_procedure(hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM,) -> LRESULT{
    // if Msg == WM_CREATE{
    //     let pCreate = lParam as *const CREATESTRUCTW;
    //     let pState = 
    // }
    // else{}
    match Msg{
        WM_NCCREATE => {
            let createstruct: *mut CREATESTRUCTW = lParam as *mut _;
            if createstruct.is_null() {
                return 0;
            }
            let boxed_i32_ptr: *mut i32 = (*createstruct).lpCreateParams.cast();
            SetWindowLongPtrW(hWnd, GWLP_USERDATA, boxed_i32_ptr as LONG_PTR);
            return 1;
        }
        WM_CLOSE => {let message = wide_str("Do you really want to exit?");
            let title = wide_str("Quit");
            let result = MessageBoxW(hWnd, message.as_ptr(), title.as_ptr(), MB_OKCANCEL);
            if result == IDOK{
            DestroyWindow(hWnd);}
            else {
                return 0;
            }
        },
        WM_DESTROY => {
            PostQuitMessage(0);
            let ptr = GetWindowLongPtrW(hWnd, GWLP_USERDATA) as *mut i32;
            Box::from_raw(ptr);
        },
        WM_PAINT => {
            let mut ps = PAINTSTRUCT::default();
            let hdc = BeginPaint(hWnd, &mut ps);
            FillRect(hdc, &ps.rcPaint, (COLOR_WINDOW +5) as HBRUSH);
            EndPaint(hWnd, &ps);
          }
         _ => return DefWindowProcW(hWnd, Msg, wParam, lParam),
        }
        0
}

pub fn register_window(name: Vec<u16>,hInstance: *mut c_void) -> WNDCLASSW{
    let mut wc = WNDCLASSW::default();
    wc.lpfnWndProc = Some(window_procedure);
    wc.hInstance = hInstance;
    wc.lpszClassName = name.as_ptr();
    wc.hCursor = unsafe{LoadCursorW(null_mut(), IDC_ARROW)};
    let atom = unsafe {RegisterClassW(&wc)};
    if atom == 0{
        let last_err = unsafe {GetLastError()};
        panic!("Could not register the window, error code: {}", last_err);
    }
    wc
}

pub fn make_window(name: Vec<u16>,hInstance: *mut c_void) -> HWND{
    let lparam: *mut i32 = Box::leak(Box::new(5_i32));
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
        lparam.cast()
        )};
    if hwnd.is_null(){
       panic!("Failed to create a window");
    }
    hwnd
}

pub fn wide_str(str: &str) -> Vec<u16> {
    str.encode_utf16().chain(Some(0)).collect()
}

}