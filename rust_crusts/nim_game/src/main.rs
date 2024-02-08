#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use c2rust_out::*;
extern "C" {
    fn scanf(_: *const i8, _: ...) -> i32;
}
fn main_0() -> i32 {
    print!("Nim Game\n\n");
    let mut Tokens: i32 = 12;
    unsafe {
        while Tokens > 0 {
            print!("How many tokens would you like to take?: ");
            let mut uin: i32 = 0;
            scanf(b"%i\0" as *const u8 as *const i8, &mut uin as *mut i32);
            let mut nextTokens: i32 = playerTurn(Tokens, uin);
            if nextTokens == Tokens {
                continue;
            }
            Tokens = nextTokens;
            Tokens = computerTurn(Tokens);
        }
    }
    print!("Computer wins.");
    return 0;
}

#[no_mangle]
pub extern "C" fn playerTurn(mut numTokens: i32, mut take: i32) -> i32 {
    if take < 1 || take > 3 {
        print!("\nTake must be between 1 and 3.\n\n");
        return numTokens;
    }
    let mut remainingTokens: i32 = numTokens - take;
    print!("\nPlayer takes {} tokens.\n", take);
    print!("{} tokens remaining.\n\n", remainingTokens);
    return remainingTokens;
}

#[no_mangle]
pub extern "C" fn computerTurn(mut numTokens: i32) -> i32 {
    let mut take: i32 = numTokens % 4;
    let mut remainingTokens: i32 = numTokens - take;
    print!("Computer takes {} tokens.\n", take);
    print!("{} tokens remaining.\n\n", remainingTokens);
    return remainingTokens;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
