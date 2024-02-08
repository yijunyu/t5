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
    fn exit(_: i32) -> !;
    static mut stderr: *mut FILE;
    fn fputs(__s: *const i8, __stream: *mut FILE) -> i32;
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
#[no_mangle]
pub extern "C" fn roman(mut s: *mut i8, mut n: u32) {
    unsafe {
        if n == 0 {
            fputs(
                b"Roman numeral for zero requested.\0" as *const u8 as *const i8,
                stderr,
            );
            exit(1);
        }
        while n >= 1000 {
            let fresh0 = s;
            s = s.offset(1);
            *fresh0 = 'M' as i8;
            n = n.wrapping_sub(1000);
        }
        if n >= 900 {
            let fresh1 = s;
            s = s.offset(1);
            *fresh1 = 'C' as i8;
            let fresh2 = s;
            s = s.offset(1);
            *fresh2 = 'M' as i8;
            n = n.wrapping_sub(900);
        }
        if n >= 500 {
            let fresh3 = s;
            s = s.offset(1);
            *fresh3 = 'D' as i8;
            n = n.wrapping_sub(500);
        }
        if n >= 400 {
            let fresh4 = s;
            s = s.offset(1);
            *fresh4 = 'C' as i8;
            let fresh5 = s;
            s = s.offset(1);
            *fresh5 = 'D' as i8;
            n = n.wrapping_sub(400);
        }
        while n >= 100 {
            let fresh6 = s;
            s = s.offset(1);
            *fresh6 = 'C' as i8;
            n = n.wrapping_sub(100);
        }
        if n >= 90 {
            let fresh7 = s;
            s = s.offset(1);
            *fresh7 = 'X' as i8;
            let fresh8 = s;
            s = s.offset(1);
            *fresh8 = 'C' as i8;
            n = n.wrapping_sub(90);
        }
        if n >= 50 {
            let fresh9 = s;
            s = s.offset(1);
            *fresh9 = 'L' as i8;
            n = n.wrapping_sub(50);
        }
        if n >= 40 {
            let fresh10 = s;
            s = s.offset(1);
            *fresh10 = 'X' as i8;
            let fresh11 = s;
            s = s.offset(1);
            *fresh11 = 'L' as i8;
            n = n.wrapping_sub(40);
        }
        while n >= 10 {
            let fresh12 = s;
            s = s.offset(1);
            *fresh12 = 'X' as i8;
            n = n.wrapping_sub(10);
        }
        if n >= 9 {
            let fresh13 = s;
            s = s.offset(1);
            *fresh13 = 'I' as i8;
            let fresh14 = s;
            s = s.offset(1);
            *fresh14 = 'X' as i8;
            n = n.wrapping_sub(9);
        }
        if n >= 5 {
            let fresh15 = s;
            s = s.offset(1);
            *fresh15 = 'V' as i8;
            n = n.wrapping_sub(5);
        }
        if n >= 4 {
            let fresh16 = s;
            s = s.offset(1);
            *fresh16 = 'I' as i8;
            let fresh17 = s;
            s = s.offset(1);
            *fresh17 = 'V' as i8;
            n = n.wrapping_sub(4);
        }
        while n >= 1 {
            let fresh18 = s;
            s = s.offset(1);
            *fresh18 = 'I' as i8;
            n = n.wrapping_sub(1);
        }
        *s = 0;
    }
}

fn main_0() -> i32 {
    let mut buffer: [i8; 16] = [0; 16];
    let mut i: u32 = 0;
    i = 1;
    unsafe {
        while i < 4000 {
            roman(buffer.as_mut_ptr(), i);
            print!(
                "{:4}: {}\n",
                i,
                build_str_from_raw_ptr(buffer.as_mut_ptr() as *mut u8)
            );
            i = i.wrapping_add(1);
            i;
        }
    }
    return 0;
}

pub fn main() {
    unsafe {
        ::std::process::exit(main_0() as i32);
    }
}
