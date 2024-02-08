#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn rc_crc32(
    mut crc: uint32_t,
    mut buf: *const libc::c_char,
    mut len: size_t,
) -> uint32_t {
    static mut table: [uint32_t; 256] = [0; 256];
    static mut have_table: libc::c_int = 0 as libc::c_int;
    let mut rem: uint32_t = 0;
    let mut octet: uint8_t = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut q: *const libc::c_char = 0 as *const libc::c_char;
    if have_table == 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < 256 as libc::c_int {
            rem = i as uint32_t;
            j = 0 as libc::c_int;
            while j < 8 as libc::c_int {
                if rem & 1 as libc::c_int as libc::c_uint != 0 {
                    rem >>= 1 as libc::c_int;
                    rem ^= 0xedb88320 as libc::c_uint;
                } else {
                    rem >>= 1 as libc::c_int;
                }
                j += 1;
                j;
            }
            table[i as usize] = rem;
            i += 1;
            i;
        }
        have_table = 1 as libc::c_int;
    }
    crc = !crc;
    q = buf.offset(len as isize);
    p = buf;
    while p < q {
        octet = *p as uint8_t;
        crc = crc >> 8 as libc::c_int
            ^ table[(crc & 0xff as libc::c_int as libc::c_uint ^ octet as libc::c_uint)
                as usize];
        p = p.offset(1);
        p;
    }
    return !crc;
}
unsafe fn main_0() -> libc::c_int {
    let mut s: *const libc::c_char = b"The quick brown fox jumps over the lazy dog\0"
        as *const u8 as *const libc::c_char;
    printf(
        b"%X\n\0" as *const u8 as *const libc::c_char,
        rc_crc32(0 as libc::c_int as uint32_t, s, strlen(s)),
    );
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
