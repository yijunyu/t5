#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
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
extern "C" {}
fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    unsafe {
        if argc < 2 {
            print!("Enter an argument. Example 1234 or dcba:\n");
            return 0;
        }
        let mut x: i32 = 0;
        x = 0;
        while *(*argv.offset(1 as isize)).offset(x as isize) as i32 != '\0' as i32 {
            x += 1;
            x;
        }
        let mut f: i32 = 0;
        let mut v: i32 = 0;
        let mut m: i32 = 0;
        f = 0;
        while f < x {
            v = x - 1;
            while v > f {
                if *(*argv.offset(1 as isize)).offset((v - 1i32) as isize) as i32
                    > *(*argv.offset(1 as isize)).offset(v as isize) as i32
                {
                    m = *(*argv.offset(1 as isize)).offset((v - 1i32) as isize) as i32;
                    *(*argv.offset(1 as isize)).offset((v - 1i32) as isize) =
                        *(*argv.offset(1 as isize)).offset(v as isize);
                    *(*argv.offset(1 as isize)).offset(v as isize) = m as i8;
                }
                v -= 1;
                v;
            }
            f += 1;
            f;
        }
        let vla = x as usize;
        let mut a: Vec<i8> = ::std::vec::from_elem(0, vla);
        let mut k: i32 = 0;
        let mut fact: i32 = k + 1;
        while k != x {
            *a.as_mut_ptr().offset(k as isize) = *(*argv.offset(1 as isize)).offset(k as isize);
            k += 1;
            k;
            fact = k * fact;
        }
        *a.as_mut_ptr().offset(k as isize) = '\0' as i8;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut y: i32 = 0;
        let mut c: i8 = 0;
        while y != fact {
            print!("{}\n", build_str_from_raw_ptr(a.as_mut_ptr() as *mut u8));
            i = x - 2;
            while *a.as_mut_ptr().offset(i as isize) as i32
                > *a.as_mut_ptr().offset((i + 1i32) as isize) as i32
            {
                i -= 1;
                i;
            }
            j = x - 1;
            while (*a.as_mut_ptr().offset(j as isize) as i32)
                < *a.as_mut_ptr().offset(i as isize) as i32
            {
                j -= 1;
                j;
            }
            c = *a.as_mut_ptr().offset(j as isize);
            *a.as_mut_ptr().offset(j as isize) = *a.as_mut_ptr().offset(i as isize);
            *a.as_mut_ptr().offset(i as isize) = c;
            i += 1;
            i;
            j = x - 1;
            while j > i {
                c = *a.as_mut_ptr().offset(i as isize);
                *a.as_mut_ptr().offset(i as isize) = *a.as_mut_ptr().offset(j as isize);
                *a.as_mut_ptr().offset(j as isize) = c;
                i += 1;
                i;
                j -= 1;
                j;
            }
            y += 1;
            y;
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
