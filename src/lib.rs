#[cfg(windows)]
pub mod win32{

use std::ffi::{c_void};
use std::ptr::{null_mut};

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
const CW_USEDEFAULT: CInt = 0x80000000_u32 as CInt;
pub const SW_SHOW: CInt = 5;
const IDC_ARROW: LPCWStr = MakeIntResourceW(32512);
const WM_CLOSE: u32 = 0x0010;
const WM_DESTROY: u32 = 0x0002;
const WM_CREATE: u32 = 0x0001;

const WM_NCCREATE: u32 = 0x0081;

const WM_PAINT: u32 = 0x000F;
const COLOR_WINDOW:u32 = 5;
const MB_OKCANCEL: u32 = 0x00000001;
const IDOK: i32 = 1;
const GWLP_USERDATA: i32 = -21;
const WM_SETCURSOR: u32 = 0x0020;


type Atom = Word;
type Bool = CInt;
type CInt = i32;
type CUInt = u32;
type DWord = u32;
type Handle = *mut c_void;
type HBrush = Handle;
type HCursor = HIcon;
type HDc = Handle;
type HIcon = Handle;
type HInstance = Handle;
type HMenu = Handle;
type HModule = HInstance;
type HWnd = Handle;
type Int = CInt;
type Long = i32;
type LongPtr = isize;
type LParam = LongPtr;
type LPMsg = *mut Msg;
type LPVoid = *mut c_void;
type LPWStr = *mut WChar;
type LPCWStr = *const WCharT;
type LResult = LongPtr;
type UInt = CUInt;
type UIntPtr = usize;
type ULongPtr = usize;
type WCharT = u16;
type WChar = WCharT;
type WParam = UIntPtr;
type Word = u16;

type WNDProc = Option<
    unsafe extern "system" fn(
        hwnd: HWnd,
        Msg: UInt,
        wParam: WParam,
        lParam: LParam,
    ) -> LResult,
>;


#[repr(C)]
pub struct WNDClassW {
  style: UInt,
  lpfnWndProc: WNDProc,
  cbClsExtra: Int,
  cbWndExtra: Int,
  hInstance: HInstance,
  hIcon: HIcon,
  hCursor: HCursor,
  hbrBackground: HBrush,
  lpszMenuName: LPCWStr,
  lpszClassName: LPCWStr,
}

#[repr(C)]
pub struct Msg {
    hwnd: HWnd,
    message: UInt,
    wParam: WParam,
    lParam: LParam,
    time: DWord,
    pt: Point,
    lPrivate: DWord,
}

#[repr(C)]
pub struct Point {
    x: Long,
    y: Long,
}

#[repr(C)]
pub struct PaintStruct{
    hdc: HDc,
    fErase: Bool,
    rcPaint: Rect,
    fRestore: Bool,
    fIncUpdate: Bool,
    rgbReserved: [u8; 32],
}

#[repr(C)]
pub struct Rect{
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}

#[repr(C)]
pub struct CREATESTRUCTW{
    lpCreateParams: LPVoid,
    hInstance: HInstance,
    hMenu: HMenu,
    hwndParent: HWnd,
    cy: CInt,
    cx: CInt,
    y: CInt,
    x: CInt,
    style: Long,
    lpszName: LPCWStr,
    lpszClass: LPCWStr,
    dwExStyle: DWord,
  }

unsafe_impl_default_zeroed!(WNDClassW);
unsafe_impl_default_zeroed!(Msg);
unsafe_impl_default_zeroed!(Rect);
unsafe_impl_default_zeroed!(PaintStruct);


#[link(name = "Kernel32")]
extern "system" {
    pub fn GetModuleHandleW (lpModuleName: LPCWStr) -> HModule;
    pub fn GetLastError() -> DWord;
    pub fn CreateWindowExW(
        dwExStyle: DWord,
        lpClassName: LPCWStr,
        lpWindowName: LPCWStr,
        dwStyle: DWord,
        X: Int,
        Y: Int,
        nWidth: Int,
        nHeight: Int,
        hWndParent: HWnd,
        hMenu: HMenu,
        h_instance: HInstance,
        lpParam: LPVoid
    ) -> HWnd;
}

#[link(name = "User32")]
extern "system" {
    pub fn ShowWindow(
        hWnd: HWnd,
        nCmdShow: Int
    ) -> Bool;
    pub fn RegisterClassW(lpWndClass: *const WNDClassW) -> Atom;
    pub fn DefWindowProcW(
      hWnd: HWnd, Msg: UInt, wParam: WParam, lParam: LParam,
    ) -> LResult;
    pub fn GetMessageW(
      lpMsg: LPMsg,
      hWnd: HWnd,
      wMsgFilterMin: UInt,
      wMsgFilterMax: UInt
    ) -> Bool;
    pub fn TranslateMessage(
        lpMsg: *const Msg
    ) -> Bool;
    pub fn DispatchMessageW(
        lpMsg: *const Msg
    ) -> LResult;
    pub fn DestroyWindow(
        hWnd: HWnd
    ) -> Bool;
    pub fn PostQuitMessage(
        nExitCode: CInt
    );
    pub fn LoadCursorW(
        h_instance: HInstance,
        lpCursorName: LPCWStr
    ) -> HCursor;
    pub fn BeginPaint(
        hWnd: HWnd,
        lpPaint: *mut PaintStruct,
    ) -> HDc;
    pub fn EndPaint(
        hWnd: HWnd,
        lpPaint: *const PaintStruct
    ) -> Bool;
    pub fn FillRect(
        hDC: HDc,
        lprc: *const Rect,
        hbr: HBrush
    ) -> Int;
    pub fn MessageBoxW(
        hWnd: HWnd,
        lpText: LPCWStr,
        lpCaption: LPCWStr,
        uType: UInt
    ) -> CInt;
    pub fn SetWindowLongPtrW(
        hWnd: HWnd,
        nIndex: Int,
        dwNewLong: LongPtr
    ) -> LongPtr;
    pub fn GetWindowLongPtrW(
        hWnd: HWnd,
        nIndex: CInt
    ) -> LongPtr;
    pub fn SetCursor(
        hCursor: HCursor
    ) -> HCursor;
}


pub const fn MakeIntResourceW(i: Word) -> LPWStr {
    i as ULongPtr as LPWStr
}

unsafe extern "system" fn window_procedure(hWnd: HWnd, Msg: UInt, wParam: WParam, lParam: LParam,) -> LResult{
    match Msg{
        WM_NCCREATE => {
            let createstruct: *mut CREATESTRUCTW = lParam as *mut _;
            if createstruct.is_null() {
                return 0;
            }
            let boxed_i32_ptr: *mut i32 = (*createstruct).lpCreateParams.cast();
            SetWindowLongPtrW(hWnd, GWLP_USERDATA, boxed_i32_ptr as LongPtr);
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
            drop(Box::from_raw(ptr));
        },
        WM_PAINT => {
            let mut ps = PaintStruct::default();
            let hdc = BeginPaint(hWnd, &mut ps);
            FillRect(hdc, &ps.rcPaint, (COLOR_WINDOW +5) as HBrush);
            EndPaint(hWnd, &ps);
          }
         _ => return DefWindowProcW(hWnd, Msg, wParam, lParam),
        }
        0
}

pub fn register_window(name: Vec<u16>,h_instance: *mut c_void) -> WNDClassW{
    let wc = WNDClassW{
    lpfnWndProc: Some(window_procedure),
    hInstance: h_instance,
    lpszClassName: name.as_ptr(),
    hCursor: unsafe{
        LoadCursorW(
            null_mut(),
            IDC_ARROW)
        },
    ..Default::default()
    };
    let atom = unsafe {RegisterClassW(&wc)};
    if atom == 0{
        let last_err = unsafe {GetLastError()};
        panic!("Could not register the window, error code: {}", last_err);
    }
    wc
}

pub fn make_window(name: Vec<u16>,h_instance: *mut c_void) -> HWnd{
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
        unsafe{h_instance},   
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