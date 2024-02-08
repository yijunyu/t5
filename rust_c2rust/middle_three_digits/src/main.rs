#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn mid3(mut n: libc::c_int) -> *mut libc::c_char {
    static mut buf: [libc::c_char; 32] = [0; 32];
    let mut l: libc::c_int = 0;
    sprintf(
        buf.as_mut_ptr(),
        b"%d\0" as *const u8 as *const libc::c_char,
        if n > 0 as libc::c_int { n } else { -n },
    );
    l = strlen(buf.as_mut_ptr()) as libc::c_int;
    if l < 3 as libc::c_int || l & 1 as libc::c_int == 0 {
        return 0 as *mut libc::c_char;
    }
    l = l / 2 as libc::c_int - 1 as libc::c_int;
    buf[(l + 3 as libc::c_int) as usize] = 0 as libc::c_int as libc::c_char;
    return buf.as_mut_ptr().offset(l as isize);
}
unsafe fn main_0() -> libc::c_int {
    let mut x: [libc::c_int; 18] = [
        123 as libc::c_int,
        12345 as libc::c_int,
        1234567 as libc::c_int,
        987654321 as libc::c_int,
        10001 as libc::c_int,
        -(10001 as libc::c_int),
        -(123 as libc::c_int),
        -(100 as libc::c_int),
        100 as libc::c_int,
        -(12345 as libc::c_int),
        1 as libc::c_int,
        2 as libc::c_int,
        -(1 as libc::c_int),
        -(10 as libc::c_int),
        2002 as libc::c_int,
        -(2002 as libc::c_int),
        0 as libc::c_int,
        1234567890 as libc::c_int,
    ];
    let mut i: libc::c_int = 0;
    let mut m: *mut libc::c_char = 0 as *mut libc::c_char;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[libc::c_int; 18]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
    {
        m = mid3(x[i as usize]);
        if m.is_null() {
            m = b"error\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        printf(b"%d: %s\n\0" as *const u8 as *const libc::c_char, x[i as usize], m);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
