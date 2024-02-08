#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use c2rust_out::*;
extern "C" {
    fn malloc(_: u64) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Factors {
    pub list: *mut i32,
    pub count: i16,
}
#[no_mangle]
pub extern "C" fn xferFactors(mut fctrs: *mut Factors, mut flist: *mut i32, mut flix: i32) {
    unsafe {
        let mut ix: i32 = 0;
        let mut ij: i32 = 0;
        let mut newSize: i32 = (*fctrs).count as i32 + flix;
        if newSize > flix {
            (*fctrs).list = realloc(
                (*fctrs).list as *mut libc::c_void,
                (newSize as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64),
            ) as *mut i32;
        } else {
            (*fctrs).list =
                malloc((newSize as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64))
                    as *mut i32;
        }
        ij = 0;
        ix = (*fctrs).count as i32;
        while ix < newSize {
            *((*fctrs).list).offset(ix as isize) = *flist.offset(ij as isize);
            ij += 1;
            ij;
            ix += 1;
            ix;
        }
        (*fctrs).count = newSize as i16;
    }
}

#[no_mangle]
pub extern "C" fn factor(mut num: i32, mut fctrs: *mut Factors) -> *mut Factors {
    unsafe {
        let mut flist: [i32; 301] = [0; 301];
        let mut flix: i32 = 0;
        let mut dvsr: i32 = 0;
        flix = 0;
        (*fctrs).count = 0;
        free((*fctrs).list as *mut libc::c_void);
        (*fctrs).list = 0 as *mut i32;
        dvsr = 1;
        while dvsr * dvsr < num {
            if !(num % dvsr != 0) {
                if flix == 300 {
                    xferFactors(fctrs, flist.as_mut_ptr(), flix);
                    flix = 0;
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
        if flix > 0 {
            xferFactors(fctrs, flist.as_mut_ptr(), flix);
        }
        return fctrs;
    }
}

fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    unsafe {
        let mut nums2factor: [i32; 4] = [2059, 223092870, 3135, 45];
        let mut ftors: Factors = {
            let mut init = Factors {
                list: 0 as *mut i32,
                count: 0,
            };
            init
        };
        let mut sep: i8 = 0;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        i = 0;
        while i < 4 {
            factor(nums2factor[i as usize], &mut ftors);
            print!("\nfactors of {} are:\n  ", nums2factor[i as usize]);
            sep = ' ' as i8;
            j = 0;
            while j < ftors.count as i32 {
                print!("{} {}", sep as i32, *(ftors.list).offset(j as isize));
                sep = ',' as i8;
                j += 1;
                j;
            }
            print!("\n");
            i += 1;
            i;
        }
        return 0;
    }
}

pub fn main() {
    let mut args: Vec<*mut i8> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    ::std::process::exit(main_0((args.len() - 1) as i32, args.as_mut_ptr() as *mut *mut i8) as i32);
}
