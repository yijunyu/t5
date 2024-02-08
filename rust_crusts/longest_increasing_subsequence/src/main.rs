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
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node {
    pub val: i32,
    pub len: i32,
    pub next: *mut node,
}
#[no_mangle]
pub extern "C" fn lis(mut v: *mut i32, mut len: i32) {
    unsafe {
        let mut i: i32 = 0;
        let mut p: *mut node = 0 as *mut node;
        let mut n: *mut node =
            calloc(len as u64, ::core::mem::size_of::<node>() as u64) as *mut node;
        i = 0;
        while i < len {
            (*n.offset(i as isize)).val = *v.offset(i as isize);
            i += 1;
            i;
        }
        i = len;
        loop {
            let fresh0 = i;
            i = i - 1;
            if !(fresh0 != 0) {
                break;
            }
            p = n.offset(i as isize);
            loop {
                let fresh1 = p;
                p = p.offset(1);
                if !(fresh1 < n.offset(len as isize)) {
                    break;
                }
                if (*p).val > (*n.offset(i as isize)).val && (*p).len >= (*n.offset(i as isize)).len
                {
                    let ref mut fresh2 = (*n.offset(i as isize)).next;
                    *fresh2 = p;
                    (*n.offset(i as isize)).len = (*p).len + 1;
                }
            }
        }
        i = 0;
        p = n;
        while i < len {
            if (*n.offset(i as isize)).len > (*p).len {
                p = n.offset(i as isize);
            }
            i += 1;
            i;
        }
        loop {
            print!(" {}", (*p).val);
            p = (*p).next;
            if p.is_null() {
                break;
            }
        }
        print!("{}", '\n' as i32);
        free(n as *mut libc::c_void);
    }
}

fn main_0() -> i32 {
    let mut x: [i32; 6] = [3, 2, 6, 4, 5, 1];
    let mut y: [i32; 16] = [0, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 11, 7, 15];
    lis(
        x.as_mut_ptr(),
        (::core::mem::size_of::<[i32; 6]>() as u64)
            .wrapping_div(::core::mem::size_of::<i32>() as u64) as i32,
    );
    lis(
        y.as_mut_ptr(),
        (::core::mem::size_of::<[i32; 16]>() as u64)
            .wrapping_div(::core::mem::size_of::<i32>() as u64) as i32,
    );
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
