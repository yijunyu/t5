#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn turn(mut base: libc::c_int, mut n: libc::c_int) -> libc::c_int {
    let mut sum: libc::c_int = 0 as libc::c_int;
    while n != 0 as libc::c_int {
        let mut rem: libc::c_int = n % base;
        n = n / base;
        sum += rem;
    }
    return sum % base;
}
#[no_mangle]
pub unsafe extern "C" fn fairshare(mut base: libc::c_int, mut count: libc::c_int) {
    let mut i: libc::c_int = 0;
    printf(b"Base %2d:\0" as *const u8 as *const libc::c_char, base);
    i = 0 as libc::c_int;
    while i < count {
        let mut t: libc::c_int = turn(base, i);
        printf(b" %2d\0" as *const u8 as *const libc::c_char, t);
        i += 1;
        i;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn turnCount(mut base: libc::c_int, mut count: libc::c_int) {
    let mut cnt: *mut libc::c_int = calloc(
        base as libc::c_ulong,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut minTurn: libc::c_int = 0;
    let mut maxTurn: libc::c_int = 0;
    let mut portion: libc::c_int = 0;
    if cnt.is_null() {
        printf(
            b"Failed to allocate space to determine the spread of turns.\n\0"
                as *const u8 as *const libc::c_char,
        );
        return;
    }
    i = 0 as libc::c_int;
    while i < count {
        let mut t: libc::c_int = turn(base, i);
        let ref mut fresh0 = *cnt.offset(t as isize);
        *fresh0 += 1;
        *fresh0;
        i += 1;
        i;
    }
    minTurn = 2147483647 as libc::c_int;
    maxTurn = -(2147483647 as libc::c_int) - 1 as libc::c_int;
    portion = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < base {
        if *cnt.offset(i as isize) > 0 as libc::c_int {
            portion += 1;
            portion;
        }
        if *cnt.offset(i as isize) < minTurn {
            minTurn = *cnt.offset(i as isize);
        }
        if *cnt.offset(i as isize) > maxTurn {
            maxTurn = *cnt.offset(i as isize);
        }
        i += 1;
        i;
    }
    printf(b"  With %d people: \0" as *const u8 as *const libc::c_char, base);
    if 0 as libc::c_int == minTurn {
        printf(b"Only %d have a turn\n\0" as *const u8 as *const libc::c_char, portion);
    } else if minTurn == maxTurn {
        printf(b"%d\n\0" as *const u8 as *const libc::c_char, minTurn);
    } else {
        printf(b"%d or %d\n\0" as *const u8 as *const libc::c_char, minTurn, maxTurn);
    }
    free(cnt as *mut libc::c_void);
}
unsafe fn main_0() -> libc::c_int {
    fairshare(2 as libc::c_int, 25 as libc::c_int);
    fairshare(3 as libc::c_int, 25 as libc::c_int);
    fairshare(5 as libc::c_int, 25 as libc::c_int);
    fairshare(11 as libc::c_int, 25 as libc::c_int);
    printf(
        b"How many times does each get a turn in 50000 iterations?\n\0" as *const u8
            as *const libc::c_char,
    );
    turnCount(191 as libc::c_int, 50000 as libc::c_int);
    turnCount(1377 as libc::c_int, 50000 as libc::c_int);
    turnCount(49999 as libc::c_int, 50000 as libc::c_int);
    turnCount(50000 as libc::c_int, 50000 as libc::c_int);
    turnCount(50001 as libc::c_int, 50000 as libc::c_int);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
