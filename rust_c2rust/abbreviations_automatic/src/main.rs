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
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
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
#[no_mangle]
pub unsafe extern "C" fn process(
    mut lineNum: libc::c_int,
    mut buffer: *mut libc::c_char,
) {
    let mut days: [[libc::c_char; 64]; 7] = [[0; 64]; 7];
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut d: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0 as libc::c_int;
    while *buffer.offset(i as isize) as libc::c_int != 0 as libc::c_int {
        if *buffer.offset(i as isize) as libc::c_int == ' ' as i32 {
            days[d as usize][j as usize] = '\0' as i32 as libc::c_char;
            d += 1;
            d;
            j = 0 as libc::c_int;
        } else if *buffer.offset(i as isize) as libc::c_int == '\n' as i32
            || *buffer.offset(i as isize) as libc::c_int == '\r' as i32
        {
            days[d as usize][j as usize] = '\0' as i32 as libc::c_char;
            d += 1;
            d;
            break;
        } else {
            days[d as usize][j as usize] = *buffer.offset(i as isize);
            j += 1;
            j;
        }
        if d >= 7 as libc::c_int {
            printf(
                b"There aren't 7 days in line %d\n\0" as *const u8
                    as *const libc::c_char,
                lineNum,
            );
            return;
        }
        i += 1;
        i;
    }
    if *buffer.offset(i as isize) as libc::c_int == '\0' as i32 {
        days[d as usize][j as usize] = '\0' as i32 as libc::c_char;
        d += 1;
        d;
    }
    if d < 7 as libc::c_int {
        printf(
            b"There aren't 7 days in line %d\n\0" as *const u8 as *const libc::c_char,
            lineNum,
        );
        return;
    } else {
        let mut len: libc::c_int = 0 as libc::c_int;
        len = 1 as libc::c_int;
        while len < 64 as libc::c_int {
            let mut current_block_35: u64;
            let mut d1: libc::c_int = 0;
            d1 = 0 as libc::c_int;
            's_113: loop {
                if !(d1 < 7 as libc::c_int) {
                    current_block_35 = 18153031941552419006;
                    break;
                }
                let mut d2: libc::c_int = 0;
                d2 = d1 + 1 as libc::c_int;
                while d2 < 7 as libc::c_int {
                    let mut unique: libc::c_int = 0 as libc::c_int;
                    i = 0 as libc::c_int;
                    while i < len {
                        if days[d1 as usize][i as usize] as libc::c_int
                            != days[d2 as usize][i as usize] as libc::c_int
                        {
                            unique = 1 as libc::c_int;
                            break;
                        } else {
                            i += 1;
                            i;
                        }
                    }
                    if unique == 0 {
                        current_block_35 = 10891380440665537214;
                        break 's_113;
                    }
                    d2 += 1;
                    d2;
                }
                d1 += 1;
                d1;
            }
            match current_block_35 {
                10891380440665537214 => {}
                _ => {
                    printf(b"%2d \0" as *const u8 as *const libc::c_char, len);
                    i = 0 as libc::c_int;
                    while i < 7 as libc::c_int {
                        printf(
                            b" %s\0" as *const u8 as *const libc::c_char,
                            (days[i as usize]).as_mut_ptr(),
                        );
                        i += 1;
                        i;
                    }
                    printf(b"\n\0" as *const u8 as *const libc::c_char);
                    return;
                }
            }
            len += 1;
            len;
        }
    }
    printf(
        b"Failed to find uniqueness within the bounds.\0" as *const u8
            as *const libc::c_char,
    );
}
unsafe fn main_0() -> libc::c_int {
    let mut buffer: [libc::c_char; 1024] = [0; 1024];
    let mut lineNum: libc::c_int = 1 as libc::c_int;
    let mut len: libc::c_int = 0;
    let mut fp: *mut FILE = 0 as *mut FILE;
    fp = fopen(
        b"days_of_week.txt\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    loop {
        memset(
            buffer.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        );
        fgets(
            buffer.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                as libc::c_int,
            fp,
        );
        len = strlen(buffer.as_mut_ptr()) as libc::c_int;
        if len == 0 as libc::c_int
            || buffer[(len - 1 as libc::c_int) as usize] as libc::c_int == '\0' as i32
        {
            break;
        }
        let fresh0 = lineNum;
        lineNum = lineNum + 1;
        process(fresh0, buffer.as_mut_ptr());
    }
    fclose(fp);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
