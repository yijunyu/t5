#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[no_mangle]
pub unsafe extern "C" fn check_reg(mut path: *const libc::c_char) -> libc::c_int {
    let mut sb: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    return (stat(path, &mut sb) == 0 as libc::c_int
        && sb.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o100000 as libc::c_int as libc::c_uint) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn check_dir(mut path: *const libc::c_char) -> libc::c_int {
    let mut sb: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    return (stat(path, &mut sb) == 0 as libc::c_int
        && sb.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint) as libc::c_int;
}
unsafe fn main_0() -> libc::c_int {
    printf(
        b"input.txt is a regular file? %s\n\0" as *const u8 as *const libc::c_char,
        if check_reg(b"input.txt\0" as *const u8 as *const libc::c_char) != 0 {
            b"yes\0" as *const u8 as *const libc::c_char
        } else {
            b"no\0" as *const u8 as *const libc::c_char
        },
    );
    printf(
        b"docs is a directory? %s\n\0" as *const u8 as *const libc::c_char,
        if check_dir(b"docs\0" as *const u8 as *const libc::c_char) != 0 {
            b"yes\0" as *const u8 as *const libc::c_char
        } else {
            b"no\0" as *const u8 as *const libc::c_char
        },
    );
    printf(
        b"/input.txt is a regular file? %s\n\0" as *const u8 as *const libc::c_char,
        if check_reg(b"/input.txt\0" as *const u8 as *const libc::c_char) != 0 {
            b"yes\0" as *const u8 as *const libc::c_char
        } else {
            b"no\0" as *const u8 as *const libc::c_char
        },
    );
    printf(
        b"/docs is a directory? %s\n\0" as *const u8 as *const libc::c_char,
        if check_dir(b"/docs\0" as *const u8 as *const libc::c_char) != 0 {
            b"yes\0" as *const u8 as *const libc::c_char
        } else {
            b"no\0" as *const u8 as *const libc::c_char
        },
    );
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
