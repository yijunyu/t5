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
use c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn toupper(_: i32) -> i32;
    static mut stdin: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn fgets(__s: *mut i8, __n: i32, __stream: *mut FILE) -> *mut i8;
    fn puts(__s: *const i8) -> i32;
    fn atoi(__nptr: *const i8) -> i32;
    fn strcat(_: *mut i8, _: *const i8) -> *mut i8;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strspn(_: *const i8, _: *const i8) -> u64;
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
#[no_mangle]
pub static mut dih: [i8; 50] = [0; 50];
#[no_mangle]
pub static mut dah: [i8; 50] = [0; 50];
#[no_mangle]
pub static mut medium: [i8; 30] = [0; 30];
#[no_mangle]
pub static mut word: [i8; 30] = [0; 30];
#[no_mangle]
pub static mut dd: [*mut i8; 2] = unsafe { [dih.as_ptr() as *mut _, dah.as_ptr() as *mut _] };
#[no_mangle]
pub static mut ascii: *const i8 =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789.,?'!/()&:;=+-_\"$@\0" as *const u8 as *const i8;
#[no_mangle]
pub static mut itu: [*const i8; 54] = [
    b"13\0" as *const u8 as *const i8,
    b"3111\0" as *const u8 as *const i8,
    b"3131\0" as *const u8 as *const i8,
    b"311\0" as *const u8 as *const i8,
    b"1\0" as *const u8 as *const i8,
    b"1131\0" as *const u8 as *const i8,
    b"331\0" as *const u8 as *const i8,
    b"1111\0" as *const u8 as *const i8,
    b"11\0" as *const u8 as *const i8,
    b"1333\0" as *const u8 as *const i8,
    b"313\0" as *const u8 as *const i8,
    b"1311\0" as *const u8 as *const i8,
    b"33\0" as *const u8 as *const i8,
    b"31\0" as *const u8 as *const i8,
    b"333\0" as *const u8 as *const i8,
    b"1331\0" as *const u8 as *const i8,
    b"3313\0" as *const u8 as *const i8,
    b"131\0" as *const u8 as *const i8,
    b"111\0" as *const u8 as *const i8,
    b"3\0" as *const u8 as *const i8,
    b"113\0" as *const u8 as *const i8,
    b"1113\0" as *const u8 as *const i8,
    b"133\0" as *const u8 as *const i8,
    b"3113\0" as *const u8 as *const i8,
    b"3133\0" as *const u8 as *const i8,
    b"3311\0" as *const u8 as *const i8,
    b"33333\0" as *const u8 as *const i8,
    b"13333\0" as *const u8 as *const i8,
    b"11333\0" as *const u8 as *const i8,
    b"11133\0" as *const u8 as *const i8,
    b"11113\0" as *const u8 as *const i8,
    b"11111\0" as *const u8 as *const i8,
    b"31111\0" as *const u8 as *const i8,
    b"33111\0" as *const u8 as *const i8,
    b"33311\0" as *const u8 as *const i8,
    b"33331\0" as *const u8 as *const i8,
    b"131313\0" as *const u8 as *const i8,
    b"331133\0" as *const u8 as *const i8,
    b"113311\0" as *const u8 as *const i8,
    b"133331\0" as *const u8 as *const i8,
    b"313133\0" as *const u8 as *const i8,
    b"31131\0" as *const u8 as *const i8,
    b"31331\0" as *const u8 as *const i8,
    b"313313\0" as *const u8 as *const i8,
    b"13111\0" as *const u8 as *const i8,
    b"333111\0" as *const u8 as *const i8,
    b"313131\0" as *const u8 as *const i8,
    b"31113\0" as *const u8 as *const i8,
    b"13131\0" as *const u8 as *const i8,
    b"311113\0" as *const u8 as *const i8,
    b"113313\0" as *const u8 as *const i8,
    b"131131\0" as *const u8 as *const i8,
    b"1113113\0" as *const u8 as *const i8,
    b"133131\0" as *const u8 as *const i8,
];
#[no_mangle]
pub extern "C" fn append(mut s: *mut i8, mut morse: *const i8) {
    unsafe {
        while *morse != 0 {
            strcat(s, dd[('3' as i32 == *morse as i32) as i32 as usize]);
            morse = morse.offset(1);
            morse;
        }
        strcat(s, medium.as_mut_ptr());
    }
}

#[no_mangle]
pub extern "C" fn translate(mut i: *const i8, mut o: *mut i8) -> *mut i8 {
    unsafe {
        let mut pc: *const i8 = 0 as *const i8;
        sprintf(o, b"beep\0" as *const u8 as *const i8);
        while *i != 0 {
            pc = strchr(ascii, toupper(*i as i32));
            if pc.is_null() {
                strcat(o, word.as_mut_ptr());
            } else {
                append(o, itu[pc.offset_from(ascii) as i64 as usize]);
            }
            i = i.offset(1);
            i;
        }
        strcat(o, word.as_mut_ptr());
        return o;
    }
}

fn main_0(mut ac: i32, mut av: *mut *mut i8) -> i32 {
    unsafe {
        let mut sin: [i8; 73] = [0; 73];
        let mut sout: [i8; 100000] = [0; 100000];
        let mut dit: i32 = 100;
        if 1 < ac {
            if strlen(*av.offset(1 as isize))
                != strspn(
                    *av.offset(1 as isize),
                    b"0123456789\0" as *const u8 as *const i8,
                )
            {
                return 0 * fprintf(
                    stderr,
                    b"use: %s [duration]   dit in ms, default %d\n\0" as *const u8 as *const i8,
                    *av,
                    dit,
                );
            }
            dit = if 1 < atoi(*av.offset(1 as isize)) {
                if atoi(*av.offset(1 as isize)) < 1000 {
                    atoi(*av.offset(1 as isize))
                } else {
                    1000
                }
            } else {
                1
            };
        }
        sprintf(
            dah.as_mut_ptr(),
            b" -n -f 440 -l %d -D %d\0" as *const u8 as *const i8,
            3 * dit,
            dit,
        );
        sprintf(
            dih.as_mut_ptr(),
            b" -n -f 440 -l %d -D %d\0" as *const u8 as *const i8,
            dit,
            dit,
        );
        sprintf(
            medium.as_mut_ptr(),
            b" -n -D %d\0" as *const u8 as *const i8,
            (3 - 1) * dit,
        );
        sprintf(
            word.as_mut_ptr(),
            b" -n -D %d\0" as *const u8 as *const i8,
            (7 - (3 - 1) - 1) * dit,
        );
        while !(fgets(sin.as_mut_ptr(), 72, stdin)).is_null() {
            puts(translate(sin.as_mut_ptr(), sout.as_mut_ptr()));
        }
        return 0;
    }
}

pub fn main() {
    let mut args: Vec<*mut i8> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    ::std::process::exit(main_0((args.len() - 1) as i32, args.as_mut_ptr() as *mut *mut i8) as i32);
}
