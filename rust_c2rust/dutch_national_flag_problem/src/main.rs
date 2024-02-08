#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn rand() -> libc::c_int;
    fn srand(__seed: libc::c_uint);
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn time(__timer: *mut time_t) -> time_t;
}
pub type size_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type time_t = __time_t;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
#[no_mangle]
pub unsafe extern "C" fn compar(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    let mut c1: libc::c_char = *(a as *const libc::c_char);
    let mut c2: libc::c_char = *(b as *const libc::c_char);
    return c1 as libc::c_int - c2 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn issorted(mut balls: *mut libc::c_char) -> bool {
    let mut i: libc::c_int = 0;
    let mut state: libc::c_int = 0;
    state = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        if (*balls.offset(i as isize) as libc::c_int) < state {
            return 0 as libc::c_int != 0;
        }
        if *balls.offset(i as isize) as libc::c_int > state {
            state = *balls.offset(i as isize) as libc::c_int;
        }
        i += 1;
        i;
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn printout(mut balls: *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    let mut str: [libc::c_char; 6] = [0; 6];
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        str[i
            as usize] = (if *balls.offset(i as isize) as libc::c_int == 0 as libc::c_int
        {
            'r' as i32
        } else if *balls.offset(i as isize) as libc::c_int == 1 as libc::c_int {
            'w' as i32
        } else {
            'b' as i32
        }) as libc::c_char;
        i += 1;
        i;
    }
    printf(b"%s\n\0" as *const u8 as *const libc::c_char, str.as_mut_ptr());
}
unsafe fn main_0() -> libc::c_int {
    let mut balls: [libc::c_char; 5] = [0; 5];
    let mut i: libc::c_int = 0;
    srand(time(0 as *mut time_t) as libc::c_uint);
    rand();
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        balls[i
            as usize] = (rand() as libc::c_double
            / 2147483647 as libc::c_int as libc::c_double
            * 3 as libc::c_int as libc::c_double) as libc::c_char;
        i += 1;
        i;
    }
    while issorted(balls.as_mut_ptr()) {
        printf(b"Accidentally still sorted: \0" as *const u8 as *const libc::c_char);
        printout(balls.as_mut_ptr());
        i = 0 as libc::c_int;
        while i < 5 as libc::c_int {
            balls[i
                as usize] = (rand() as libc::c_double
                / 2147483647 as libc::c_int as libc::c_double
                * 3 as libc::c_int as libc::c_double) as libc::c_char;
            i += 1;
            i;
        }
    }
    printf(b"Non-sorted: \0" as *const u8 as *const libc::c_char);
    printout(balls.as_mut_ptr());
    qsort(
        balls.as_mut_ptr() as *mut libc::c_void,
        5 as libc::c_int as size_t,
        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
        Some(
            compar
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    if issorted(balls.as_mut_ptr()) {
        printf(b"Sorted: \0" as *const u8 as *const libc::c_char);
        printout(balls.as_mut_ptr());
    } else {
        printf(b"Sort failed: \0" as *const u8 as *const libc::c_char);
        printout(balls.as_mut_ptr());
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
