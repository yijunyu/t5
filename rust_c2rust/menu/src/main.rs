#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
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
unsafe fn main_0() -> libc::c_int {
    let mut items: [*const libc::c_char; 5] = [
        b"fee fie\0" as *const u8 as *const libc::c_char,
        b"huff and puff\0" as *const u8 as *const libc::c_char,
        b"mirror mirror\0" as *const u8 as *const libc::c_char,
        b"tick tock\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut prompt: *const libc::c_char = b"Which is from the three pigs?\0" as *const u8
        as *const libc::c_char;
    printf(
        b"You chose %s.\n\0" as *const u8 as *const libc::c_char,
        menu_select(items.as_mut_ptr(), prompt),
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn menu_select(
    mut items: *const *const libc::c_char,
    mut prompt: *const libc::c_char,
) -> *const libc::c_char {
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    let mut i: libc::c_int = 0;
    let mut choice: libc::c_int = 0;
    let mut choice_max: libc::c_int = 0;
    if items.is_null() {
        return 0 as *const libc::c_char;
    }
    loop {
        i = 0 as libc::c_int;
        while !(*items.offset(i as isize)).is_null() {
            printf(
                b"%d) %s\n\0" as *const u8 as *const libc::c_char,
                i + 1 as libc::c_int,
                *items.offset(i as isize),
            );
            i += 1;
            i;
        }
        choice_max = i;
        if !prompt.is_null() {
            printf(b"%s \0" as *const u8 as *const libc::c_char, prompt);
        } else {
            printf(b"Choice? \0" as *const u8 as *const libc::c_char);
        }
        if !(fgets(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong
                as libc::c_int,
            stdin,
        ))
            .is_null()
        {
            choice = atoi(buf.as_mut_ptr());
        }
        if !(1 as libc::c_int > choice || choice > choice_max) {
            break;
        }
    }
    return *items.offset((choice - 1 as libc::c_int) as isize);
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
