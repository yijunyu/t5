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
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
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
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cidr_tag {
    pub address: uint32_t,
    pub mask_length: libc::c_uint,
}
pub type cidr_t = cidr_tag;
#[no_mangle]
pub unsafe extern "C" fn cidr_parse(
    mut str: *const libc::c_char,
    mut cidr: *mut cidr_t,
) -> bool {
    let mut a: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    if sscanf(
        str,
        b"%d.%d.%d.%d/%d\0" as *const u8 as *const libc::c_char,
        &mut a as *mut libc::c_int,
        &mut b as *mut libc::c_int,
        &mut c as *mut libc::c_int,
        &mut d as *mut libc::c_int,
        &mut m as *mut libc::c_int,
    ) != 5 as libc::c_int
    {
        return 0 as libc::c_int != 0;
    }
    if m < 1 as libc::c_int || m > 32 as libc::c_int || a < 0 as libc::c_int
        || a > 255 as libc::c_int || b < 0 as libc::c_int || b > 255 as libc::c_int
        || c < 0 as libc::c_int || c > 255 as libc::c_int || d < 0 as libc::c_int
        || d > 255 as libc::c_int
    {
        return 0 as libc::c_int != 0;
    }
    let mut mask: uint32_t = !(((1 as libc::c_int) << 32 as libc::c_int - m)
        - 1 as libc::c_int) as uint32_t;
    let mut address: uint32_t = ((a << 24 as libc::c_int) + (b << 16 as libc::c_int)
        + (c << 8 as libc::c_int) + d) as uint32_t;
    address &= mask;
    (*cidr).address = address;
    (*cidr).mask_length = m as libc::c_uint;
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn cidr_format(
    mut cidr: *const cidr_t,
    mut str: *mut libc::c_char,
    mut size: size_t,
) {
    let mut address: uint32_t = (*cidr).address;
    let mut d: libc::c_uint = address & 255 as libc::c_int as libc::c_uint;
    address >>= 8 as libc::c_int;
    let mut c: libc::c_uint = address & 255 as libc::c_int as libc::c_uint;
    address >>= 8 as libc::c_int;
    let mut b: libc::c_uint = address & 255 as libc::c_int as libc::c_uint;
    address >>= 8 as libc::c_int;
    let mut a: libc::c_uint = address & 255 as libc::c_int as libc::c_uint;
    snprintf(
        str,
        size,
        b"%u.%u.%u.%u/%u\0" as *const u8 as *const libc::c_char,
        a,
        b,
        c,
        d,
        (*cidr).mask_length,
    );
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut tests: [*const libc::c_char; 6] = [
        b"87.70.141.1/22\0" as *const u8 as *const libc::c_char,
        b"36.18.154.103/12\0" as *const u8 as *const libc::c_char,
        b"62.62.197.11/29\0" as *const u8 as *const libc::c_char,
        b"67.137.119.181/4\0" as *const u8 as *const libc::c_char,
        b"161.214.74.21/24\0" as *const u8 as *const libc::c_char,
        b"184.232.176.184/18\0" as *const u8 as *const libc::c_char,
    ];
    let mut i: libc::c_int = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[*const libc::c_char; 6]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
    {
        let mut cidr: cidr_t = cidr_t {
            address: 0,
            mask_length: 0,
        };
        if cidr_parse(tests[i as usize], &mut cidr) {
            let mut out: [libc::c_char; 32] = [0; 32];
            cidr_format(
                &mut cidr,
                out.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            );
            printf(
                b"%-18s -> %s\n\0" as *const u8 as *const libc::c_char,
                tests[i as usize],
                out.as_mut_ptr(),
            );
        } else {
            fprintf(
                stderr,
                b"%s: invalid CIDR\n\0" as *const u8 as *const libc::c_char,
                tests[i as usize],
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
