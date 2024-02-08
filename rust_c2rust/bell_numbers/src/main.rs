#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn bellIndex(
    mut row: libc::c_int,
    mut col: libc::c_int,
) -> size_t {
    return (row * (row - 1 as libc::c_int) / 2 as libc::c_int + col) as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn getBell(
    mut bellTri: *mut libc::c_int,
    mut row: libc::c_int,
    mut col: libc::c_int,
) -> libc::c_int {
    let mut index: size_t = bellIndex(row, col);
    return *bellTri.offset(index as isize);
}
#[no_mangle]
pub unsafe extern "C" fn setBell(
    mut bellTri: *mut libc::c_int,
    mut row: libc::c_int,
    mut col: libc::c_int,
    mut value: libc::c_int,
) {
    let mut index: size_t = bellIndex(row, col);
    *bellTri.offset(index as isize) = value;
}
#[no_mangle]
pub unsafe extern "C" fn bellTriangle(mut n: libc::c_int) -> *mut libc::c_int {
    let mut length: size_t = (n * (n + 1 as libc::c_int) / 2 as libc::c_int) as size_t;
    let mut tri: *mut libc::c_int = calloc(
        length,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    setBell(tri, 1 as libc::c_int, 0 as libc::c_int, 1 as libc::c_int);
    i = 2 as libc::c_int;
    while i <= n {
        setBell(
            tri,
            i,
            0 as libc::c_int,
            getBell(tri, i - 1 as libc::c_int, i - 2 as libc::c_int),
        );
        j = 1 as libc::c_int;
        while j < i {
            let mut value: libc::c_int = getBell(tri, i, j - 1 as libc::c_int)
                + getBell(tri, i - 1 as libc::c_int, j - 1 as libc::c_int);
            setBell(tri, i, j, value);
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    return tri;
}
unsafe fn main_0() -> libc::c_int {
    let rows: libc::c_int = 15 as libc::c_int;
    let mut bt: *mut libc::c_int = bellTriangle(rows);
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    printf(b"First fifteen Bell numbers:\n\0" as *const u8 as *const libc::c_char);
    i = 1 as libc::c_int;
    while i <= rows {
        printf(
            b"%2d: %d\n\0" as *const u8 as *const libc::c_char,
            i,
            getBell(bt, i, 0 as libc::c_int),
        );
        i += 1;
        i;
    }
    printf(
        b"\nThe first ten rows of Bell's triangle:\n\0" as *const u8
            as *const libc::c_char,
    );
    i = 1 as libc::c_int;
    while i <= 10 as libc::c_int {
        printf(
            b"%d\0" as *const u8 as *const libc::c_char,
            getBell(bt, i, 0 as libc::c_int),
        );
        j = 1 as libc::c_int;
        while j < i {
            printf(b", %d\0" as *const u8 as *const libc::c_char, getBell(bt, i, j));
            j += 1;
            j;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        i += 1;
        i;
    }
    free(bt as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
