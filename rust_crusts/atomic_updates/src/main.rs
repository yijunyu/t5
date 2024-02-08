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
    fn rand() -> i32;
    fn sleep(__seconds: u32) -> u32;
    fn pthread_create(
        __newthread: *mut u64,
        __attr: *const pthread_attr_t,
        __start_routine: Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        __arg: *mut libc::c_void,
    ) -> i32;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> i32;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> i32;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> i32;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> i32;
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
#[no_mangle]
pub static mut bucket_mutex: [pthread_mutex_t; 15] = [pthread_mutex_t {
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
}; 15];
#[no_mangle]
pub static mut buckets: [i32; 15] = [0; 15];
#[no_mangle]
pub static mut equalizer: u64 = 0;
#[no_mangle]
pub static mut randomizer: u64 = 0;
#[no_mangle]
pub extern "C" fn transfer_value(mut from: i32, mut to: i32, mut howmuch: i32) {
    let mut swapped: bool = 0 != 0;
    if from == to || howmuch < 0 || from < 0 || to < 0 || from >= 15 || to >= 15 {
        return;
    }
    if from > to {
        let mut temp1: i32 = from;
        from = to;
        to = temp1;
        swapped = 1 != 0;
        howmuch = -howmuch;
    }
    unsafe {
        pthread_mutex_lock(&mut *bucket_mutex.as_mut_ptr().offset(from as isize));
        pthread_mutex_lock(&mut *bucket_mutex.as_mut_ptr().offset(to as isize));
        if howmuch > buckets[from as usize] && !swapped {
            howmuch = buckets[from as usize];
        }
        if -howmuch > buckets[to as usize] && swapped as i32 != 0 {
            howmuch = -buckets[to as usize];
        }
        buckets[from as usize] -= howmuch;
        buckets[to as usize] += howmuch;
        pthread_mutex_unlock(&mut *bucket_mutex.as_mut_ptr().offset(from as isize));
        pthread_mutex_unlock(&mut *bucket_mutex.as_mut_ptr().offset(to as isize));
    }
}

#[no_mangle]
pub extern "C" fn print_buckets() {
    let mut i: i32 = 0;
    let mut sum: i32 = 0;
    i = 0;
    unsafe {
        while i < 15 {
            pthread_mutex_lock(&mut *bucket_mutex.as_mut_ptr().offset(i as isize));
            i += 1;
            i;
        }
    }
    i = 0;
    unsafe {
        while i < 15 {
            print!("{:3} ", buckets[i as usize]);
            sum += buckets[i as usize];
            i += 1;
            i;
        }
    }
    print!("= {}\n", sum);
    i = 0;
    unsafe {
        while i < 15 {
            pthread_mutex_unlock(&mut *bucket_mutex.as_mut_ptr().offset(i as isize));
            i += 1;
            i;
        }
    }
}

#[no_mangle]
pub extern "C" fn equalizer_start(mut t: *mut libc::c_void) -> *mut libc::c_void {
    unsafe {
        loop {
            let mut b1: i32 = rand() % 15;
            let mut b2: i32 = rand() % 15;
            let mut diff: i32 = buckets[b1 as usize] - buckets[b2 as usize];
            if diff < 0 {
                transfer_value(b2, b1, -diff / 2);
            } else {
                transfer_value(b1, b2, diff / 2);
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn randomizer_start(mut t: *mut libc::c_void) -> *mut libc::c_void {
    unsafe {
        loop {
            let mut b1: i32 = rand() % 15;
            let mut b2: i32 = rand() % 15;
            let mut diff: i32 = rand() % (buckets[b1 as usize] + 1);
            transfer_value(b1, b2, diff);
        }
    }
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    let mut total: i32 = 0;
    i = 0;
    unsafe {
        while i < 15 {
            pthread_mutex_init(
                &mut *bucket_mutex.as_mut_ptr().offset(i as isize),
                0 as *const pthread_mutexattr_t,
            );
            i += 1;
            i;
        }
    }
    i = 0;
    unsafe {
        while i < 15 {
            buckets[i as usize] = rand() % 100;
            total += buckets[i as usize];
            print!("{:3} ", buckets[i as usize]);
            i += 1;
            i;
        }
    }
    print!("= {}\n", total);
    unsafe {
        pthread_create(
            &mut equalizer,
            0 as *const pthread_attr_t,
            Some(equalizer_start as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
            0 as *mut libc::c_void,
        );
        pthread_create(
            &mut randomizer,
            0 as *const pthread_attr_t,
            Some(randomizer_start as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
            0 as *mut libc::c_void,
        );
    }
    let mut i_0: i32 = 0;
    unsafe {
        while i_0 < 2 {
            sleep(1);
            print_buckets();
            i_0 += 1;
            i_0;
        }
    }
    i = 0;
    unsafe {
        while i < 15 {
            pthread_mutex_destroy(bucket_mutex.as_mut_ptr().offset(i as isize));
            i += 1;
            i;
        }
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
