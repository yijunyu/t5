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
use c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fopen(_: *const i8, _: *const i8) -> *mut FILE;
    fn fgetc(__stream: *mut FILE) -> i32;
    fn fputc(__c: i32, __stream: *mut FILE) -> i32;
    fn perror(__s: *const i8);
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
static mut rot13_table: [i8; 256] = [0; 256];
extern "C" fn init_rot13_table() {
    unsafe {
        static mut upper: [u8; 27] = unsafe {
            *::core::mem::transmute::<&[u8; 27], &[u8; 27]>(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\0")
        };
        static mut lower: [u8; 27] = unsafe {
            *::core::mem::transmute::<&[u8; 27], &[u8; 27]>(b"abcdefghijklmnopqrstuvwxyz\0")
        };
        let mut ch: i32 = '\0' as i32;
        while ch <= 127 * 2 + 1 {
            rot13_table[ch as usize] = ch as i8;
            ch += 1;
            ch;
        }
        let mut p: *const u8 = upper.as_ptr();
        while *p.offset(13 as isize) as i32 != '\0' as i32 {
            rot13_table[*p.offset(0 as isize) as usize] = *p.offset(13 as isize) as i8;
            rot13_table[*p.offset(13 as isize) as usize] = *p.offset(0 as isize) as i8;
            p = p.offset(1);
            p;
        }
        let mut p_0: *const u8 = lower.as_ptr();
        while *p_0.offset(13 as isize) as i32 != '\0' as i32 {
            rot13_table[*p_0.offset(0 as isize) as usize] = *p_0.offset(13 as isize) as i8;
            rot13_table[*p_0.offset(13 as isize) as usize] = *p_0.offset(0 as isize) as i8;
            p_0 = p_0.offset(1);
            p_0;
        }
    }
}

extern "C" fn rot13_file(mut fp: *mut FILE) {
    unsafe {
        let mut ch: i32 = 0;
        loop {
            ch = fgetc(fp);
            if !(ch != -1) {
                break;
            }
            fputc(rot13_table[ch as usize] as i32, stdout);
        }
    }
}

fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    unsafe {
        init_rot13_table();
        if argc > 1 {
            let mut i: i32 = 1;
            while i < argc {
                let mut fp: *mut FILE =
                    fopen(*argv.offset(i as isize), b"r\0" as *const u8 as *const i8);
                if fp.is_null() {
                    perror(*argv.offset(i as isize));
                    return 1;
                }
                rot13_file(fp);
                fclose(fp);
                i += 1;
                i;
            }
        } else {
            rot13_file(stdin);
        }
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
