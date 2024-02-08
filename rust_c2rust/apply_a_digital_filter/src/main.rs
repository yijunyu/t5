#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn atof(__nptr: *const libc::c_char) -> libc::c_double;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vector {
    pub values: *mut libc::c_float,
    pub size: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn extractVector(mut str: *mut libc::c_char) -> vector {
    let mut coeff: vector = vector {
        values: 0 as *mut libc::c_float,
        size: 0,
    };
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut count: libc::c_int = 1 as libc::c_int;
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    while *str.offset(i as isize) as libc::c_int != 0 as libc::c_int {
        let fresh0 = i;
        i = i + 1;
        if *str.offset(fresh0 as isize) as libc::c_int == ' ' as i32 {
            count += 1;
            count;
        }
    }
    coeff
        .values = malloc(
        (count as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_float>() as libc::c_ulong),
    ) as *mut libc::c_float;
    coeff.size = count;
    token = strtok(str, b" \0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while !token.is_null() {
        let fresh1 = i;
        i = i + 1;
        *(coeff.values).offset(fresh1 as isize) = atof(token) as libc::c_float;
        token = strtok(
            0 as *mut libc::c_char,
            b" \0" as *const u8 as *const libc::c_char,
        );
    }
    return coeff;
}
#[no_mangle]
pub unsafe extern "C" fn processSignalFile(mut fileName: *mut libc::c_char) -> vector {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut sum: libc::c_float = 0.;
    let mut str: [libc::c_char; 1000] = [0; 1000];
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
    let mut fp: *mut FILE = fopen(fileName, b"r\0" as *const u8 as *const libc::c_char);
    fgets(str.as_mut_ptr(), 1000 as libc::c_int, fp);
    coeff1 = extractVector(str.as_mut_ptr());
    fgets(str.as_mut_ptr(), 1000 as libc::c_int, fp);
    coeff2 = extractVector(str.as_mut_ptr());
    fgets(str.as_mut_ptr(), 1000 as libc::c_int, fp);
    signal = extractVector(str.as_mut_ptr());
    fclose(fp);
    filteredSignal
        .values = calloc(
        signal.size as libc::c_ulong,
        ::core::mem::size_of::<libc::c_float>() as libc::c_ulong,
    ) as *mut libc::c_float;
    filteredSignal.size = signal.size;
    i = 0 as libc::c_int;
    while i < signal.size {
        sum = 0 as libc::c_int as libc::c_float;
        j = 0 as libc::c_int;
        while j < coeff2.size {
            if i - j >= 0 as libc::c_int {
                sum
                    += *(coeff2.values).offset(j as isize)
                        * *(signal.values).offset((i - j) as isize);
            }
            j += 1;
            j;
        }
        j = 0 as libc::c_int;
        while j < coeff1.size {
            if i - j >= 0 as libc::c_int {
                sum
                    -= *(coeff1.values).offset(j as isize)
                        * *(filteredSignal.values).offset((i - j) as isize);
            }
            j += 1;
            j;
        }
        sum /= *(coeff1.values).offset(0 as libc::c_int as isize);
        *(filteredSignal.values).offset(i as isize) = sum;
        i += 1;
        i;
    }
    return filteredSignal;
}
#[no_mangle]
pub unsafe extern "C" fn printVector(mut v: vector, mut outputFile: *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    if outputFile.is_null() {
        printf(b"[\0" as *const u8 as *const libc::c_char);
        i = 0 as libc::c_int;
        while i < v.size {
            printf(
                b"%.12f, \0" as *const u8 as *const libc::c_char,
                *(v.values).offset(i as isize) as libc::c_double,
            );
            i += 1;
            i;
        }
        printf(b"\x08\x08]\0" as *const u8 as *const libc::c_char);
    } else {
        let mut fp: *mut FILE = fopen(
            outputFile,
            b"w\0" as *const u8 as *const libc::c_char,
        );
        i = 0 as libc::c_int;
        while i < v.size - 1 as libc::c_int {
            fprintf(
                fp,
                b"%.12f, \0" as *const u8 as *const libc::c_char,
                *(v.values).offset(i as isize) as libc::c_double,
            );
            i += 1;
            i;
        }
        fprintf(
            fp,
            b"%.12f\0" as *const u8 as *const libc::c_char,
            *(v.values).offset(i as isize) as libc::c_double,
        );
        fclose(fp);
    };
}
unsafe fn main_0(
    mut argC: libc::c_int,
    mut argV: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    if argC < 2 as libc::c_int || argC > 3 as libc::c_int {
        printf(
            b"Usage : %s <name of signal data file and optional output file.>\0"
                as *const u8 as *const libc::c_char,
            *argV.offset(0 as libc::c_int as isize),
        );
    } else {
        if argC != 2 as libc::c_int {
            str = malloc(
                (strlen(*argV.offset(2 as libc::c_int as isize)))
                    .wrapping_add(strlen(str))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    ),
            ) as *mut libc::c_char;
            strcpy(str, b"written to \0" as *const u8 as *const libc::c_char);
        }
        printf(
            b"Filtered signal %s\0" as *const u8 as *const libc::c_char,
            if argC == 2 as libc::c_int {
                b"is:\n\0" as *const u8 as *const libc::c_char
            } else {
                strcat(str, *argV.offset(2 as libc::c_int as isize))
                    as *const libc::c_char
            },
        );
        printVector(
            processSignalFile(*argV.offset(1 as libc::c_int as isize)),
            *argV.offset(2 as libc::c_int as isize),
        );
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
