#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
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

#[macro_use]
extern crate c2rust_bitfields;

use c2rust_out::*;
extern "C" {
    pub type re_dfa_t;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn regcomp(__preg: *mut regex_t, __pattern: *const i8, __cflags: i32) -> i32;
    fn regexec(
        __preg: *const regex_t,
        __String: *const i8,
        __nmatch: u64,
        __pmatch: *mut regmatch_t,
        __eflags: i32,
    ) -> i32;
    fn regfree(__preg: *mut regex_t);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strlen(_: *const i8) -> u64;
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct re_pattern_buffer {
    pub __buffer: *mut re_dfa_t,
    pub __allocated: u64,
    pub __used: u64,
    pub __syntax: u64,
    pub __fastmap: *mut i8,
    pub __translate: *mut u8,
    pub re_nsub: u64,
    #[bitfield(name = "__can_be_null", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "__regs_allocated", ty = "libc::c_uint", bits = "1..=2")]
    #[bitfield(name = "__fastmap_accurate", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "__no_sub", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "__not_bol", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "__not_eol", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "__newline_anchor", ty = "libc::c_uint", bits = "7..=7")]
    pub __can_be_null___regs_allocated___fastmap_accurate___no_sub___not_bol___not_eol___newline_anchor:
        [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type regex_t = re_pattern_buffer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regmatch_t {
    pub rm_so: i32,
    pub rm_eo: i32,
}
fn main_0() -> i32 {
    unsafe {
        let mut preg : regex_t = regex_t {
            __buffer : 0 as * mut re_dfa_t,
            __allocated : 0,
            __used : 0,
            __syntax : 0,
            __fastmap : 0 as * mut i8,
            __translate : 0 as * mut u8,
            re_nsub : 0,
            __can_be_null___regs_allocated___fastmap_accurate___no_sub___not_bol___not_eol___newline_anchor : [0; 1],
            c2rust_padding : [0; 7],
        };
        let mut substmatch: [regmatch_t; 1] = [regmatch_t { rm_so: 0, rm_eo: 0 }; 1];
        let mut tp: *const i8 = b"string$\0" as *const u8 as *const i8;
        let mut t1: *const i8 = b"this is a matching string\0" as *const u8 as *const i8;
        let mut t2: *const i8 = b"this is not a matching string!\0" as *const u8 as *const i8;
        let mut ss: *const i8 = b"istyfied\0" as *const u8 as *const i8;
        regcomp(&mut preg, b"string$\0" as *const u8 as *const i8, 1);
        if regexec(&mut preg, t1, 0, 0 as *mut regmatch_t, 0) == 0 {
            print!(
                "{} {}matched with {}\n",
                build_str_from_raw_ptr(t1 as *mut u8),
                "\0",
                build_str_from_raw_ptr(tp as *mut u8)
            )
        } else {
            print!(
                "{} {}matched with {}\n",
                build_str_from_raw_ptr(t1 as *mut u8),
                "did not \0",
                build_str_from_raw_ptr(tp as *mut u8)
            )
        };
        if regexec(&mut preg, t2, 0, 0 as *mut regmatch_t, 0) == 0 {
            print!(
                "{} {}matched with {}\n",
                build_str_from_raw_ptr(t2 as *mut u8),
                "\0",
                build_str_from_raw_ptr(tp as *mut u8)
            )
        } else {
            print!(
                "{} {}matched with {}\n",
                build_str_from_raw_ptr(t2 as *mut u8),
                "did not \0",
                build_str_from_raw_ptr(tp as *mut u8)
            )
        };
        regfree(&mut preg);
        regcomp(&mut preg, b"a[a-z]+\0" as *const u8 as *const i8, 1);
        if regexec(&mut preg, t1, 1, substmatch.as_mut_ptr(), 0) == 0 {
            let mut ns: *mut i8 = malloc(
                ((substmatch[0 as usize].rm_so + 1i32) as u64)
                    .wrapping_add(strlen(ss))
                    .wrapping_add((strlen(t1)).wrapping_sub(substmatch[0 as usize].rm_eo as u64))
                    .wrapping_add(2),
            ) as *mut i8;
            memcpy(
                ns as *mut libc::c_void,
                t1 as *const libc::c_void,
                (substmatch[0 as usize].rm_so + 1i32) as u64,
            );
            memcpy(
                &mut *ns.offset((*substmatch.as_mut_ptr().offset(0 as isize)).rm_so as isize)
                    as *mut i8 as *mut libc::c_void,
                ss as *const libc::c_void,
                strlen(ss),
            );
            memcpy(
                &mut *ns.offset(
                    ((*substmatch.as_mut_ptr().offset(0 as isize)).rm_so as u64)
                        .wrapping_add((strlen as unsafe extern "C" fn(*const i8) -> u64)(ss))
                        as isize,
                ) as *mut i8 as *mut libc::c_void,
                &*t1.offset((*substmatch.as_mut_ptr().offset(0 as isize)).rm_eo as isize)
                    as *const i8 as *const libc::c_void,
                strlen(&*t1.offset((*substmatch.as_mut_ptr().offset(0 as isize)).rm_eo as isize)),
            );
            *ns.offset(
                (substmatch[0 as usize].rm_so as u64)
                    .wrapping_add(strlen(ss))
                    .wrapping_add(strlen(
                        &*t1.offset((*substmatch.as_mut_ptr().offset(0 as isize)).rm_eo as isize),
                    )) as isize,
            ) = 0;
            print!("mod string: {}\n", build_str_from_raw_ptr(ns as *mut u8));
            free(ns as *mut libc::c_void);
        } else {
            print!(
                "the string {} is the same: no matching!\n",
                build_str_from_raw_ptr(t1 as *mut u8)
            );
        }
        regfree(&mut preg);
        return 0;
    }
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
