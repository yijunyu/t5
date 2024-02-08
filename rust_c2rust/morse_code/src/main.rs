#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn toupper(_: libc::c_int) -> libc::c_int;
    static mut stdin: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
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
pub static mut dih: [libc::c_char; 50] = [0; 50];
#[no_mangle]
pub static mut dah: [libc::c_char; 50] = [0; 50];
#[no_mangle]
pub static mut medium: [libc::c_char; 30] = [0; 30];
#[no_mangle]
pub static mut word: [libc::c_char; 30] = [0; 30];
#[no_mangle]
pub static mut dd: [*mut libc::c_char; 2] = unsafe {
    [dih.as_ptr() as *mut _, dah.as_ptr() as *mut _]
};
#[no_mangle]
pub static mut ascii: *const libc::c_char = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789.,?'!/()&:;=+-_\"$@\0"
    as *const u8 as *const libc::c_char;
#[no_mangle]
pub static mut itu: [*const libc::c_char; 54] = [
    b"13\0" as *const u8 as *const libc::c_char,
    b"3111\0" as *const u8 as *const libc::c_char,
    b"3131\0" as *const u8 as *const libc::c_char,
    b"311\0" as *const u8 as *const libc::c_char,
    b"1\0" as *const u8 as *const libc::c_char,
    b"1131\0" as *const u8 as *const libc::c_char,
    b"331\0" as *const u8 as *const libc::c_char,
    b"1111\0" as *const u8 as *const libc::c_char,
    b"11\0" as *const u8 as *const libc::c_char,
    b"1333\0" as *const u8 as *const libc::c_char,
    b"313\0" as *const u8 as *const libc::c_char,
    b"1311\0" as *const u8 as *const libc::c_char,
    b"33\0" as *const u8 as *const libc::c_char,
    b"31\0" as *const u8 as *const libc::c_char,
    b"333\0" as *const u8 as *const libc::c_char,
    b"1331\0" as *const u8 as *const libc::c_char,
    b"3313\0" as *const u8 as *const libc::c_char,
    b"131\0" as *const u8 as *const libc::c_char,
    b"111\0" as *const u8 as *const libc::c_char,
    b"3\0" as *const u8 as *const libc::c_char,
    b"113\0" as *const u8 as *const libc::c_char,
    b"1113\0" as *const u8 as *const libc::c_char,
    b"133\0" as *const u8 as *const libc::c_char,
    b"3113\0" as *const u8 as *const libc::c_char,
    b"3133\0" as *const u8 as *const libc::c_char,
    b"3311\0" as *const u8 as *const libc::c_char,
    b"33333\0" as *const u8 as *const libc::c_char,
    b"13333\0" as *const u8 as *const libc::c_char,
    b"11333\0" as *const u8 as *const libc::c_char,
    b"11133\0" as *const u8 as *const libc::c_char,
    b"11113\0" as *const u8 as *const libc::c_char,
    b"11111\0" as *const u8 as *const libc::c_char,
    b"31111\0" as *const u8 as *const libc::c_char,
    b"33111\0" as *const u8 as *const libc::c_char,
    b"33311\0" as *const u8 as *const libc::c_char,
    b"33331\0" as *const u8 as *const libc::c_char,
    b"131313\0" as *const u8 as *const libc::c_char,
    b"331133\0" as *const u8 as *const libc::c_char,
    b"113311\0" as *const u8 as *const libc::c_char,
    b"133331\0" as *const u8 as *const libc::c_char,
    b"313133\0" as *const u8 as *const libc::c_char,
    b"31131\0" as *const u8 as *const libc::c_char,
    b"31331\0" as *const u8 as *const libc::c_char,
    b"313313\0" as *const u8 as *const libc::c_char,
    b"13111\0" as *const u8 as *const libc::c_char,
    b"333111\0" as *const u8 as *const libc::c_char,
    b"313131\0" as *const u8 as *const libc::c_char,
    b"31113\0" as *const u8 as *const libc::c_char,
    b"13131\0" as *const u8 as *const libc::c_char,
    b"311113\0" as *const u8 as *const libc::c_char,
    b"113313\0" as *const u8 as *const libc::c_char,
    b"131131\0" as *const u8 as *const libc::c_char,
    b"1113113\0" as *const u8 as *const libc::c_char,
    b"133131\0" as *const u8 as *const libc::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn append(
    mut s: *mut libc::c_char,
    mut morse: *const libc::c_char,
) {
    while *morse != 0 {
        strcat(s, dd[('3' as i32 == *morse as libc::c_int) as libc::c_int as usize]);
        morse = morse.offset(1);
        morse;
    }
    strcat(s, medium.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn translate(
    mut i: *const libc::c_char,
    mut o: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut pc: *const libc::c_char = 0 as *const libc::c_char;
    sprintf(o, b"beep\0" as *const u8 as *const libc::c_char);
    while *i != 0 {
        pc = strchr(ascii, toupper(*i as libc::c_int));
        if pc.is_null() {
            strcat(o, word.as_mut_ptr());
        } else {
            append(o, itu[pc.offset_from(ascii) as libc::c_long as usize]);
        }
        i = i.offset(1);
        i;
    }
    strcat(o, word.as_mut_ptr());
    return o;
}
unsafe fn main_0(mut ac: libc::c_int, mut av: *mut *mut libc::c_char) -> libc::c_int {
    let mut sin: [libc::c_char; 73] = [0; 73];
    let mut sout: [libc::c_char; 100000] = [0; 100000];
    let mut dit: libc::c_int = 100 as libc::c_int;
    if (1 as libc::c_int) < ac {
        if strlen(*av.offset(1 as libc::c_int as isize))
            != strspn(
                *av.offset(1 as libc::c_int as isize),
                b"0123456789\0" as *const u8 as *const libc::c_char,
            )
        {
            return 0 as libc::c_int
                * fprintf(
                    stderr,
                    b"use: %s [duration]   dit in ms, default %d\n\0" as *const u8
                        as *const libc::c_char,
                    *av,
                    dit,
                );
        }
        dit = if (1 as libc::c_int) < atoi(*av.offset(1 as libc::c_int as isize)) {
            if atoi(*av.offset(1 as libc::c_int as isize)) < 1000 as libc::c_int {
                atoi(*av.offset(1 as libc::c_int as isize))
            } else {
                1000 as libc::c_int
            }
        } else {
            1 as libc::c_int
        };
    }
    sprintf(
        dah.as_mut_ptr(),
        b" -n -f 440 -l %d -D %d\0" as *const u8 as *const libc::c_char,
        3 as libc::c_int * dit,
        dit,
    );
    sprintf(
        dih.as_mut_ptr(),
        b" -n -f 440 -l %d -D %d\0" as *const u8 as *const libc::c_char,
        dit,
        dit,
    );
    sprintf(
        medium.as_mut_ptr(),
        b" -n -D %d\0" as *const u8 as *const libc::c_char,
        (3 as libc::c_int - 1 as libc::c_int) * dit,
    );
    sprintf(
        word.as_mut_ptr(),
        b" -n -D %d\0" as *const u8 as *const libc::c_char,
        (7 as libc::c_int - (3 as libc::c_int - 1 as libc::c_int) - 1 as libc::c_int)
            * dit,
    );
    while !(fgets(sin.as_mut_ptr(), 72 as libc::c_int, stdin)).is_null() {
        puts(translate(sin.as_mut_ptr(), sout.as_mut_ptr()));
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
