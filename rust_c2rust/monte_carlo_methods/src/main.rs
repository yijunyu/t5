#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn rand() -> libc::c_int;
    fn sqrt(_: libc::c_double) -> libc::c_double;
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
pub unsafe extern "C" fn pi(mut tolerance: libc::c_double) -> libc::c_double {
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    let mut val: libc::c_double = 0.;
    let mut error: libc::c_double = 0.;
    let mut sampled: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut hit: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut i: libc::c_ulong = 0;
    loop {
        i = 1000000 as libc::c_int as libc::c_ulong;
        while i != 0 {
            x = rand() as libc::c_double
                / (2147483647 as libc::c_int as libc::c_double + 1.0f64);
            y = rand() as libc::c_double
                / (2147483647 as libc::c_int as libc::c_double + 1.0f64);
            if x * x + y * y < 1 as libc::c_int as libc::c_double {
                hit = hit.wrapping_add(1);
                hit;
            }
            i = i.wrapping_sub(1);
            i;
            sampled = sampled.wrapping_add(1);
            sampled;
        }
        val = hit as libc::c_double / sampled as libc::c_double;
        error = sqrt(
            val * (1 as libc::c_int as libc::c_double - val) / sampled as libc::c_double,
        ) * 4 as libc::c_int as libc::c_double;
        val *= 4 as libc::c_int as libc::c_double;
        fprintf(
            stderr,
            b"Pi = %f +/- %5.3e at %ldM samples.\r\0" as *const u8
                as *const libc::c_char,
            val,
            error,
            sampled.wrapping_div(1000000 as libc::c_int as libc::c_ulong),
        );
        if !(hit == 0 || error > tolerance) {
            break;
        }
    }
    return val;
}
unsafe fn main_0() -> libc::c_int {
    printf(b"Pi is %f\n\0" as *const u8 as *const libc::c_char, pi(3e-4f64));
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
