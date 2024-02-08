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
    fn perror(__s: *const libc::c_char);
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn log10f(_: libc::c_float) -> libc::c_float;
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
pub unsafe extern "C" fn benford_distribution() -> *mut libc::c_float {
    static mut prob: [libc::c_float; 9] = [0.; 9];
    let mut i: libc::c_int = 1 as libc::c_int;
    while i < 10 as libc::c_int {
        prob[(i - 1 as libc::c_int)
            as usize] = log10f(
            (1 as libc::c_int as libc::c_double + 1.0f64 / i as libc::c_double)
                as libc::c_float,
        );
        i += 1;
        i;
    }
    return prob.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn get_actual_distribution(
    mut fn_0: *mut libc::c_char,
) -> *mut libc::c_float {
    let mut input: *mut FILE = fopen(fn_0, b"r\0" as *const u8 as *const libc::c_char);
    if input.is_null() {
        perror(b"Can't open file\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    let mut tally: [libc::c_int; 9] = [0 as libc::c_int, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut c: libc::c_char = 0;
    let mut total: libc::c_int = 0 as libc::c_int;
    loop {
        c = getc(input) as libc::c_char;
        if !(c as libc::c_int != -(1 as libc::c_int)) {
            break;
        }
        while (c as libc::c_int) < '1' as i32 || c as libc::c_int > '9' as i32 {
            c = getc(input) as libc::c_char;
        }
        tally[(c as libc::c_int - '1' as i32) as usize] += 1;
        tally[(c as libc::c_int - '1' as i32) as usize];
        total += 1;
        total;
        loop {
            c = getc(input) as libc::c_char;
            if !(c as libc::c_int != '\n' as i32
                && c as libc::c_int != -(1 as libc::c_int))
            {
                break;
            }
        }
    }
    fclose(input);
    static mut freq: [libc::c_float; 9] = [0.; 9];
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 9 as libc::c_int {
        freq[i as usize] = tally[i as usize] as libc::c_float / total as libc::c_float;
        i += 1;
        i;
    }
    return freq.as_mut_ptr();
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    if argc != 2 as libc::c_int {
        printf(b"Usage: benford <file>\n\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    let mut actual: *mut libc::c_float = get_actual_distribution(
        *argv.offset(1 as libc::c_int as isize),
    );
    let mut expected: *mut libc::c_float = benford_distribution();
    puts(b"digit\tactual\texpected\0" as *const u8 as *const libc::c_char);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 9 as libc::c_int {
        printf(
            b"%d\t%.3f\t%.3f\n\0" as *const u8 as *const libc::c_char,
            i + 1 as libc::c_int,
            *actual.offset(i as isize) as libc::c_double,
            *expected.offset(i as isize) as libc::c_double,
        );
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
