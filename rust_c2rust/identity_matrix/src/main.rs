#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn abort() -> !;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    if argc < 2 as libc::c_int {
        printf(
            b"usage: identitymatrix <number of rows>\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    let mut rowsize: libc::c_int = atoi(*argv.offset(1 as libc::c_int as isize));
    if rowsize < 0 as libc::c_int {
        printf(
            b"Dimensions of matrix cannot be negative\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    let mut numElements: libc::c_int = rowsize * rowsize;
    if numElements < rowsize {
        printf(
            b"Squaring %d caused result to overflow to %d.\n\0" as *const u8
                as *const libc::c_char,
            rowsize,
            numElements,
        );
        abort();
    }
    let mut matrix: *mut *mut libc::c_int = calloc(
        numElements as libc::c_ulong,
        ::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong,
    ) as *mut *mut libc::c_int;
    if matrix.is_null() {
        printf(
            b"Failed to allocate %d elements of %d bytes each\n\0" as *const u8
                as *const libc::c_char,
            numElements,
            ::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong,
        );
        abort();
    }
    let mut row: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while row < rowsize as libc::c_uint {
        let ref mut fresh0 = *matrix.offset(row as isize);
        *fresh0 = calloc(
            numElements as libc::c_ulong,
            ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        ) as *mut libc::c_int;
        if (*matrix.offset(row as isize)).is_null() {
            printf(
                b"Failed to allocate %d elements of %d bytes each\n\0" as *const u8
                    as *const libc::c_char,
                numElements,
                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
            );
            abort();
        }
        *(*matrix.offset(row as isize)).offset(row as isize) = 1 as libc::c_int;
        row = row.wrapping_add(1);
        row;
    }
    printf(b"Matrix is: \n\0" as *const u8 as *const libc::c_char);
    let mut row_0: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while row_0 < rowsize as libc::c_uint {
        let mut column: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while column < rowsize as libc::c_uint {
            printf(
                b"%d \0" as *const u8 as *const libc::c_char,
                *(*matrix.offset(row_0 as isize)).offset(column as isize),
            );
            column = column.wrapping_add(1);
            column;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        row_0 = row_0.wrapping_add(1);
        row_0;
    }
    return 0;
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
