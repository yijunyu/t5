#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn dlopen(__file: *const libc::c_char, __mode: libc::c_int) -> *mut libc::c_void;
    fn dlclose(__handle: *mut libc::c_void) -> libc::c_int;
    fn dlsym(
        __handle: *mut libc::c_void,
        __name: *const libc::c_char,
    ) -> *mut libc::c_void;
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
#[no_mangle]
pub unsafe extern "C" fn myopenimage(mut in_0: *const libc::c_char) -> libc::c_int {
    static mut handle: libc::c_int = 0 as libc::c_int;
    fprintf(
        stderr,
        b"internal openimage opens %s...\n\0" as *const u8 as *const libc::c_char,
        in_0,
    );
    let fresh0 = handle;
    handle = handle + 1;
    return fresh0;
}
unsafe fn main_0() -> libc::c_int {
    let mut imglib: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut extopenimage: Option::<
        unsafe extern "C" fn(*const libc::c_char) -> libc::c_int,
    > = None;
    let mut imghandle: libc::c_int = 0;
    imglib = dlopen(
        b"./fakeimglib.so\0" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    );
    if !imglib.is_null() {
        let ref mut fresh1 = *(&mut extopenimage
            as *mut Option::<unsafe extern "C" fn(*const libc::c_char) -> libc::c_int>
            as *mut *mut libc::c_void);
        *fresh1 = dlsym(imglib, b"openimage\0" as *const u8 as *const libc::c_char);
        imghandle = extopenimage
            .expect(
                "non-null function pointer",
            )(b"fake.img\0" as *const u8 as *const libc::c_char);
    } else {
        imghandle = myopenimage(b"fake.img\0" as *const u8 as *const libc::c_char);
    }
    printf(b"opened with handle %d\n\0" as *const u8 as *const libc::c_char, imghandle);
    if !imglib.is_null() {
        dlclose(imglib);
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
