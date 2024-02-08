#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
fn build_str_from_raw_ptr(raw_ptr: *mut u8) -> String {
    unsafe {
        let mut str_size: usize = 0;
        while *raw_ptr.offset(str_size as isize) != 0 {
            str_size += 1;
        }
        return std::str::from_utf8_unchecked(std::slice::from_raw_parts(raw_ptr, str_size))
            .to_owned();
    }
}

use c2rust_out::*;
extern "C" {
    fn scanf(_: *const i8, _: ...) -> i32;
    fn rand() -> i32;
}
#[no_mangle]
pub extern "C" fn rand_idx(mut p: *mut f64, mut n: i32) -> i32 {
    unsafe {
        let mut s: f64 = rand() as f64 / (2147483647 as f64 + 1.0f64);
        let mut i: i32 = 0;
        i = 0;
        while i < n - 1 && {
            s -= *p.offset(i as isize);
            s >= 0 as f64
        } {
            i += 1;
            i;
        }
        return i;
    }
}

fn main_0() -> i32 {
    let mut user_action: i32 = 0;
    let mut my_action: i32 = 0;
    let mut user_rec: [i32; 3] = [0; 3];
    let mut names: [*const i8; 3] = [
        b"Rock\0" as *const u8 as *const i8,
        b"Paper\0" as *const u8 as *const i8,
        b"Scissors\0" as *const u8 as *const i8,
    ];
    let mut str: [i8; 2] = [0; 2];
    let mut winner: [*const i8; 3] = [
        b"We tied.\0" as *const u8 as *const i8,
        b"Meself winned.\0" as *const u8 as *const i8,
        b"You win.\0" as *const u8 as *const i8,
    ];
    let mut p: [f64; 3] = [1.0f64 / 3 as f64, 1.0f64 / 3 as f64, 1.0f64 / 3 as f64];
    unsafe {
        loop {
            my_action = rand_idx(p.as_mut_ptr(), 3);
            print!("\nYour choice [1-3]:\n  1. Rock\n  2. Paper\n  3. Scissors\n> ");
            if scanf(
                b"%d\0" as *const u8 as *const i8,
                &mut user_action as *mut i32,
            ) == 0
            {
                scanf(b"%1s\0" as *const u8 as *const i8, str.as_mut_ptr());
                if *str.as_mut_ptr() as i32 == 'q' as i32 {
                    print!(
                        "Your choices [rock : {} , paper :  {} , scissors {}] ",
                        user_rec[0 as usize], user_rec[1 as usize], user_rec[2 as usize]
                    );
                    return 0;
                }
            } else {
                user_action -= 1;
                user_action;
                if user_action > 2 || user_action < 0 {
                    print!("invalid choice; again\n");
                } else {
                    print!(
                        "You chose {}; I chose {}. {}\n",
                        build_str_from_raw_ptr(names[user_action as usize] as *mut u8),
                        build_str_from_raw_ptr(names[my_action as usize] as *mut u8),
                        build_str_from_raw_ptr(
                            winner[((my_action - user_action + 3i32) % 3i32) as usize] as *mut u8
                        )
                    );
                    user_rec[user_action as usize] += 1;
                    user_rec[user_action as usize];
                }
            }
        }
    }
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
