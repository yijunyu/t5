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
    fn malloc(_: u64) -> *mut libc::c_void;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fopen(_: *const i8, _: *const i8) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn fwrite(_: *const libc::c_void, _: u64, _: u64, _: *mut FILE) -> u64;
    fn fabs(_: f64) -> f64;
    fn fmod(_: f64, _: f64) -> f64;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
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
pub extern "C" fn hue_to_rgb(mut hue: f64, mut sat: f64, mut p: *mut u8) {
    unsafe {
        let mut x: f64 = 0.;
        let mut c: i32 = (255 as f64 * sat) as i32;
        hue /= 60 as f64;
        x = (1 as f64 - fabs(fmod(hue, 2 as f64) - 1 as f64)) * 255 as f64;
        match hue as i32 {
            0 => {
                *p.offset(0 as isize) = c as u8;
                *p.offset(1 as isize) = x as u8;
                *p.offset(2 as isize) = 0;
                return;
            }
            1 => {
                *p.offset(0 as isize) = x as u8;
                *p.offset(1 as isize) = c as u8;
                *p.offset(2 as isize) = 0;
                return;
            }
            2 => {
                *p.offset(0 as isize) = 0;
                *p.offset(1 as isize) = c as u8;
                *p.offset(2 as isize) = x as u8;
                return;
            }
            3 => {
                *p.offset(0 as isize) = 0;
                *p.offset(1 as isize) = x as u8;
                *p.offset(2 as isize) = c as u8;
                return;
            }
            4 => {
                *p.offset(0 as isize) = x as u8;
                *p.offset(1 as isize) = 0;
                *p.offset(2 as isize) = c as u8;
                return;
            }
            5 => {
                *p.offset(0 as isize) = c as u8;
                *p.offset(1 as isize) = 0;
                *p.offset(2 as isize) = x as u8;
                return;
            }
            _ => {}
        };
    }
}

fn main_0() -> i32 {
    unsafe {
        let size: i32 = 512;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut colors: *mut u8 = malloc((size * 3i32) as u64) as *mut u8;
        let mut pix: *mut u8 = malloc((size * size * 3i32) as u64) as *mut u8;
        let mut p: *mut u8 = 0 as *mut u8;
        let mut fp: *mut FILE = 0 as *mut FILE;
        i = 0;
        while i < size {
            hue_to_rgb(
                i as f64 * 240.0f64 / size as f64,
                i as f64 * 1.0f64 / size as f64,
                colors.offset((3 * i) as isize),
            );
            i += 1;
            i;
        }
        i = 0;
        p = pix;
        while i < size {
            j = 0;
            while j < size {
                memcpy(
                    p as *mut libc::c_void,
                    colors.offset(((i ^ j) * 3i32) as isize) as *const libc::c_void,
                    3,
                );
                j += 1;
                j;
                p = p.offset(3 as isize);
            }
            i += 1;
            i;
        }
        fp = fopen(
            b"xor.ppm\0" as *const u8 as *const i8,
            b"wb\0" as *const u8 as *const i8,
        );
        fprintf(
            fp,
            b"P6\n%d %d\n255\n\0" as *const u8 as *const i8,
            size,
            size,
        );
        fwrite(
            pix as *const libc::c_void,
            (size * size * 3i32) as u64,
            1,
            fp,
        );
        fclose(fp);
        return 0;
    }
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
