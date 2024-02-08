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

use std::time::SystemTime;
pub fn rust_time(ref_result: Option<&mut i64>) -> i64 {
    let result = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    };
    match ref_result {
        Some(r) => *r = result,
        None => {}
    }
    return result as i64;
}

use c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    fn getline(__lineptr: *mut *mut i8, __n: *mut u64, __stream: *mut FILE) -> i64;
    fn rand() -> i32;
    fn srand(__seed: u32);
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
fn main_0() -> i32 {
    unsafe {
        let mut question: *mut i8 = 0 as *mut i8;
        let mut len: u64 = 0;
        let mut read: i64 = 0;
        let mut answers: [*const i8; 20] = [
            b"It is certain\0" as *const u8 as *const i8,
            b"It is decidedly so\0" as *const u8 as *const i8,
            b"Without a doubt\0" as *const u8 as *const i8,
            b"Yes, definitely\0" as *const u8 as *const i8,
            b"You may rely on it\0" as *const u8 as *const i8,
            b"As I see it, yes\0" as *const u8 as *const i8,
            b"Most likely\0" as *const u8 as *const i8,
            b"Outlook good\0" as *const u8 as *const i8,
            b"Signs point to yes\0" as *const u8 as *const i8,
            b"Yes\0" as *const u8 as *const i8,
            b"Reply hazy, try again\0" as *const u8 as *const i8,
            b"Ask again later\0" as *const u8 as *const i8,
            b"Better not tell you now\0" as *const u8 as *const i8,
            b"Cannot predict now\0" as *const u8 as *const i8,
            b"Concentrate and ask again\0" as *const u8 as *const i8,
            b"Don't bet on it\0" as *const u8 as *const i8,
            b"My reply is no\0" as *const u8 as *const i8,
            b"My sources say no\0" as *const u8 as *const i8,
            b"Outlook not so good\0" as *const u8 as *const i8,
            b"Very doubtful\0" as *const u8 as *const i8,
        ];
        srand(rust_time(None) as u32);
        print!("Please enter your question or a blank line to quit.\n");
        loop {
            print!("\n? : ");
            read = getline(&mut question, &mut len, stdin);
            if read < 2 {
                break;
            }
            print!(
                "\n{}\n",
                build_str_from_raw_ptr(answers[(rand() % 20i32) as usize] as *mut u8)
            );
        }
        if !question.is_null() {
            free(question as *mut libc::c_void);
        }
        return 0;
    }
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
