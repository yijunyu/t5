#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn getline(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn atof(__nptr: *const libc::c_char) -> libc::c_double;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
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
pub type ssize_t = __ssize_t;
unsafe fn main_0() -> libc::c_int {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0 as libc::c_int as size_t;
    let mut read: ssize_t = 0;
    let mut lw: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lt: *mut libc::c_char = 0 as *mut libc::c_char;
    fp = fopen(
        b"data.txt\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if fp.is_null() {
        printf(b"Unable to open file\n\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    printf(
        b"Those earthquakes with a magnitude > 6.0 are:\n\n\0" as *const u8
            as *const libc::c_char,
    );
    loop {
        read = getline(&mut line, &mut len, fp);
        if !(read != -(1 as libc::c_int) as libc::c_long) {
            break;
        }
        if read < 2 as libc::c_int as libc::c_long {
            continue;
        }
        lw = strrchr(line, ' ' as i32);
        lt = strrchr(line, '\t' as i32);
        if lw.is_null() && lt.is_null() {
            continue;
        }
        if lt > lw {
            lw = lt;
        }
        if atof(lw.offset(1 as libc::c_int as isize)) > 6.0f64 {
            printf(b"%s\0" as *const u8 as *const libc::c_char, line);
        }
    }
    fclose(fp);
    if !line.is_null() {
        free(line as *mut libc::c_void);
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
