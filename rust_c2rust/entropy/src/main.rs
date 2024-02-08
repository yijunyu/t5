#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn log2(_: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn makehist(
    mut S: *mut libc::c_char,
    mut hist: *mut libc::c_int,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut wherechar: [libc::c_int; 256] = [0; 256];
    let mut i: libc::c_int = 0;
    let mut histlen: libc::c_int = 0;
    histlen = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        wherechar[i as usize] = -(1 as libc::c_int);
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < len {
        if wherechar[*S.offset(i as isize) as libc::c_int as usize]
            == -(1 as libc::c_int)
        {
            wherechar[*S.offset(i as isize) as libc::c_int as usize] = histlen;
            histlen += 1;
            histlen;
        }
        let ref mut fresh0 = *hist
            .offset(wherechar[*S.offset(i as isize) as libc::c_int as usize] as isize);
        *fresh0 += 1;
        *fresh0;
        i += 1;
        i;
    }
    return histlen;
}
#[no_mangle]
pub unsafe extern "C" fn entropy(
    mut hist: *mut libc::c_int,
    mut histlen: libc::c_int,
    mut len: libc::c_int,
) -> libc::c_double {
    let mut i: libc::c_int = 0;
    let mut H: libc::c_double = 0.;
    H = 0 as libc::c_int as libc::c_double;
    i = 0 as libc::c_int;
    while i < histlen {
        H
            -= *hist.offset(i as isize) as libc::c_double / len as libc::c_double
                * log2(
                    *hist.offset(i as isize) as libc::c_double / len as libc::c_double,
                );
        i += 1;
        i;
    }
    return H;
}
unsafe fn main_0() -> libc::c_int {
    let mut S: [libc::c_char; 100] = [0; 100];
    let mut len: libc::c_int = 0;
    let mut hist: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut histlen: libc::c_int = 0;
    let mut H: libc::c_double = 0.;
    scanf(b"%[^\n]\0" as *const u8 as *const libc::c_char, S.as_mut_ptr());
    len = strlen(S.as_mut_ptr()) as libc::c_int;
    hist = calloc(
        len as libc::c_ulong,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    histlen = makehist(S.as_mut_ptr(), hist, len);
    H = entropy(hist, histlen, len);
    printf(b"%lf\n\0" as *const u8 as *const libc::c_char, H);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
