#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn self_desc(mut xx: libc::c_ulonglong) -> libc::c_int {
    let mut d: libc::c_uint = 0;
    let mut x: libc::c_uint = 0;
    let mut cnt: [libc::c_uchar; 10] = [
        0 as libc::c_int as libc::c_uchar,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut dig: [libc::c_uchar; 10] = [
        0 as libc::c_int as libc::c_uchar,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    d = 0 as libc::c_int as libc::c_uint;
    while xx > !(0 as libc::c_uint) as libc::c_ulonglong {
        let fresh0 = d;
        d = d.wrapping_add(1);
        dig[fresh0
            as usize] = xx.wrapping_rem(10 as libc::c_int as libc::c_ulonglong)
            as libc::c_uchar;
        cnt[dig[fresh0 as usize]
            as usize] = (cnt[dig[fresh0 as usize] as usize]).wrapping_add(1);
        cnt[dig[fresh0 as usize] as usize];
        xx = xx.wrapping_div(10 as libc::c_int as libc::c_ulonglong);
    }
    x = xx as libc::c_uint;
    while x != 0 {
        let fresh1 = d;
        d = d.wrapping_add(1);
        dig[fresh1
            as usize] = x.wrapping_rem(10 as libc::c_int as libc::c_uint)
            as libc::c_uchar;
        cnt[dig[fresh1 as usize]
            as usize] = (cnt[dig[fresh1 as usize] as usize]).wrapping_add(1);
        cnt[dig[fresh1 as usize] as usize];
        x = x.wrapping_div(10 as libc::c_int as libc::c_uint);
    }
    loop {
        let fresh2 = d;
        d = d.wrapping_sub(1);
        if !(fresh2 != 0
            && {
                let fresh3 = x;
                x = x.wrapping_add(1);
                dig[fresh3 as usize] as libc::c_int == cnt[d as usize] as libc::c_int
            })
        {
            break;
        }
    }
    return (d == -(1 as libc::c_int) as libc::c_uint) as libc::c_int;
}
unsafe fn main_0() -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < 100000000 as libc::c_int {
        if self_desc(i as libc::c_ulonglong) != 0 {
            printf(b"%d\n\0" as *const u8 as *const libc::c_char, i);
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
