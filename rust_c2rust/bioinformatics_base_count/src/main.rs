#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn free(_: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct genome {
    pub strand: *mut libc::c_char,
    pub length: libc::c_int,
    pub next: *mut genome,
}
#[no_mangle]
pub static mut genomeData: *mut genome = 0 as *const genome as *mut genome;
#[no_mangle]
pub static mut totalLength: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut Adenine: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut Cytosine: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut Guanine: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut Thymine: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn numDigits(mut num: libc::c_int) -> libc::c_int {
    let mut len: libc::c_int = 1 as libc::c_int;
    while num > 10 as libc::c_int {
        num = num / 10 as libc::c_int;
        len += 1;
        len;
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn buildGenome(mut str: *mut libc::c_char) {
    let mut len: libc::c_int = strlen(str as *const libc::c_char) as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut genomeIterator: *mut genome = 0 as *mut genome;
    let mut newGenome: *mut genome = 0 as *mut genome;
    totalLength += len;
    i = 0 as libc::c_int;
    while i < len {
        match *str.offset(i as isize) as libc::c_int {
            65 => {
                Adenine += 1;
                Adenine;
            }
            84 => {
                Thymine += 1;
                Thymine;
            }
            67 => {
                Cytosine += 1;
                Cytosine;
            }
            71 => {
                Guanine += 1;
                Guanine;
            }
            _ => {}
        }
        i += 1;
        i;
    }
    if genomeData.is_null() {
        genomeData = malloc(::core::mem::size_of::<genome>() as libc::c_ulong)
            as *mut genome;
        (*genomeData)
            .strand = malloc(
            (len as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        ) as *mut libc::c_char;
        strcpy((*genomeData).strand, str as *const libc::c_char);
        (*genomeData).length = len;
        (*genomeData).next = 0 as *mut genome;
    } else {
        genomeIterator = genomeData;
        while !((*genomeIterator).next).is_null() {
            genomeIterator = (*genomeIterator).next;
        }
        newGenome = malloc(::core::mem::size_of::<genome>() as libc::c_ulong)
            as *mut genome;
        (*newGenome)
            .strand = malloc(
            (len as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        ) as *mut libc::c_char;
        strcpy((*newGenome).strand, str as *const libc::c_char);
        (*newGenome).length = len;
        (*newGenome).next = 0 as *mut genome;
        (*genomeIterator).next = newGenome;
    };
}
#[no_mangle]
pub unsafe extern "C" fn printGenome() {
    let mut genomeIterator: *mut genome = genomeData;
    let mut width: libc::c_int = numDigits(totalLength);
    let mut len: libc::c_int = 0 as libc::c_int;
    printf(b"Sequence:\n\0" as *const u8 as *const libc::c_char);
    while !genomeIterator.is_null() {
        printf(
            b"\n%*d%3s%3s\0" as *const u8 as *const libc::c_char,
            width + 1 as libc::c_int,
            len,
            b":\0" as *const u8 as *const libc::c_char,
            (*genomeIterator).strand,
        );
        len += (*genomeIterator).length;
        genomeIterator = (*genomeIterator).next;
    }
    printf(b"\n\nBase Count\n----------\n\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"%3c%3s%*d\n\0" as *const u8 as *const libc::c_char,
        'A' as i32,
        b":\0" as *const u8 as *const libc::c_char,
        width + 1 as libc::c_int,
        Adenine,
    );
    printf(
        b"%3c%3s%*d\n\0" as *const u8 as *const libc::c_char,
        'T' as i32,
        b":\0" as *const u8 as *const libc::c_char,
        width + 1 as libc::c_int,
        Thymine,
    );
    printf(
        b"%3c%3s%*d\n\0" as *const u8 as *const libc::c_char,
        'C' as i32,
        b":\0" as *const u8 as *const libc::c_char,
        width + 1 as libc::c_int,
        Cytosine,
    );
    printf(
        b"%3c%3s%*d\n\0" as *const u8 as *const libc::c_char,
        'G' as i32,
        b":\0" as *const u8 as *const libc::c_char,
        width + 1 as libc::c_int,
        Guanine,
    );
    printf(
        b"\n%3s%*d\n\0" as *const u8 as *const libc::c_char,
        b"Total:\0" as *const u8 as *const libc::c_char,
        width + 1 as libc::c_int,
        Adenine + Thymine + Cytosine + Guanine,
    );
    free(genomeData as *mut libc::c_void);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut str: [libc::c_char; 100] = [0; 100];
    let mut counter: libc::c_int = 0 as libc::c_int;
    let mut len: libc::c_int = 0;
    if argc != 2 as libc::c_int {
        printf(
            b"Usage : %s <Gene file name>\n\0" as *const u8 as *const libc::c_char,
            *argv.offset(0 as libc::c_int as isize),
        );
        return 0 as libc::c_int;
    }
    let mut fp: *mut FILE = fopen(
        *argv.offset(1 as libc::c_int as isize),
        b"r\0" as *const u8 as *const libc::c_char,
    );
    while fscanf(fp, b"%s\0" as *const u8 as *const libc::c_char, str.as_mut_ptr())
        != -(1 as libc::c_int)
    {
        buildGenome(str.as_mut_ptr());
    }
    fclose(fp);
    printGenome();
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
