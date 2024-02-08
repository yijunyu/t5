#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flock {
    pub l_type: libc::c_short,
    pub l_whence: libc::c_short,
    pub l_start: __off_t,
    pub l_len: __off_t,
    pub l_pid: __pid_t,
}
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
#[no_mangle]
pub unsafe extern "C" fn fail(mut message: *const libc::c_char) {
    perror(message);
    exit(1 as libc::c_int);
}
static mut ooi_path: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn ooi_unlink() {
    unlink(ooi_path);
}
#[no_mangle]
pub unsafe extern "C" fn only_one_instance() {
    let mut fl: flock = flock {
        l_type: 0,
        l_whence: 0,
        l_start: 0,
        l_len: 0,
        l_pid: 0,
    };
    let mut dirlen: size_t = 0;
    let mut fd: libc::c_int = 0;
    let mut dir: *mut libc::c_char = 0 as *mut libc::c_char;
    dir = getenv(b"HOME\0" as *const u8 as *const libc::c_char);
    if dir.is_null()
        || *dir.offset(0 as libc::c_int as isize) as libc::c_int != '/' as i32
    {
        fputs(b"Bad home directory.\n\0" as *const u8 as *const libc::c_char, stderr);
        exit(1 as libc::c_int);
    }
    dirlen = strlen(dir);
    ooi_path = malloc(
        dirlen
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong),
    ) as *mut libc::c_char;
    if ooi_path.is_null() {
        fail(b"malloc\0" as *const u8 as *const libc::c_char);
    }
    memcpy(ooi_path as *mut libc::c_void, dir as *const libc::c_void, dirlen);
    memcpy(
        ooi_path.offset(dirlen as isize) as *mut libc::c_void,
        b"/rosetta-code-lock\0" as *const u8 as *const libc::c_char
            as *const libc::c_void,
        ::core::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong,
    );
    fd = open(ooi_path, 0o2 as libc::c_int | 0o100 as libc::c_int, 0o600 as libc::c_int);
    if fd < 0 as libc::c_int {
        fail(ooi_path);
    }
    fl.l_start = 0 as libc::c_int as __off_t;
    fl.l_len = 0 as libc::c_int as __off_t;
    fl.l_type = 1 as libc::c_int as libc::c_short;
    fl.l_whence = 0 as libc::c_int as libc::c_short;
    if fcntl(fd, 6 as libc::c_int, &mut fl as *mut flock) < 0 as libc::c_int {
        fputs(
            b"Another instance of this program is running.\n\0" as *const u8
                as *const libc::c_char,
            stderr,
        );
        exit(1 as libc::c_int);
    }
    atexit(Some(ooi_unlink as unsafe extern "C" fn() -> ()));
}
unsafe fn main_0() -> libc::c_int {
    let mut i: libc::c_int = 0;
    only_one_instance();
    i = 10 as libc::c_int;
    while i > 0 as libc::c_int {
        printf(
            b"%d...%s\0" as *const u8 as *const libc::c_char,
            i,
            if i % 5 as libc::c_int == 1 as libc::c_int {
                b"\n\0" as *const u8 as *const libc::c_char
            } else {
                b" \0" as *const u8 as *const libc::c_char
            },
        );
        fflush(stdout);
        sleep(1 as libc::c_int as libc::c_uint);
        i -= 1;
        i;
    }
    puts(b"Fin!\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
