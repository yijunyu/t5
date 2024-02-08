#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Range {
    pub start: libc::c_int,
    pub end: libc::c_int,
    pub sum: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn maxSubseq(
    mut sequence: *const libc::c_int,
    len: libc::c_int,
) -> Range {
    let mut maxSum: libc::c_int = 0 as libc::c_int;
    let mut thisSum: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut start: libc::c_int = 0 as libc::c_int;
    let mut end: libc::c_int = -(1 as libc::c_int);
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < len {
        thisSum += *sequence.offset(j as isize);
        if thisSum < 0 as libc::c_int {
            i = j + 1 as libc::c_int;
            thisSum = 0 as libc::c_int;
        } else if thisSum > maxSum {
            maxSum = thisSum;
            start = i;
            end = j;
        }
        j += 1;
        j;
    }
    let mut r: Range = Range { start: 0, end: 0, sum: 0 };
    if start <= end && start >= 0 as libc::c_int && end >= 0 as libc::c_int {
        r.start = start;
        r.end = end + 1 as libc::c_int;
        r.sum = maxSum;
    } else {
        r.start = 0 as libc::c_int;
        r.end = 0 as libc::c_int;
        r.sum = 0 as libc::c_int;
    }
    return r;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut a: [libc::c_int; 11] = [
        -(1 as libc::c_int),
        -(2 as libc::c_int),
        3 as libc::c_int,
        5 as libc::c_int,
        6 as libc::c_int,
        -(2 as libc::c_int),
        -(1 as libc::c_int),
        4 as libc::c_int,
        -(4 as libc::c_int),
        2 as libc::c_int,
        -(1 as libc::c_int),
    ];
    let mut alength: libc::c_int = (::core::mem::size_of::<[libc::c_int; 11]>()
        as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
        as libc::c_int;
    let mut r: Range = maxSubseq(a.as_mut_ptr() as *const libc::c_int, alength);
    printf(b"Max sum = %d\n\0" as *const u8 as *const libc::c_char, r.sum);
    let mut i: libc::c_int = 0;
    i = r.start;
    while i < r.end {
        printf(b"%d \0" as *const u8 as *const libc::c_char, a[i as usize]);
        i += 1;
        i;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
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
