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
    fn cos(_: f64) -> f64;
    fn sin(_: f64) -> f64;
    fn sqrt(_: f64) -> f64;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fopen(_: *const i8, _: *const i8) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cursor_tag {
    pub x: f64,
    pub y: f64,
    pub angle: i32,
}
pub type cursor_t = cursor_tag;
#[no_mangle]
pub extern "C" fn turn(mut cursor: *mut cursor_t, mut angle: i32) {
    unsafe {
        (*cursor).angle = ((*cursor).angle + angle) % 360;
    }
}

#[no_mangle]
pub extern "C" fn draw_line(mut out: *mut FILE, mut cursor: *mut cursor_t, mut length: f64) {
    unsafe {
        let mut theta: f64 = 3.14159265358979323846f64 * (*cursor).angle as f64 / 180.0f64;
        (*cursor).x += length * cos(theta);
        (*cursor).y += length * sin(theta);
        fprintf(
            out,
            b"L%g,%g\n\0" as *const u8 as *const i8,
            (*cursor).x,
            (*cursor).y,
        );
    }
}

#[no_mangle]
pub extern "C" fn curve(
    mut out: *mut FILE,
    mut order: i32,
    mut length: f64,
    mut cursor: *mut cursor_t,
    mut angle: i32,
) {
    unsafe {
        if order == 0 {
            draw_line(out, cursor, length);
        } else {
            curve(out, order - 1, length / 2 as f64, cursor, -angle);
            turn(cursor, angle);
            curve(out, order - 1, length / 2 as f64, cursor, angle);
            turn(cursor, angle);
            curve(out, order - 1, length / 2 as f64, cursor, -angle);
        };
    }
}

#[no_mangle]
pub extern "C" fn write_sierpinski_arrowhead(mut out: *mut FILE, mut size: i32, mut order: i32) {
    unsafe {
        let margin: f64 = 20.0f64;
        let side: f64 = size as f64 - 2.0f64 * margin;
        let mut cursor: cursor_t = cursor_t {
            x: 0.,
            y: 0.,
            angle: 0,
        };
        cursor.angle = 0;
        cursor.x = margin;
        cursor.y = 0.5f64 * size as f64 + 0.25f64 * sqrt(3 as f64) * side;
        if order & 1 != 0 {
            turn(&mut cursor, -60);
        }
        fprintf(
            out,
            b"<svg xmlns='http://www.w3.org/2000/svg' width='%d' height='%d'>\n\0" as *const u8
                as *const i8,
            size,
            size,
        );
        fprintf(
            out,
            b"<rect width='100%%' height='100%%' fill='white'/>\n\0" as *const u8 as *const i8,
        );
        fprintf(
            out,
            b"<path stroke-width='1' stroke='black' fill='none' d='\0" as *const u8 as *const i8,
        );
        fprintf(
            out,
            b"M%g,%g\n\0" as *const u8 as *const i8,
            cursor.x,
            cursor.y,
        );
        curve(out, order, side, &mut cursor, 60);
        fprintf(out, b"'/>\n</svg>\n\0" as *const u8 as *const i8);
    }
}

fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    unsafe {
        let mut filename: *const i8 = b"sierpinski_arrowhead.svg\0" as *const u8 as *const i8;
        if argc == 2 {
            filename = *argv.offset(1 as isize);
        }
        let mut out: *mut FILE = fopen(filename, b"w\0" as *const u8 as *const i8);
        if out.is_null() {
            perror(filename);
            return 1;
        }
        write_sierpinski_arrowhead(out, 600, 8);
        fclose(out);
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
