#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn rand() -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
}
#[no_mangle]
pub static mut target: [libc::c_char; 29] = unsafe {
    *::core::mem::transmute::<
        &[u8; 29],
        &[libc::c_char; 29],
    >(b"METHINKS IT IS LIKE A WEASEL\0")
};
#[no_mangle]
pub static mut tbl: [libc::c_char; 28] = unsafe {
    *::core::mem::transmute::<
        &[u8; 28],
        &[libc::c_char; 28],
    >(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ \0")
};
#[no_mangle]
pub unsafe extern "C" fn irand(mut n: libc::c_int) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut rand_max: libc::c_int = 2147483647 as libc::c_int
        - 2147483647 as libc::c_int % n;
    loop {
        r = rand();
        if !(r >= rand_max) {
            break;
        }
    }
    return r / (rand_max / n);
}
#[no_mangle]
pub unsafe extern "C" fn unfitness(
    mut a: *const libc::c_char,
    mut b: *const libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut sum: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while *a.offset(i as isize) != 0 {
        sum
            += (*a.offset(i as isize) as libc::c_int
                != *b.offset(i as isize) as libc::c_int) as libc::c_int;
        i += 1;
        i;
    }
    return sum;
}
#[no_mangle]
pub unsafe extern "C" fn mutate(mut a: *const libc::c_char, mut b: *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while *a.offset(i as isize) != 0 {
        *b
            .offset(
                i as isize,
            ) = (if irand(15 as libc::c_int) != 0 {
            *a.offset(i as isize) as libc::c_int
        } else {
            tbl[irand(
                (::core::mem::size_of::<[libc::c_char; 28]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
            ) as usize] as libc::c_int
        }) as libc::c_char;
        i += 1;
        i;
    }
    *b.offset(i as isize) = '\0' as i32 as libc::c_char;
}
unsafe fn main_0() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut best_i: libc::c_int = 0;
    let mut unfit: libc::c_int = 0;
    let mut best: libc::c_int = 0;
    let mut iters: libc::c_int = 0 as libc::c_int;
    let mut specimen: [[libc::c_char; 29]; 30] = [[0; 29]; 30];
    i = 0 as libc::c_int;
    while target[i as usize] != 0 {
        specimen[0 as libc::c_int
            as usize][i
            as usize] = tbl[irand(
            (::core::mem::size_of::<[libc::c_char; 28]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        ) as usize];
        i += 1;
        i;
    }
    specimen[0 as libc::c_int as usize][i as usize] = 0 as libc::c_int as libc::c_char;
    loop {
        i = 1 as libc::c_int;
        while i < 30 as libc::c_int {
            mutate(
                (specimen[0 as libc::c_int as usize]).as_mut_ptr(),
                (specimen[i as usize]).as_mut_ptr(),
            );
            i += 1;
            i;
        }
        i = 0 as libc::c_int;
        best_i = i;
        while i < 30 as libc::c_int {
            unfit = unfitness(target.as_ptr(), (specimen[i as usize]).as_mut_ptr());
            if unfit < best || i == 0 {
                best = unfit;
                best_i = i;
            }
            i += 1;
            i;
        }
        if best_i != 0 {
            strcpy(
                (specimen[0 as libc::c_int as usize]).as_mut_ptr(),
                (specimen[best_i as usize]).as_mut_ptr(),
            );
        }
        let fresh0 = iters;
        iters = iters + 1;
        printf(
            b"iter %d, score %d: %s\n\0" as *const u8 as *const libc::c_char,
            fresh0,
            best,
            (specimen[0 as libc::c_int as usize]).as_mut_ptr(),
        );
        if !(best != 0) {
            break;
        }
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
