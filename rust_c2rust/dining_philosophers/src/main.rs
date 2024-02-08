#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(c_variadic, extern_types)]
use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vprintf(_: *const libc::c_char, _: ::core::ffi::VaList) -> libc::c_int;
    fn rand() -> libc::c_int;
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type size_t = libc::c_ulong;
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
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type C2RustUnnamed = libc::c_uint;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed = 0;
pub type va_list = __builtin_va_list;
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
pub static mut names: [*const libc::c_char; 5] = [
    b"Aristotle\0" as *const u8 as *const libc::c_char,
    b"Kant\0" as *const u8 as *const libc::c_char,
    b"Spinoza\0" as *const u8 as *const libc::c_char,
    b"Marx\0" as *const u8 as *const libc::c_char,
    b"Russell\0" as *const u8 as *const libc::c_char,
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
pub static mut topic: [*const libc::c_char; 5] = [
    b"Spaghetti!\0" as *const u8 as *const libc::c_char,
    b"Life\0" as *const u8 as *const libc::c_char,
    b"Universe\0" as *const u8 as *const libc::c_char,
    b"Everything\0" as *const u8 as *const libc::c_char,
    b"Bathroom\0" as *const u8 as *const libc::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn print(
    mut y: libc::c_int,
    mut x: libc::c_int,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    static mut screen: pthread_mutex_t = pthread_mutex_t {
        __data: {
            let mut init = __pthread_mutex_s {
                __lock: 0 as libc::c_int,
                __count: 0 as libc::c_int as libc::c_uint,
                __owner: 0 as libc::c_int,
                __nusers: 0 as libc::c_int as libc::c_uint,
                __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
                __spins: 0 as libc::c_int as libc::c_short,
                __elision: 0 as libc::c_int as libc::c_short,
                __list: {
                    let mut init = __pthread_internal_list {
                        __prev: 0 as *const __pthread_internal_list
                            as *mut __pthread_internal_list,
                        __next: 0 as *const __pthread_internal_list
                            as *mut __pthread_internal_list,
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
    printf(
        b"\x1B[%d;%dH\0" as *const u8 as *const libc::c_char,
        y + 1 as libc::c_int,
        x,
    );
    vprintf(fmt, ap.as_va_list());
    printf(
        b"\x1B[%d;%dH\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int + 1 as libc::c_int,
        1 as libc::c_int,
    );
    fflush(stdout);
    pthread_mutex_unlock(&mut screen);
}
#[no_mangle]
pub unsafe extern "C" fn eat(mut id: libc::c_int) {
    let mut f: [libc::c_int; 2] = [0; 2];
    let mut ration: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    f[1 as libc::c_int as usize] = id;
    f[0 as libc::c_int as usize] = f[1 as libc::c_int as usize];
    f[(id & 1 as libc::c_int) as usize] = (id + 1 as libc::c_int) % 5 as libc::c_int;
    print(id, 12 as libc::c_int, b"\x1B[K\0" as *const u8 as *const libc::c_char);
    print(
        id,
        12 as libc::c_int,
        b"..oO (forks, need forks)\0" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        pthread_mutex_lock(forks.as_mut_ptr().offset(f[i as usize] as isize));
        if i == 0 {
            print(
                id,
                12 as libc::c_int,
                b"\x1B[K\0" as *const u8 as *const libc::c_char,
            );
        }
        print(
            id,
            12 as libc::c_int + (f[i as usize] != id) as libc::c_int * 6 as libc::c_int,
            b"fork%d\0" as *const u8 as *const libc::c_char,
            f[i as usize],
        );
        sleep(1 as libc::c_int as libc::c_uint);
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    ration = 3 as libc::c_int + rand() % 8 as libc::c_int;
    while i < ration {
        print(
            id,
            24 as libc::c_int + i * 4 as libc::c_int,
            b"nom\0" as *const u8 as *const libc::c_char,
        );
        sleep(1 as libc::c_int as libc::c_uint);
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        pthread_mutex_unlock(forks.as_mut_ptr().offset(f[i as usize] as isize));
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn think(mut id: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut buf: [libc::c_char; 64] = [
        0 as libc::c_int as libc::c_char,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    loop {
        print(id, 12 as libc::c_int, b"\x1B[K\0" as *const u8 as *const libc::c_char);
        t = rand() % 5 as libc::c_int;
        sprintf(
            buf.as_mut_ptr(),
            b"..oO (%s)\0" as *const u8 as *const libc::c_char,
            topic[t as usize],
        );
        i = 0 as libc::c_int;
        while buf[i as usize] != 0 {
            print(
                id,
                i + 12 as libc::c_int,
                b"%c\0" as *const u8 as *const libc::c_char,
                buf[i as usize] as libc::c_int,
            );
            if i < 5 as libc::c_int {
                usleep(200000 as libc::c_int as __useconds_t);
            }
            i += 1;
            i;
        }
        usleep(
            (500000 as libc::c_int + rand() % 1000000 as libc::c_int) as __useconds_t,
        );
        if !(t != 0) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn philosophize(mut a: *mut libc::c_void) -> *mut libc::c_void {
    let mut id: libc::c_int = *(a as *mut libc::c_int);
    print(
        id,
        1 as libc::c_int,
        b"%10s\0" as *const u8 as *const libc::c_char,
        names[id as usize],
    );
    loop {
        think(id);
        eat(id);
    };
}
unsafe fn main_0() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut id: [libc::c_int; 5] = [0; 5];
    let mut tid: [pthread_t; 5] = [0; 5];
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        id[i as usize] = i;
        pthread_mutex_init(
            forks.as_mut_ptr().offset(id[i as usize] as isize),
            0 as *const pthread_mutexattr_t,
        );
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        pthread_create(
            tid.as_mut_ptr().offset(i as isize),
            0 as *const pthread_attr_t,
            Some(
                philosophize
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            id.as_mut_ptr().offset(i as isize) as *mut libc::c_void,
        );
        i += 1;
        i;
    }
    return pthread_join(tid[0 as libc::c_int as usize], 0 as *mut *mut libc::c_void);
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
