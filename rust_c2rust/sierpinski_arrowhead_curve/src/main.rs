#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
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
pub struct cursor_tag {
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub angle: libc::c_int,
}
pub type cursor_t = cursor_tag;
#[no_mangle]
pub unsafe extern "C" fn turn(mut cursor: *mut cursor_t, mut angle: libc::c_int) {
    (*cursor).angle = ((*cursor).angle + angle) % 360 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn draw_line(
    mut out: *mut FILE,
    mut cursor: *mut cursor_t,
    mut length: libc::c_double,
) {
    let mut theta: libc::c_double = 3.14159265358979323846f64
        * (*cursor).angle as libc::c_double / 180.0f64;
    (*cursor).x += length * cos(theta);
    (*cursor).y += length * sin(theta);
    fprintf(
        out,
        b"L%g,%g\n\0" as *const u8 as *const libc::c_char,
        (*cursor).x,
        (*cursor).y,
    );
}
#[no_mangle]
pub unsafe extern "C" fn curve(
    mut out: *mut FILE,
    mut order: libc::c_int,
    mut length: libc::c_double,
    mut cursor: *mut cursor_t,
    mut angle: libc::c_int,
) {
    if order == 0 as libc::c_int {
        draw_line(out, cursor, length);
    } else {
        curve(
            out,
            order - 1 as libc::c_int,
            length / 2 as libc::c_int as libc::c_double,
            cursor,
            -angle,
        );
        turn(cursor, angle);
        curve(
            out,
            order - 1 as libc::c_int,
            length / 2 as libc::c_int as libc::c_double,
            cursor,
            angle,
        );
        turn(cursor, angle);
        curve(
            out,
            order - 1 as libc::c_int,
            length / 2 as libc::c_int as libc::c_double,
            cursor,
            -angle,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn write_sierpinski_arrowhead(
    mut out: *mut FILE,
    mut size: libc::c_int,
    mut order: libc::c_int,
) {
    let margin: libc::c_double = 20.0f64;
    let side: libc::c_double = size as libc::c_double - 2.0f64 * margin;
    let mut cursor: cursor_t = cursor_t { x: 0., y: 0., angle: 0 };
    cursor.angle = 0 as libc::c_int;
    cursor.x = margin;
    cursor
        .y = 0.5f64 * size as libc::c_double
        + 0.25f64 * sqrt(3 as libc::c_int as libc::c_double) * side;
    if order & 1 as libc::c_int != 0 as libc::c_int {
        turn(&mut cursor, -(60 as libc::c_int));
    }
    fprintf(
        out,
        b"<svg xmlns='http://www.w3.org/2000/svg' width='%d' height='%d'>\n\0"
            as *const u8 as *const libc::c_char,
        size,
        size,
    );
    fprintf(
        out,
        b"<rect width='100%%' height='100%%' fill='white'/>\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        out,
        b"<path stroke-width='1' stroke='black' fill='none' d='\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(out, b"M%g,%g\n\0" as *const u8 as *const libc::c_char, cursor.x, cursor.y);
    curve(out, order, side, &mut cursor, 60 as libc::c_int);
    fprintf(out, b"'/>\n</svg>\n\0" as *const u8 as *const libc::c_char);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut filename: *const libc::c_char = b"sierpinski_arrowhead.svg\0" as *const u8
        as *const libc::c_char;
    if argc == 2 as libc::c_int {
        filename = *argv.offset(1 as libc::c_int as isize);
    }
    let mut out: *mut FILE = fopen(filename, b"w\0" as *const u8 as *const libc::c_char);
    if out.is_null() {
        perror(filename);
        return 1 as libc::c_int;
    }
    write_sierpinski_arrowhead(out, 600 as libc::c_int, 8 as libc::c_int);
    fclose(out);
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
