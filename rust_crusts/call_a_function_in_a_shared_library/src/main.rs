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
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn dlopen(__file: *const i8, __mode: i32) -> *mut libc::c_void;
    fn dlclose(__handle: *mut libc::c_void) -> i32;
    fn dlsym(__handle: *mut libc::c_void, __name: *const i8) -> *mut libc::c_void;
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
#[no_mangle]
pub extern "C" fn myopenimage(mut in_0: *const i8) -> i32 {
    unsafe {
        static mut handle: i32 = 0;
        fprintf(
            stderr,
            b"internal openimage opens %s...\n\0" as *const u8 as *const i8,
            in_0,
        );
        let fresh0 = handle;
        handle = handle + 1;
        return fresh0;
    }
}

fn main_0() -> i32 {
    unsafe {
        let mut imglib: *mut libc::c_void = 0 as *mut libc::c_void;
        let mut extopenimage: Option<unsafe extern "C" fn(*const i8) -> i32> = None;
        let mut imghandle: i32 = 0;
        imglib = dlopen(b"./fakeimglib.so\0" as *const u8 as *const i8, 0x1);
        if !imglib.is_null() {
            let ref mut fresh1 = *(&mut extopenimage
                as *mut Option<unsafe extern "C" fn(*const i8) -> i32>
                as *mut *mut libc::c_void);
            *fresh1 = dlsym(imglib, b"openimage\0" as *const u8 as *const i8);
            imghandle = extopenimage.expect("non-null function pointer")(
                b"fake.img\0" as *const u8 as *const i8,
            );
        } else {
            imghandle = myopenimage(b"fake.img\0" as *const u8 as *const i8);
        }
        print!("opened with handle {}\n", imghandle);
        if !imglib.is_null() {
            dlclose(imglib);
        }
        return 0;
    }
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
