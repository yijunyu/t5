#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn rand() -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rand_idx(
    mut p: *mut libc::c_double,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut s: libc::c_double = rand() as libc::c_double
        / (2147483647 as libc::c_int as libc::c_double + 1.0f64);
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n - 1 as libc::c_int
        && {
            s -= *p.offset(i as isize);
            s >= 0 as libc::c_int as libc::c_double
        }
    {
        i += 1;
        i;
    }
    return i;
}
unsafe fn main_0() -> libc::c_int {
    let mut user_action: libc::c_int = 0;
    let mut my_action: libc::c_int = 0;
    let mut user_rec: [libc::c_int; 3] = [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    ];
    let mut names: [*const libc::c_char; 3] = [
        b"Rock\0" as *const u8 as *const libc::c_char,
        b"Paper\0" as *const u8 as *const libc::c_char,
        b"Scissors\0" as *const u8 as *const libc::c_char,
    ];
    let mut str: [libc::c_char; 2] = [0; 2];
    let mut winner: [*const libc::c_char; 3] = [
        b"We tied.\0" as *const u8 as *const libc::c_char,
        b"Meself winned.\0" as *const u8 as *const libc::c_char,
        b"You win.\0" as *const u8 as *const libc::c_char,
    ];
    let mut p: [libc::c_double; 3] = [
        1.0f64 / 3 as libc::c_int as libc::c_double,
        1.0f64 / 3 as libc::c_int as libc::c_double,
        1.0f64 / 3 as libc::c_int as libc::c_double,
    ];
    loop {
        my_action = rand_idx(p.as_mut_ptr(), 3 as libc::c_int);
        printf(
            b"\nYour choice [1-3]:\n  1. Rock\n  2. Paper\n  3. Scissors\n> \0"
                as *const u8 as *const libc::c_char,
        );
        if scanf(
            b"%d\0" as *const u8 as *const libc::c_char,
            &mut user_action as *mut libc::c_int,
        ) == 0
        {
            scanf(b"%1s\0" as *const u8 as *const libc::c_char, str.as_mut_ptr());
            if *str.as_mut_ptr() as libc::c_int == 'q' as i32 {
                printf(
                    b"Your choices [rock : %d , paper :  %d , scissors %d] \0"
                        as *const u8 as *const libc::c_char,
                    user_rec[0 as libc::c_int as usize],
                    user_rec[1 as libc::c_int as usize],
                    user_rec[2 as libc::c_int as usize],
                );
                return 0 as libc::c_int;
            }
        } else {
            user_action -= 1;
            user_action;
            if user_action > 2 as libc::c_int || user_action < 0 as libc::c_int {
                printf(b"invalid choice; again\n\0" as *const u8 as *const libc::c_char);
            } else {
                printf(
                    b"You chose %s; I chose %s. %s\n\0" as *const u8
                        as *const libc::c_char,
                    names[user_action as usize],
                    names[my_action as usize],
                    winner[((my_action - user_action + 3 as libc::c_int)
                        % 3 as libc::c_int) as usize],
                );
                user_rec[user_action as usize] += 1;
                user_rec[user_action as usize];
            }
        }
    };
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
