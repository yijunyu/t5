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
    fn rand() -> i32;
}
fn main_0() -> i32 {
    let mut i: i32 = 0;
    print! ("<table style=\"text-align:center; border: 1px solid\"><th></th><th>X</th><th>Y</th><th>Z</th>");
    i = 0;
    unsafe {
        while i < 4 {
            print!(
                "<tr><th>{}</th><td>{}</td><td>{}</td><td>{}</td></tr>",
                i,
                rand() % 10000,
                rand() % 10000,
                rand() % 10000
            );
            i += 1;
            i;
        }
    }
    print!("</table>");
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
