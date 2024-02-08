#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_exit(__retval: *mut libc::c_void) -> !;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __atomic_wide_counter {
    pub __value64: libc::c_ulonglong,
    pub __value32: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
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
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub __wseq: __atomic_wide_counter,
    pub __g1_start: __atomic_wide_counter,
    pub __g_refs: [libc::c_uint; 2],
    pub __g_size: [libc::c_uint; 2],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2],
}
pub type pthread_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_longlong,
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed_0 = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed_0 = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed_0 = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed_0 = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed_0 = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed_0 = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed_0 = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed_0 = 0;
pub type threadfunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
>;
#[no_mangle]
pub static mut condm: pthread_mutex_t = pthread_mutex_t {
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
#[no_mangle]
pub static mut cond: pthread_cond_t = pthread_cond_t {
    __data: {
        let mut init = __pthread_cond_s {
            __wseq: __atomic_wide_counter {
                __value64: 0 as libc::c_int as libc::c_ulonglong,
            },
            __g1_start: __atomic_wide_counter {
                __value64: 0 as libc::c_int as libc::c_ulonglong,
            },
            __g_refs: [
                0 as libc::c_int as libc::c_uint,
                0 as libc::c_int as libc::c_uint,
            ],
            __g_size: [
                0 as libc::c_int as libc::c_uint,
                0 as libc::c_int as libc::c_uint,
            ],
            __g1_orig_size: 0 as libc::c_int as libc::c_uint,
            __wrefs: 0 as libc::c_int as libc::c_uint,
            __g_signals: [
                0 as libc::c_int as libc::c_uint,
                0 as libc::c_int as libc::c_uint,
            ],
        };
        init
    },
};
#[no_mangle]
pub static mut bang: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn t_enjoy(mut p: *mut libc::c_void) -> *mut libc::c_void {
    pthread_mutex_lock(&mut condm);
    while bang == 0 as libc::c_int {
        pthread_cond_wait(&mut cond, &mut condm);
    }
    pthread_mutex_unlock(&mut condm);
    printf(b"Enjoy\n\0" as *const u8 as *const libc::c_char);
    pthread_exit(0 as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn t_rosetta(mut p: *mut libc::c_void) -> *mut libc::c_void {
    pthread_mutex_lock(&mut condm);
    while bang == 0 as libc::c_int {
        pthread_cond_wait(&mut cond, &mut condm);
    }
    pthread_mutex_unlock(&mut condm);
    printf(b"Rosetta\n\0" as *const u8 as *const libc::c_char);
    pthread_exit(0 as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn t_code(mut p: *mut libc::c_void) -> *mut libc::c_void {
    pthread_mutex_lock(&mut condm);
    while bang == 0 as libc::c_int {
        pthread_cond_wait(&mut cond, &mut condm);
    }
    pthread_mutex_unlock(&mut condm);
    printf(b"Code\n\0" as *const u8 as *const libc::c_char);
    pthread_exit(0 as *mut libc::c_void);
}
unsafe fn main_0() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut a: [pthread_t; 3] = [0; 3];
    let mut p: [threadfunc; 3] = [
        Some(t_enjoy as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        Some(t_rosetta as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        Some(t_code as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
    ];
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        pthread_create(
            &mut *a.as_mut_ptr().offset(i as isize),
            0 as *const pthread_attr_t,
            p[i as usize],
            0 as *mut libc::c_void,
        );
        i += 1;
        i;
    }
    sleep(1 as libc::c_int as libc::c_uint);
    bang = 1 as libc::c_int;
    pthread_cond_broadcast(&mut cond);
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        pthread_join(a[i as usize], 0 as *mut *mut libc::c_void);
        i += 1;
        i;
    }
    return 0;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
