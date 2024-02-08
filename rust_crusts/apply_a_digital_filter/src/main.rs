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
    fn malloc(_: u64) -> *mut libc::c_void;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcat(_: *mut i8, _: *const i8) -> *mut i8;
    fn strtok(_: *mut i8, _: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fopen(_: *const i8, _: *const i8) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vector {
    pub values: *mut libc::c_float,
    pub size: i32,
}
#[no_mangle]
pub extern "C" fn extractVector(mut str: *mut i8) -> vector {
    unsafe {
        let mut coeff: vector = vector {
            values: 0 as *mut libc::c_float,
            size: 0,
        };
        let mut i: i32 = 0;
        let mut count: i32 = 1;
        let mut token: *mut i8 = 0 as *mut i8;
        while *str.offset(i as isize) as i32 != 0 {
            let fresh0 = i;
            i = i + 1;
            if *str.offset(fresh0 as isize) as i32 == ' ' as i32 {
                count += 1;
                count;
            }
        }
        coeff.values =
            malloc((count as u64).wrapping_mul(::core::mem::size_of::<libc::c_float>() as u64))
                as *mut libc::c_float;
        coeff.size = count;
        token = strtok(str, b" \0" as *const u8 as *const i8);
        i = 0;
        while !token.is_null() {
            let fresh1 = i;
            i = i + 1;
            *(coeff.values).offset(fresh1 as isize) = atof(token) as libc::c_float;
            token = strtok(0 as *mut i8, b" \0" as *const u8 as *const i8);
        }
        return coeff;
    }
}

#[no_mangle]
pub extern "C" fn processSignalFile(mut fileName: *mut i8) -> vector {
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut sum: libc::c_float = 0.;
        let mut str: [i8; 1000] = [0; 1000];
        let mut coeff1: vector = vector {
            values: 0 as *mut libc::c_float,
            size: 0,
        };
        let mut coeff2: vector = vector {
            values: 0 as *mut libc::c_float,
            size: 0,
        };
        let mut signal: vector = vector {
            values: 0 as *mut libc::c_float,
            size: 0,
        };
        let mut filteredSignal: vector = vector {
            values: 0 as *mut libc::c_float,
            size: 0,
        };
        let mut fp: *mut FILE = fopen(fileName, b"r\0" as *const u8 as *const i8);
        fgets(str.as_mut_ptr(), 1000, fp);
        coeff1 = extractVector(str.as_mut_ptr());
        fgets(str.as_mut_ptr(), 1000, fp);
        coeff2 = extractVector(str.as_mut_ptr());
        fgets(str.as_mut_ptr(), 1000, fp);
        signal = extractVector(str.as_mut_ptr());
        fclose(fp);
        filteredSignal.values = calloc(
            signal.size as u64,
            ::core::mem::size_of::<libc::c_float>() as u64,
        ) as *mut libc::c_float;
        filteredSignal.size = signal.size;
        i = 0;
        while i < signal.size {
            sum = 0 as libc::c_float;
            j = 0;
            while j < coeff2.size {
                if i - j >= 0 {
                    sum += *(coeff2.values).offset(j as isize)
                        * *(signal.values).offset((i - j) as isize);
                }
                j += 1;
                j;
            }
            j = 0;
            while j < coeff1.size {
                if i - j >= 0 {
                    sum -= *(coeff1.values).offset(j as isize)
                        * *(filteredSignal.values).offset((i - j) as isize);
                }
                j += 1;
                j;
            }
            sum /= *(coeff1.values).offset(0 as isize);
            *(filteredSignal.values).offset(i as isize) = sum;
            i += 1;
            i;
        }
        return filteredSignal;
    }
}

#[no_mangle]
pub extern "C" fn printVector(mut v: vector, mut outputFile: *mut i8) {
    unsafe {
        let mut i: i32 = 0;
        if outputFile.is_null() {
            print!("[");
            i = 0;
            while i < v.size {
                print!("{:.12}, ", *(v.values).offset(i as isize) as f64);
                i += 1;
                i;
            }
            print!("\x08\x08]");
        } else {
            let mut fp: *mut FILE = fopen(outputFile, b"w\0" as *const u8 as *const i8);
            i = 0;
            while i < v.size - 1 {
                fprintf(
                    fp,
                    b"%.12f, \0" as *const u8 as *const i8,
                    *(v.values).offset(i as isize) as f64,
                );
                i += 1;
                i;
            }
            fprintf(
                fp,
                b"%.12f\0" as *const u8 as *const i8,
                *(v.values).offset(i as isize) as f64,
            );
            fclose(fp);
        };
    }
}

fn main_0(mut argC: i32, mut argV: *mut *mut i8) -> i32 {
    unsafe {
        let mut str: *mut i8 = 0 as *mut i8;
        if argC < 2 || argC > 3 {
            print!(
                "Usage : {} <name of signal data file and optional output file.>",
                build_str_from_raw_ptr(*argV.offset(0 as isize) as *mut u8)
            );
        } else {
            if argC != 2 {
                str = malloc(
                    (strlen(*argV.offset(2 as isize)))
                        .wrapping_add(strlen(str))
                        .wrapping_add(1)
                        .wrapping_mul(::core::mem::size_of::<i8>() as u64),
                ) as *mut i8;
                strcpy(str, b"written to \0" as *const u8 as *const i8);
            }
            if argC == 2 {
                print!("Filtered signal {}", "is:\n\0")
            } else {
                print!(
                    "Filtered signal {}",
                    build_str_from_raw_ptr(
                        strcat(str, *argV.offset(2 as isize)) as *const i8 as *mut u8
                    )
                )
            };
            printVector(
                processSignalFile(*argV.offset(1 as isize)),
                *argV.offset(2 as isize),
            );
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
