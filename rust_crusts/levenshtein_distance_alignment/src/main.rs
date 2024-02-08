#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
fn build_str_from_raw_ptr(raw_ptr: *mut u8) -> String {
    unsafe {
        let mut str_size: usize = 0;
        while *raw_ptr.offset(str_size as isize) != 0 {
            str_size += 1;
        }
        return std::str::from_utf8_unchecked(std::slice::from_raw_parts(raw_ptr, str_size))
            .to_owned();
    }
}

use c2rust_out::*;
extern "C" {
    fn malloc(_: u64) -> *mut libc::c_void;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn strlen(_: *const i8) -> u64;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct edit_s {
    pub c1: i8,
    pub c2: i8,
    pub n: i32,
    pub next: edit,
}
pub type edit = *mut edit_s;
pub type edit_t = edit_s;
#[no_mangle]
pub extern "C" fn leven(mut a: *mut i8, mut b: *mut i8) {
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut la: i32 = strlen(a) as i32;
        let mut lb: i32 = strlen(b) as i32;
        let mut tbl: *mut edit =
            malloc((::core::mem::size_of::<edit>() as u64).wrapping_mul((1 + la) as u64))
                as *mut edit;
        let ref mut fresh0 = *tbl.offset(0 as isize);
        *fresh0 = calloc(
            ((1 + la) * (1 + lb)) as u64,
            ::core::mem::size_of::<edit_t>() as u64,
        ) as edit;
        i = 1;
        while i <= la {
            let ref mut fresh1 = *tbl.offset(i as isize);
            *fresh1 = (*tbl.offset((i - 1i32) as isize)).offset((1 + lb) as isize);
            i += 1;
            i;
        }
        i = la;
        while i >= 0 {
            let mut aa: *mut i8 = a.offset(i as isize);
            j = lb;
            while j >= 0 {
                let mut bb: *mut i8 = b.offset(j as isize);
                if !(*aa == 0 && *bb == 0) {
                    let mut e: edit =
                        &mut *(*tbl.offset(i as isize)).offset(j as isize) as *mut edit_s;
                    let mut repl: edit = &mut *(*tbl.offset((i + 1i32) as isize))
                        .offset((j + 1i32) as isize)
                        as *mut edit_s;
                    let mut dela: edit =
                        &mut *(*tbl.offset((i + 1i32) as isize)).offset(j as isize) as *mut edit_s;
                    let mut delb: edit =
                        &mut *(*tbl.offset(i as isize)).offset((j + 1i32) as isize) as *mut edit_s;
                    (*e).c1 = *aa;
                    (*e).c2 = *bb;
                    if *aa == 0 {
                        (*e).next = delb;
                        (*e).n = (*(*e).next).n + 1;
                    } else if *bb == 0 {
                        (*e).next = dela;
                        (*e).n = (*(*e).next).n + 1;
                    } else {
                        (*e).next = repl;
                        if *aa as i32 == *bb as i32 {
                            (*e).n = (*(*e).next).n;
                        } else {
                            if (*(*e).next).n > (*delb).n {
                                (*e).next = delb;
                                (*e).c1 = 0;
                            }
                            if (*(*e).next).n > (*dela).n {
                                (*e).next = dela;
                                (*e).c1 = *aa;
                                (*e).c2 = 0;
                            };
                            (*e).n = (*(*e).next).n + 1;
                        }
                    }
                }
                j -= 1;
                j;
            }
            i -= 1;
            i;
        }
        let mut p: edit = *tbl.offset(0 as isize);
        print!(
            "{} -> {}: {} edits\n",
            build_str_from_raw_ptr(a as *mut u8),
            build_str_from_raw_ptr(b as *mut u8),
            (*p).n
        );
        while !((*p).next).is_null() {
            if (*p).c1 as i32 == (*p).c2 as i32 {
                print!("{}", (*p).c1 as i32);
            } else {
                print!("{}", '(' as i32);
                if (*p).c1 != 0 {
                    print!("{}", (*p).c1 as i32);
                }
                print!("{}", ',' as i32);
                if (*p).c2 != 0 {
                    print!("{}", (*p).c2 as i32);
                }
                print!("{}", ')' as i32);
            }
            p = (*p).next;
        }
        print!("{}", '\n' as i32);
        free(*tbl.offset(0 as isize) as *mut libc::c_void);
        free(tbl as *mut libc::c_void);
    }
}

fn main_0() -> i32 {
    leven(
        b"raisethysword\0" as *const u8 as *const i8 as *mut i8,
        b"rosettacode\0" as *const u8 as *const i8 as *mut i8,
    );
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
