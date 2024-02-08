#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn srand(__seed: libc::c_uint);
    fn rand() -> libc::c_int;
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
pub unsafe extern "C" fn compareInts(
    mut i1: *const libc::c_void,
    mut i2: *const libc::c_void,
) -> libc::c_int {
    let mut a: libc::c_int = *(i1 as *mut libc::c_int);
    let mut b: libc::c_int = *(i2 as *mut libc::c_int);
    return a - b;
}
unsafe fn main_0() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut nsum: libc::c_int = 0;
    let mut vsum: libc::c_int = 0;
    let mut vcount: libc::c_int = 0;
    let mut values: [libc::c_int; 6] = [0; 6];
    let mut numbers: [libc::c_int; 4] = [0; 4];
    srand(time(0 as *mut time_t) as libc::c_uint);
    loop {
        vsum = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < 6 as libc::c_int {
            j = 0 as libc::c_int;
            while j < 4 as libc::c_int {
                numbers[j as usize] = 1 as libc::c_int + rand() % 6 as libc::c_int;
                j += 1;
                j;
            }
            qsort(
                numbers.as_mut_ptr() as *mut libc::c_void,
                4 as libc::c_int as size_t,
                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                Some(
                    compareInts
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> libc::c_int,
                ),
            );
            nsum = 0 as libc::c_int;
            j = 1 as libc::c_int;
            while j < 4 as libc::c_int {
                nsum += numbers[j as usize];
                j += 1;
                j;
            }
            values[i as usize] = nsum;
            vsum += values[i as usize];
            i += 1;
            i;
        }
        if vsum < 75 as libc::c_int {
            continue;
        }
        vcount = 0 as libc::c_int;
        j = 0 as libc::c_int;
        while j < 6 as libc::c_int {
            if values[j as usize] >= 15 as libc::c_int {
                vcount += 1;
                vcount;
            }
            j += 1;
            j;
        }
        if vcount < 2 as libc::c_int {
            continue;
        }
        printf(
            b"The 6 random numbers generated are:\n\0" as *const u8
                as *const libc::c_char,
        );
        printf(b"[\0" as *const u8 as *const libc::c_char);
        j = 0 as libc::c_int;
        while j < 6 as libc::c_int {
            printf(b"%d \0" as *const u8 as *const libc::c_char, values[j as usize]);
            j += 1;
            j;
        }
        printf(b"\x08]\n\0" as *const u8 as *const libc::c_char);
        printf(
            b"\nTheir sum is %d and %d of them are >= 15\n\0" as *const u8
                as *const libc::c_char,
            vsum,
            vcount,
        );
        break;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
