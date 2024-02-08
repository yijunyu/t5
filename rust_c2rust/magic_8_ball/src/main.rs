#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn getline(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn rand() -> libc::c_int;
    fn srand(__seed: libc::c_uint);
    fn free(_: *mut libc::c_void);
    fn time(__timer: *mut time_t) -> time_t;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
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
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
unsafe fn main_0() -> libc::c_int {
    let mut question: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0 as libc::c_int as size_t;
    let mut read: ssize_t = 0;
    let mut answers: [*const libc::c_char; 20] = [
        b"It is certain\0" as *const u8 as *const libc::c_char,
        b"It is decidedly so\0" as *const u8 as *const libc::c_char,
        b"Without a doubt\0" as *const u8 as *const libc::c_char,
        b"Yes, definitely\0" as *const u8 as *const libc::c_char,
        b"You may rely on it\0" as *const u8 as *const libc::c_char,
        b"As I see it, yes\0" as *const u8 as *const libc::c_char,
        b"Most likely\0" as *const u8 as *const libc::c_char,
        b"Outlook good\0" as *const u8 as *const libc::c_char,
        b"Signs point to yes\0" as *const u8 as *const libc::c_char,
        b"Yes\0" as *const u8 as *const libc::c_char,
        b"Reply hazy, try again\0" as *const u8 as *const libc::c_char,
        b"Ask again later\0" as *const u8 as *const libc::c_char,
        b"Better not tell you now\0" as *const u8 as *const libc::c_char,
        b"Cannot predict now\0" as *const u8 as *const libc::c_char,
        b"Concentrate and ask again\0" as *const u8 as *const libc::c_char,
        b"Don't bet on it\0" as *const u8 as *const libc::c_char,
        b"My reply is no\0" as *const u8 as *const libc::c_char,
        b"My sources say no\0" as *const u8 as *const libc::c_char,
        b"Outlook not so good\0" as *const u8 as *const libc::c_char,
        b"Very doubtful\0" as *const u8 as *const libc::c_char,
    ];
    srand(time(0 as *mut time_t) as libc::c_uint);
    printf(
        b"Please enter your question or a blank line to quit.\n\0" as *const u8
            as *const libc::c_char,
    );
    loop {
        printf(b"\n? : \0" as *const u8 as *const libc::c_char);
        read = getline(&mut question, &mut len, stdin);
        if read < 2 as libc::c_int as libc::c_long {
            break;
        }
        printf(
            b"\n%s\n\0" as *const u8 as *const libc::c_char,
            answers[(rand() % 20 as libc::c_int) as usize],
        );
    }
    if !question.is_null() {
        free(question as *mut libc::c_void);
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
