#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    fn ungetc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
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
static mut width: libc::c_int = 0;
static mut height: libc::c_int = 0;
static mut bitmap: [[libc::c_uchar; 2048]; 2048] = [[0; 2048]; 2048];
static mut oldColor: libc::c_uchar = 0;
static mut newColor: libc::c_uchar = 0;
#[no_mangle]
pub unsafe extern "C" fn floodFill(mut i: libc::c_int, mut j: libc::c_int) {
    if 0 as libc::c_int <= i && i < height && 0 as libc::c_int <= j && j < width
        && bitmap[i as usize][j as usize] as libc::c_int == oldColor as libc::c_int
    {
        bitmap[i as usize][j as usize] = newColor;
        floodFill(i - 1 as libc::c_int, j);
        floodFill(i + 1 as libc::c_int, j);
        floodFill(i, j - 1 as libc::c_int);
        floodFill(i, j + 1 as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn skipLine(mut file: *mut FILE) {
    while ferror(file) == 0 && feof(file) == 0 && fgetc(file) != '\n' as i32 {}
}
#[no_mangle]
pub unsafe extern "C" fn skipCommentLines(mut file: *mut FILE) {
    let mut c: libc::c_int = 0;
    let mut comment: libc::c_int = '#' as i32;
    loop {
        c = fgetc(file);
        if !(c == comment) {
            break;
        }
        skipLine(file);
    }
    ungetc(c, file);
}
#[no_mangle]
pub unsafe extern "C" fn readPortableBitMap(mut file: *mut FILE) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    skipLine(file);
    skipCommentLines(file);
    fscanf(
        file,
        b"%d\0" as *const u8 as *const libc::c_char,
        &mut width as *mut libc::c_int,
    );
    skipCommentLines(file);
    fscanf(
        file,
        b"%d\0" as *const u8 as *const libc::c_char,
        &mut height as *mut libc::c_int,
    );
    skipCommentLines(file);
    if width <= 2048 as libc::c_int && height <= 2048 as libc::c_int {
        i = 0 as libc::c_int;
        while i < height {
            j = 0 as libc::c_int;
            while j < width {
                fscanf(
                    file,
                    b"%1d\0" as *const u8 as *const libc::c_char,
                    &mut *(*bitmap.as_mut_ptr().offset(i as isize))
                        .as_mut_ptr()
                        .offset(j as isize) as *mut libc::c_uchar,
                );
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    } else {
        exit(1 as libc::c_int);
    }
    panic!("Reached end of non-void function without returning");
}
#[no_mangle]
pub unsafe extern "C" fn writePortableBitMap(mut file: *mut FILE) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    fprintf(file, b"P1\n\0" as *const u8 as *const libc::c_char);
    fprintf(file, b"%d %d\n\0" as *const u8 as *const libc::c_char, width, height);
    i = 0 as libc::c_int;
    while i < height {
        j = 0 as libc::c_int;
        while j < width {
            fprintf(
                file,
                b"%1d\0" as *const u8 as *const libc::c_char,
                bitmap[i as usize][j as usize] as libc::c_int,
            );
            j += 1;
            j;
        }
        fprintf(file, b"\n\0" as *const u8 as *const libc::c_char);
        i += 1;
        i;
    }
}
unsafe fn main_0() -> libc::c_int {
    oldColor = 1 as libc::c_int as libc::c_uchar;
    newColor = (if oldColor as libc::c_int != 0 {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    }) as libc::c_uchar;
    readPortableBitMap(stdin);
    floodFill(height / 2 as libc::c_int, width / 2 as libc::c_int);
    writePortableBitMap(stdout);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
