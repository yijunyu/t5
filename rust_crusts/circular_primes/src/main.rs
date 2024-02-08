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
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: i32) -> !;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
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
pub extern "C" fn is_prime(mut n: u32) -> bool {
    if n == 2 {
        return 1 != 0;
    }
    if n < 2 || n.wrapping_rem(2) == 0 {
        return 0 != 0;
    }
    let mut p: u32 = 3;
    while p.wrapping_mul(p) <= n {
        if n.wrapping_rem(p) == 0 {
            return 0 != 0;
        }
        p = (p).wrapping_add(2) as u32;
    }
    return 1 != 0;
}

#[no_mangle]
pub extern "C" fn cycle(mut n: u32) -> u32 {
    let mut m: u32 = n;
    let mut p: u32 = 1;
    while m >= 10 {
        p = (p).wrapping_mul(10) as u32;
        m = (m).wrapping_div(10) as u32;
    }
    return m.wrapping_add(10u32.wrapping_mul(n.wrapping_rem(p)));
}

#[no_mangle]
pub extern "C" fn is_circular_prime(mut p: u32) -> bool {
    if !is_prime(p) {
        return 0 != 0;
    }
    let mut p2: u32 = cycle(p);
    while p2 != p {
        if p2 < p || !is_prime(p2) {
            return 0 != 0;
        }
        p2 = cycle(p2);
    }
    return 1 != 0;
}

pub fn main() {
    unsafe {
        ::std::process::exit(main_0() as i32);
    }
}
