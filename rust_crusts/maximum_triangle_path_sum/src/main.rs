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
    fn sqrt(_: f64) -> f64;
}
#[no_mangle]
pub static mut tri: [i32; 171] = [
    55, 94, 48, 95, 30, 96, 77, 71, 26, 67, 97, 13, 76, 38, 45, 7, 36, 79, 16, 37, 68, 48, 7, 9,
    18, 70, 26, 6, 18, 72, 79, 46, 59, 79, 29, 90, 20, 76, 87, 11, 32, 7, 7, 49, 18, 27, 83, 58,
    35, 71, 11, 25, 57, 29, 85, 14, 64, 36, 96, 27, 11, 58, 56, 92, 18, 55, 2, 90, 3, 60, 48, 49,
    41, 46, 33, 36, 47, 23, 92, 50, 48, 2, 36, 59, 42, 79, 72, 20, 82, 77, 42, 56, 78, 38, 80, 39,
    75, 2, 71, 66, 66, 1, 3, 55, 72, 44, 25, 67, 84, 71, 67, 11, 61, 40, 57, 58, 89, 40, 56, 36,
    85, 32, 25, 85, 57, 48, 84, 35, 47, 62, 17, 1, 1, 99, 89, 52, 6, 71, 28, 75, 94, 48, 37, 10,
    23, 51, 6, 48, 53, 18, 74, 98, 15, 27, 2, 92, 23, 8, 71, 76, 84, 15, 52, 92, 63, 81, 10, 44,
    10, 69, 93,
];
fn main_0() -> i32 {
    let len: i32 = (::core::mem::size_of::<[i32; 171]>() as u64)
        .wrapping_div(::core::mem::size_of::<i32>() as u64) as i32;
    unsafe {
        let base: i32 = ((sqrt((8 * len + 1i32) as f64) - 1 as f64) / 2 as f64) as i32;
        let mut step: i32 = base - 1;
        let mut stepc: i32 = 0;
        let mut i: i32 = 0;
        i = len - base - 1;
        while i >= 0 {
            tri[i as usize] += if tri[(i + step) as usize] > tri[(i + step + 1i32) as usize] {
                tri[(i + step) as usize]
            } else {
                tri[(i + step + 1i32) as usize]
            };
            stepc += 1;
            if stepc == step {
                step -= 1;
                step;
                stepc = 0;
            }
            i -= 1;
            i;
        }
        print!("{}\n", tri[0 as usize]);
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
