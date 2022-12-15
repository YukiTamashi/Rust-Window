use std::mem::size_of;

use windows::Win32::{Graphics::{OpenGL::*, Gdi::{HDC, ReleaseDC, GetDC}}, Foundation::{WIN32_ERROR, HWND}, UI::WindowsAndMessaging::DestroyWindow};
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

pub fn pf_index() -> PIXELFORMATDESCRIPTOR{
    let fake = make_window("fake", get_handle());
    let hdc = get_dc(fake).unwrap();
    let index = get_pixel(hdc).unwrap();
    check_pf(hdc);
    set_pixel(hdc, index).unwrap();
    let pf = pixel_format(hdc, index).unwrap();
    cleanup(fake, hdc).unwrap();
    pf
}

fn get_dc(hwnd: HWND) -> Option<HDC> {
    let dc = unsafe{ GetDC(hwnd) };
    if dc.is_invalid(){
        None
    }
    else{
        Some(dc)
    }
}

fn cleanup(hwnd: HWND, hdc: HDC) -> Result<(),WIN32_ERROR>{
    unsafe {
        let release = ReleaseDC(hwnd, hdc);
        let destroy = DestroyWindow(hwnd);
        if destroy.as_bool() && release == 0{
            Ok(())
        }
        else{
            Err(last_error())
        }
    }
}

fn get_pixel(hdc: HDC) -> Result<i32, WIN32_ERROR> {
    let index = unsafe{ ChoosePixelFormat(hdc, &pfd()) };
    if index != 0{
        Ok(index)
    }
    else{
        Err(last_error())
    }
}
fn set_pixel(hdc: HDC, index: i32) -> Result<(), WIN32_ERROR>{
    unsafe{ 
        let result = SetPixelFormat(hdc, index, &pfd());
        if result.as_bool(){
            Ok(())
        }
        else{
            Err(last_error())
        }
    }
}

fn pixel_format(hdc: HDC, index: i32) -> Result<PIXELFORMATDESCRIPTOR, WIN32_ERROR>{
    let mut pfd = pfd();
    let max = unsafe{ DescribePixelFormat(hdc, PFD_PIXEL_TYPE(index as i8), size_of::<PIXELFORMATDESCRIPTOR>() as u32, Some(&mut pfd))};
    if max == 0{
        Err(last_error())
    }
    else{
        Ok(pfd)
    }
}

fn check_pf(hdc: HDC) -> i32 {
    let mut pfd = pfd();
    let max = unsafe { DescribePixelFormat(hdc, PFD_PIXEL_TYPE(1), size_of::<PIXELFORMATDESCRIPTOR>() as u32, Some(&mut pfd))};
    println!("pfd: {pfd:?}");
    println!("index: {max}");
    max
}