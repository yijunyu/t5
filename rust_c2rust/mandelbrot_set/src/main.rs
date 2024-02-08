#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fabs(_: libc::c_double) -> libc::c_double;
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
    let mut iX: libc::c_int = 0;
    let mut iY: libc::c_int = 0;
    let iXmax: libc::c_int = 800 as libc::c_int;
    let iYmax: libc::c_int = 800 as libc::c_int;
    let mut Cx: libc::c_double = 0.;
    let mut Cy: libc::c_double = 0.;
    let CxMin: libc::c_double = -2.5f64;
    let CxMax: libc::c_double = 1.5f64;
    let CyMin: libc::c_double = -2.0f64;
    let CyMax: libc::c_double = 2.0f64;
    let mut PixelWidth: libc::c_double = (CxMax - CxMin) / iXmax as libc::c_double;
    let mut PixelHeight: libc::c_double = (CyMax - CyMin) / iYmax as libc::c_double;
    let MaxColorComponentValue: libc::c_int = 255 as libc::c_int;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut filename: *mut libc::c_char = b"new1.ppm\0" as *const u8
        as *const libc::c_char as *mut libc::c_char;
    let mut comment: *mut libc::c_char = b"# \0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    static mut color: [libc::c_uchar; 3] = [0; 3];
    let mut Zx: libc::c_double = 0.;
    let mut Zy: libc::c_double = 0.;
    let mut Zx2: libc::c_double = 0.;
    let mut Zy2: libc::c_double = 0.;
    let mut Iteration: libc::c_int = 0;
    let IterationMax: libc::c_int = 200 as libc::c_int;
    let EscapeRadius: libc::c_double = 2 as libc::c_int as libc::c_double;
    let mut ER2: libc::c_double = EscapeRadius * EscapeRadius;
    fp = fopen(filename, b"wb\0" as *const u8 as *const libc::c_char);
    fprintf(
        fp,
        b"P6\n %s\n %d\n %d\n %d\n\0" as *const u8 as *const libc::c_char,
        comment,
        iXmax,
        iYmax,
        MaxColorComponentValue,
    );
    iY = 0 as libc::c_int;
    while iY < iYmax {
        Cy = CyMin + iY as libc::c_double * PixelHeight;
        if fabs(Cy) < PixelHeight / 2 as libc::c_int as libc::c_double {
            Cy = 0.0f64;
        }
        iX = 0 as libc::c_int;
        while iX < iXmax {
            Cx = CxMin + iX as libc::c_double * PixelWidth;
            Zx = 0.0f64;
            Zy = 0.0f64;
            Zx2 = Zx * Zx;
            Zy2 = Zy * Zy;
            Iteration = 0 as libc::c_int;
            while Iteration < IterationMax && Zx2 + Zy2 < ER2 {
                Zy = 2 as libc::c_int as libc::c_double * Zx * Zy + Cy;
                Zx = Zx2 - Zy2 + Cx;
                Zx2 = Zx * Zx;
                Zy2 = Zy * Zy;
                Iteration += 1;
                Iteration;
            }
            if Iteration == IterationMax {
                color[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
                color[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
                color[2 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
            } else {
                color[0 as libc::c_int as usize] = 255 as libc::c_int as libc::c_uchar;
                color[1 as libc::c_int as usize] = 255 as libc::c_int as libc::c_uchar;
                color[2 as libc::c_int as usize] = 255 as libc::c_int as libc::c_uchar;
            }
            fwrite(
                color.as_mut_ptr() as *const libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                3 as libc::c_int as libc::c_ulong,
                fp,
            );
            iX += 1;
            iX;
        }
        iY += 1;
        iY;
    }
    fclose(fp);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
