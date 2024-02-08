#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn putchar(__c: libc::c_int) -> libc::c_int;
}
pub type ull = libc::c_ulonglong;
#[no_mangle]
pub unsafe extern "C" fn evolve(mut state: ull, mut rule: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut q: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    p = 0 as libc::c_int;
    while p < 10 as libc::c_int {
        b = 0 as libc::c_int;
        q = 8 as libc::c_int;
        loop {
            let fresh0 = q;
            q = q - 1;
            if !(fresh0 != 0) {
                break;
            }
            let mut st: ull = state;
            b = (b as libc::c_ulonglong
                | (st & 1 as libc::c_int as libc::c_ulonglong) << q) as libc::c_int;
            i = 0 as libc::c_int;
            state = i as ull;
            while (i as libc::c_ulong)
                < (::core::mem::size_of::<ull>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            {
                if rule as libc::c_ulonglong
                    & (1 as libc::c_ulonglong)
                        << (7 as libc::c_int as libc::c_ulonglong
                            & (st >> i - 1 as libc::c_int
                                | st
                                    << (::core::mem::size_of::<ull>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(i as libc::c_ulong))) != 0
                {
                    state |= (1 as libc::c_ulonglong) << i;
                }
                i += 1;
                i;
            }
        }
        printf(b" %d\0" as *const u8 as *const libc::c_char, b);
        p += 1;
        p;
    }
    putchar('\n' as i32);
}
unsafe fn main_0() -> libc::c_int {
    evolve(1 as libc::c_int as ull, 30 as libc::c_int);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
