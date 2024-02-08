#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strtoumax(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
    ) -> uintmax_t;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn open_memstream(
        __bufloc: *mut *mut libc::c_char,
        __sizeloc: *mut size_t,
    ) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn free(_: *mut libc::c_void);
}
pub type __uintmax_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type uintmax_t = __uintmax_t;
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
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut input: uintmax_t = 0;
    let mut a: *mut libc::c_char = 0 as *mut libc::c_char;
    if argc < 2 as libc::c_int {
        printf(
            b"usage: %s #seconds\n\0" as *const u8 as *const libc::c_char,
            *argv.offset(0 as libc::c_int as isize),
        );
        return 1 as libc::c_int;
    }
    input = strtoumax(
        *argv.offset(1 as libc::c_int as isize),
        0 as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    if input < 1 as libc::c_int as libc::c_ulong {
        printf(
            b"Bad input: %s\n\0" as *const u8 as *const libc::c_char,
            *argv.offset(1 as libc::c_int as isize),
        );
        printf(
            b"usage: %s #seconds\n\0" as *const u8 as *const libc::c_char,
            *argv.offset(0 as libc::c_int as isize),
        );
        return 1 as libc::c_int;
    }
    printf(b"Number entered: %lu\n\0" as *const u8 as *const libc::c_char, input);
    a = format_sec(input);
    printf(a);
    free(a as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn format_sec(mut input: uintmax_t) -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    let mut first: bool = false;
    let mut weeks: uintmax_t = 0;
    let mut days: uintmax_t = 0;
    let mut hours: uintmax_t = 0;
    let mut mins: uintmax_t = 0;
    let mut retval: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut stream: *mut FILE = 0 as *mut FILE;
    let mut size: size_t = 0;
    let mut traverse: [*mut uintmax_t; 5] = [
        &mut weeks,
        &mut days,
        &mut hours,
        &mut mins,
        &mut input,
    ];
    let mut labels: [*mut libc::c_char; 5] = [
        b"wk\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"hr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"min\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"sec\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ];
    weeks = sec_to_week(input);
    input = input.wrapping_sub(week_to_sec(weeks));
    days = sec_to_day(input);
    input = input.wrapping_sub(day_to_sec(days));
    hours = sec_to_hour(input);
    input = input.wrapping_sub(hour_to_sec(hours));
    mins = sec_to_min(input);
    input = input.wrapping_sub(min_to_sec(mins));
    stream = open_memstream(&mut retval, &mut size);
    if stream.is_null() {
        fprintf(
            stderr,
            b"Unable to allocate memory\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut libc::c_char;
    }
    first = 1 as libc::c_int != 0;
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        if *traverse[i as usize] != 0 as libc::c_int as libc::c_ulong {
            if !first {
                fprintf(
                    stream,
                    b", %lu %s\0" as *const u8 as *const libc::c_char,
                    *traverse[i as usize],
                    labels[i as usize],
                );
            } else {
                fprintf(
                    stream,
                    b"%lu %s\0" as *const u8 as *const libc::c_char,
                    *traverse[i as usize],
                    labels[i as usize],
                );
            }
            fflush(stream);
            first = 0 as libc::c_int != 0;
        }
        i += 1;
        i;
    }
    fprintf(stream, b"\n\0" as *const u8 as *const libc::c_char);
    fclose(stream);
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn sec_to_week(mut seconds: uintmax_t) -> uintmax_t {
    return (sec_to_day(seconds)).wrapping_div(7 as libc::c_int as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn sec_to_day(mut seconds: uintmax_t) -> uintmax_t {
    return (sec_to_hour(seconds)).wrapping_div(24 as libc::c_int as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn sec_to_hour(mut seconds: uintmax_t) -> uintmax_t {
    return (sec_to_min(seconds)).wrapping_div(60 as libc::c_int as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn sec_to_min(mut seconds: uintmax_t) -> uintmax_t {
    return seconds.wrapping_div(60 as libc::c_int as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn week_to_sec(mut weeks: uintmax_t) -> uintmax_t {
    return day_to_sec(weeks.wrapping_mul(7 as libc::c_int as libc::c_ulong));
}
#[no_mangle]
pub unsafe extern "C" fn day_to_sec(mut days: uintmax_t) -> uintmax_t {
    return hour_to_sec(days.wrapping_mul(24 as libc::c_int as libc::c_ulong));
}
#[no_mangle]
pub unsafe extern "C" fn hour_to_sec(mut hours: uintmax_t) -> uintmax_t {
    return min_to_sec(hours.wrapping_mul(60 as libc::c_int as libc::c_ulong));
}
#[no_mangle]
pub unsafe extern "C" fn min_to_sec(mut minutes: uintmax_t) -> uintmax_t {
    return minutes.wrapping_mul(60 as libc::c_int as libc::c_ulong);
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
