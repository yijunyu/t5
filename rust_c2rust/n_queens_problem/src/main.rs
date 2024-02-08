#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn putchar(__c: libc::c_int) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn abs(_: libc::c_int) -> libc::c_int;
}
#[no_mangle]
pub static mut count: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn solve(
    mut n: libc::c_int,
    mut col: libc::c_int,
    mut hist: *mut libc::c_int,
) {
    if col == n {
        count += 1;
        printf(b"\nNo. %d\n-----\n\0" as *const u8 as *const libc::c_char, count);
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < n {
            let mut j: libc::c_int = 0 as libc::c_int;
            while j < n {
                putchar(
                    if j == *hist.offset(i as isize) {
                        'Q' as i32
                    } else if i + j & 1 as libc::c_int != 0 {
                        ' ' as i32
                    } else {
                        '.' as i32
                    },
                );
                j += 1;
                j;
            }
            i += 1;
            i;
            putchar('\n' as i32);
        }
        return;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    let mut j_0: libc::c_int = 0 as libc::c_int;
    while i_0 < n {
        j_0 = 0 as libc::c_int;
        while j_0 < col
            && !(*hist.offset(j_0 as isize) == i_0
                || abs(*hist.offset(j_0 as isize) - i_0) == col - j_0)
        {
            j_0 += 1;
            j_0;
        }
        if !(j_0 < col) {
            *hist.offset(col as isize) = i_0;
            solve(n, col + 1 as libc::c_int, hist);
        }
        i_0 += 1;
        i_0;
    }
}
unsafe fn main_0(mut n: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    if n <= 1 as libc::c_int
        || {
            n = atoi(*argv.offset(1 as libc::c_int as isize));
            n <= 0 as libc::c_int
        }
    {
        n = 8 as libc::c_int;
    }
    let vla = n as usize;
    let mut hist: Vec::<libc::c_int> = ::std::vec::from_elem(0, vla);
    solve(n, 0 as libc::c_int, hist.as_mut_ptr());
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
