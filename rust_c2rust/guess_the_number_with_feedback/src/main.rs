#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
unsafe fn main_0() -> libc::c_int {
    let mut bounds: [libc::c_int; 2] = [1 as libc::c_int, 100 as libc::c_int];
    let mut input: [libc::c_char; 2] = *::core::mem::transmute::<
        &[u8; 2],
        &mut [libc::c_char; 2],
    >(b"  ");
    let mut choice: libc::c_int = (bounds[0 as libc::c_int as usize]
        + bounds[1 as libc::c_int as usize]) / 2 as libc::c_int;
    printf(
        b"Choose a number between %d and %d.\n\0" as *const u8 as *const libc::c_char,
        bounds[0 as libc::c_int as usize],
        bounds[1 as libc::c_int as usize],
    );
    loop {
        match input[0 as libc::c_int as usize] as libc::c_int {
            72 => {
                bounds[1 as libc::c_int as usize] = choice;
            }
            76 => {
                bounds[0 as libc::c_int as usize] = choice;
            }
            89 => {
                printf(b"\nAwwwright\n\0" as *const u8 as *const libc::c_char);
                return 0 as libc::c_int;
            }
            _ => {}
        }
        choice = (bounds[0 as libc::c_int as usize] + bounds[1 as libc::c_int as usize])
            / 2 as libc::c_int;
        printf(
            b"Is the number %d? (Y/H/L) \0" as *const u8 as *const libc::c_char,
            choice,
        );
        if !(scanf(b"%1s\0" as *const u8 as *const libc::c_char, input.as_mut_ptr())
            == 1 as libc::c_int)
        {
            break;
        }
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
