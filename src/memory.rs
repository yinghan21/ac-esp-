use std::ffi::c_void;
use std::mem::size_of;
use std::sync::OnceLock;

use windows_sys::Win32::UI::WindowsAndMessaging::*;
use windows_sys::Win32::System::LibraryLoader::*;
use windows_sys::Win32::System::Memory::*;
use windows_sys::Win32::Foundation::*;
use windows_sys::s;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
impl Vec2 {
    pub fn new(x: f32, y:f32) -> Self {
        Self {
            x,y
        }
    }
    pub fn to_array(self) -> [f32;2] {
        [self.x, self.y]
    }
}

pub fn module_base() -> Option<usize> {
    let name = s!("ac_client.exe");
    let handle = unsafe{ GetModuleHandleA(name)};
    if handle.is_null() {
        return None;
    }
    Some(handle as usize)
}

pub fn is_readable(addr: usize, size: usize) -> bool {
    if addr == 0 || size == 0 {
        return false;
    }

    let Some(end) = addr.checked_add(size) else {
        return false;
    };

    let mut current = addr;

    let mut mbi = unsafe{ std::mem::zeroed::<MEMORY_BASIC_INFORMATION>()};
    let mbi_size = std::mem::size_of::<MEMORY_BASIC_INFORMATION>();

    while current < end {
        let ret = unsafe { VirtualQuery(current as *const c_void, &mut mbi, mbi_size)};
        if ret == 0 {
            return false;
        }
        if mbi.State != MEM_COMMIT {
            return false;
        }
        if mbi.Protect & (PAGE_NOACCESS | PAGE_GUARD) !=0 {
            return false;
        }

        let region_end = (mbi.BaseAddress as usize).saturating_add(mbi.RegionSize);
        if region_end <= current {
            return false;
        }
        current = region_end;
    } 
    true
}

pub fn read<T:Copy>(addr: usize) -> Option<T> {
    if !is_readable(addr,size_of::<T>()) {
        return None;
    }
    Some(unsafe{ std::ptr::read_unaligned(addr as *const T) })
}

pub fn world_to_screen(world: Vec3, matrix: &[f32;16], width: f32, height: f32) -> Option<Vec2> {
    let m = matrix;

    let x = world.x * m[0] + world.y * m[4] + world.z * m[8] + m[12];
    let y = world.x * m[1] + world.y * m[5] + world.z * m[9] + m[13];
    let w = world.x * m[3] + world.y * m[7] + world.z * m[11] + m[15];
    if w < 0.001 || !w.is_finite() {
        return None;
    }

    let ndc_x = x / w;
    let ndc_y = y / w;
    if !ndc_x.is_finite() || !ndc_y.is_finite() {
        return None;
    }

    let center_x = width * 0.5;
    let center_y = height * 0.5;

    Some(Vec2::new(
        center_x + ndc_x * center_x,
        center_y - ndc_y * center_y
    ))
} 

fn find_game_window() -> Option<HWND> {
    if let Some(hwnd) = hudhook::hooks::find_process_hwnd() {
        return Some(hwnd.0 as HWND);
    }

    let hwnd = unsafe{ FindWindowA(std::ptr::null(), s!("AssaultCube")) };
    if hwnd.is_null() {
        None
    } else {
        Some(hwnd)
    }
}

static GAME_HWND: OnceLock<isize> = OnceLock::new();
pub fn game_hwnd() -> Option<HWND> {
    if let Some(&raw) = GAME_HWND.get() {
        let hwnd = raw as HWND;
        if unsafe{ IsWindow(hwnd)} != 0 {
            return Some(hwnd);
        }
    }

    let hwnd = find_game_window()?;
    let _ = GAME_HWND.set(hwnd as isize);
    Some(hwnd)
}

pub fn game_client_size() -> Option<(f32, f32)> {
    let hwnd = game_hwnd()?;
    let mut rect: RECT = unsafe{ std::mem::zeroed::<RECT>() };

    let ok = unsafe { GetClientRect(hwnd, &mut rect) };
    if ok == 0 {
        return None;
    }

    let w = (rect.right - rect.left) as f32;
    let h = (rect.bottom - rect.top) as f32;

    if w <= 0.0 || h <= 0.0 {
        return None;
    }
    Some((w, h))
}