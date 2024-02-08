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
fn build_str_from_raw_ptr(raw_ptr: *mut u8) -> String {
    unsafe {
        let mut str_size: usize = 0;
        while *raw_ptr.offset(str_size as isize) != 0 {
            str_size += 1;
        }
        return std::str::from_utf8_unchecked(std::slice::from_raw_parts(raw_ptr, str_size))
            .to_owned();
    }
}

use c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn atof(__nptr: *const i8) -> f64;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fopen(_: *const i8, _: *const i8) -> *mut FILE;
    fn fscanf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn fmod(_: f64, _: f64) -> f64;
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
pub extern "C" fn processFile(mut name: *mut i8) {
    unsafe {
        let mut i: i32 = 0;
        let mut records: i32 = 0;
        let mut diff: f64 = 0.;
        let mut b1: f64 = 0.;
        let mut b2: f64 = 0.;
        let mut fp: *mut FILE = fopen(name, b"r\0" as *const u8 as *const i8);
        fscanf(
            fp,
            b"%d\n\0" as *const u8 as *const i8,
            &mut records as *mut i32,
        );
        i = 0;
        while i < records {
            fscanf(
                fp,
                b"%lf%lf\0" as *const u8 as *const i8,
                &mut b1 as *mut f64,
                &mut b2 as *mut f64,
            );
            diff = fmod(b2 - b1, 360.0f64);
            if diff < -180i32 as f64 {
                print!(
                    "\nDifference between b2({}) and b1({}) is {}",
                    b2,
                    b1,
                    diff + 360 as f64
                )
            } else {
                if diff >= 180 as f64 {
                    print!(
                        "\nDifference between b2({}) and b1({}) is {}",
                        b2,
                        b1,
                        diff - 360 as f64
                    )
                } else {
                    print!("\nDifference between b2({}) and b1({}) is {}", b2, b1, diff)
                }
            };
            i += 1;
            i;
        }
        fclose(fp);
    }
}

fn main_0(mut argC: i32, mut argV: *mut *mut i8) -> i32 {
    unsafe {
        let mut diff: f64 = 0.;
        if argC < 2 {
            print! ("Usage : {} <bearings separated by a space OR full file name which contains the bearing list>", build_str_from_raw_ptr (* argV.offset (0 as isize) as * mut u8));
        } else if argC == 2 {
            processFile(*argV.offset(1 as isize));
        } else {
            diff = fmod(
                atof(*argV.offset(2 as isize)) - atof(*argV.offset(1 as isize)),
                360.0f64,
            );
            if diff < -180i32 as f64 {
                print!(
                    "Difference between b2({}) and b1({}) is {}",
                    build_str_from_raw_ptr(*argV.offset(2 as isize) as *mut u8),
                    build_str_from_raw_ptr(*argV.offset(1 as isize) as *mut u8),
                    diff + 360 as f64
                )
            } else {
                if diff >= 180 as f64 {
                    print!(
                        "Difference between b2({}) and b1({}) is {}",
                        build_str_from_raw_ptr(*argV.offset(2 as isize) as *mut u8),
                        build_str_from_raw_ptr(*argV.offset(1 as isize) as *mut u8),
                        diff - 360 as f64
                    )
                } else {
                    print!(
                        "Difference between b2({}) and b1({}) is {}",
                        build_str_from_raw_ptr(*argV.offset(2 as isize) as *mut u8),
                        build_str_from_raw_ptr(*argV.offset(1 as isize) as *mut u8),
                        diff
                    )
                }
            };
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
