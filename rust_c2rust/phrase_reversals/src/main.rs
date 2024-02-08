#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn reverse_section(
    mut s: *mut libc::c_char,
    mut length: size_t,
) -> *mut libc::c_char {
    if length == 0 as libc::c_int as libc::c_ulong {
        return s;
    }
    let mut i: size_t = 0;
    let mut temp: libc::c_char = 0;
    i = 0 as libc::c_int as size_t;
    while i
        < length
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
    {
        temp = *s.offset(i as isize);
        *s.offset(i as isize) = *s.offset(length.wrapping_sub(i) as isize);
        *s.offset(length.wrapping_sub(i) as isize) = temp;
        i = i.wrapping_add(1);
        i;
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn reverse_words_in_order(
    mut s: *mut libc::c_char,
    mut delim: libc::c_char,
) -> *mut libc::c_char {
    if strlen(s) == 0 {
        return s;
    }
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (strlen(s)).wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        j = 0 as libc::c_int as size_t;
        while *s.offset(i.wrapping_add(j) as isize) as libc::c_int != 0 as libc::c_int
            && *s.offset(i.wrapping_add(j) as isize) as libc::c_int
                != delim as libc::c_int
        {
            j = j.wrapping_add(1);
            j;
        }
        reverse_section(
            s.offset(i as isize),
            j.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        s = s.offset(j as isize);
        i = i.wrapping_add(1);
        i;
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn reverse_string(mut s: *mut libc::c_char) -> *mut libc::c_char {
    return if strlen(s) != 0 {
        reverse_section(s, (strlen(s)).wrapping_sub(1 as libc::c_int as libc::c_ulong))
    } else {
        s
    };
}
#[no_mangle]
pub unsafe extern "C" fn reverse_order_of_words(
    mut s: *mut libc::c_char,
    mut delim: libc::c_char,
) -> *mut libc::c_char {
    reverse_string(s);
    reverse_words_in_order(s, delim);
    return s;
}
unsafe fn main_0() -> libc::c_int {
    let mut str: [libc::c_char; 29] = *::core::mem::transmute::<
        &[u8; 29],
        &mut [libc::c_char; 29],
    >(b"rosetta code phrase reversal\0");
    let mut lenstr: size_t = (::core::mem::size_of::<[libc::c_char; 29]>()
        as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong);
    let vla = lenstr as usize;
    let mut scopy: Vec::<libc::c_char> = ::std::vec::from_elem(0, vla);
    let mut delim: libc::c_char = ' ' as i32 as libc::c_char;
    printf(
        b"Original:       \"%s\"\n\0" as *const u8 as *const libc::c_char,
        str.as_mut_ptr(),
    );
    strncpy(scopy.as_mut_ptr(), str.as_mut_ptr(), lenstr);
    reverse_string(scopy.as_mut_ptr());
    printf(
        b"Reversed:       \"%s\"\n\0" as *const u8 as *const libc::c_char,
        scopy.as_mut_ptr(),
    );
    strncpy(scopy.as_mut_ptr(), str.as_mut_ptr(), lenstr);
    reverse_words_in_order(scopy.as_mut_ptr(), delim);
    printf(
        b"Reversed words: \"%s\"\n\0" as *const u8 as *const libc::c_char,
        scopy.as_mut_ptr(),
    );
    strncpy(scopy.as_mut_ptr(), str.as_mut_ptr(), lenstr);
    reverse_order_of_words(scopy.as_mut_ptr(), delim);
    printf(
        b"Reversed order: \"%s\"\n\0" as *const u8 as *const libc::c_char,
        scopy.as_mut_ptr(),
    );
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
