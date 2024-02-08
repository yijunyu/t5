#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn move_to_front(
    mut str: *mut libc::c_char,
    mut c: libc::c_char,
) -> libc::c_int {
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut shift: libc::c_int = 0 as libc::c_int;
    p = malloc((strlen(str)).wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    strcpy(p, str);
    q = strchr(p, c as libc::c_int);
    shift = q.offset_from(p) as libc::c_long as libc::c_int;
    strncpy(str.offset(1 as libc::c_int as isize), p, shift as libc::c_ulong);
    *str.offset(0 as libc::c_int as isize) = c;
    free(p as *mut libc::c_void);
    return shift;
}
#[no_mangle]
pub unsafe extern "C" fn decode(
    mut pass: *mut libc::c_int,
    mut size: libc::c_int,
    mut sym: *mut libc::c_char,
) {
    let mut i: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    let mut c: libc::c_char = 0;
    let mut table: [libc::c_char; 27] = *::core::mem::transmute::<
        &[u8; 27],
        &mut [libc::c_char; 27],
    >(b"abcdefghijklmnopqrstuvwxyz\0");
    i = 0 as libc::c_int;
    while i < size {
        c = table[*pass.offset(i as isize) as usize];
        index = move_to_front(table.as_mut_ptr(), c);
        if *pass.offset(i as isize) != index {
            printf(b"there is an error\0" as *const u8 as *const libc::c_char);
        }
        *sym.offset(i as isize) = c;
        i += 1;
        i;
    }
    *sym.offset(size as isize) = '\0' as i32 as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn encode(
    mut sym: *mut libc::c_char,
    mut size: libc::c_int,
    mut pass: *mut libc::c_int,
) {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut c: libc::c_char = 0;
    let mut table: [libc::c_char; 27] = *::core::mem::transmute::<
        &[u8; 27],
        &mut [libc::c_char; 27],
    >(b"abcdefghijklmnopqrstuvwxyz\0");
    i = 0 as libc::c_int;
    while i < size {
        c = *sym.offset(i as isize);
        *pass.offset(i as isize) = move_to_front(table.as_mut_ptr(), c);
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn check(
    mut sym: *mut libc::c_char,
    mut size: libc::c_int,
    mut pass: *mut libc::c_int,
) -> libc::c_int {
    let mut pass2: *mut libc::c_int = malloc(
        (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(size as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut sym2: *mut libc::c_char = malloc(
        (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_mul(size as libc::c_ulong),
    ) as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut val: libc::c_int = 1 as libc::c_int;
    encode(sym, size, pass2);
    i = 0 as libc::c_int;
    while i < size && *pass.offset(i as isize) == *pass2.offset(i as isize) {
        i += 1;
        i;
    }
    if i != size {
        val = 0 as libc::c_int;
    }
    decode(pass, size, sym2);
    if strcmp(sym, sym2) != 0 as libc::c_int {
        val = 0 as libc::c_int;
    }
    free(sym2 as *mut libc::c_void);
    free(pass2 as *mut libc::c_void);
    return val;
}
unsafe fn main_0() -> libc::c_int {
    let mut sym: [[libc::c_char; 100]; 3] = [
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"broood\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"bananaaa\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"hiphophiphop\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
    ];
    let mut pass: [libc::c_int; 100] = [
        0 as libc::c_int,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        len = strlen((sym[i as usize]).as_mut_ptr()) as libc::c_int;
        encode((sym[i as usize]).as_mut_ptr(), len, pass.as_mut_ptr());
        printf(
            b"%s : [\0" as *const u8 as *const libc::c_char,
            (sym[i as usize]).as_mut_ptr(),
        );
        j = 0 as libc::c_int;
        while j < len {
            printf(b"%d \0" as *const u8 as *const libc::c_char, pass[j as usize]);
            j += 1;
            j;
        }
        printf(b"]\n\0" as *const u8 as *const libc::c_char);
        if check((sym[i as usize]).as_mut_ptr(), len, pass.as_mut_ptr()) != 0 {
            printf(b"Correct :)\n\0" as *const u8 as *const libc::c_char);
        } else {
            printf(b"Incorrect :(\n\0" as *const u8 as *const libc::c_char);
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
