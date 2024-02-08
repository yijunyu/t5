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
    fn fclose(__stream: *mut FILE) -> i32;
    fn fopen(_: *const i8, _: *const i8) -> *mut FILE;
    fn getline(__lineptr: *mut *mut i8, __n: *mut u64, __stream: *mut FILE) -> i64;
    fn strrchr(_: *const i8, _: i32) -> *mut i8;
    fn atof(__nptr: *const i8) -> f64;
    fn free(_: *mut libc::c_void);
    fn exit(_: i32) -> !;
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
fn main_0() -> i32 {
    unsafe {
        let mut fp: *mut FILE = 0 as *mut FILE;
        let mut line: *mut i8 = 0 as *mut i8;
        let mut len: u64 = 0;
        let mut read: i64 = 0;
        let mut lw: *mut i8 = 0 as *mut i8;
        let mut lt: *mut i8 = 0 as *mut i8;
        fp = fopen(
            b"data.txt\0" as *const u8 as *const i8,
            b"r\0" as *const u8 as *const i8,
        );
        if fp.is_null() {
            print!("Unable to open file\n");
            exit(1);
        }
        print!("Those earthquakes with a magnitude > 6.0 are:\n\n");
        loop {
            read = getline(&mut line, &mut len, fp);
            if !(read != -1 as i64) {
                break;
            }
            if read < 2 {
                continue;
            }
            lw = strrchr(line, ' ' as i32);
            lt = strrchr(line, '\t' as i32);
            if lw.is_null() && lt.is_null() {
                continue;
            }
            if lt > lw {
                lw = lt;
            }
            if atof(lw.offset(1 as isize)) > 6.0f64 {
                print!("{}", build_str_from_raw_ptr(line as *mut u8));
            }
        }
        fclose(fp);
        if !line.is_null() {
            free(line as *mut libc::c_void);
        }
        return 0;
    }
}

pub fn main() {
    unsafe {
        ::std::process::exit(main_0() as i32);
    }
}
