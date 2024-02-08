#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
}
#[no_mangle]
pub static mut trans: [libc::c_char; 9] = unsafe {
    *::core::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"___#_##_\0")
};
#[no_mangle]
pub unsafe extern "C" fn evolve(
    mut cell: *mut libc::c_char,
    mut backup: *mut libc::c_char,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut diff: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < len {
        *backup
            .offset(
                i as isize,
            ) = trans[((*cell.offset((i - 1 as libc::c_int) as isize) as libc::c_int
            != '_' as i32) as libc::c_int * 4 as libc::c_int
            + (*cell.offset(i as isize) as libc::c_int != '_' as i32) as libc::c_int
                * 2 as libc::c_int
            + (*cell.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                != '_' as i32) as libc::c_int) as usize];
        diff
            += (*backup.offset(i as isize) as libc::c_int
                != *cell.offset(i as isize) as libc::c_int) as libc::c_int;
        i += 1;
        i;
    }
    strcpy(cell, backup as *const libc::c_char);
    return diff;
}
unsafe fn main_0() -> libc::c_int {
    let mut c: [libc::c_char; 22] = *::core::mem::transmute::<
        &[u8; 22],
        &mut [libc::c_char; 22],
    >(b"_###_##_#_#_#_#__#__\n\0");
    let mut b: [libc::c_char; 22] = *::core::mem::transmute::<
        &[u8; 22],
        &mut [libc::c_char; 22],
    >(b"____________________\n\0");
    loop {
        printf(c.as_mut_ptr().offset(1 as libc::c_int as isize));
        if !(evolve(
            c.as_mut_ptr().offset(1 as libc::c_int as isize),
            b.as_mut_ptr().offset(1 as libc::c_int as isize),
            (::core::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong)
                .wrapping_sub(3 as libc::c_int as libc::c_ulong) as libc::c_int,
        ) != 0)
        {
            break;
        }
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
