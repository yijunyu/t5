#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn srand(__seed: libc::c_uint);
    fn rand() -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
}
pub type __time_t = libc::c_long;
pub type time_t = __time_t;
unsafe fn main_0() -> libc::c_int {
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut k: libc::c_uint = 0;
    let mut choice: libc::c_uint = 0;
    let mut winsbyswitch: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut door: [libc::c_uint; 3] = [0; 3];
    srand(time(0 as *mut time_t) as libc::c_uint);
    i = 0 as libc::c_int as libc::c_uint;
    while i < 3000000 as libc::c_int as libc::c_uint {
        door[0 as libc::c_int
            as usize] = (if rand() % 2 as libc::c_int == 0 {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        }) as libc::c_uint;
        if door[0 as libc::c_int as usize] != 0 {
            door[2 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uint;
            door[1 as libc::c_int as usize] = door[2 as libc::c_int as usize];
        } else {
            door[1 as libc::c_int
                as usize] = (if rand() % 2 as libc::c_int == 0 {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            }) as libc::c_uint;
            door[2 as libc::c_int
                as usize] = (if door[1 as libc::c_int as usize] == 0 {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            }) as libc::c_uint;
        }
        choice = (rand() % 3 as libc::c_int) as libc::c_uint;
        if door[choice
            .wrapping_add(1 as libc::c_int as libc::c_uint)
            .wrapping_rem(3 as libc::c_int as libc::c_uint) as usize] == 0
            && door[choice
                .wrapping_add(2 as libc::c_int as libc::c_uint)
                .wrapping_rem(3 as libc::c_int as libc::c_uint) as usize] != 0
            || door[choice
                .wrapping_add(2 as libc::c_int as libc::c_uint)
                .wrapping_rem(3 as libc::c_int as libc::c_uint) as usize] == 0
                && door[choice
                    .wrapping_add(1 as libc::c_int as libc::c_uint)
                    .wrapping_rem(3 as libc::c_int as libc::c_uint) as usize] != 0
        {
            winsbyswitch = winsbyswitch.wrapping_add(1);
            winsbyswitch;
        }
        i = i.wrapping_add(1);
        i;
    }
    printf(
        b"\nAfter %u games, I won %u by switching.  That is %f%%. \0" as *const u8
            as *const libc::c_char,
        3000000 as libc::c_int,
        winsbyswitch,
        winsbyswitch as libc::c_float as libc::c_double * 100.0f64
            / i as libc::c_float as libc::c_double,
    );
    return 0;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
