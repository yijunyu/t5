#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
#[macro_use]
extern crate c2rust_bitfields;
use ::c2rust_out::*;
extern "C" {
    pub type re_dfa_t;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn regcomp(
        __preg: *mut regex_t,
        __pattern: *const libc::c_char,
        __cflags: libc::c_int,
    ) -> libc::c_int;
    fn regexec(
        __preg: *const regex_t,
        __String: *const libc::c_char,
        __nmatch: size_t,
        __pmatch: *mut regmatch_t,
        __eflags: libc::c_int,
    ) -> libc::c_int;
    fn regfree(__preg: *mut regex_t);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
pub type __re_long_size_t = libc::c_ulong;
pub type reg_syntax_t = libc::c_ulong;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct re_pattern_buffer {
    pub __buffer: *mut re_dfa_t,
    pub __allocated: __re_long_size_t,
    pub __used: __re_long_size_t,
    pub __syntax: reg_syntax_t,
    pub __fastmap: *mut libc::c_char,
    pub __translate: *mut libc::c_uchar,
    pub re_nsub: size_t,
    #[bitfield(name = "__can_be_null", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "__regs_allocated", ty = "libc::c_uint", bits = "1..=2")]
    #[bitfield(name = "__fastmap_accurate", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "__no_sub", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "__not_bol", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "__not_eol", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "__newline_anchor", ty = "libc::c_uint", bits = "7..=7")]
    pub __can_be_null___regs_allocated___fastmap_accurate___no_sub___not_bol___not_eol___newline_anchor: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type regex_t = re_pattern_buffer;
pub type regoff_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regmatch_t {
    pub rm_so: regoff_t,
    pub rm_eo: regoff_t,
}
unsafe fn main_0() -> libc::c_int {
    let mut preg: regex_t = regex_t {
        __buffer: 0 as *mut re_dfa_t,
        __allocated: 0,
        __used: 0,
        __syntax: 0,
        __fastmap: 0 as *mut libc::c_char,
        __translate: 0 as *mut libc::c_uchar,
        re_nsub: 0,
        __can_be_null___regs_allocated___fastmap_accurate___no_sub___not_bol___not_eol___newline_anchor: [0; 1],
        c2rust_padding: [0; 7],
    };
    let mut substmatch: [regmatch_t; 1] = [regmatch_t { rm_so: 0, rm_eo: 0 }; 1];
    let mut tp: *const libc::c_char = b"string$\0" as *const u8 as *const libc::c_char;
    let mut t1: *const libc::c_char = b"this is a matching string\0" as *const u8
        as *const libc::c_char;
    let mut t2: *const libc::c_char = b"this is not a matching string!\0" as *const u8
        as *const libc::c_char;
    let mut ss: *const libc::c_char = b"istyfied\0" as *const u8 as *const libc::c_char;
    regcomp(
        &mut preg,
        b"string$\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    printf(
        b"'%s' %smatched with '%s'\n\0" as *const u8 as *const libc::c_char,
        t1,
        if regexec(
            &mut preg,
            t1,
            0 as libc::c_int as size_t,
            0 as *mut regmatch_t,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {
            b"\0" as *const u8 as *const libc::c_char
        } else {
            b"did not \0" as *const u8 as *const libc::c_char
        },
        tp,
    );
    printf(
        b"'%s' %smatched with '%s'\n\0" as *const u8 as *const libc::c_char,
        t2,
        if regexec(
            &mut preg,
            t2,
            0 as libc::c_int as size_t,
            0 as *mut regmatch_t,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {
            b"\0" as *const u8 as *const libc::c_char
        } else {
            b"did not \0" as *const u8 as *const libc::c_char
        },
        tp,
    );
    regfree(&mut preg);
    regcomp(
        &mut preg,
        b"a[a-z]+\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    if regexec(
        &mut preg,
        t1,
        1 as libc::c_int as size_t,
        substmatch.as_mut_ptr(),
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {
        let mut ns: *mut libc::c_char = malloc(
            ((substmatch[0 as libc::c_int as usize].rm_so + 1 as libc::c_int)
                as libc::c_ulong)
                .wrapping_add(strlen(ss))
                .wrapping_add(
                    (strlen(t1))
                        .wrapping_sub(
                            substmatch[0 as libc::c_int as usize].rm_eo as libc::c_ulong,
                        ),
                )
                .wrapping_add(2 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        memcpy(
            ns as *mut libc::c_void,
            t1 as *const libc::c_void,
            (substmatch[0 as libc::c_int as usize].rm_so + 1 as libc::c_int)
                as libc::c_ulong,
        );
        memcpy(
            &mut *ns
                .offset(
                    (*substmatch.as_mut_ptr().offset(0 as libc::c_int as isize)).rm_so
                        as isize,
                ) as *mut libc::c_char as *mut libc::c_void,
            ss as *const libc::c_void,
            strlen(ss),
        );
        memcpy(
            &mut *ns
                .offset(
                    ((*substmatch.as_mut_ptr().offset(0 as libc::c_int as isize)).rm_so
                        as libc::c_ulong)
                        .wrapping_add(
                            (strlen
                                as unsafe extern "C" fn(
                                    *const libc::c_char,
                                ) -> libc::c_ulong)(ss),
                        ) as isize,
                ) as *mut libc::c_char as *mut libc::c_void,
            &*t1
                .offset(
                    (*substmatch.as_mut_ptr().offset(0 as libc::c_int as isize)).rm_eo
                        as isize,
                ) as *const libc::c_char as *const libc::c_void,
            strlen(
                &*t1
                    .offset(
                        (*substmatch.as_mut_ptr().offset(0 as libc::c_int as isize))
                            .rm_eo as isize,
                    ),
            ),
        );
        *ns
            .offset(
                (substmatch[0 as libc::c_int as usize].rm_so as libc::c_ulong)
                    .wrapping_add(strlen(ss))
                    .wrapping_add(
                        strlen(
                            &*t1
                                .offset(
                                    (*substmatch.as_mut_ptr().offset(0 as libc::c_int as isize))
                                        .rm_eo as isize,
                                ),
                        ),
                    ) as isize,
            ) = 0 as libc::c_int as libc::c_char;
        printf(b"mod string: '%s'\n\0" as *const u8 as *const libc::c_char, ns);
        free(ns as *mut libc::c_void);
    } else {
        printf(
            b"the string '%s' is the same: no matching!\n\0" as *const u8
                as *const libc::c_char,
            t1,
        );
    }
    regfree(&mut preg);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
