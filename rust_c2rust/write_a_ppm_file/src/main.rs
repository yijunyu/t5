#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
unsafe fn main_0() -> libc::c_int {
    let dimx: libc::c_int = 800 as libc::c_int;
    let dimy: libc::c_int = 800 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut fp: *mut FILE = fopen(
        b"first.ppm\0" as *const u8 as *const libc::c_char,
        b"wb\0" as *const u8 as *const libc::c_char,
    );
    fprintf(fp, b"P6\n%d %d\n255\n\0" as *const u8 as *const libc::c_char, dimx, dimy);
    j = 0 as libc::c_int;
    while j < dimy {
        i = 0 as libc::c_int;
        while i < dimx {
            static mut color: [libc::c_uchar; 3] = [0; 3];
            color[0 as libc::c_int as usize] = (i % 256 as libc::c_int) as libc::c_uchar;
            color[1 as libc::c_int as usize] = (j % 256 as libc::c_int) as libc::c_uchar;
            color[2 as libc::c_int
                as usize] = (i * j % 256 as libc::c_int) as libc::c_uchar;
            fwrite(
                color.as_mut_ptr() as *const libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                3 as libc::c_int as libc::c_ulong,
                fp,
            );
            i += 1;
            i;
        }
        j += 1;
        j;
    }
    fclose(fp);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
