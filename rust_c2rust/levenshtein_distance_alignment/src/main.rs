#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn putchar(__c: libc::c_int) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct edit_s {
    pub c1: libc::c_char,
    pub c2: libc::c_char,
    pub n: libc::c_int,
    pub next: edit,
}
pub type edit = *mut edit_s;
pub type edit_t = edit_s;
#[no_mangle]
pub unsafe extern "C" fn leven(mut a: *mut libc::c_char, mut b: *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut la: libc::c_int = strlen(a) as libc::c_int;
    let mut lb: libc::c_int = strlen(b) as libc::c_int;
    let mut tbl: *mut edit = malloc(
        (::core::mem::size_of::<edit>() as libc::c_ulong)
            .wrapping_mul((1 as libc::c_int + la) as libc::c_ulong),
    ) as *mut edit;
    let ref mut fresh0 = *tbl.offset(0 as libc::c_int as isize);
    *fresh0 = calloc(
        ((1 as libc::c_int + la) * (1 as libc::c_int + lb)) as libc::c_ulong,
        ::core::mem::size_of::<edit_t>() as libc::c_ulong,
    ) as edit;
    i = 1 as libc::c_int;
    while i <= la {
        let ref mut fresh1 = *tbl.offset(i as isize);
        *fresh1 = (*tbl.offset((i - 1 as libc::c_int) as isize))
            .offset((1 as libc::c_int + lb) as isize);
        i += 1;
        i;
    }
    i = la;
    while i >= 0 as libc::c_int {
        let mut aa: *mut libc::c_char = a.offset(i as isize);
        j = lb;
        while j >= 0 as libc::c_int {
            let mut bb: *mut libc::c_char = b.offset(j as isize);
            if !(*aa == 0 && *bb == 0) {
                let mut e: edit = &mut *(*tbl.offset(i as isize)).offset(j as isize)
                    as *mut edit_s;
                let mut repl: edit = &mut *(*tbl.offset((i + 1 as libc::c_int) as isize))
                    .offset((j + 1 as libc::c_int) as isize) as *mut edit_s;
                let mut dela: edit = &mut *(*tbl.offset((i + 1 as libc::c_int) as isize))
                    .offset(j as isize) as *mut edit_s;
                let mut delb: edit = &mut *(*tbl.offset(i as isize))
                    .offset((j + 1 as libc::c_int) as isize) as *mut edit_s;
                (*e).c1 = *aa;
                (*e).c2 = *bb;
                if *aa == 0 {
                    (*e).next = delb;
                    (*e).n = (*(*e).next).n + 1 as libc::c_int;
                } else if *bb == 0 {
                    (*e).next = dela;
                    (*e).n = (*(*e).next).n + 1 as libc::c_int;
                } else {
                    (*e).next = repl;
                    if *aa as libc::c_int == *bb as libc::c_int {
                        (*e).n = (*(*e).next).n;
                    } else {
                        if (*(*e).next).n > (*delb).n {
                            (*e).next = delb;
                            (*e).c1 = 0 as libc::c_int as libc::c_char;
                        }
                        if (*(*e).next).n > (*dela).n {
                            (*e).next = dela;
                            (*e).c1 = *aa;
                            (*e).c2 = 0 as libc::c_int as libc::c_char;
                        }
                        (*e).n = (*(*e).next).n + 1 as libc::c_int;
                    }
                }
            }
            j -= 1;
            j;
        }
        i -= 1;
        i;
    }
    let mut p: edit = *tbl.offset(0 as libc::c_int as isize);
    printf(b"%s -> %s: %d edits\n\0" as *const u8 as *const libc::c_char, a, b, (*p).n);
    while !((*p).next).is_null() {
        if (*p).c1 as libc::c_int == (*p).c2 as libc::c_int {
            printf(b"%c\0" as *const u8 as *const libc::c_char, (*p).c1 as libc::c_int);
        } else {
            putchar('(' as i32);
            if (*p).c1 != 0 {
                putchar((*p).c1 as libc::c_int);
            }
            putchar(',' as i32);
            if (*p).c2 != 0 {
                putchar((*p).c2 as libc::c_int);
            }
            putchar(')' as i32);
        }
        p = (*p).next;
    }
    putchar('\n' as i32);
    free(*tbl.offset(0 as libc::c_int as isize) as *mut libc::c_void);
    free(tbl as *mut libc::c_void);
}
unsafe fn main_0() -> libc::c_int {
    leven(
        b"raisethysword\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"rosettacode\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
