use libc;
use std;

use ffi;
use types;

pub struct Screen(*mut types::ScreenImpl);

impl Screen {
    pub fn new(rows: i32, cols: i32) -> Screen {
        let screen_impl = unsafe {
            ffi::vt100_screen_new(rows as libc::c_int, cols as libc::c_int)
        };
        Screen(screen_impl)
    }

    pub fn rows(&self) -> i32 {
        let Screen(screen_impl) = *self;
        unsafe { ffi::vt100_wrapper_rows(screen_impl) }
    }

    pub fn cols(&self) -> i32 {
        let Screen(screen_impl) = *self;
        unsafe { ffi::vt100_wrapper_cols(screen_impl) }
    }

    pub fn set_window_size(&self, rows: i32, cols: i32) {
        let Screen(screen_impl) = *self;
        unsafe { ffi::vt100_screen_set_window_size(screen_impl, rows, cols) };
    }

    pub fn set_scrollback_length(&self, rows: i32) {
        let Screen(screen_impl) = *self;
        unsafe { ffi::vt100_screen_set_scrollback_length(screen_impl, rows) };
    }

    pub fn process(&mut self, s: &[u8]) -> u64 {
        let Screen(screen_impl) = *self;
        unsafe {
            ffi::vt100_screen_process_string(
                screen_impl,
                s.as_ptr() as *const libc::c_char,
                s.len()
            ) as u64
        }
    }

    pub fn window_contents(&self,
        row_start: i32,
        col_start: i32,
        row_end: i32,
        col_end: i32
    ) -> String {
        let Screen(screen_impl) = *self;
        let row_start = std::cmp::min(
            std::cmp::max(row_start, 0),
            self.rows() - 1
        );
        let col_start = std::cmp::min(
            std::cmp::max(col_start, 0),
            self.cols() - 1
        );
        let row_end = std::cmp::min(
            std::cmp::max(row_end, 0),
            self.rows() - 1
        );
        let col_end = std::cmp::min(
            std::cmp::max(col_end, 0),
            self.cols() - 1
        );

        let start_loc = types::Loc { row: row_start, col: col_start };
        let end_loc = types::Loc { row: row_end, col: col_end };

        let mut plaintext: *mut libc::c_char = unsafe { std::mem::uninitialized() };
        let mut len: libc::size_t = unsafe { std::mem::uninitialized() };
        unsafe {
            ffi::vt100_screen_get_string_plaintext(
                screen_impl,
                &start_loc as *const types::Loc,
                &end_loc as *const types::Loc,
                &mut plaintext as *mut *mut libc::c_char,
                &mut len as *mut libc::size_t,
            )
        };
        let rust_plaintext = unsafe {
            std::slice::from_raw_parts(
                plaintext as *mut libc::c_uchar,
                len
            )
        }.to_vec();
        std::string::String::from_utf8(rust_plaintext).unwrap()
    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        let Screen(screen_impl) = *self;
        unsafe { ffi::vt100_screen_delete(screen_impl) };
    }
}
