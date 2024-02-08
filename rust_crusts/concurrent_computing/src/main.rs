#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use c2rust_out::*;
extern "C" {
    fn sleep(__seconds: u32) -> u32;
    fn pthread_create(
        __newthread: *mut u64,
        __attr: *const pthread_attr_t,
        __start_routine: Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        __arg: *mut libc::c_void,
    ) -> i32;
    fn pthread_exit(__retval: *mut libc::c_void) -> !;
    fn pthread_join(__th: u64, __thread_return: *mut *mut libc::c_void) -> i32;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> i32;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> i32;
    fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> i32;
    fn pthread_cond_wait(__cond: *mut pthread_cond_t, __mutex: *mut pthread_mutex_t) -> i32;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __atomic_wide_counter {
    pub __value64: u64,
    pub __value32: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub __low: u32,
    pub __high: u32,
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
pub struct __pthread_cond_s {
    pub __wseq: __atomic_wide_counter,
    pub __g1_start: __atomic_wide_counter,
    pub __g_refs: [u32; 2],
    pub __g_size: [u32; 2],
    pub __g1_orig_size: u32,
    pub __wrefs: u32,
    pub __g_signals: [u32; 2],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [i8; 48],
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
pub type threadfunc = Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>;
#[no_mangle]
pub static mut condm: pthread_mutex_t = pthread_mutex_t {
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
#[no_mangle]
pub static mut cond: pthread_cond_t = pthread_cond_t {
    __data: {
        let mut init = __pthread_cond_s {
            __wseq: __atomic_wide_counter { __value64: 0 },
            __g1_start: __atomic_wide_counter { __value64: 0 },
            __g_refs: [0; 2],
            __g_size: [0; 2],
            __g1_orig_size: 0,
            __wrefs: 0,
            __g_signals: [0; 2],
        };
        init
    },
};
#[no_mangle]
pub static mut bang: i32 = 0;
#[no_mangle]
pub extern "C" fn t_enjoy(mut p: *mut libc::c_void) -> *mut libc::c_void {
    unsafe {
        pthread_mutex_lock(&mut condm);
        while bang == 0 {
            pthread_cond_wait(&mut cond, &mut condm);
        }
        pthread_mutex_unlock(&mut condm);
        print!("Enjoy\n");
        pthread_exit(0 as *mut libc::c_void);
    }
}

#[no_mangle]
pub extern "C" fn t_rosetta(mut p: *mut libc::c_void) -> *mut libc::c_void {
    unsafe {
        pthread_mutex_lock(&mut condm);
        while bang == 0 {
            pthread_cond_wait(&mut cond, &mut condm);
        }
        pthread_mutex_unlock(&mut condm);
        print!("Rosetta\n");
        pthread_exit(0 as *mut libc::c_void);
    }
}

#[no_mangle]
pub extern "C" fn t_code(mut p: *mut libc::c_void) -> *mut libc::c_void {
    unsafe {
        pthread_mutex_lock(&mut condm);
        while bang == 0 {
            pthread_cond_wait(&mut cond, &mut condm);
        }
        pthread_mutex_unlock(&mut condm);
        print!("Code\n");
        pthread_exit(0 as *mut libc::c_void);
    }
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    let mut a: [u64; 3] = [0; 3];
    unsafe {
        let mut p: [threadfunc; 3] = [
            Some(t_enjoy as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
            Some(t_rosetta as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
            Some(t_code as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        ];
        i = 0;
        while i < 3 {
            pthread_create(
                &mut *a.as_mut_ptr().offset(i as isize),
                0 as *const pthread_attr_t,
                p[i as usize],
                0 as *mut libc::c_void,
            );
            i += 1;
            i;
        }
        sleep(1);
        bang = 1;
        pthread_cond_broadcast(&mut cond);
    }
    i = 0;
    unsafe {
        while i < 3 {
            pthread_join(a[i as usize], 0 as *mut *mut libc::c_void);
            i += 1;
            i;
        }
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
