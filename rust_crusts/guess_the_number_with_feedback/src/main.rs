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
    let mut bounds: [i32; 2] = [1, 100];
    unsafe {
        let mut input: [i8; 2] = *::core::mem::transmute::<&[u8; 2], &mut [i8; 2]>(b"  ");
        let mut choice: i32 = (bounds[0 as usize] + bounds[1 as usize]) / 2;
        print!(
            "Choose a number between {} and {}.\n",
            bounds[0 as usize], bounds[1 as usize]
        );
        loop {
            match input[0 as usize] as i32 {
                72 => {
                    bounds[1 as usize] = choice;
                }
                76 => {
                    bounds[0 as usize] = choice;
                }
                89 => {
                    print!("\nAwwwright\n");
                    return 0;
                }
                _ => {}
            }
            choice = (bounds[0 as usize] + bounds[1 as usize]) / 2;
            print!("Is the number {}? (Y/H/L) ", choice);
            if !(scanf(b"%1s\0" as *const u8 as *const i8, input.as_mut_ptr()) == 1) {
                break;
            }
        }
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
