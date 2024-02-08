#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type byte = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bit_io_t {
    pub fp: *mut FILE,
    pub accu: uint32_t,
    pub bits: libc::c_int,
}
pub type bit_filter = *mut bit_io_t;
#[no_mangle]
pub unsafe extern "C" fn b_attach(mut f: *mut FILE) -> bit_filter {
    let mut b: bit_filter = malloc(::core::mem::size_of::<bit_io_t>() as libc::c_ulong)
        as bit_filter;
    (*b).accu = 0 as libc::c_int as uint32_t;
    (*b).bits = (*b).accu as libc::c_int;
    (*b).fp = f;
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn b_write(
    mut buf: *mut byte,
    mut n_bits: size_t,
    mut shift: size_t,
    mut bf: bit_filter,
) {
    let mut accu: uint32_t = (*bf).accu;
    let mut bits: libc::c_int = (*bf).bits;
    buf = buf.offset(shift.wrapping_div(8 as libc::c_int as libc::c_ulong) as isize);
    shift = (shift as libc::c_ulong).wrapping_rem(8 as libc::c_int as libc::c_ulong)
        as size_t as size_t;
    while n_bits != 0 || bits >= 8 as libc::c_int {
        while bits >= 8 as libc::c_int {
            bits -= 8 as libc::c_int;
            fputc((accu >> bits) as libc::c_int, (*bf).fp);
            accu &= (((1 as libc::c_int) << bits) - 1 as libc::c_int) as libc::c_uint;
        }
        while bits < 8 as libc::c_int && n_bits != 0 {
            accu = accu << 1 as libc::c_int
                | ((128 as libc::c_int >> shift & *buf as libc::c_int)
                    >> (7 as libc::c_int as libc::c_ulong).wrapping_sub(shift))
                    as libc::c_uint;
            n_bits = n_bits.wrapping_sub(1);
            n_bits;
            bits += 1;
            bits;
            shift = shift.wrapping_add(1);
            if shift == 8 as libc::c_int as libc::c_ulong {
                shift = 0 as libc::c_int as size_t;
                buf = buf.offset(1);
                buf;
            }
        }
    }
    (*bf).accu = accu;
    (*bf).bits = bits;
}
#[no_mangle]
pub unsafe extern "C" fn b_read(
    mut buf: *mut byte,
    mut n_bits: size_t,
    mut shift: size_t,
    mut bf: bit_filter,
) -> size_t {
    let mut accu: uint32_t = (*bf).accu;
    let mut bits: libc::c_int = (*bf).bits;
    let mut mask: libc::c_int = 0;
    let mut i: libc::c_int = 0 as libc::c_int;
    buf = buf.offset(shift.wrapping_div(8 as libc::c_int as libc::c_ulong) as isize);
    shift = (shift as libc::c_ulong).wrapping_rem(8 as libc::c_int as libc::c_ulong)
        as size_t as size_t;
    while n_bits != 0 {
        while bits != 0 && n_bits != 0 {
            mask = 128 as libc::c_int >> shift;
            if accu & ((1 as libc::c_int) << bits - 1 as libc::c_int) as libc::c_uint
                != 0
            {
                *buf = (*buf as libc::c_int | mask) as byte;
            } else {
                *buf = (*buf as libc::c_int & !mask) as byte;
            }
            n_bits = n_bits.wrapping_sub(1);
            n_bits;
            bits -= 1;
            bits;
            shift = shift.wrapping_add(1);
            if shift >= 8 as libc::c_int as libc::c_ulong {
                shift = 0 as libc::c_int as size_t;
                buf = buf.offset(1);
                buf;
            }
        }
        if n_bits == 0 {
            break;
        }
        accu = accu << 8 as libc::c_int | fgetc((*bf).fp) as libc::c_uint;
        bits += 8 as libc::c_int;
    }
    (*bf).accu = accu;
    (*bf).bits = bits;
    return i as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn b_detach(mut bf: bit_filter) {
    if (*bf).bits != 0 {
        (*bf).accu <<= 8 as libc::c_int - (*bf).bits;
        fputc((*bf).accu as libc::c_int, (*bf).fp);
    }
    free(bf as *mut libc::c_void);
}
unsafe fn main_0() -> libc::c_int {
    let mut s: [libc::c_uchar; 12] = *::core::mem::transmute::<
        &[u8; 12],
        &mut [libc::c_uchar; 12],
    >(b"abcdefghijk\0");
    let mut s2: [libc::c_uchar; 11] = [
        0 as libc::c_int as libc::c_uchar,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut i: libc::c_int = 0;
    let mut f: *mut FILE = fopen(
        b"test.bin\0" as *const u8 as *const libc::c_char,
        b"wb\0" as *const u8 as *const libc::c_char,
    );
    let mut b: bit_filter = b_attach(f);
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        b_write(
            s.as_mut_ptr().offset(i as isize),
            7 as libc::c_int as size_t,
            1 as libc::c_int as size_t,
            b,
        );
        i += 1;
        i;
    }
    b_detach(b);
    fclose(f);
    f = fopen(
        b"test.bin\0" as *const u8 as *const libc::c_char,
        b"rb\0" as *const u8 as *const libc::c_char,
    );
    b = b_attach(f);
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        b_read(
            s2.as_mut_ptr().offset(i as isize),
            7 as libc::c_int as size_t,
            1 as libc::c_int as size_t,
            b,
        );
        i += 1;
        i;
    }
    b_detach(b);
    fclose(f);
    printf(b"%10s\n\0" as *const u8 as *const libc::c_char, s2.as_mut_ptr());
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
