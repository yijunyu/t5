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

use c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fopen(_: *const i8, _: *const i8) -> *mut FILE;
    fn fgetc(__stream: *mut FILE) -> i32;
    fn fputc(__c: i32, __stream: *mut FILE) -> i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut i8,
    pub _IO_read_end: *mut i8,
    pub _IO_read_base: *mut i8,
    pub _IO_write_base: *mut i8,
    pub _IO_write_ptr: *mut i8,
    pub _IO_write_end: *mut i8,
    pub _IO_buf_base: *mut i8,
    pub _IO_buf_end: *mut i8,
    pub _IO_save_base: *mut i8,
    pub _IO_backup_base: *mut i8,
    pub _IO_save_end: *mut i8,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: i64,
    pub _cur_column: u16,
    pub _vtable_offset: i8,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: i64,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: u64,
    pub _mode: i32,
    pub _unused2: [i8; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bit_io_t {
    pub fp: *mut FILE,
    pub accu: u32,
    pub bits: i32,
}
pub type bit_filter = *mut bit_io_t;
#[no_mangle]
pub extern "C" fn b_attach(mut f: *mut FILE) -> bit_filter {
    unsafe {
        let mut b: bit_filter = malloc(::core::mem::size_of::<bit_io_t>() as u64) as bit_filter;
        (*b).accu = 0;
        (*b).bits = (*b).accu as i32;
        (*b).fp = f;
        return b;
    }
}

#[no_mangle]
pub extern "C" fn b_write(mut buf: *mut u8, mut n_bits: u64, mut shift: u64, mut bf: bit_filter) {
    unsafe {
        let mut accu: u32 = (*bf).accu;
        let mut bits: i32 = (*bf).bits;
        buf = buf.offset(shift.wrapping_div(8) as isize);
        shift = (shift as u64).wrapping_rem(8) as u64;
        while n_bits != 0 || bits >= 8 {
            while bits >= 8 {
                bits -= 8;
                fputc((accu >> bits) as i32, (*bf).fp);
                accu &= ((1i32 << bits) - 1) as u32;
            }
            while bits < 8 && n_bits != 0 {
                accu =
                    accu << 1 | ((128 >> shift & *buf as i32) >> 7u64.wrapping_sub(shift)) as u32;
                n_bits = n_bits.wrapping_sub(1);
                n_bits;
                bits += 1;
                bits;
                shift = shift.wrapping_add(1);
                if shift == 8 {
                    shift = 0;
                    buf = buf.offset(1);
                    buf;
                }
            }
        }
        (*bf).accu = accu;
        (*bf).bits = bits;
    }
}

#[no_mangle]
pub extern "C" fn b_read(
    mut buf: *mut u8,
    mut n_bits: u64,
    mut shift: u64,
    mut bf: bit_filter,
) -> u64 {
    unsafe {
        let mut accu: u32 = (*bf).accu;
        let mut bits: i32 = (*bf).bits;
        let mut mask: i32 = 0;
        let mut i: i32 = 0;
        buf = buf.offset(shift.wrapping_div(8) as isize);
        shift = (shift as u64).wrapping_rem(8) as u64;
        while n_bits != 0 {
            while bits != 0 && n_bits != 0 {
                mask = 128 >> shift;
                if accu & (1i32 << bits - 1) as u32 != 0 {
                    *buf = (*buf as i32 | mask) as u8;
                } else {
                    *buf = (*buf as i32 & !mask) as u8;
                }
                n_bits = n_bits.wrapping_sub(1);
                n_bits;
                bits -= 1;
                bits;
                shift = shift.wrapping_add(1);
                if shift >= 8 {
                    shift = 0;
                    buf = buf.offset(1);
                    buf;
                }
            }
            if n_bits == 0 {
                break;
            }
            accu = accu << 8 | fgetc((*bf).fp) as u32;
            bits += 8;
        }
        (*bf).accu = accu;
        (*bf).bits = bits;
        return i as u64;
    }
}

#[no_mangle]
pub extern "C" fn b_detach(mut bf: bit_filter) {
    unsafe {
        if (*bf).bits != 0 {
            (*bf).accu <<= 8 - (*bf).bits;
            fputc((*bf).accu as i32, (*bf).fp);
        }
        free(bf as *mut libc::c_void);
    }
}

fn main_0() -> i32 {
    unsafe {
        let mut s: [u8; 12] = *::core::mem::transmute::<&[u8; 12], &mut [u8; 12]>(b"abcdefghijk\0");
        let mut s2: [u8; 11] = [0; 11];
        let mut i: i32 = 0;
        let mut f: *mut FILE = fopen(
            b"test.bin\0" as *const u8 as *const i8,
            b"wb\0" as *const u8 as *const i8,
        );
        let mut b: bit_filter = b_attach(f);
        i = 0;
        while i < 10 {
            b_write(s.as_mut_ptr().offset(i as isize), 7, 1, b);
            i += 1;
            i;
        }
        b_detach(b);
        fclose(f);
        f = fopen(
            b"test.bin\0" as *const u8 as *const i8,
            b"rb\0" as *const u8 as *const i8,
        );
        b = b_attach(f);
        i = 0;
        while i < 10 {
            b_read(s2.as_mut_ptr().offset(i as isize), 7, 1, b);
            i += 1;
            i;
        }
        b_detach(b);
        fclose(f);
        print!(
            "{:10}\n",
            build_str_from_raw_ptr(s2.as_mut_ptr() as *mut u8)
        );
        return 0;
    }
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
