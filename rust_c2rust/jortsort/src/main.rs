#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn number_of_digits(mut x: libc::c_int) -> libc::c_int {
    let mut NumberOfDigits: libc::c_int = 0;
    NumberOfDigits = 0 as libc::c_int;
    while x != 0 as libc::c_int {
        x = x / 10 as libc::c_int;
        NumberOfDigits += 1;
        NumberOfDigits;
    }
    return NumberOfDigits;
}
#[no_mangle]
pub unsafe extern "C" fn convert_array(
    mut array: *mut libc::c_char,
    mut NumberOfElements: libc::c_int,
) -> *mut libc::c_int {
    let mut convertedArray: *mut libc::c_int = malloc(
        (NumberOfElements as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut originalElement: libc::c_int = 0;
    let mut convertedElement: libc::c_int = 0;
    convertedElement = 0 as libc::c_int;
    originalElement = 0 as libc::c_int;
    while convertedElement < NumberOfElements {
        *convertedArray
            .offset(
                convertedElement as isize,
            ) = atoi(&mut *array.offset(originalElement as isize));
        originalElement
            += number_of_digits(*convertedArray.offset(convertedElement as isize))
                + 1 as libc::c_int;
        convertedElement += 1;
        convertedElement;
    }
    return convertedArray;
}
#[no_mangle]
pub unsafe extern "C" fn isSorted(
    mut array: *mut libc::c_int,
    mut numberOfElements: libc::c_int,
) -> libc::c_int {
    let mut sorted: libc::c_int = 1 as libc::c_int;
    let mut counter: libc::c_int = 0 as libc::c_int;
    while counter < numberOfElements {
        if counter != 0 as libc::c_int
            && *array.offset((counter - 1 as libc::c_int) as isize)
                > *array.offset(counter as isize)
        {
            sorted -= 1;
            sorted;
        }
        counter += 1;
        counter;
    }
    return sorted;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut convertedArray: *mut libc::c_int = 0 as *mut libc::c_int;
    convertedArray = convert_array(
        *argv.offset(1 as libc::c_int as isize),
        argc - 1 as libc::c_int,
    );
    if isSorted(convertedArray, argc - 1 as libc::c_int) == 1 as libc::c_int {
        printf(
            b"Did you forgot to turn on your brain?! This array is already sorted!\n\0"
                as *const u8 as *const libc::c_char,
        );
    } else if argc - 1 as libc::c_int <= 10 as libc::c_int {
        printf(
            b"Am I really supposed to sort this? Sort it by yourself!\n\0" as *const u8
                as *const libc::c_char,
        );
    } else {
        printf(
            b"Am I really supposed to sort this? Bhahahaha!\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    free(convertedArray as *mut libc::c_void);
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
