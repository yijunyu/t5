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
    fn strtoumax(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> u64;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fflush(__stream: *mut FILE) -> i32;
    fn open_memstream(__bufloc: *mut *mut i8, __sizeloc: *mut u64) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn printf(_: *const i8, _: ...) -> i32;
    fn free(_: *mut libc::c_void);
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
fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    unsafe {
        let mut input: u64 = 0;
        let mut a: *mut i8 = 0 as *mut i8;
        if argc < 2 {
            print!(
                "usage: {} #seconds\n",
                build_str_from_raw_ptr(*argv.offset(0 as isize) as *mut u8)
            );
            return 1;
        }
        input = strtoumax(*argv.offset(1 as isize), 0 as *mut *mut i8, 10);
        if input < 1 {
            print!(
                "Bad input: {}\n",
                build_str_from_raw_ptr(*argv.offset(1 as isize) as *mut u8)
            );
            print!(
                "usage: {} #seconds\n",
                build_str_from_raw_ptr(*argv.offset(0 as isize) as *mut u8)
            );
            return 1;
        }
        print!("Number entered: {}\n", input);
        a = format_sec(input);
        printf(a);
        free(a as *mut libc::c_void);
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn format_sec(mut input: u64) -> *mut i8 {
    unsafe {
        let mut i: i32 = 0;
        let mut first: bool = false;
        let mut weeks: u64 = 0;
        let mut days: u64 = 0;
        let mut hours: u64 = 0;
        let mut mins: u64 = 0;
        let mut retval: *mut i8 = 0 as *mut i8;
        let mut stream: *mut FILE = 0 as *mut FILE;
        let mut size: u64 = 0;
        let mut traverse: [*mut u64; 5] =
            [&mut weeks, &mut days, &mut hours, &mut mins, &mut input];
        let mut labels: [*mut i8; 5] = [
            b"wk\0" as *const u8 as *const i8 as *mut i8,
            b"d\0" as *const u8 as *const i8 as *mut i8,
            b"hr\0" as *const u8 as *const i8 as *mut i8,
            b"min\0" as *const u8 as *const i8 as *mut i8,
            b"sec\0" as *const u8 as *const i8 as *mut i8,
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
                b"Unable to allocate memory\0" as *const u8 as *const i8,
            );
            return 0 as *mut i8;
        }
        first = 1 != 0;
        i = 0;
        while i < 5 {
            if *traverse[i as usize] != 0 {
                if !first {
                    fprintf(
                        stream,
                        b", %lu %s\0" as *const u8 as *const i8,
                        *traverse[i as usize],
                        labels[i as usize],
                    );
                } else {
                    fprintf(
                        stream,
                        b"%lu %s\0" as *const u8 as *const i8,
                        *traverse[i as usize],
                        labels[i as usize],
                    );
                }
                fflush(stream);
                first = 0 != 0;
            }
            i += 1;
            i;
        }
        fprintf(stream, b"\n\0" as *const u8 as *const i8);
        fclose(stream);
        return retval;
    }
}

#[no_mangle]
pub extern "C" fn sec_to_week(mut seconds: u64) -> u64 {
    return (sec_to_day(seconds)).wrapping_div(7);
}

#[no_mangle]
pub extern "C" fn sec_to_day(mut seconds: u64) -> u64 {
    return (sec_to_hour(seconds)).wrapping_div(24);
}

#[no_mangle]
pub extern "C" fn sec_to_hour(mut seconds: u64) -> u64 {
    return (sec_to_min(seconds)).wrapping_div(60);
}

#[no_mangle]
pub extern "C" fn sec_to_min(mut seconds: u64) -> u64 {
    return seconds.wrapping_div(60);
}

#[no_mangle]
pub extern "C" fn week_to_sec(mut weeks: u64) -> u64 {
    return day_to_sec(weeks.wrapping_mul(7));
}

#[no_mangle]
pub extern "C" fn day_to_sec(mut days: u64) -> u64 {
    return hour_to_sec(days.wrapping_mul(24));
}

#[no_mangle]
pub extern "C" fn hour_to_sec(mut hours: u64) -> u64 {
    return min_to_sec(hours.wrapping_mul(60));
}

#[no_mangle]
pub extern "C" fn min_to_sec(mut minutes: u64) -> u64 {
    return minutes.wrapping_mul(60);
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
