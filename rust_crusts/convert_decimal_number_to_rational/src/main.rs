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
    fn atan2(_: f64, _: f64) -> f64;
    fn floor(_: f64) -> f64;
}
#[no_mangle]
pub extern "C" fn rat_approx(mut f: f64, mut md: i64, mut num: *mut i64, mut denom: *mut i64) {
    unsafe {
        let mut a: i64 = 0;
        let mut h: [i64; 3] = [0, 1, 0];
        let mut k: [i64; 3] = [1, 0, 0];
        let mut x: i64 = 0;
        let mut d: i64 = 0;
        let mut n: i64 = 1;
        let mut i: i32 = 0;
        let mut neg: i32 = 0;
        if md <= 1 {
            *denom = 1;
            *num = f as i64;
            return;
        }
        if f < 0 as f64 {
            neg = 1;
            f = -f;
        }
        while f != floor(f) {
            n <<= 1;
            f *= 2 as f64;
        }
        d = f as i64;
        i = 0;
        while i < 64 {
            a = if n != 0 { d / n } else { 0 };
            if i != 0 && a == 0 {
                break;
            }
            x = d;
            d = n;
            n = x % n;
            x = a;
            if k[1 as usize] * a + k[0 as usize] >= md {
                x = (md - k[0 as usize]) / k[1 as usize];
                if !(x * 2 >= a || k[1 as usize] >= md) {
                    break;
                }
                i = 65;
            }
            h[2 as usize] = x * h[1 as usize] + h[0 as usize];
            h[0 as usize] = h[1 as usize];
            h[1 as usize] = h[2 as usize];
            k[2 as usize] = x * k[1 as usize] + k[0 as usize];
            k[0 as usize] = k[1 as usize];
            k[1 as usize] = k[2 as usize];
            i += 1;
            i;
        }
        *denom = k[1 as usize];
        *num = if neg != 0 {
            -h[1 as usize]
        } else {
            h[1 as usize]
        };
    }
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    let mut d: i64 = 0;
    let mut n: i64 = 0;
    let mut f: f64 = 0.;
    f = 1.0f64 / 7 as f64;
    print!("f = {:16.14}\n", f);
    i = 1;
    while i <= 20000000 {
        print!("denom <= {}: ", i);
        rat_approx(f, i as i64, &mut n, &mut d);
        print!("{}/{}\n", n, d);
        i *= 16;
    }
    unsafe {
        f = atan2(1 as f64, 1 as f64) * 4 as f64;
    }
    print!("\nf = {:16.14}\n", f);
    i = 1;
    while i <= 20000000 {
        print!("denom <= {}: ", i);
        rat_approx(f, i as i64, &mut n, &mut d);
        print!("{}/{}\n", n, d);
        i *= 16;
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
