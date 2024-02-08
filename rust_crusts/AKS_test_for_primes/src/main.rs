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
    fn abort() -> !;
}
#[no_mangle]
pub static mut c: [i64; 100] = [0; 100];
#[no_mangle]
pub extern "C" fn coef(mut n: i32) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    unsafe {
        if n < 0 || n > 63 {
            abort();
        }
    }
    i = 0;
    unsafe {
        c[i as usize] = 1;
        while i < n {
            j = i;
            c[(1 + j) as usize] = 1;
            while j > 0 {
                c[j as usize] = c[(j - 1i32) as usize] - c[j as usize];
                j -= 1;
                j;
            }
            c[0 as usize] = -c[0 as usize];
            i += 1;
            i;
        }
    }
}

#[no_mangle]
pub extern "C" fn is_prime(mut n: i32) -> i32 {
    let mut i: i32 = 0;
    coef(n);
    unsafe {
        c[0 as usize] += 1;
    }
    i = n;
    unsafe {
        c[i as usize] -= 1;
        loop {
            let fresh0 = i;
            i = i - 1;
            if !(fresh0 != 0 && c[i as usize] % n as i64 == 0) {
                break;
            }
        }
    }
    return (i < 0) as i32;
}

#[no_mangle]
pub extern "C" fn show(mut n: i32) {
    unsafe {
        loop {
            print!("{:+}x^{}", c[n as usize], n);
            let fresh1 = n;
            n = n - 1;
            if !(fresh1 != 0) {
                break;
            }
        }
    }
}

fn main_0() -> i32 {
    let mut n: i32 = 0;
    n = 0;
    while n < 10 {
        coef(n);
        print!("(x-1)^{} = ", n);
        show(n);
        print!("{}", '\n' as i32);
        n += 1;
        n;
    }
    print!("\nprimes (never mind the 1):");
    n = 1;
    while n <= 63 {
        if is_prime(n) != 0 {
            print!(" {}", n);
        }
        n += 1;
        n;
    }
    print!("{}", '\n' as i32);
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
