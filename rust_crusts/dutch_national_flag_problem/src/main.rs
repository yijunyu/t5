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

use std::time::SystemTime;
pub fn rust_time(ref_result: Option<&mut i64>) -> i64 {
    let result = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    };
    match ref_result {
        Some(r) => *r = result,
        None => {}
    }
    return result as i64;
}

use c2rust_out::*;
extern "C" {
    fn rand() -> i32;
    fn srand(__seed: u32);
    fn qsort(__base: *mut libc::c_void, __nmemb: u64, __size: u64, __compar: __compar_fn_t);
}
pub type __compar_fn_t =
    Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32>;
#[no_mangle]
pub extern "C" fn compar(mut a: *const libc::c_void, mut b: *const libc::c_void) -> i32 {
    unsafe {
        let mut c1: i8 = *(a as *const i8);
        let mut c2: i8 = *(b as *const i8);
        return c1 as i32 - c2 as i32;
    }
}

#[no_mangle]
pub extern "C" fn issorted(mut balls: *mut i8) -> bool {
    unsafe {
        let mut i: i32 = 0;
        let mut state: i32 = 0;
        state = 0;
        i = 0;
        while i < 5 {
            if (*balls.offset(i as isize) as i32) < state {
                return 0 != 0;
            }
            if *balls.offset(i as isize) as i32 > state {
                state = *balls.offset(i as isize) as i32;
            }
            i += 1;
            i;
        }
        return 1 != 0;
    }
}

#[no_mangle]
pub extern "C" fn printout(mut balls: *mut i8) {
    unsafe {
        let mut i: i32 = 0;
        let mut str: [i8; 6] = [0; 6];
        i = 0;
        while i < 5 {
            str[i as usize] = (if *balls.offset(i as isize) as i32 == 0 {
                'r' as i32
            } else if *balls.offset(i as isize) as i32 == 1 {
                'w' as i32
            } else {
                'b' as i32
            }) as i8;
            i += 1;
            i;
        }
        print!("{}\n", build_str_from_raw_ptr(str.as_mut_ptr() as *mut u8));
    }
}

fn main_0() -> i32 {
    let mut balls: [i8; 5] = [0; 5];
    let mut i: i32 = 0;
    unsafe {
        srand(rust_time(None) as u32);
        rand();
    }
    i = 0;
    unsafe {
        while i < 5 {
            balls[i as usize] = (rand() as f64 / 2147483647 as f64 * 3 as f64) as i8;
            i += 1;
            i;
        }
        while issorted(balls.as_mut_ptr()) {
            print!("Accidentally still sorted: ");
            printout(balls.as_mut_ptr());
            i = 0;
            while i < 5 {
                balls[i as usize] = (rand() as f64 / 2147483647 as f64 * 3 as f64) as i8;
                i += 1;
                i;
            }
        }
    }
    print!("Non-sorted: ");
    printout(balls.as_mut_ptr());
    unsafe {
        qsort(
            balls.as_mut_ptr() as *mut libc::c_void,
            5,
            ::core::mem::size_of::<i8>() as u64,
            Some(compar as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32),
        );
    }
    if issorted(balls.as_mut_ptr()) {
        print!("Sorted: ");
        printout(balls.as_mut_ptr());
    } else {
        print!("Sort failed: ");
        printout(balls.as_mut_ptr());
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
