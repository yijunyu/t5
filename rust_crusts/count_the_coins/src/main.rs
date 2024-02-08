#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use c2rust_out::*;
extern "C" {
    fn malloc(_: u64) -> *mut libc::c_void;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct i128_0 {
    pub x: [u64; 2],
}
#[no_mangle]
pub extern "C" fn show(mut v: i128_0) {
    let mut x: [u32; 4] = [
        v.x[0 as usize] as u32,
        (v.x[0 as usize] >> 32i32) as u32,
        v.x[1 as usize] as u32,
        (v.x[1 as usize] >> 32i32) as u32,
    ];
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut len: i32 = 4;
    let mut buf: [i8; 100] = [0; 100];
    loop {
        let mut c: u64 = 0;
        i = len;
        loop {
            let fresh0 = i;
            i = i - 1;
            if !(fresh0 != 0) {
                break;
            }
            c = (c << 32i32).wrapping_add(x[i as usize] as u64);
            x[i as usize] = c.wrapping_div(10) as u32;
            c = (c).wrapping_rem(10) as u64;
        }
        let fresh1 = j;
        j = j + 1;
        buf[fresh1 as usize] = c.wrapping_add('0' as u64) as i8;
        len = 4;
        while x[(len - 1i32) as usize] == 0 {
            len -= 1;
            len;
        }
        if !(len != 0) {
            break;
        }
    }
    loop {
        let fresh2 = j;
        j = j - 1;
        if !(fresh2 != 0) {
            break;
        }
        print!("{}", buf[j as usize] as i32);
    }
    print!("{}", '\n' as i32);
}

#[no_mangle]
pub extern "C" fn count(mut sum: i32, mut coins: *mut i32) -> i128_0 {
    unsafe {
        let mut n: i32 = 0;
        let mut i: i32 = 0;
        let mut k: i32 = 0;
        n = 0;
        while *coins.offset(n as isize) != 0 {
            n += 1;
            n;
        }
        let mut v: *mut *mut i128_0 =
            malloc((::core::mem::size_of::<*mut i32>() as u64).wrapping_mul(n as u64))
                as *mut *mut i128_0;
        let mut idx: *mut i32 =
            malloc((::core::mem::size_of::<i32>() as u64).wrapping_mul(n as u64)) as *mut i32;
        i = 0;
        while i < n {
            *idx.offset(i as isize) = *coins.offset(i as isize);
            let ref mut fresh3 = *v.offset(i as isize);
            *fresh3 = calloc(
                ::core::mem::size_of::<i128_0>() as u64,
                *coins.offset(i as isize) as u64,
            ) as *mut i128_0;
            i += 1;
            i;
        }
        *(*v.offset(0 as isize)).offset((*coins.offset(0 as isize) - 1i32) as isize) = {
            let mut init = i128_0 { x: [1, 0] };
            init
        };
        k = 0;
        while k <= sum {
            i = 0;
            while i < n {
                let ref mut fresh4 = *idx.offset(i as isize);
                let fresh5 = *fresh4;
                *fresh4 = *fresh4 - 1;
                if fresh5 == 0 {
                    *idx.offset(i as isize) = *coins.offset(i as isize) - 1;
                }
                i += 1;
                i;
            }
            let mut c: i128_0 = *(*v.offset(0 as isize)).offset(*idx.offset(0 as isize) as isize);
            i = 1;
            while i < n {
                let mut p: *mut i128_0 =
                    (*v.offset(i as isize)).offset(*idx.offset(i as isize) as isize);
                (*p).x[0 as usize] =
                    ((*p).x[0 as usize] as u64).wrapping_add(c.x[0 as usize]) as u64;
                (*p).x[1 as usize] =
                    ((*p).x[1 as usize] as u64).wrapping_add(c.x[1 as usize]) as u64;
                if (*p).x[0 as usize] < c.x[0 as usize] {
                    (*p).x[1 as usize] = ((*p).x[1 as usize]).wrapping_add(1);
                    (*p).x[1 as usize];
                }
                c = *p;
                i += 1;
                i;
            }
            k += 1;
            k;
        }
        let mut r: i128_0 =
            *(*v.offset((n - 1i32) as isize)).offset(*idx.offset((n - 1i32) as isize) as isize);
        i = 0;
        while i < n {
            free(*v.offset(i as isize) as *mut libc::c_void);
            i += 1;
            i;
        }
        free(v as *mut libc::c_void);
        free(idx as *mut libc::c_void);
        return r;
    }
}

#[no_mangle]
pub extern "C" fn count2(mut sum: i32, mut coins: *mut i32) -> i32 {
    unsafe {
        if *coins == 0 || sum < 0 {
            return 0;
        }
        if sum == 0 {
            return 1;
        }
        return count2(sum - *coins, coins) + count2(sum, coins.offset(1 as isize));
    }
}

fn main_0() -> i32 {
    let mut us_coins: [i32; 7] = [100, 50, 25, 10, 5, 1, 0];
    let mut eu_coins: [i32; 9] = [200, 100, 50, 20, 10, 5, 2, 1, 0];
    unsafe {
        show(count(100, us_coins.as_mut_ptr().offset(2 as isize)));
    }
    show(count(1000, us_coins.as_mut_ptr()));
    show(count(1000 * 100, us_coins.as_mut_ptr()));
    show(count(10000 * 100, us_coins.as_mut_ptr()));
    show(count(100000 * 100, us_coins.as_mut_ptr()));
    print!("{}", '\n' as i32);
    show(count(1 * 100, eu_coins.as_mut_ptr()));
    show(count(1000 * 100, eu_coins.as_mut_ptr()));
    show(count(10000 * 100, eu_coins.as_mut_ptr()));
    show(count(100000 * 100, eu_coins.as_mut_ptr()));
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
