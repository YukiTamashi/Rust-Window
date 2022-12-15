use std::mem::size_of;

use windows::Win32::{Graphics::{OpenGL::*, Gdi::HDC}, Foundation::WIN32_ERROR};
use super::win32::*;

pub fn pfd() -> PIXELFORMATDESCRIPTOR{
    PIXELFORMATDESCRIPTOR{
        nSize: size_of::<PIXELFORMATDESCRIPTOR>() as u16,
        nVersion: 1,
        dwFlags: PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER,
        iPixelType: PFD_TYPE_RGBA,
        cColorBits: 32,
        cDepthBits: 24,
        cStencilBits: 8,
        cAuxBuffers: 0,
        iLayerType: PFD_MAIN_PLANE,
        ..Default::default()
    }
}

pub fn set_format(pfd: PIXELFORMATDESCRIPTOR, hdc: HDC) -> Result<(), WIN32_ERROR>{
    let pf = unsafe{ ChoosePixelFormat(hdc, &pfd) };
    if pf == 0{
        return Err(last_error())
    }
    let ok = unsafe{ SetPixelFormat(hdc, pf, &pfd) }.as_bool();
    if ok{
        Ok(())
    }
    else{
        Err(WIN32_ERROR(0))
    }
}