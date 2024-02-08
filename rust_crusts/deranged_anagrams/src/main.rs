#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
fn build_str_from_raw_ptr(raw_ptr: *mut u8) -> String {
    unsafe {
        let mut str_size: usize = 0;
        while *raw_ptr.offset(str_size as isize) != 0 {
            str_size += 1;
        }
        return std::str::from_utf8_unchecked(std::slice::from_raw_parts(raw_ptr, str_size))
            .to_owned();
    }
}

use c2rust_out::*;
extern "C" {
    fn malloc(_: u64) -> *mut libc::c_void;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn close(__fd: i32) -> i32;
    fn read(__fd: i32, __buf: *mut libc::c_void, __nbytes: u64) -> i64;
    fn open(__file: *const i8, __oflag: i32, _: ...) -> i32;
    fn fstat(__fd: i32, __buf: *mut stat) -> i32;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_nlink: u64,
    pub st_mode: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub __pad0: i32,
    pub st_rdev: u64,
    pub st_size: i64,
    pub st_blksize: i64,
    pub st_blocks: i64,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [i64; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct word {
    pub w: *const i8,
    pub next: *mut word,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union node {
    pub down: [*mut node; 10],
    pub list: [*mut word; 10],
}
#[no_mangle]
pub static mut freq: *const i8 = b"zqxjkvbpygfwmucldrhsnioate\0" as *const u8 as *const i8;
#[no_mangle]
pub static mut char_to_idx: [i32; 128] = [0; 128];
#[no_mangle]
pub extern "C" fn deranged(mut s1: *const i8, mut s2: *const i8) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        i = 0;
        while *s1.offset(i as isize) != 0 {
            if *s1.offset(i as isize) as i32 == *s2.offset(i as isize) as i32 {
                return 0;
            }
            i += 1;
            i;
        }
        return 1;
    }
}

#[no_mangle]
pub extern "C" fn count_letters(mut s: *const i8, mut c: *mut u8) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut len: i32 = 0;
        memset(c as *mut libc::c_void, 0, 26);
        i = 0;
        len = i;
        while *s.offset(i as isize) != 0 {
            if (*s.offset(i as isize) as i32) < 'a' as i32
                || *s.offset(i as isize) as i32 > 'z' as i32
            {
                return 0;
            }
            len += 1;
            len;
            let ref mut fresh0 =
                *c.offset(char_to_idx[*s.offset(i as isize) as u8 as usize] as isize);
            *fresh0 = (*fresh0).wrapping_add(1);
            *fresh0;
            i += 1;
            i;
        }
        return len;
    }
}

#[no_mangle]
pub extern "C" fn insert(mut root: *mut node, mut s: *const i8, mut cnt: *mut u8) -> *const i8 {
    unsafe {
        let mut i: i32 = 0;
        let mut n: *mut node = 0 as *mut node;
        let mut v: *mut word = 0 as *mut word;
        let mut w: *mut word = 0 as *mut word;
        i = 0;
        while i < 25 {
            n = (*root).down[*cnt.offset(i as isize) as usize];
            if n.is_null() {
                n = calloc(1, ::core::mem::size_of::<node>() as u64) as *mut node;
                (*root).down[*cnt.offset(i as isize) as usize] = n;
            }
            i += 1;
            i;
            root = n;
        }
        w = malloc(::core::mem::size_of::<word>() as u64) as *mut word;
        (*w).w = s;
        (*w).next = (*root).list[*cnt.offset(25 as isize) as usize];
        (*root).list[*cnt.offset(25 as isize) as usize] = w;
        v = (*w).next;
        while !v.is_null() {
            if deranged((*w).w, (*v).w) != 0 {
                return (*v).w;
            }
            v = (*v).next;
        }
        return 0 as *const i8;
    }
}

fn main_0(mut c: i32, mut v: *mut *mut i8) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut words: *mut i8 = 0 as *mut i8;
        let mut st: stat = stat {
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
            st_atim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_mtim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_ctim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            __glibc_reserved: [0; 3],
        };
        let mut fd: i32 = open(
            if c < 2 {
                b"unixdict.txt\0" as *const u8 as *const i8
            } else {
                *v.offset(1 as isize) as *const i8
            },
            0,
        );
        if fstat(fd, &mut st) < 0 {
            return 1;
        }
        words = malloc(st.st_size as u64) as *mut i8;
        read(fd, words as *mut libc::c_void, st.st_size as u64);
        close(fd);
        let mut root: node = node {
            down: [
                0 as *mut node,
                0 as *mut node,
                0 as *mut node,
                0 as *mut node,
                0 as *mut node,
                0 as *mut node,
                0 as *mut node,
                0 as *mut node,
                0 as *mut node,
                0 as *mut node,
            ],
        };
        let mut cnt: [u8; 26] = [0; 26];
        let mut best_len: i32 = 0;
        let mut b1: *const i8 = 0 as *const i8;
        let mut b2: *const i8 = 0 as *const i8;
        i = 0;
        while *freq.offset(i as isize) != 0 {
            char_to_idx[*freq.offset(i as isize) as u8 as usize] = i;
            i += 1;
            i;
        }
        j = 0;
        i = j;
        while (i as i64) < st.st_size {
            if !(*words.offset(i as isize) as i32 != '\n' as i32) {
                *words.offset(i as isize) = '\0' as i8;
                if i - j > best_len {
                    count_letters(words.offset(j as isize), cnt.as_mut_ptr());
                    let mut match_0: *const i8 =
                        insert(&mut root, words.offset(j as isize), cnt.as_mut_ptr());
                    if !match_0.is_null() {
                        best_len = i - j;
                        b1 = words.offset(j as isize);
                        b2 = match_0;
                    }
                }
                i += 1;
                j = i;
            }
            i += 1;
            i;
        }
        if best_len != 0 {
            print!(
                "longest derangement: {} {}\n",
                build_str_from_raw_ptr(b1 as *mut u8),
                build_str_from_raw_ptr(b2 as *mut u8)
            );
        }
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
