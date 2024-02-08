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
    fn puts(__s: *const i8) -> i32;
    fn rand() -> i32;
    fn srand(__seed: u32);
    fn pow(_: f64, _: f64) -> f64;
}
#[no_mangle]
pub extern "C" fn factorial(mut n: i32) -> f64 {
    let mut f: f64 = 1 as f64;
    let mut i: i32 = 0;
    i = 1;
    while i <= n {
        f *= i as f64;
        i += 1;
        i;
    }
    return f;
}

#[no_mangle]
pub extern "C" fn expected(mut n: i32) -> f64 {
    let mut sum: f64 = 0 as f64;
    let mut i: i32 = 0;
    i = 1;
    unsafe {
        while i <= n {
            sum += factorial(n) / pow(n as f64, i as f64) / factorial(n - i);
            i += 1;
            i;
        }
    }
    return sum;
}

#[no_mangle]
pub extern "C" fn randint(mut n: i32) -> i32 {
    let mut r: i32 = 0;
    let mut rmax: i32 = 2147483647 / n * n;
    unsafe {
        loop {
            r = rand();
            if !(r >= rmax) {
                break;
            }
        }
    }
    return r / (2147483647 / n);
}

#[no_mangle]
pub extern "C" fn test(mut n: i32, mut times: i32) -> i32 {
    let mut i: i32 = 0;
    let mut count: i32 = 0;
    i = 0;
    while i < times {
        let mut x: i32 = 1;
        let mut bits: i32 = 0;
        while bits & x == 0 {
            count += 1;
            count;
            bits |= x;
            x = 1 << randint(n);
        }
        i += 1;
        i;
    }
    return count;
}

fn main_0() -> i32 {
    unsafe {
        srand(rust_time(None) as u32);
        puts(b" n\tavg\texp.\tdiff\n-------------------------------\0" as *const u8 as *const i8);
    }
    let mut n: i32 = 0;
    n = 1;
    while n <= 20 {
        let mut cnt: i32 = test(n, 1000000);
        let mut avg: f64 = cnt as f64 / 1000000 as f64;
        let mut theory: f64 = expected(n);
        let mut diff: f64 = (avg / theory - 1 as f64) * 100 as f64;
        print!("{:2} {:8.4} {:8.4} {:6.3}%\n", n, avg, theory, diff);
        n += 1;
        n;
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
