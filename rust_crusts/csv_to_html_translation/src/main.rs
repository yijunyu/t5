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
    fn puts(__s: *const i8) -> i32;
}
#[no_mangle]
pub static mut input : * const i8 =
b"Character,Speech\nThe multitude,The messiah! Show us the messiah!\nBrians mother,<angry>Now you listen here! He's not the messiah; he's a very naughty boy! Now go away!</angry>\nThe multitude,Who are you?\nBrians mother,I'm his mother; that's who!\nThe multitude,Behold his mother! Behold his mother!\0"
 as * const u8 as * const i8;
fn main_0() -> i32 {
    unsafe {
        let mut s: *const i8 = 0 as *const i8;
        print!("<table>\n<tr><td>");
        s = input;
        while *s != 0 {
            match *s as i32 {
                10 => {
                    print!("</td></tr>\n<tr><td>");
                }
                44 => {
                    print!("</td><td>");
                }
                60 => {
                    print!("&lt;");
                }
                62 => {
                    print!("&gt;");
                }
                38 => {
                    print!("&amp;");
                }
                _ => {
                    print!("{}", *s as i32);
                }
            }
            s = s.offset(1);
            s;
        }
        puts(b"</td></tr>\n</table>\0" as *const u8 as *const i8);
        return 0;
    }
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
