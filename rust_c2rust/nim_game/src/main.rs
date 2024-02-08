#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
unsafe fn main_0() -> libc::c_int {
    printf(b"Nim Game\n\n\0" as *const u8 as *const libc::c_char);
    let mut Tokens: libc::c_int = 12 as libc::c_int;
    while Tokens > 0 as libc::c_int {
        printf(
            b"How many tokens would you like to take?: \0" as *const u8
                as *const libc::c_char,
        );
        let mut uin: libc::c_int = 0;
        scanf(b"%i\0" as *const u8 as *const libc::c_char, &mut uin as *mut libc::c_int);
        let mut nextTokens: libc::c_int = playerTurn(Tokens, uin);
        if nextTokens == Tokens {
            continue;
        }
        Tokens = nextTokens;
        Tokens = computerTurn(Tokens);
    }
    printf(b"Computer wins.\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn playerTurn(
    mut numTokens: libc::c_int,
    mut take: libc::c_int,
) -> libc::c_int {
    if take < 1 as libc::c_int || take > 3 as libc::c_int {
        printf(
            b"\nTake must be between 1 and 3.\n\n\0" as *const u8 as *const libc::c_char,
        );
        return numTokens;
    }
    let mut remainingTokens: libc::c_int = numTokens - take;
    printf(b"\nPlayer takes %i tokens.\n\0" as *const u8 as *const libc::c_char, take);
    printf(
        b"%i tokens remaining.\n\n\0" as *const u8 as *const libc::c_char,
        remainingTokens,
    );
    return remainingTokens;
}
#[no_mangle]
pub unsafe extern "C" fn computerTurn(mut numTokens: libc::c_int) -> libc::c_int {
    let mut take: libc::c_int = numTokens % 4 as libc::c_int;
    let mut remainingTokens: libc::c_int = numTokens - take;
    printf(b"Computer takes %u tokens.\n\0" as *const u8 as *const libc::c_char, take);
    printf(
        b"%i tokens remaining.\n\n\0" as *const u8 as *const libc::c_char,
        remainingTokens,
    );
    return remainingTokens;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
