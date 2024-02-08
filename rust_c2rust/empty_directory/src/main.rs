#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __ino_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub type DIR = __dirstream;
#[no_mangle]
pub unsafe extern "C" fn dir_empty(mut path: *const libc::c_char) -> libc::c_int {
    let mut ent: *mut dirent = 0 as *mut dirent;
    let mut ret: libc::c_int = 1 as libc::c_int;
    let mut d: *mut DIR = opendir(path);
    if d.is_null() {
        fprintf(stderr, b"%s: \0" as *const u8 as *const libc::c_char, path);
        perror(b"\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    loop {
        ent = readdir(d);
        if ent.is_null() {
            break;
        }
        if strcmp(
            ((*ent).d_name).as_mut_ptr(),
            b".\0" as *const u8 as *const libc::c_char,
        ) == 0
            || strcmp(
                ((*ent).d_name).as_mut_ptr(),
                b"..\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            continue;
        }
        ret = 0 as libc::c_int;
        break;
    }
    closedir(d);
    return ret;
}
unsafe fn main_0(mut c: libc::c_int, mut v: *mut *mut libc::c_char) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    if c < 2 as libc::c_int {
        return -(1 as libc::c_int);
    }
    i = 1 as libc::c_int;
    while i < c {
        ret = dir_empty(*v.offset(i as isize));
        if ret >= 0 as libc::c_int {
            printf(
                b"%s: %sempty\n\0" as *const u8 as *const libc::c_char,
                *v.offset(i as isize),
                if ret != 0 {
                    b"\0" as *const u8 as *const libc::c_char
                } else {
                    b"not \0" as *const u8 as *const libc::c_char
                },
            );
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
