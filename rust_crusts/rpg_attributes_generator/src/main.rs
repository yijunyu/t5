#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
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
    fn srand(__seed: u32);
    fn rand() -> i32;
    fn qsort(__base: *mut libc::c_void, __nmemb: u64, __size: u64, __compar: __compar_fn_t);
}
pub type __compar_fn_t =
    Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32>;
#[no_mangle]
pub extern "C" fn compareInts(mut i1: *const libc::c_void, mut i2: *const libc::c_void) -> i32 {
    unsafe {
        let mut a: i32 = *(i1 as *mut i32);
        let mut b: i32 = *(i2 as *mut i32);
        return a - b;
    }
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut nsum: i32 = 0;
    let mut vsum: i32 = 0;
    let mut vcount: i32 = 0;
    let mut values: [i32; 6] = [0; 6];
    let mut numbers: [i32; 4] = [0; 4];
    unsafe {
        srand(rust_time(None) as u32);
        loop {
            vsum = 0;
            i = 0;
            while i < 6 {
                j = 0;
                while j < 4 {
                    numbers[j as usize] = 1 + rand() % 6;
                    j += 1;
                    j;
                }
                qsort(
                    numbers.as_mut_ptr() as *mut libc::c_void,
                    4,
                    ::core::mem::size_of::<i32>() as u64,
                    Some(
                        compareInts
                            as unsafe extern "C" fn(
                                *const libc::c_void,
                                *const libc::c_void,
                            ) -> i32,
                    ),
                );
                nsum = 0;
                j = 1;
                while j < 4 {
                    nsum += numbers[j as usize];
                    j += 1;
                    j;
                }
                values[i as usize] = nsum;
                vsum += values[i as usize];
                i += 1;
                i;
            }
            if vsum < 75 {
                continue;
            }
            vcount = 0;
            j = 0;
            while j < 6 {
                if values[j as usize] >= 15 {
                    vcount += 1;
                    vcount;
                }
                j += 1;
                j;
            }
            if vcount < 2 {
                continue;
            }
            print!("The 6 random numbers generated are:\n");
            print!("[");
            j = 0;
            while j < 6 {
                print!("{} ", values[j as usize]);
                j += 1;
                j;
            }
            print!("\x08]\n");
            print!("\nTheir sum is {} and {} of them are >= 15\n", vsum, vcount);
            break;
        }
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
