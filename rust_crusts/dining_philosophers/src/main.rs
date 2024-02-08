#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(c_variadic, extern_types)]
use c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn pthread_create(
        __newthread: *mut u64,
        __attr: *const pthread_attr_t,
        __start_routine: Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        __arg: *mut libc::c_void,
    ) -> i32;
    fn pthread_join(__th: u64, __thread_return: *mut *mut libc::c_void) -> i32;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> i32;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> i32;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> i32;
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn vprintf(_: *const i8, _: ::core::ffi::VaList) -> i32;
    fn rand() -> i32;
    fn sleep(__seconds: u32) -> u32;
    fn usleep(__useconds: u32) -> i32;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: u32,
    pub fp_offset: u32,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: i32,
    pub __count: u32,
    pub __owner: i32,
    pub __nusers: u32,
    pub __kind: i32,
    pub __spins: i16,
    pub __elision: i16,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [i8; 4],
    pub __align: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [i8; 56],
    pub __align: i64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [i8; 40],
    pub __align: i64,
}
pub const PTHREAD_MUTEX_DEFAULT: u32 = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: u32 = 2;
pub const PTHREAD_MUTEX_RECURSIVE: u32 = 1;
pub const PTHREAD_MUTEX_NORMAL: u32 = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: u32 = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: u32 = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: u32 = 1;
pub const PTHREAD_MUTEX_TIMED_NP: u32 = 0;
pub type va_list = __builtin_va_list;
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
pub static mut names: [*const i8; 5] = [
    b"Aristotle\0" as *const u8 as *const i8,
    b"Kant\0" as *const u8 as *const i8,
    b"Spinoza\0" as *const u8 as *const i8,
    b"Marx\0" as *const u8 as *const i8,
    b"Russell\0" as *const u8 as *const i8,
];
#[no_mangle]
pub static mut forks: [pthread_mutex_t; 5] = [pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0,
        __count: 0,
        __owner: 0,
        __nusers: 0,
        __kind: 0,
        __spins: 0,
        __elision: 0,
        __list: __pthread_list_t {
            __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
            __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
        },
    },
}; 5];
#[no_mangle]
pub static mut topic: [*const i8; 5] = [
    b"Spaghetti!\0" as *const u8 as *const i8,
    b"Life\0" as *const u8 as *const i8,
    b"Universe\0" as *const u8 as *const i8,
    b"Everything\0" as *const u8 as *const i8,
    b"Bathroom\0" as *const u8 as *const i8,
];
#[no_mangle]
pub unsafe extern "C" fn print(mut y: i32, mut x: i32, mut fmt: *const i8, mut args: ...) {
    static mut screen: pthread_mutex_t = pthread_mutex_t {
        __data: {
            let mut init = __pthread_mutex_s {
                __lock: 0,
                __count: 0,
                __owner: 0,
                __nusers: 0,
                __kind: PTHREAD_MUTEX_TIMED_NP as i32,
                __spins: 0,
                __elision: 0,
                __list: {
                    let mut init = __pthread_internal_list {
                        __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                        __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                    };
                    init
                },
            };
            init
        },
    };
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    pthread_mutex_lock(&mut screen);
    print!("\x1B[{};{}H", y + 1, x);
    vprintf(fmt, ap.as_va_list());
    print!("\x1B[{};{}H", 5 + 1, 1);
    fflush(stdout);
    pthread_mutex_unlock(&mut screen);
}

#[no_mangle]
pub extern "C" fn eat(mut id: i32) {
    let mut f: [i32; 2] = [0; 2];
    let mut ration: i32 = 0;
    let mut i: i32 = 0;
    f[1 as usize] = id;
    f[0 as usize] = f[1 as usize];
    f[(id & 1i32) as usize] = (id + 1) % 5;
    unsafe {
        print(id, 12, b"\x1B[K\0" as *const u8 as *const i8);
        print(
            id,
            12,
            b"..oO (forks, need forks)\0" as *const u8 as *const i8,
        );
    }
    i = 0;
    unsafe {
        while i < 2 {
            pthread_mutex_lock(forks.as_mut_ptr().offset(f[i as usize] as isize));
            if i == 0 {
                print(id, 12, b"\x1B[K\0" as *const u8 as *const i8);
            }
            print(
                id,
                12 + (f[i as usize] != id) as i32 * 6,
                b"fork%d\0" as *const u8 as *const i8,
                f[i as usize],
            );
            sleep(1);
            i += 1;
            i;
        }
    }
    i = 0;
    unsafe {
        ration = 3 + rand() % 8;
        while i < ration {
            print(id, 24 + i * 4, b"nom\0" as *const u8 as *const i8);
            sleep(1);
            i += 1;
            i;
        }
    }
    i = 0;
    unsafe {
        while i < 2 {
            pthread_mutex_unlock(forks.as_mut_ptr().offset(f[i as usize] as isize));
            i += 1;
            i;
        }
    }
}

#[no_mangle]
pub extern "C" fn think(mut id: i32) {
    let mut i: i32 = 0;
    let mut t: i32 = 0;
    let mut buf: [i8; 64] = [0; 64];
    unsafe {
        loop {
            print(id, 12, b"\x1B[K\0" as *const u8 as *const i8);
            t = rand() % 5;
            sprintf(
                buf.as_mut_ptr(),
                b"..oO (%s)\0" as *const u8 as *const i8,
                topic[t as usize],
            );
            i = 0;
            while buf[i as usize] != 0 {
                print(
                    id,
                    i + 12,
                    b"%c\0" as *const u8 as *const i8,
                    buf[i as usize] as i32,
                );
                if i < 5 {
                    usleep(200000);
                }
                i += 1;
                i;
            }
            usleep((500000 + rand() % 1000000i32) as u32);
            if !(t != 0) {
                break;
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn philosophize(mut a: *mut libc::c_void) -> *mut libc::c_void {
    unsafe {
        let mut id: i32 = *(a as *mut i32);
        print(
            id,
            1,
            b"%10s\0" as *const u8 as *const i8,
            names[id as usize],
        );
        loop {
            think(id);
            eat(id);
        }
    }
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    let mut id: [i32; 5] = [0; 5];
    let mut tid: [u64; 5] = [0; 5];
    i = 0;
    unsafe {
        while i < 5 {
            id[i as usize] = i;
            pthread_mutex_init(
                forks.as_mut_ptr().offset(id[i as usize] as isize),
                0 as *const pthread_mutexattr_t,
            );
            i += 1;
            i;
        }
    }
    i = 0;
    unsafe {
        while i < 5 {
            pthread_create(
                tid.as_mut_ptr().offset(i as isize),
                0 as *const pthread_attr_t,
                Some(philosophize as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
                id.as_mut_ptr().offset(i as isize) as *mut libc::c_void,
            );
            i += 1;
            i;
        }
        return pthread_join(tid[0 as usize], 0 as *mut *mut libc::c_void);
    }
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
