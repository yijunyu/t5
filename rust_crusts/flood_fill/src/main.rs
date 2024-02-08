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
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn fscanf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn fgetc(__stream: *mut FILE) -> i32;
    fn ungetc(__c: i32, __stream: *mut FILE) -> i32;
    fn feof(__stream: *mut FILE) -> i32;
    fn ferror(__stream: *mut FILE) -> i32;
    fn exit(_: i32) -> !;
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
static mut width: i32 = 0;
static mut height: i32 = 0;
static mut bitmap: [[u8; 2048]; 2048] = [[0; 2048]; 2048];
static mut oldColor: u8 = 0;
static mut newColor: u8 = 0;
#[no_mangle]
pub extern "C" fn floodFill(mut i: i32, mut j: i32) {
    unsafe {
        if 0 <= i
            && i < height
            && 0 <= j
            && j < width
            && bitmap[i as usize][j as usize] as i32 == oldColor as i32
        {
            bitmap[i as usize][j as usize] = newColor;
            floodFill(i - 1, j);
            floodFill(i + 1, j);
            floodFill(i, j - 1);
            floodFill(i, j + 1);
        }
    }
}

#[no_mangle]
pub extern "C" fn skipLine(mut file: *mut FILE) {
    unsafe { while ferror(file) == 0 && feof(file) == 0 && fgetc(file) != '\n' as i32 {} }
}

#[no_mangle]
pub extern "C" fn skipCommentLines(mut file: *mut FILE) {
    unsafe {
        let mut c: i32 = 0;
        let mut comment: i32 = '#' as i32;
        loop {
            c = fgetc(file);
            if !(c == comment) {
                break;
            }
            skipLine(file);
        }
        ungetc(c, file);
    }
}

#[no_mangle]
pub extern "C" fn readPortableBitMap(mut file: *mut FILE) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        skipLine(file);
        skipCommentLines(file);
        fscanf(
            file,
            b"%d\0" as *const u8 as *const i8,
            &mut width as *mut i32,
        );
        skipCommentLines(file);
        fscanf(
            file,
            b"%d\0" as *const u8 as *const i8,
            &mut height as *mut i32,
        );
        skipCommentLines(file);
        if width <= 2048 && height <= 2048 {
            i = 0;
            while i < height {
                j = 0;
                while j < width {
                    fscanf(
                        file,
                        b"%1d\0" as *const u8 as *const i8,
                        &mut *(*bitmap.as_mut_ptr().offset(i as isize))
                            .as_mut_ptr()
                            .offset(j as isize) as *mut u8,
                    );
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
        } else {
            exit(1);
        }
        panic!("Reached end of non-void function without returning");
    }
}

#[no_mangle]
pub extern "C" fn writePortableBitMap(mut file: *mut FILE) {
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        fprintf(file, b"P1\n\0" as *const u8 as *const i8);
        fprintf(file, b"%d %d\n\0" as *const u8 as *const i8, width, height);
        i = 0;
        while i < height {
            j = 0;
            while j < width {
                fprintf(
                    file,
                    b"%1d\0" as *const u8 as *const i8,
                    bitmap[i as usize][j as usize] as i32,
                );
                j += 1;
                j;
            }
            fprintf(file, b"\n\0" as *const u8 as *const i8);
            i += 1;
            i;
        }
    }
}

fn main_0() -> i32 {
    unsafe {
        oldColor = 1;
        newColor = (if oldColor as i32 != 0 { 0 } else { 1 }) as u8;
        readPortableBitMap(stdin);
        floodFill(height / 2, width / 2);
        writePortableBitMap(stdout);
    }
    return 0;
}

pub fn main() {
    unsafe {
        ::std::process::exit(main_0() as i32);
    }
}
