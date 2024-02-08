#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn putchar(__c: libc::c_int) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
}
#[no_mangle]
pub static mut input: *const libc::c_char = b"Character,Speech\nThe multitude,The messiah! Show us the messiah!\nBrians mother,<angry>Now you listen here! He's not the messiah; he's a very naughty boy! Now go away!</angry>\nThe multitude,Who are you?\nBrians mother,I'm his mother; that's who!\nThe multitude,Behold his mother! Behold his mother!\0"
    as *const u8 as *const libc::c_char;
unsafe fn main_0() -> libc::c_int {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    printf(b"<table>\n<tr><td>\0" as *const u8 as *const libc::c_char);
    s = input;
    while *s != 0 {
        match *s as libc::c_int {
            10 => {
                printf(b"</td></tr>\n<tr><td>\0" as *const u8 as *const libc::c_char);
            }
            44 => {
                printf(b"</td><td>\0" as *const u8 as *const libc::c_char);
            }
            60 => {
                printf(b"&lt;\0" as *const u8 as *const libc::c_char);
            }
            62 => {
                printf(b"&gt;\0" as *const u8 as *const libc::c_char);
            }
            38 => {
                printf(b"&amp;\0" as *const u8 as *const libc::c_char);
            }
            _ => {
                putchar(*s as libc::c_int);
            }
        }
        s = s.offset(1);
        s;
    }
    puts(b"</td></tr>\n</table>\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
