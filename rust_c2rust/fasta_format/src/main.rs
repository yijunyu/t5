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
unsafe fn main_0() {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0 as libc::c_int as size_t;
    let mut read: ssize_t = 0;
    fp = fopen(
        b"fasta.txt\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if fp.is_null() {
        exit(1 as libc::c_int);
    }
    let mut state: libc::c_int = 0 as libc::c_int;
    loop {
        read = getline(&mut line, &mut len, fp);
        if !(read != -(1 as libc::c_int) as libc::c_long) {
            break;
        }
        if *line.offset((read - 1 as libc::c_int as libc::c_long) as isize)
            as libc::c_int == '\n' as i32
        {
            *line
                .offset(
                    (read - 1 as libc::c_int as libc::c_long) as isize,
                ) = 0 as libc::c_int as libc::c_char;
        }
        if *line.offset(0 as libc::c_int as isize) as libc::c_int == '>' as i32 {
            if state == 1 as libc::c_int {
                printf(b"\n\0" as *const u8 as *const libc::c_char);
            }
            printf(
                b"%s: \0" as *const u8 as *const libc::c_char,
                line.offset(1 as libc::c_int as isize),
            );
            state = 1 as libc::c_int;
        } else {
            printf(b"%s\0" as *const u8 as *const libc::c_char, line);
        }
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    fclose(fp);
    if !line.is_null() {
        free(line as *mut libc::c_void);
    }
    exit(0 as libc::c_int);
}
pub fn main() {
    unsafe { main_0() }
    ::std::process::exit(0i32);
}
