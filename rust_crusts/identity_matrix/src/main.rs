#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use c2rust_out::*;
extern "C" {
    fn atoi(__nptr: *const i8) -> i32;
    fn exit(_: i32) -> !;
    fn abort() -> !;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
}
fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    unsafe {
        if argc < 2 {
            print!("usage: identitymatrix <number of rows>\n");
            exit(1);
        }
        let mut rowsize: i32 = atoi(*argv.offset(1 as isize));
        if rowsize < 0 {
            print!("Dimensions of matrix cannot be negative\n");
            exit(1);
        }
        let mut numElements: i32 = rowsize * rowsize;
        if numElements < rowsize {
            print!(
                "Squaring {} caused result to overflow to {}.\n",
                rowsize, numElements
            );
            abort();
        }
        let mut matrix: *mut *mut i32 = calloc(
            numElements as u64,
            ::core::mem::size_of::<*mut i32>() as u64,
        ) as *mut *mut i32;
        if matrix.is_null() {
            print!(
                "Failed to allocate {} elements of {} bytes each\n",
                numElements,
                ::core::mem::size_of::<*mut i32>() as u64
            );
            abort();
        }
        let mut row: u32 = 0;
        while row < rowsize as u32 {
            let ref mut fresh0 = *matrix.offset(row as isize);
            *fresh0 = calloc(numElements as u64, ::core::mem::size_of::<i32>() as u64) as *mut i32;
            if (*matrix.offset(row as isize)).is_null() {
                print!(
                    "Failed to allocate {} elements of {} bytes each\n",
                    numElements,
                    ::core::mem::size_of::<i32>() as u64
                );
                abort();
            };
            *(*matrix.offset(row as isize)).offset(row as isize) = 1;
            row = row.wrapping_add(1);
            row;
        }
        print!("Matrix is: \n");
        let mut row_0: u32 = 0;
        while row_0 < rowsize as u32 {
            let mut column: u32 = 0;
            while column < rowsize as u32 {
                print!(
                    "{} ",
                    *(*matrix.offset(row_0 as isize)).offset(column as isize)
                );
                column = column.wrapping_add(1);
                column;
            }
            print!("\n");
            row_0 = row_0.wrapping_add(1);
            row_0;
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
    unsafe {
        ::std::process::exit(
            main_0((args.len() - 1) as i32, args.as_mut_ptr() as *mut *mut i8) as i32,
        );
    }
}
