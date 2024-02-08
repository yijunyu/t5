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
    static mut stdin: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fopen(_: *const i8, _: *const i8) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn fgets(__s: *mut i8, __n: i32, __stream: *mut FILE) -> *mut i8;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: i32) -> !;
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strncpy(_: *mut i8, _: *const i8, _: u64) -> *mut i8;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strstr(_: *const i8, _: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
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
pub struct dstr {
    pub data: *mut i8,
    pub alloc: u64,
    pub length: u64,
}
#[no_mangle]
pub extern "C" fn dstr_space(mut s: *mut dstr, mut grow_amount: u64) -> i32 {
    unsafe {
        return (((*s).length).wrapping_add(grow_amount) < (*s).alloc) as i32;
    }
}

#[no_mangle]
pub extern "C" fn dstr_grow(mut s: *mut dstr) -> i32 {
    unsafe {
        (*s).alloc = ((*s).alloc as u64).wrapping_mul(2) as u64;
        let mut attempt: *mut i8 = realloc((*s).data as *mut libc::c_void, (*s).alloc) as *mut i8;
        if attempt.is_null() {
            return 0;
        } else {
            (*s).data = attempt;
        }
        return 1;
    }
}

#[no_mangle]
pub extern "C" fn dstr_init(to_allocate: u64) -> *mut dstr {
    unsafe {
        let mut s: *mut dstr = malloc(::core::mem::size_of::<dstr>() as u64) as *mut dstr;
        if !s.is_null() {
            (*s).length = 0;
            (*s).alloc = to_allocate;
            (*s).data = malloc((*s).alloc) as *mut i8;
            if !((*s).data).is_null() {
                return s;
            }
        }
        if !((*s).data).is_null() {
            free((*s).data as *mut libc::c_void);
        }
        if !s.is_null() {
            free(s as *mut libc::c_void);
        }
        return 0 as *mut dstr;
    }
}

#[no_mangle]
pub extern "C" fn dstr_delete(mut s: *mut dstr) {
    unsafe {
        if !((*s).data).is_null() {
            free((*s).data as *mut libc::c_void);
        }
        if !s.is_null() {
            free(s as *mut libc::c_void);
        }
    }
}

#[no_mangle]
pub extern "C" fn readinput(mut fd: *mut FILE) -> *mut dstr {
    unsafe {
        let mut current_block: u64;
        static mut buffer_size: u64 = 4096;
        let vla = buffer_size as usize;
        let mut buffer: Vec<i8> = ::std::vec::from_elem(0, vla);
        let mut s: *mut dstr = dstr_init(buffer_size);
        if s.is_null() {
            current_block = 17156671859070965445;
        } else {
            current_block = 8258075665625361029;
        }
        '_failure: loop {
            match current_block {
                17156671859070965445 => {
                    dstr_delete(s);
                    return 0 as *mut dstr;
                }
                _ => {
                    if !(fgets(buffer.as_mut_ptr(), buffer_size as i32, fd)).is_null() {
                        while dstr_space(s, buffer_size) == 0 {
                            if dstr_grow(s) == 0 {
                                current_block = 17156671859070965445;
                                continue '_failure;
                            }
                        }
                        strncpy(
                            ((*s).data).offset((*s).length as isize),
                            buffer.as_mut_ptr(),
                            buffer_size,
                        );
                        (*s).length =
                            ((*s).length as u64).wrapping_add(strlen(buffer.as_mut_ptr())) as u64;
                        current_block = 8258075665625361029;
                    } else {
                        return s;
                    }
                }
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn dstr_replace_all(
    mut story: *mut dstr,
    mut replace: *const i8,
    mut insert: *const i8,
) {
    unsafe {
        let replace_l: u64 = strlen(replace);
        let insert_l: u64 = strlen(insert);
        let mut start: *mut i8 = (*story).data;
        loop {
            start = strstr(start, replace);
            if start.is_null() {
                break;
            }
            if dstr_space(story, insert_l.wrapping_sub(replace_l)) == 0 {
                if dstr_grow(story) == 0 {
                    fprintf(
                        stderr,
                        b"Failed to allocate memory\0" as *const u8 as *const i8,
                    );
                    exit(1);
                }
            }
            if insert_l != replace_l {
                memmove(
                    start.offset(insert_l as isize) as *mut libc::c_void,
                    start.offset(replace_l as isize) as *const libc::c_void,
                    ((*story).length).wrapping_sub(
                        start.offset(replace_l as isize).offset_from((*story).data) as u64,
                    ),
                );
                (*story).length =
                    ((*story).length as u64).wrapping_add(insert_l.wrapping_sub(replace_l)) as u64;
                *((*story).data).offset((*story).length as isize) = 0;
            }
            memmove(
                start as *mut libc::c_void,
                insert as *const libc::c_void,
                insert_l,
            );
        }
    }
}

#[no_mangle]
pub extern "C" fn madlibs(mut story: *mut dstr) {
    unsafe {
        static mut buffer_size: u64 = 128;
        let vla = buffer_size as usize;
        let mut insert: Vec<i8> = ::std::vec::from_elem(0, vla);
        let vla_0 = buffer_size as usize;
        let mut replace: Vec<i8> = ::std::vec::from_elem(0, vla_0);
        let mut start: *mut i8 = 0 as *mut i8;
        let mut end: *mut i8 = (*story).data;
        loop {
            start = strchr(end, '<' as i32);
            if start.is_null() {
                break;
            }
            end = strchr(start, '>' as i32);
            if end.is_null() {
                fprintf(
                    stderr,
                    b"Malformed brackets in input\0" as *const u8 as *const i8,
                );
                exit(1);
            }
            strncpy(
                replace.as_mut_ptr(),
                start,
                (end.offset_from(start) as i64 + 1i64) as u64,
            );
            *replace
                .as_mut_ptr()
                .offset((end.offset_from(start) as i64 + 1i64) as isize) = '\0' as i8;
            print!(
                "Enter value for field {}: ",
                build_str_from_raw_ptr(replace.as_mut_ptr() as *mut u8)
            );
            fgets(insert.as_mut_ptr(), buffer_size as i32, stdin);
            let il: u64 = (strlen(insert.as_mut_ptr())).wrapping_sub(1);
            if *insert.as_mut_ptr().offset(il as isize) as i32 == '\n' as i32 {
                *insert.as_mut_ptr().offset(il as isize) = '\0' as i8;
            }
            dstr_replace_all(story, replace.as_mut_ptr(), insert.as_mut_ptr());
        }
        print!("\n");
    }
}

fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    unsafe {
        if argc < 2 {
            return 0;
        }
        let mut fd: *mut FILE = fopen(*argv.offset(1 as isize), b"r\0" as *const u8 as *const i8);
        if fd.is_null() {
            fprintf(
                stderr,
                b"Could not open file: '%s\n\0" as *const u8 as *const i8,
                *argv.offset(1 as isize),
            );
            exit(1);
        }
        let mut story: *mut dstr = readinput(fd);
        fclose(fd);
        if story.is_null() {
            fprintf(
                stderr,
                b"Failed to allocate memory\0" as *const u8 as *const i8,
            );
            exit(1);
        }
        madlibs(story);
        print!("{}\n", build_str_from_raw_ptr((*story).data as *mut u8));
        dstr_delete(story);
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
    unsafe {
        ::std::process::exit(
            main_0((args.len() - 1) as i32, args.as_mut_ptr() as *mut *mut i8) as i32,
        );
    }
}
