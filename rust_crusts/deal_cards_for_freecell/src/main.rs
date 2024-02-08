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
    fn setlocale(__category: i32, __locale: *const i8) -> *mut i8;
}
#[no_mangle]
pub static mut s_suits: [i32; 5] = unsafe {
    *::core::mem::transmute::<&[u8; 20], &mut [i32; 5]>(b"c&\0\0f&\0\0e&\0\0`&\0\0\0\0\0\0")
};
#[no_mangle]
pub static mut s_nums: [i32; 14] = unsafe {
    * :: core :: mem :: transmute :: < & [u8; 56], & mut [i32; 14], > (b"A\0\0\x002\0\0\x003\0\0\x004\0\0\x005\0\0\x006\0\0\x007\0\0\08\0\0\09\0\0\0T\0\0\0J\0\0\0Q\0\0\0K\0\0\0\0\0\0\0",)
};
static mut seed: i32 = 1;
#[no_mangle]
pub extern "C" fn rnd() -> i32 {
    unsafe {
        seed = ((seed * 214013 + 2531011i32) as u32 & (1u32 << 31i32).wrapping_sub(1)) as i32;
        return seed >> 16;
    }
}

#[no_mangle]
pub extern "C" fn srnd(mut x: i32) {
    unsafe {
        seed = x;
    }
}

#[no_mangle]
pub extern "C" fn show(mut c: *const i32) {
    unsafe {
        let mut i: i32 = 0;
        i = 0;
        while i < 52 {
            print!(
                "  \x1B[{}m{}\x1B[m{}",
                32 - (1 + *c) % 4 / 2,
                s_suits[(*c % 4i32) as usize],
                s_nums[(*c / 4i32) as usize]
            );
            i += 1;
            if i % 8 == 0 || i == 52 {
                print!("{}", '\n' as i32);
            }
            c = c.offset(1);
            c;
        }
    }
}

#[no_mangle]
pub extern "C" fn deal(mut s: i32, mut t: *mut i32) {
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        srnd(s);
        i = 0;
        while i < 52 {
            *t.offset(i as isize) = 51 - i;
            i += 1;
            i;
        }
        i = 0;
        while i < 51 {
            j = 51 - rnd() % (52 - i);
            s = *t.offset(i as isize);
            *t.offset(i as isize) = *t.offset(j as isize);
            *t.offset(j as isize) = s;
            i += 1;
            i;
        }
    }
}

fn main_0(mut c: i32, mut v: *mut *mut i8) -> i32 {
    unsafe {
        let mut s: i32 = 0;
        let mut card: [i32; 52] = [0; 52];
        if c < 2 || {
            s = atoi(*v.offset(1 as isize));
            s <= 0
        } {
            s = 11982;
        }
        setlocale(6, b"\0" as *const u8 as *const i8);
        deal(s, card.as_mut_ptr());
        print!("Hand {}\n", s);
        show(card.as_mut_ptr());
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
