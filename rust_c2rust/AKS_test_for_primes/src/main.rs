#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn putchar(__c: libc::c_int) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn abort() -> !;
}
#[no_mangle]
pub static mut c: [libc::c_longlong; 100] = [0; 100];
#[no_mangle]
pub unsafe extern "C" fn coef(mut n: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if n < 0 as libc::c_int || n > 63 as libc::c_int {
        abort();
    }
    i = 0 as libc::c_int;
    c[i as usize] = 1 as libc::c_int as libc::c_longlong;
    while i < n {
        j = i;
        c[(1 as libc::c_int + j) as usize] = 1 as libc::c_int as libc::c_longlong;
        while j > 0 as libc::c_int {
            c[j as usize] = c[(j - 1 as libc::c_int) as usize] - c[j as usize];
            j -= 1;
            j;
        }
        c[0 as libc::c_int as usize] = -c[0 as libc::c_int as usize];
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn is_prime(mut n: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    coef(n);
    c[0 as libc::c_int as usize] += 1 as libc::c_int as libc::c_longlong;
    i = n;
    c[i as usize] -= 1 as libc::c_int as libc::c_longlong;
    loop {
        let fresh0 = i;
        i = i - 1;
        if !(fresh0 != 0 && c[i as usize] % n as libc::c_longlong == 0) {
            break;
        }
    }
    return (i < 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn show(mut n: libc::c_int) {
    loop {
        printf(b"%+lldx^%d\0" as *const u8 as *const libc::c_char, c[n as usize], n);
        let fresh1 = n;
        n = n - 1;
        if !(fresh1 != 0) {
            break;
        }
    };
}
unsafe fn main_0() -> libc::c_int {
    let mut n: libc::c_int = 0;
    n = 0 as libc::c_int;
    while n < 10 as libc::c_int {
        coef(n);
        printf(b"(x-1)^%d = \0" as *const u8 as *const libc::c_char, n);
        show(n);
        putchar('\n' as i32);
        n += 1;
        n;
    }
    printf(b"\nprimes (never mind the 1):\0" as *const u8 as *const libc::c_char);
    n = 1 as libc::c_int;
    while n <= 63 as libc::c_int {
        if is_prime(n) != 0 {
            printf(b" %d\0" as *const u8 as *const libc::c_char, n);
        }
        n += 1;
        n;
    }
    putchar('\n' as i32);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
