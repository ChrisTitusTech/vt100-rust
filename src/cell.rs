use libc;
use std;

use color;
use types;

pub struct Cell(*mut types::CellImpl);

#[repr(C)]
struct CellAttrs {
    fgcolor: types::ColorImpl,
    bgcolor: types::ColorImpl,
    attrs: libc::c_uchar,
}

#[repr(C)]
struct CellPrefix {
    pub contents: [libc::c_char; 8],
    pub len: libc::size_t,
    pub attrs: CellAttrs,
}

impl Cell {
    pub fn new(cell_impl: *mut types::CellImpl) -> Cell {
        Cell(cell_impl)
    }

    pub fn contents(&self) -> &str {
        let Cell(cell_impl) = *self;
        let prefix: *mut CellPrefix = unsafe {
            std::mem::transmute(cell_impl)
        };
        let contents: &[u8] = unsafe {
            std::slice::from_raw_parts(
                &(*prefix).contents as *const i8 as *const u8,
                (*prefix).len
            )
        };
        std::str::from_utf8(contents).unwrap()
    }

    pub fn fgcolor(&self) -> color::Color {
        let Cell(cell_impl) = *self;
        let prefix: *mut CellPrefix = unsafe {
            std::mem::transmute(cell_impl)
        };
        let attrs = unsafe { &(*prefix).attrs };
        color::Color::new(&attrs.fgcolor)
    }

    pub fn bgcolor(&self) -> color::Color {
        let Cell(cell_impl) = *self;
        let prefix: *mut CellPrefix = unsafe {
            std::mem::transmute(cell_impl)
        };
        let attrs = unsafe { &(*prefix).attrs };
        color::Color::new(&attrs.bgcolor)
    }
}