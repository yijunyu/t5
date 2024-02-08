#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn putchar(__c: libc::c_int) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type ptrdiff_t = libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn lcs(
    sa: *const libc::c_char,
    sb: *const libc::c_char,
    beg: *mut *mut libc::c_char,
    end: *mut *mut libc::c_char,
) {
    let mut apos: size_t = 0;
    let mut bpos: size_t = 0;
    let mut len: ptrdiff_t = 0;
    *beg = 0 as *mut libc::c_char;
    *end = 0 as *mut libc::c_char;
    len = 0 as libc::c_int;
    apos = 0 as libc::c_int as size_t;
    while *sa.offset(apos as isize) as libc::c_int != 0 as libc::c_int {
        bpos = 0 as libc::c_int as size_t;
        while *sb.offset(bpos as isize) as libc::c_int != 0 as libc::c_int {
            if *sa.offset(apos as isize) as libc::c_int
                == *sb.offset(bpos as isize) as libc::c_int
            {
                len = 1 as libc::c_int;
                while *sa.offset(apos.wrapping_add(len as libc::c_ulong) as isize)
                    as libc::c_int != 0 as libc::c_int
                    && *sb.offset(bpos.wrapping_add(len as libc::c_ulong) as isize)
                        as libc::c_int != 0 as libc::c_int
                    && *sa.offset(apos.wrapping_add(len as libc::c_ulong) as isize)
                        as libc::c_int
                        == *sb.offset(bpos.wrapping_add(len as libc::c_ulong) as isize)
                            as libc::c_int
                {
                    len += 1;
                    len;
                }
            }
            if len as libc::c_long > (*end).offset_from(*beg) as libc::c_long {
                *beg = sa.offset(apos as isize) as *mut libc::c_char;
                *end = (*beg).offset(len as isize);
                len = 0 as libc::c_int;
            }
            bpos = bpos.wrapping_add(1);
            bpos;
        }
        apos = apos.wrapping_add(1);
        apos;
    }
}
unsafe fn main_0() -> libc::c_int {
    let mut s1: *mut libc::c_char = b"thisisatest\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    let mut s2: *mut libc::c_char = b"testing123testing\0" as *const u8
        as *const libc::c_char as *mut libc::c_char;
    let mut beg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut it: *mut libc::c_char = 0 as *mut libc::c_char;
    lcs(s1, s2, &mut beg, &mut end);
    it = beg;
    while it != end {
        putchar(*it as libc::c_int);
        it = it.offset(1);
        it;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
