#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    static mut stdin: *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
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
#[no_mangle]
pub unsafe extern "C" fn get_line(mut fp: *mut FILE) -> *mut libc::c_char {
    let mut len: libc::c_int = 0 as libc::c_int;
    let mut got: libc::c_int = 0 as libc::c_int;
    let mut c: libc::c_int = 0;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    loop {
        c = fgetc(fp);
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        if got + 1 as libc::c_int >= len {
            len *= 2 as libc::c_int;
            if len < 4 as libc::c_int {
                len = 4 as libc::c_int;
            }
            buf = realloc(buf as *mut libc::c_void, len as libc::c_ulong)
                as *mut libc::c_char;
        }
        let fresh0 = got;
        got = got + 1;
        *buf.offset(fresh0 as isize) = c as libc::c_char;
        if c == '\n' as i32 {
            break;
        }
    }
    if c == -(1 as libc::c_int) && got == 0 {
        return 0 as *mut libc::c_char;
    }
    let fresh1 = got;
    got = got + 1;
    *buf.offset(fresh1 as isize) = '\0' as i32 as libc::c_char;
    return buf;
}
unsafe fn main_0() -> libc::c_int {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    loop {
        s = get_line(stdin);
        if s.is_null() {
            break;
        }
        printf(b"%s\0" as *const u8 as *const libc::c_char, s);
        free(s as *mut libc::c_void);
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
