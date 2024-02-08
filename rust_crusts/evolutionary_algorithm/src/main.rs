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
extern "C" {
    fn rand() -> i32;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
}
#[no_mangle]
pub static mut target: [i8; 29] =
    unsafe { *::core::mem::transmute::<&[u8; 29], &[i8; 29]>(b"METHINKS IT IS LIKE A WEASEL\0") };
#[no_mangle]
pub static mut tbl: [i8; 28] =
    unsafe { *::core::mem::transmute::<&[u8; 28], &[i8; 28]>(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ \0") };
#[no_mangle]
pub extern "C" fn irand(mut n: i32) -> i32 {
    let mut r: i32 = 0;
    let mut rand_max: i32 = 2147483647 - 2147483647 % n;
    unsafe {
        loop {
            r = rand();
            if !(r >= rand_max) {
                break;
            }
        }
    }
    return r / (rand_max / n);
}

#[no_mangle]
pub extern "C" fn unfitness(mut a: *const i8, mut b: *const i8) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut sum: i32 = 0;
        i = 0;
        while *a.offset(i as isize) != 0 {
            sum += (*a.offset(i as isize) as i32 != *b.offset(i as isize) as i32) as i32;
            i += 1;
            i;
        }
        return sum;
    }
}

#[no_mangle]
pub extern "C" fn mutate(mut a: *const i8, mut b: *mut i8) {
    unsafe {
        let mut i: i32 = 0;
        i = 0;
        while *a.offset(i as isize) != 0 {
            *b.offset(i as isize) = (if irand(15) != 0 {
                *a.offset(i as isize) as i32
            } else {
                tbl[irand((::core::mem::size_of::<[i8; 28]>() as u64).wrapping_sub(1) as i32)
                    as usize] as i32
            }) as i8;
            i += 1;
            i;
        }
        *b.offset(i as isize) = '\0' as i8;
    }
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    let mut best_i: i32 = 0;
    let mut unfit: i32 = 0;
    let mut best: i32 = 0;
    let mut iters: i32 = 0;
    let mut specimen: [[i8; 29]; 30] = [[0; 29]; 30];
    i = 0;
    unsafe {
        while target[i as usize] != 0 {
            specimen[0 as usize][i as usize] =
                tbl[irand((::core::mem::size_of::<[i8; 28]>() as u64).wrapping_sub(1) as i32)
                    as usize];
            i += 1;
            i;
        }
    }
    specimen[0 as usize][i as usize] = 0;
    unsafe {
        loop {
            i = 1;
            while i < 30 {
                mutate(
                    (specimen[0 as usize]).as_mut_ptr(),
                    (specimen[i as usize]).as_mut_ptr(),
                );
                i += 1;
                i;
            }
            i = 0;
            best_i = i;
            while i < 30 {
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
                    (specimen[0 as usize]).as_mut_ptr(),
                    (specimen[best_i as usize]).as_mut_ptr(),
                );
            }
            let fresh0 = iters;
            iters = iters + 1;
            print!(
                "iter {}, score {}: {}\n",
                fresh0,
                best,
                build_str_from_raw_ptr((specimen[0 as usize]).as_mut_ptr() as *mut u8)
            );
            if !(best != 0) {
                break;
            }
        }
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
