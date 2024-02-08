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
    fn rand() -> i32;
    fn srand(__seed: u32);
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn puts(__s: *const i8) -> i32;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct s_env {
    pub n: u32,
    pub i: u32,
    pub size: u64,
    pub sample: *mut libc::c_void,
}
#[no_mangle]
pub extern "C" fn s_of_n_init(mut s_env: *mut s_env, mut size: u64, mut n: u32) {
    unsafe {
        (*s_env).i = 0;
        (*s_env).n = n;
        (*s_env).size = size;
        (*s_env).sample = malloc((n as u64).wrapping_mul(size));
    }
}

#[no_mangle]
pub extern "C" fn sample_set_i(mut s_env: *mut s_env, mut i: u32, mut item: *mut libc::c_void) {
    unsafe {
        memcpy(
            ((*s_env).sample).offset((i as u64).wrapping_mul((*s_env).size) as isize),
            item,
            (*s_env).size,
        );
    }
}

#[no_mangle]
pub extern "C" fn s_of_n(mut s_env: *mut s_env, mut item: *mut libc::c_void) -> *mut libc::c_void {
    unsafe {
        (*s_env).i = ((*s_env).i).wrapping_add(1);
        (*s_env).i;
        if (*s_env).i <= (*s_env).n {
            sample_set_i(s_env, ((*s_env).i).wrapping_sub(1), item);
        } else if (rand() as u32).wrapping_rem((*s_env).i) < (*s_env).n {
            sample_set_i(s_env, (rand() as u32).wrapping_rem((*s_env).n), item);
        }
        return (*s_env).sample;
    }
}

#[no_mangle]
pub extern "C" fn test(mut n: u32, mut items_set: *mut i32, mut num_items: u32) -> *mut i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut s_env: s_env = s_env {
            n: 0,
            i: 0,
            size: 0,
            sample: 0 as *mut libc::c_void,
        };
        s_of_n_init(&mut s_env, ::core::mem::size_of::<i32>() as u64, n);
        i = 0;
        while (i as u32) < num_items {
            s_of_n(
                &mut s_env,
                &mut *items_set.offset(i as isize) as *mut i32 as *mut libc::c_void,
            );
            i += 1;
            i;
        }
        return s_env.sample as *mut i32;
    }
}

fn main_0() -> i32 {
    unsafe {
        let mut i: u32 = 0;
        let mut j: u32 = 0;
        let mut n: u32 = 3;
        let mut num_items: u32 = 10;
        let mut frequencies: *mut u32 = 0 as *mut u32;
        let mut items_set: *mut i32 = 0 as *mut i32;
        srand(rust_time(None) as u32);
        items_set = malloc((num_items as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64))
            as *mut i32;
        frequencies = malloc((num_items as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64))
            as *mut u32;
        i = 0;
        while i < num_items {
            *items_set.offset(i as isize) = i as i32;
            *frequencies.offset(i as isize) = 0;
            i = i.wrapping_add(1);
            i;
        }
        i = 0;
        while i < 100000 {
            let mut res: *mut i32 = test(n, items_set, num_items);
            j = 0;
            while j < n {
                let ref mut fresh0 = *frequencies.offset(*res.offset(j as isize) as isize);
                *fresh0 = (*fresh0).wrapping_add(1);
                *fresh0;
                j = j.wrapping_add(1);
                j;
            }
            free(res as *mut libc::c_void);
            i = i.wrapping_add(1);
            i;
        }
        i = 0;
        while i < num_items {
            print!(" {}", *frequencies.offset(i as isize));
            i = i.wrapping_add(1);
            i;
        }
        puts(b"\0" as *const u8 as *const i8);
        return 0;
    }
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
