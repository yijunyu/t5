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
    fn free(_: *mut libc::c_void);
    fn strlen(_: *const i8) -> u64;
}
#[no_mangle]
pub extern "C" fn mpz_left_fac_ui(mut rop: i32, mut op: u64) {
    let mut i: u64 = 0;
    i = 1;
    while i <= op {
        i = i.wrapping_add(1);
        i;
    }
}

#[no_mangle]
pub extern "C" fn mpz_digitcount(mut op: i32) -> u64 {
    unsafe {
        let mut t: *mut i8 = 0 as *mut i8;
        let mut ret: u64 = strlen(t);
        free(t as *mut libc::c_void);
        return ret;
    }
}

fn main_0() -> i32 {
    let mut i: u64 = 0;
    i = 0;
    while i <= 110 {
        i <= 10 || i.wrapping_rem(10) == 0;
        i = i.wrapping_add(1);
        i;
    }
    i = 1000;
    while i <= 10000 {
        i = (i).wrapping_add(1000) as u64;
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
