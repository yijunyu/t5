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
    fn log10(_: f64) -> f64;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
fn main_0() -> i32 {
    unsafe {
        let mut i: u64 = 0;
        while i < 20 {
            let mut binstr: *mut i8 = bin(i as u32);
            print!("{}\n", build_str_from_raw_ptr(binstr as *mut u8));
            free(binstr as *mut libc::c_void);
            i = i.wrapping_add(1);
            i;
        }
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn bin(mut x: u32) -> *mut i8 {
    unsafe {
        let mut bits: u64 = (if x == 0u32 {
            1 as f64
        } else {
            log10(x as f64) / log10(2 as f64) + 1 as f64
        }) as u64;
        let mut ret: *mut i8 = malloc(
            bits.wrapping_add(1)
                .wrapping_mul(::core::mem::size_of::<i8>() as u64),
        ) as *mut i8;
        let mut i: u64 = 0;
        while i < bits {
            *ret.offset(bits.wrapping_sub(i).wrapping_sub(1) as isize) =
                (if x & 1 != 0 { '1' as i32 } else { '0' as i32 }) as i8;
            x >>= 1;
            i = i.wrapping_add(1);
            i;
        }
        *ret.offset(bits as isize) = '\0' as i8;
        return ret;
    }
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
