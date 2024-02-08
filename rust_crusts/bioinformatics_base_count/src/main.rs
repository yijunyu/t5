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
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn free(_: *mut libc::c_void);
    fn malloc(_: u64) -> *mut libc::c_void;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fopen(_: *const i8, _: *const i8) -> *mut FILE;
    fn fscanf(_: *mut FILE, _: *const i8, _: ...) -> i32;
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
pub struct genome {
    pub strand: *mut i8,
    pub length: i32,
    pub next: *mut genome,
}
#[no_mangle]
pub static mut genomeData: *mut genome = 0 as *const genome as *mut genome;
#[no_mangle]
pub static mut totalLength: i32 = 0;
#[no_mangle]
pub static mut Adenine: i32 = 0;
#[no_mangle]
pub static mut Cytosine: i32 = 0;
#[no_mangle]
pub static mut Guanine: i32 = 0;
#[no_mangle]
pub static mut Thymine: i32 = 0;
#[no_mangle]
pub extern "C" fn numDigits(mut num: i32) -> i32 {
    let mut len: i32 = 1;
    while num > 10 {
        num = num / 10;
        len += 1;
        len;
    }
    return len;
}

#[no_mangle]
pub extern "C" fn buildGenome(mut str: *mut i8) {
    unsafe {
        let mut len: i32 = strlen(str as *const i8) as i32;
        let mut i: i32 = 0;
        let mut genomeIterator: *mut genome = 0 as *mut genome;
        let mut newGenome: *mut genome = 0 as *mut genome;
        totalLength += len;
        i = 0;
        while i < len {
            match *str.offset(i as isize) as i32 {
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
            genomeData = malloc(::core::mem::size_of::<genome>() as u64) as *mut genome;
            (*genomeData).strand =
                malloc((len as u64).wrapping_mul(::core::mem::size_of::<i8>() as u64)) as *mut i8;
            strcpy((*genomeData).strand, str as *const i8);
            (*genomeData).length = len;
            (*genomeData).next = 0 as *mut genome;
        } else {
            genomeIterator = genomeData;
            while !((*genomeIterator).next).is_null() {
                genomeIterator = (*genomeIterator).next;
            }
            newGenome = malloc(::core::mem::size_of::<genome>() as u64) as *mut genome;
            (*newGenome).strand =
                malloc((len as u64).wrapping_mul(::core::mem::size_of::<i8>() as u64)) as *mut i8;
            strcpy((*newGenome).strand, str as *const i8);
            (*newGenome).length = len;
            (*newGenome).next = 0 as *mut genome;
            (*genomeIterator).next = newGenome;
        };
    }
}

#[no_mangle]
pub extern "C" fn printGenome() {
    unsafe {
        let mut genomeIterator: *mut genome = genomeData;
        let mut width: i32 = numDigits(totalLength);
        let mut len: i32 = 0;
        print!("Sequence:\n");
        while !genomeIterator.is_null() {
            print!(
                "\n{1:0$}{2:3}{3:3}",
                (width + 1).abs() as usize,
                len,
                ":\0",
                build_str_from_raw_ptr((*genomeIterator).strand as *mut u8)
            );
            len += (*genomeIterator).length;
            genomeIterator = (*genomeIterator).next;
        }
        print!("\n\nBase Count\n----------\n\n");
        print!(
            "{0:3}{1:3}{3:2$}\n",
            'A' as i32,
            ":\0",
            (width + 1).abs() as usize,
            Adenine
        );
        print!(
            "{0:3}{1:3}{3:2$}\n",
            'T' as i32,
            ":\0",
            (width + 1).abs() as usize,
            Thymine
        );
        print!(
            "{0:3}{1:3}{3:2$}\n",
            'C' as i32,
            ":\0",
            (width + 1).abs() as usize,
            Cytosine
        );
        print!(
            "{0:3}{1:3}{3:2$}\n",
            'G' as i32,
            ":\0",
            (width + 1).abs() as usize,
            Guanine
        );
        print!(
            "\n{0:3}{2:1$}\n",
            "Total:\0",
            (width + 1).abs() as usize,
            Adenine + Thymine + Cytosine + Guanine
        );
        free(genomeData as *mut libc::c_void);
    }
}

fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    unsafe {
        let mut str: [i8; 100] = [0; 100];
        let mut counter: i32 = 0;
        let mut len: i32 = 0;
        if argc != 2 {
            print!(
                "Usage : {} <Gene file name>\n",
                build_str_from_raw_ptr(*argv.offset(0 as isize) as *mut u8)
            );
            return 0;
        }
        let mut fp: *mut FILE = fopen(*argv.offset(1 as isize), b"r\0" as *const u8 as *const i8);
        while fscanf(fp, b"%s\0" as *const u8 as *const i8, str.as_mut_ptr()) != -1 {
            buildGenome(str.as_mut_ptr());
        }
        fclose(fp);
        printGenome();
        return 0;
    }
}

pub fn main() {
    let mut args: Vec<*mut i8> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    ::std::process::exit(main_0((args.len() - 1) as i32, args.as_mut_ptr() as *mut *mut i8) as i32);
}
