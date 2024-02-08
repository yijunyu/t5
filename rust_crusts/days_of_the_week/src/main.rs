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
    fn printf(_: *const i8, _: ...) -> i32;
}
#[no_mangle]
pub extern "C" fn wday(mut year: i32, mut month: i32, mut day: i32) -> i32 {
    let mut adjustment: i32 = 0;
    let mut mm: i32 = 0;
    let mut yy: i32 = 0;
    adjustment = (14 - month) / 12;
    mm = month + 12 * adjustment - 2;
    yy = year - adjustment;
    return (day + (13 * mm - 1) / 5 + yy + yy / 4 - yy / 100 + yy / 400) % 7;
}

fn main_0() -> i32 {
    let mut y: i32 = 0;
    y = 2008;
    unsafe {
        while y <= 2121 {
            if wday(y, 12, 25) == 0 {
                printf(b"%04d-12-25\n\0" as *const u8 as *const i8, y);
            }
            y += 1;
            y;
        }
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
