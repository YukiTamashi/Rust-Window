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

pub const SW_SHOW: CInt = 5;
pub const WS_OVERLAPPED: u32 = 0x00000000;
pub const WS_CAPTION: u32 = 0x00C00000;
pub const WS_SYSMENU: u32 = 0x00080000;
pub const WS_THICKFRAME: u32 = 0x00040000;
pub const WS_MINIMIZEBOX: u32 = 0x00020000;
pub const WS_MAXIMIZEBOX: u32 = 0x00010000;
pub const WS_OVERLAPPEDWINDOW: u32 = WS_OVERLAPPED
  | WS_CAPTION
  | WS_SYSMENU
  | WS_THICKFRAME
  | WS_MINIMIZEBOX
  | WS_MAXIMIZEBOX;
pub const CW_USEDEFAULT: CInt = 0x80000000_u32 as CInt;
pub const IDC_ARROW: LPCWStr = MakeIntResourceW(32512);
pub const WM_CLOSE: u32 = 0x0010;
pub const WM_DESTROY: u32 = 0x0002;
pub const WM_CREATE: u32 = 0x0001;
pub const WM_NCCREATE: u32 = 0x0081;
pub const WM_PAINT: u32 = 0x000F;
pub const COLOR_WINDOW:u32 = 5;
pub const MB_OKCANCEL: u32 = 0x00000001;
pub const IDOK: i32 = 1;
pub const GWLP_USERDATA: i32 = -21;
pub const WM_SETCURSOR: u32 = 0x0020;

pub type Atom = Word;
pub type Bool = CInt;
pub type CInt = i32;
pub type CUInt = u32;
pub type DWord = u32;
pub type Handle = *mut c_void;
pub type HBrush = Handle;
pub type HCursor = HIcon;
pub type HDc = Handle;
pub type HIcon = Handle;
pub type HInstance = Handle;
pub type HMenu = Handle;
pub type HModule = HInstance;
pub type HWnd = Handle;
pub type Int = CInt;
pub type Long = i32;
pub type LongPtr = isize;
pub type LParam = LongPtr;
pub type LPMsg = *mut Msg;
pub type LPVoid = *mut c_void;
pub type LPWStr = *mut WChar;
pub type LPCWStr = *const WCharT;
pub type LResult = LongPtr;
pub type UInt = CUInt;
pub type UIntPtr = usize;
pub type ULongPtr = usize;
pub type WCharT = u16;
pub type WChar = WCharT;
pub type WParam = UIntPtr;
pub type Word = u16;
pub type WNDProc = Option<
    unsafe extern "system" fn(
        hwnd: HWnd,
        Msg: UInt,
        wParam: WParam,
        lParam: LParam,
    ) -> LResult,
>;

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

impl PaintStruct{
    pub fn rcPaint(&self) -> &Rect{
        &self.rcPaint
    }
}
#[repr(C)]
pub struct Rect{
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}

#[repr(C)]
pub struct CreateStructW{
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

impl CreateStructW{
    pub fn lpCreateParams(&self) -> LPVoid{
        self.lpCreateParams
    }
}

unsafe_impl_default_zeroed!(WNDClassW);
unsafe_impl_default_zeroed!(Msg);
unsafe_impl_default_zeroed!(Rect);
unsafe_impl_default_zeroed!(PaintStruct);

#[link(name = "Kernel32")]
extern "system" {
    fn GetModuleHandleW (lpModuleName: LPCWStr) -> HModule;
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
    fn GetMessageW(
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

fn get_handle() -> HModule {
    unsafe{GetModuleHandleW(std::ptr::null())}
}

const fn MakeIntResourceW(i: Word) -> LPWStr {
    i as ULongPtr as LPWStr
}

fn register_window(name: Vec<u16>, h_instance: *mut c_void, hCursor: HCursor, procedure: unsafe extern "system" fn(HWnd, UInt, WParam, LParam) -> LResult) -> WNDClassW{
    let wc = WNDClassW{
    lpfnWndProc: Some(procedure),
    hInstance: h_instance,
    lpszClassName: name.as_ptr(),
    hCursor,
    ..Default::default()
    };
    let atom = unsafe {RegisterClassW(&wc)};
    if atom == 0{
        let last_err = unsafe {GetLastError()};
        panic!("Could not register the window, error code: {}", last_err);
    }
    wc
}

fn make_window(name: Vec<u16>,h_instance: *mut c_void) -> HWnd{
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

//Handles entire creation of the window.
//Returns window handle.
pub fn create_window(name: Vec<u16>, hCursor: HCursor, procedure: unsafe extern "system" fn(HWnd, UInt, WParam, LParam) -> LResult) -> HWnd {
    let hinstance = get_handle();
    register_window(name.clone(), hinstance, hCursor, procedure);
    let hwnd = make_window(name, hinstance);
    unsafe {ShowWindow(hwnd, SW_SHOW)};
    hwnd
}

//Returns cursor handle based on type called.
pub fn cursor(name: IDCursor) -> Result<HCursor, ()> {
    let cursor = unsafe{
        LoadCursorW(
            null_mut(),
            unsafe{MakeIntResourceW(name as Word)})
    };
    if cursor.is_null(){
        Err(())
    }
    else{
        Ok(cursor)
    }
    
}

//Converts into UTF-8 string.
pub fn wide_str(str: &str) -> Vec<u16> {
    str.encode_utf16().chain(Some(0)).collect()
}

//Calls for system message updates.
//If it receives call to close, returns true.
pub fn update(msg: &mut Msg) -> bool{
    let message = unsafe{GetMessageW(msg, null_mut(), 0, 0)};
    if message == 0{return true}
    else if message == -1{
        panic!();
    }
    else {
        unsafe{TranslateMessage(msg); 
        DispatchMessageW(msg);}
    }
    false
}


}