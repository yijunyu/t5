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
    fn fclose(__stream: *mut FILE) -> i32;
    fn fopen(_: *const i8, _: *const i8) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn fwrite(_: *const libc::c_void, _: u64, _: u64, _: *mut FILE) -> u64;
    fn fabs(_: f64) -> f64;
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
        let mut iX: i32 = 0;
        let mut iY: i32 = 0;
        let iXmax: i32 = 800;
        let iYmax: i32 = 800;
        let mut Cx: f64 = 0.;
        let mut Cy: f64 = 0.;
        let CxMin: f64 = -2.5f64;
        let CxMax: f64 = 1.5f64;
        let CyMin: f64 = -2.0f64;
        let CyMax: f64 = 2.0f64;
        let mut PixelWidth: f64 = (CxMax - CxMin) / iXmax as f64;
        let mut PixelHeight: f64 = (CyMax - CyMin) / iYmax as f64;
        let MaxColorComponentValue: i32 = 255;
        let mut fp: *mut FILE = 0 as *mut FILE;
        let mut filename: *mut i8 = b"new1.ppm\0" as *const u8 as *const i8 as *mut i8;
        let mut comment: *mut i8 = b"# \0" as *const u8 as *const i8 as *mut i8;
        static mut color: [u8; 3] = [0; 3];
        let mut Zx: f64 = 0.;
        let mut Zy: f64 = 0.;
        let mut Zx2: f64 = 0.;
        let mut Zy2: f64 = 0.;
        let mut Iteration: i32 = 0;
        let IterationMax: i32 = 200;
        let EscapeRadius: f64 = 2 as f64;
        let mut ER2: f64 = EscapeRadius * EscapeRadius;
        fp = fopen(filename, b"wb\0" as *const u8 as *const i8);
        fprintf(
            fp,
            b"P6\n %s\n %d\n %d\n %d\n\0" as *const u8 as *const i8,
            comment,
            iXmax,
            iYmax,
            MaxColorComponentValue,
        );
        iY = 0;
        while iY < iYmax {
            Cy = CyMin + iY as f64 * PixelHeight;
            if fabs(Cy) < PixelHeight / 2 as f64 {
                Cy = 0.0f64;
            }
            iX = 0;
            while iX < iXmax {
                Cx = CxMin + iX as f64 * PixelWidth;
                Zx = 0.0f64;
                Zy = 0.0f64;
                Zx2 = Zx * Zx;
                Zy2 = Zy * Zy;
                Iteration = 0;
                while Iteration < IterationMax && Zx2 + Zy2 < ER2 {
                    Zy = 2 as f64 * Zx * Zy + Cy;
                    Zx = Zx2 - Zy2 + Cx;
                    Zx2 = Zx * Zx;
                    Zy2 = Zy * Zy;
                    Iteration += 1;
                    Iteration;
                }
                if Iteration == IterationMax {
                    color[0 as usize] = 0;
                    color[1 as usize] = 0;
                    color[2 as usize] = 0;
                } else {
                    color[0 as usize] = 255;
                    color[1 as usize] = 255;
                    color[2 as usize] = 255;
                }
                fwrite(color.as_mut_ptr() as *const libc::c_void, 1, 3, fp);
                iX += 1;
                iX;
            }
            iY += 1;
            iY;
        }
        fclose(fp);
        return 0;
    }
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
