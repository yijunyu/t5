#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Factors {
    pub list: *mut libc::c_int,
    pub count: libc::c_short,
}
#[no_mangle]
pub unsafe extern "C" fn xferFactors(
    mut fctrs: *mut Factors,
    mut flist: *mut libc::c_int,
    mut flix: libc::c_int,
) {
    let mut ix: libc::c_int = 0;
    let mut ij: libc::c_int = 0;
    let mut newSize: libc::c_int = (*fctrs).count as libc::c_int + flix;
    if newSize > flix {
        (*fctrs)
            .list = realloc(
            (*fctrs).list as *mut libc::c_void,
            (newSize as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
    } else {
        (*fctrs)
            .list = malloc(
            (newSize as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
    }
    ij = 0 as libc::c_int;
    ix = (*fctrs).count as libc::c_int;
    while ix < newSize {
        *((*fctrs).list).offset(ix as isize) = *flist.offset(ij as isize);
        ij += 1;
        ij;
        ix += 1;
        ix;
    }
    (*fctrs).count = newSize as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn factor(
    mut num: libc::c_int,
    mut fctrs: *mut Factors,
) -> *mut Factors {
    let mut flist: [libc::c_int; 301] = [0; 301];
    let mut flix: libc::c_int = 0;
    let mut dvsr: libc::c_int = 0;
    flix = 0 as libc::c_int;
    (*fctrs).count = 0 as libc::c_int as libc::c_short;
    free((*fctrs).list as *mut libc::c_void);
    (*fctrs).list = 0 as *mut libc::c_int;
    dvsr = 1 as libc::c_int;
    while dvsr * dvsr < num {
        if !(num % dvsr != 0 as libc::c_int) {
            if flix == 300 as libc::c_int {
                xferFactors(fctrs, flist.as_mut_ptr(), flix);
                flix = 0 as libc::c_int;
            }
            let fresh0 = flix;
            flix = flix + 1;
            flist[fresh0 as usize] = dvsr;
            let fresh1 = flix;
            flix = flix + 1;
            flist[fresh1 as usize] = num / dvsr;
        }
        dvsr += 1;
        dvsr;
    }
    if dvsr * dvsr == num {
        let fresh2 = flix;
        flix = flix + 1;
        flist[fresh2 as usize] = dvsr;
    }
    if flix > 0 as libc::c_int {
        xferFactors(fctrs, flist.as_mut_ptr(), flix);
    }
    return fctrs;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut nums2factor: [libc::c_int; 4] = [
        2059 as libc::c_int,
        223092870 as libc::c_int,
        3135 as libc::c_int,
        45 as libc::c_int,
    ];
    let mut ftors: Factors = {
        let mut init = Factors {
            list: 0 as *mut libc::c_int,
            count: 0 as libc::c_int as libc::c_short,
        };
        init
    };
    let mut sep: libc::c_char = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        factor(nums2factor[i as usize], &mut ftors);
        printf(
            b"\nfactors of %d are:\n  \0" as *const u8 as *const libc::c_char,
            nums2factor[i as usize],
        );
        sep = ' ' as i32 as libc::c_char;
        j = 0 as libc::c_int;
        while j < ftors.count as libc::c_int {
            printf(
                b"%c %d\0" as *const u8 as *const libc::c_char,
                sep as libc::c_int,
                *(ftors.list).offset(j as isize),
            );
            sep = ',' as i32 as libc::c_char;
            j += 1;
            j;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
