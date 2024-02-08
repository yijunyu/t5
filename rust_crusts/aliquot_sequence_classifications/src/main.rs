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
    fn strtoull(_: *const i8, _: *mut *mut i8, _: i32) -> u64;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fopen(_: *const i8, _: *const i8) -> *mut FILE;
    fn fgets(__s: *mut i8, __n: i32, __stream: *mut FILE) -> *mut i8;
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
pub extern "C" fn bruteForceProperDivisorSum(mut n: u64) -> u64 {
    let mut i: u64 = 0;
    let mut sum: u64 = 0;
    i = 1;
    while i < n.wrapping_add(1).wrapping_div(2) {
        if n.wrapping_rem(i) == 0 && n != i {
            sum = sum.wrapping_add(i);
        }
        i = i.wrapping_add(1);
        i;
    }
    return sum;
}

#[no_mangle]
pub extern "C" fn printSeries(mut arr: *mut u64, mut size: i32, mut type_0: *mut i8) {
    unsafe {
        let mut i: i32 = 0;
        print!(
            "\nInteger : {}, Type : {}, Series : ",
            *arr.offset(0 as isize),
            build_str_from_raw_ptr(type_0 as *mut u8)
        );
        i = 0;
        while i < size - 1 {
            print!("{}, ", *arr.offset(i as isize));
            i += 1;
            i;
        }
        print!("{}", *arr.offset(i as isize));
    }
}

#[no_mangle]
pub extern "C" fn aliquotClassifier(mut n: u64) {
    let mut arr: [u64; 16] = [0; 16];
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    arr[0 as usize] = n;
    i = 1;
    while i < 16 {
        arr[i as usize] = bruteForceProperDivisorSum(arr[(i - 1i32) as usize]);
        if arr[i as usize] == 0
            || arr[i as usize] == n
            || arr[i as usize] == arr[(i - 1i32) as usize] && arr[i as usize] != n
        {
            printSeries(
                arr.as_mut_ptr(),
                i + 1,
                (if arr[i as usize] == 0u64 {
                    b"Terminating\0" as *const u8 as *const i8
                } else if arr[i as usize] == n && i == 1i32 {
                    b"Perfect\0" as *const u8 as *const i8
                } else if arr[i as usize] == n && i == 2 {
                    b"Amicable\0" as *const u8 as *const i8
                } else if arr[i as usize] == arr[(i - 1i32) as usize] && arr[i as usize] != n {
                    b"Aspiring\0" as *const u8 as *const i8
                } else {
                    b"Sociable\0" as *const u8 as *const i8
                }) as *mut i8,
            );
            return;
        }
        j = 1;
        while j < i {
            if arr[j as usize] == arr[i as usize] {
                printSeries(
                    arr.as_mut_ptr(),
                    i + 1,
                    b"Cyclic\0" as *const u8 as *const i8 as *mut i8,
                );
                return;
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    printSeries(
        arr.as_mut_ptr(),
        i + 1,
        b"Non-Terminating\0" as *const u8 as *const i8 as *mut i8,
    );
}

#[no_mangle]
pub extern "C" fn processFile(mut fileName: *mut i8) {
    unsafe {
        let mut fp: *mut FILE = fopen(fileName, b"r\0" as *const u8 as *const i8);
        let mut str: [i8; 21] = [0; 21];
        while !(fgets(str.as_mut_ptr(), 21, fp)).is_null() {
            aliquotClassifier(strtoull(
                str.as_mut_ptr(),
                0 as *mut libc::c_void as *mut *mut i8,
                10,
            ));
        }
        fclose(fp);
    }
}

fn main_0(mut argC: i32, mut argV: *mut *mut i8) -> i32 {
    unsafe {
        if argC != 2 {
            print!(
                "Usage : {} <positive integer>",
                build_str_from_raw_ptr(*argV.offset(0 as isize) as *mut u8)
            );
        } else if !(strchr(*argV.offset(1 as isize), '.' as i32)).is_null() {
            processFile(*argV.offset(1 as isize));
        } else {
            aliquotClassifier(strtoull(
                *argV.offset(1 as isize),
                0 as *mut libc::c_void as *mut *mut i8,
                10,
            ));
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
