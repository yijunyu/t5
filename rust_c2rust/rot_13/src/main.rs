#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
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
static mut rot13_table: [libc::c_char; 256] = [0; 256];
unsafe extern "C" fn init_rot13_table() {
    static mut upper: [libc::c_uchar; 27] = unsafe {
        *::core::mem::transmute::<
            &[u8; 27],
            &[libc::c_uchar; 27],
        >(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\0")
    };
    static mut lower: [libc::c_uchar; 27] = unsafe {
        *::core::mem::transmute::<
            &[u8; 27],
            &[libc::c_uchar; 27],
        >(b"abcdefghijklmnopqrstuvwxyz\0")
    };
    let mut ch: libc::c_int = '\0' as i32;
    while ch <= 127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int {
        rot13_table[ch as usize] = ch as libc::c_char;
        ch += 1;
        ch;
    }
    let mut p: *const libc::c_uchar = upper.as_ptr();
    while *p.offset(13 as libc::c_int as isize) as libc::c_int != '\0' as i32 {
        rot13_table[*p.offset(0 as libc::c_int as isize)
            as usize] = *p.offset(13 as libc::c_int as isize) as libc::c_char;
        rot13_table[*p.offset(13 as libc::c_int as isize)
            as usize] = *p.offset(0 as libc::c_int as isize) as libc::c_char;
        p = p.offset(1);
        p;
    }
    let mut p_0: *const libc::c_uchar = lower.as_ptr();
    while *p_0.offset(13 as libc::c_int as isize) as libc::c_int != '\0' as i32 {
        rot13_table[*p_0.offset(0 as libc::c_int as isize)
            as usize] = *p_0.offset(13 as libc::c_int as isize) as libc::c_char;
        rot13_table[*p_0.offset(13 as libc::c_int as isize)
            as usize] = *p_0.offset(0 as libc::c_int as isize) as libc::c_char;
        p_0 = p_0.offset(1);
        p_0;
    }
}
unsafe extern "C" fn rot13_file(mut fp: *mut FILE) {
    let mut ch: libc::c_int = 0;
    loop {
        ch = fgetc(fp);
        if !(ch != -(1 as libc::c_int)) {
            break;
        }
        fputc(rot13_table[ch as usize] as libc::c_int, stdout);
    };
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    init_rot13_table();
    if argc > 1 as libc::c_int {
        let mut i: libc::c_int = 1 as libc::c_int;
        while i < argc {
            let mut fp: *mut FILE = fopen(
                *argv.offset(i as isize),
                b"r\0" as *const u8 as *const libc::c_char,
            );
            if fp.is_null() {
                perror(*argv.offset(i as isize));
                return 1 as libc::c_int;
            }
            rot13_file(fp);
            fclose(fp);
            i += 1;
            i;
        }
    } else {
        rot13_file(stdin);
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
