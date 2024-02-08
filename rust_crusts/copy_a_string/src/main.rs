#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
fn build_str_from_raw_ptr(raw_ptr: *mut u8) -> String {
    unsafe {
        let mut str_size: usize = 0;
        while *raw_ptr.offset(str_size as isize) != 0 {
            str_size += 1;
        }
        return std::str::from_utf8_unchecked(std::slice::from_raw_parts(raw_ptr, str_size))
            .to_owned();
    }
}

use c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn free(_: *mut libc::c_void);
    fn exit(_: i32) -> !;
    static mut stderr: *mut FILE;
    fn fputs(__s: *const i8, __stream: *mut FILE) -> i32;
    fn perror(__s: *const i8);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strdup(_: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut i8,
    pub _IO_read_end: *mut i8,
    pub _IO_read_base: *mut i8,
    pub _IO_write_base: *mut i8,
    pub _IO_write_ptr: *mut i8,
    pub _IO_write_end: *mut i8,
    pub _IO_buf_base: *mut i8,
    pub _IO_buf_end: *mut i8,
    pub _IO_save_base: *mut i8,
    pub _IO_backup_base: *mut i8,
    pub _IO_save_end: *mut i8,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: i64,
    pub _cur_column: u16,
    pub _vtable_offset: i8,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: i64,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: u64,
    pub _mode: i32,
    pub _unused2: [i8; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
fn main_0() -> i32 {
    unsafe {
        let mut len: u64 = 0;
        let mut src: [i8; 6] = *::core::mem::transmute::<&[u8; 6], &mut [i8; 6]>(b"Hello\0");
        let mut dst1: [i8; 80] = [0; 80];
        let mut dst2: [i8; 80] = [0; 80];
        let mut dst3: *mut i8 = 0 as *mut i8;
        let mut ref_0: *mut i8 = 0 as *mut i8;
        strcpy(dst1.as_mut_ptr(), src.as_mut_ptr());
        len = strlen(src.as_mut_ptr());
        if len >= ::core::mem::size_of::<[i8; 80]>() as u64 {
            fputs(
                b"The buffer is too small!\n\0" as *const u8 as *const i8,
                stderr,
            );
            exit(1);
        }
        memcpy(
            dst2.as_mut_ptr() as *mut libc::c_void,
            src.as_mut_ptr() as *const libc::c_void,
            len.wrapping_add(1),
        );
        dst3 = strdup(src.as_mut_ptr());
        if dst3.is_null() {
            perror(b"strdup\0" as *const u8 as *const i8);
            exit(1);
        }
        ref_0 = src.as_mut_ptr();
        memset(src.as_mut_ptr() as *mut libc::c_void, '-' as i32, 5);
        print!(
            " src: {}\n",
            build_str_from_raw_ptr(src.as_mut_ptr() as *mut u8)
        );
        print!(
            "dst1: {}\n",
            build_str_from_raw_ptr(dst1.as_mut_ptr() as *mut u8)
        );
        print!(
            "dst2: {}\n",
            build_str_from_raw_ptr(dst2.as_mut_ptr() as *mut u8)
        );
        print!("dst3: {}\n", build_str_from_raw_ptr(dst3 as *mut u8));
        print!(" ref: {}\n", build_str_from_raw_ptr(ref_0 as *mut u8));
        free(dst3 as *mut libc::c_void);
        return 0;
    }
}

pub fn main() {
    unsafe {
        ::std::process::exit(main_0() as i32);
    }
}
