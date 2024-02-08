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
extern "C" {}
#[no_mangle]
pub extern "C" fn lcs(sa: *const i8, sb: *const i8, beg: *mut *mut i8, end: *mut *mut i8) {
    unsafe {
        let mut apos: u64 = 0;
        let mut bpos: u64 = 0;
        let mut len: i32 = 0;
        *beg = 0 as *mut i8;
        *end = 0 as *mut i8;
        len = 0;
        apos = 0;
        while *sa.offset(apos as isize) as i32 != 0 {
            bpos = 0;
            while *sb.offset(bpos as isize) as i32 != 0 {
                if *sa.offset(apos as isize) as i32 == *sb.offset(bpos as isize) as i32 {
                    len = 1;
                    while *sa.offset(apos.wrapping_add(len as u64) as isize) as i32 != 0
                        && *sb.offset(bpos.wrapping_add(len as u64) as isize) as i32 != 0
                        && *sa.offset(apos.wrapping_add(len as u64) as isize) as i32
                            == *sb.offset(bpos.wrapping_add(len as u64) as isize) as i32
                    {
                        len += 1;
                        len;
                    }
                }
                if len as i64 > (*end).offset_from(*beg) as i64 {
                    *beg = sa.offset(apos as isize) as *mut i8;
                    *end = (*beg).offset(len as isize);
                    len = 0;
                }
                bpos = bpos.wrapping_add(1);
                bpos;
            }
            apos = apos.wrapping_add(1);
            apos;
        }
    }
}

fn main_0() -> i32 {
    unsafe {
        let mut s1: *mut i8 = b"thisisatest\0" as *const u8 as *const i8 as *mut i8;
        let mut s2: *mut i8 = b"testing123testing\0" as *const u8 as *const i8 as *mut i8;
        let mut beg: *mut i8 = 0 as *mut i8;
        let mut end: *mut i8 = 0 as *mut i8;
        let mut it: *mut i8 = 0 as *mut i8;
        lcs(s1, s2, &mut beg, &mut end);
        it = beg;
        while it != end {
            print!("{}", *it as i32);
            it = it.offset(1);
            it;
        }
        print!("\n");
        return 0;
    }
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
