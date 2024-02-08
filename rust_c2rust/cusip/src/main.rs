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
    fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
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
pub unsafe extern "C" fn cusipCheck(mut str: *mut libc::c_char) -> libc::c_int {
    let mut sum: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        if *str.offset(i as isize) as libc::c_int >= '0' as i32
            && *str.offset(i as isize) as libc::c_int <= '9' as i32
        {
            v = *str.offset(i as isize) as libc::c_int - '0' as i32;
        } else if *str.offset(i as isize) as libc::c_int >= 'A' as i32
            && *str.offset(i as isize) as libc::c_int <= 'Z' as i32
        {
            v = *str.offset(i as isize) as libc::c_int - 'A' as i32 + 10 as libc::c_int;
        } else if *str.offset(i as isize) as libc::c_int == '*' as i32 {
            v = 36 as libc::c_int;
        } else if *str.offset(i as isize) as libc::c_int == '@' as i32 {
            v = 37 as libc::c_int;
        } else if *str.offset(i as isize) as libc::c_int == '#' as i32 {
            v = 38 as libc::c_int;
        }
        if i % 2 as libc::c_int != 0 as libc::c_int {
            v *= 2 as libc::c_int;
        }
        sum += v / 10 as libc::c_int + v % 10 as libc::c_int;
        i += 1;
        i;
    }
    return (10 as libc::c_int - sum % 10 as libc::c_int) % 10 as libc::c_int;
}
unsafe fn main_0(
    mut argC: libc::c_int,
    mut argV: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut cusipStr: [libc::c_char; 10] = [0; 10];
    let mut i: libc::c_int = 0;
    let mut numLines: libc::c_int = 0;
    if argC == 1 as libc::c_int {
        printf(
            b"Usage : %s <full path of CUSIP Data file>\0" as *const u8
                as *const libc::c_char,
            *argV.offset(0 as libc::c_int as isize),
        );
    } else {
        let mut fp: *mut FILE = fopen(
            *argV.offset(1 as libc::c_int as isize),
            b"r\0" as *const u8 as *const libc::c_char,
        );
        fscanf(
            fp,
            b"%d\0" as *const u8 as *const libc::c_char,
            &mut numLines as *mut libc::c_int,
        );
        printf(b"CUSIP       Verdict\n\0" as *const u8 as *const libc::c_char);
        printf(b"-------------------\0" as *const u8 as *const libc::c_char);
        i = 0 as libc::c_int;
        while i < numLines {
            fscanf(
                fp,
                b"%s\0" as *const u8 as *const libc::c_char,
                cusipStr.as_mut_ptr(),
            );
            printf(
                b"\n%s : %s\0" as *const u8 as *const libc::c_char,
                cusipStr.as_mut_ptr(),
                if cusipCheck(cusipStr.as_mut_ptr())
                    == cusipStr[8 as libc::c_int as usize] as libc::c_int - '0' as i32
                {
                    b"Valid\0" as *const u8 as *const libc::c_char
                } else {
                    b"Invalid\0" as *const u8 as *const libc::c_char
                },
            );
            i += 1;
            i;
        }
        fclose(fp);
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
