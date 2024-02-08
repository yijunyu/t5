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
    fn scanf(_: *const i8, _: ...) -> i32;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn strlen(_: *const i8) -> u64;
    fn log2(_: f64) -> f64;
}
#[no_mangle]
pub extern "C" fn makehist(mut S: *mut i8, mut hist: *mut i32, mut len: i32) -> i32 {
    unsafe {
        let mut wherechar: [i32; 256] = [0; 256];
        let mut i: i32 = 0;
        let mut histlen: i32 = 0;
        histlen = 0;
        i = 0;
        while i < 256 {
            wherechar[i as usize] = -1;
            i += 1;
            i;
        }
        i = 0;
        while i < len {
            if wherechar[*S.offset(i as isize) as i32 as usize] == -1 {
                wherechar[*S.offset(i as isize) as i32 as usize] = histlen;
                histlen += 1;
                histlen;
            }
            let ref mut fresh0 =
                *hist.offset(wherechar[*S.offset(i as isize) as i32 as usize] as isize);
            *fresh0 += 1;
            *fresh0;
            i += 1;
            i;
        }
        return histlen;
    }
}

#[no_mangle]
pub extern "C" fn entropy(mut hist: *mut i32, mut histlen: i32, mut len: i32) -> f64 {
    unsafe {
        let mut i: i32 = 0;
        let mut H: f64 = 0.;
        H = 0 as f64;
        i = 0;
        while i < histlen {
            H -= *hist.offset(i as isize) as f64 / len as f64
                * log2(*hist.offset(i as isize) as f64 / len as f64);
            i += 1;
            i;
        }
        return H;
    }
}

fn main_0() -> i32 {
    unsafe {
        let mut S: [i8; 100] = [0; 100];
        let mut len: i32 = 0;
        let mut hist: *mut i32 = 0 as *mut i32;
        let mut histlen: i32 = 0;
        let mut H: f64 = 0.;
        scanf(b"%[^\n]\0" as *const u8 as *const i8, S.as_mut_ptr());
        len = strlen(S.as_mut_ptr()) as i32;
        hist = calloc(len as u64, ::core::mem::size_of::<i32>() as u64) as *mut i32;
        histlen = makehist(S.as_mut_ptr(), hist, len);
        H = entropy(hist, histlen, len);
        print!("{}\n", H);
        return 0;
    }
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
