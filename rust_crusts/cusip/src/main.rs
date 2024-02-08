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
    fn fscanf(_: *mut FILE, _: *const i8, _: ...) -> i32;
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
pub extern "C" fn cusipCheck(mut str: *mut i8) -> i32 {
    unsafe {
        let mut sum: i32 = 0;
        let mut i: i32 = 0;
        let mut v: i32 = 0;
        i = 0;
        while i < 8 {
            if *str.offset(i as isize) as i32 >= '0' as i32
                && *str.offset(i as isize) as i32 <= '9' as i32
            {
                v = *str.offset(i as isize) as i32 - '0' as i32;
            } else if *str.offset(i as isize) as i32 >= 'A' as i32
                && *str.offset(i as isize) as i32 <= 'Z' as i32
            {
                v = *str.offset(i as isize) as i32 - 'A' as i32 + 10;
            } else if *str.offset(i as isize) as i32 == '*' as i32 {
                v = 36;
            } else if *str.offset(i as isize) as i32 == '@' as i32 {
                v = 37;
            } else if *str.offset(i as isize) as i32 == '#' as i32 {
                v = 38;
            }
            if i % 2 != 0 {
                v *= 2;
            }
            sum += v / 10 + v % 10;
            i += 1;
            i;
        }
        return (10 - sum % 10) % 10;
    }
}

fn main_0(mut argC: i32, mut argV: *mut *mut i8) -> i32 {
    unsafe {
        let mut cusipStr: [i8; 10] = [0; 10];
        let mut i: i32 = 0;
        let mut numLines: i32 = 0;
        if argC == 1 {
            print!(
                "Usage : {} <full path of CUSIP Data file>",
                build_str_from_raw_ptr(*argV.offset(0 as isize) as *mut u8)
            );
        } else {
            let mut fp: *mut FILE =
                fopen(*argV.offset(1 as isize), b"r\0" as *const u8 as *const i8);
            fscanf(
                fp,
                b"%d\0" as *const u8 as *const i8,
                &mut numLines as *mut i32,
            );
            print!("CUSIP       Verdict\n");
            print!("-------------------");
            i = 0;
            while i < numLines {
                fscanf(fp, b"%s\0" as *const u8 as *const i8, cusipStr.as_mut_ptr());
                if cusipCheck(cusipStr.as_mut_ptr()) == cusipStr[8 as usize] as i32 - '0' as i32 {
                    print!(
                        "\n{} : {}",
                        build_str_from_raw_ptr(cusipStr.as_mut_ptr() as *mut u8),
                        "Valid\0"
                    )
                } else {
                    print!(
                        "\n{} : {}",
                        build_str_from_raw_ptr(cusipStr.as_mut_ptr() as *mut u8),
                        "Invalid\0"
                    )
                };
                i += 1;
                i;
            }
            fclose(fp);
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
