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
    fn rand() -> i32;
    fn sqrt(_: f64) -> f64;
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
pub extern "C" fn pi(mut tolerance: f64) -> f64 {
    let mut x: f64 = 0.;
    let mut y: f64 = 0.;
    let mut val: f64 = 0.;
    let mut error: f64 = 0.;
    let mut sampled: u64 = 0;
    let mut hit: u64 = 0;
    let mut i: u64 = 0;
    unsafe {
        loop {
            i = 1000000;
            while i != 0 {
                x = rand() as f64 / (2147483647 as f64 + 1.0f64);
                y = rand() as f64 / (2147483647 as f64 + 1.0f64);
                if x * x + y * y < 1 as f64 {
                    hit = hit.wrapping_add(1);
                    hit;
                }
                i = i.wrapping_sub(1);
                i;
                sampled = sampled.wrapping_add(1);
                sampled;
            }
            val = hit as f64 / sampled as f64;
            error = sqrt(val * (1 as f64 - val) / sampled as f64) * 4 as f64;
            val *= 4 as f64;
            fprintf(
                stderr,
                b"Pi = %f +/- %5.3e at %ldM samples.\r\0" as *const u8 as *const i8,
                val,
                error,
                sampled.wrapping_div(1000000),
            );
            if !(hit == 0 || error > tolerance) {
                break;
            }
        }
    }
    return val;
}

fn main_0() -> i32 {
    print!("Pi is {}\n", pi(3e-4f64));
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
