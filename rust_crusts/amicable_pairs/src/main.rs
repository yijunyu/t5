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
    fn atoi(__nptr: *const i8) -> i32;
    fn malloc(_: u64) -> *mut libc::c_void;
}
fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    unsafe {
        let mut top: u32 = atoi(*argv.offset(1 as isize)) as u32;
        let mut divsum: *mut u32 = malloc(
            (top.wrapping_add(1u32) as u64).wrapping_mul(::core::mem::size_of::<u32>() as u64),
        ) as *mut u32;
        let mut pows: [u32; 32] = [
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ];
        let mut i: u32 = 0;
        while i <= top {
            *divsum.offset(i as isize) = 1;
            i = i.wrapping_add(1);
            i;
        }
        let mut p: u32 = 2;
        while p.wrapping_add(p) <= top {
            if *divsum.offset(p as isize) > 1 {
                let ref mut fresh0 = *divsum.offset(p as isize);
                *fresh0 = (*fresh0 as u32).wrapping_sub(p) as u32;
            } else {
                let mut x: u32 = 0;
                x = 1;
                while pows[x.wrapping_sub(1) as usize] <= top.wrapping_div(p) {
                    pows[x as usize] = p.wrapping_mul(pows[x.wrapping_sub(1) as usize]);
                    x = x.wrapping_add(1);
                    x;
                }
                let mut k: u32 = p.wrapping_sub(1);
                let mut n: u32 = p.wrapping_add(p);
                while n <= top {
                    let mut s: u32 = 1u32.wrapping_add(pows[1 as usize]);
                    k = k.wrapping_sub(1);
                    k;
                    if k == 0 {
                        let mut i_0: u32 = 2;
                        while i_0 < x && n.wrapping_rem(pows[i_0 as usize]) == 0 {
                            let fresh1 = i_0;
                            i_0 = i_0.wrapping_add(1);
                            s = (s).wrapping_add(pows[fresh1 as usize]) as u32;
                        }
                        k = p;
                    }
                    let ref mut fresh2 = *divsum.offset(n as isize);
                    *fresh2 = (*fresh2 as u32).wrapping_mul(s) as u32;
                    n = (n).wrapping_add(p) as u32;
                }
            }
            p = p.wrapping_add(1);
            p;
        }
        let mut p_0: u32 = (top >> 1i32).wrapping_add(1);
        while p_0 <= top {
            if *divsum.offset(p_0 as isize) > 1 {
                let ref mut fresh3 = *divsum.offset(p_0 as isize);
                *fresh3 = (*fresh3 as u32).wrapping_sub(p_0) as u32;
            }
            p_0 = p_0.wrapping_add(1);
            p_0;
        }
        let mut cnt: u32 = 0;
        let mut a: u32 = 1;
        while a <= top {
            let mut b: u32 = *divsum.offset(a as isize);
            if b > a && b <= top && *divsum.offset(b as isize) == a {
                print!("{} {}\n", a, b);
                cnt = cnt.wrapping_add(1);
                cnt;
            }
            a = a.wrapping_add(1);
            a;
        }
        print!("\nTop {} count : {}\n", top, cnt);
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
