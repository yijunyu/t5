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
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn sqrt(_: f64) -> f64;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quaternion {
    pub q: [f64; 4],
}
pub type quaternion_t = quaternion;
#[no_mangle]
pub extern "C" fn quaternion_new() -> *mut quaternion_t {
    unsafe {
        return malloc(::core::mem::size_of::<quaternion_t>() as u64) as *mut quaternion_t;
    }
}

#[no_mangle]
pub extern "C" fn quaternion_new_set(
    mut q1: f64,
    mut q2: f64,
    mut q3: f64,
    mut q4: f64,
) -> *mut quaternion_t {
    unsafe {
        let mut q: *mut quaternion_t =
            malloc(::core::mem::size_of::<quaternion_t>() as u64) as *mut quaternion_t;
        if !q.is_null() {
            (*q).q[0 as usize] = q1;
            (*q).q[1 as usize] = q2;
            (*q).q[2 as usize] = q3;
            (*q).q[3 as usize] = q4;
        }
        return q;
    }
}

#[no_mangle]
pub extern "C" fn quaternion_copy(mut r: *mut quaternion_t, mut q: *mut quaternion_t) {
    unsafe {
        let mut i: u64 = 0;
        if r.is_null() || q.is_null() {
            return;
        }
        i = 0;
        while i < 4 {
            (*r).q[i as usize] = (*q).q[i as usize];
            i = i.wrapping_add(1);
            i;
        }
    }
}

#[no_mangle]
pub extern "C" fn quaternion_norm(mut q: *mut quaternion_t) -> f64 {
    unsafe {
        let mut i: u64 = 0;
        let mut r: f64 = 0.0f64;
        if q.is_null() {
            fprintf(
                stderr,
                b"NULL quaternion in norm\n\0" as *const u8 as *const i8,
            );
            return 0.0f64;
        }
        i = 0;
        while i < 4 {
            r += (*q).q[i as usize] * (*q).q[i as usize];
            i = i.wrapping_add(1);
            i;
        }
        return sqrt(r);
    }
}

#[no_mangle]
pub extern "C" fn quaternion_neg(mut r: *mut quaternion_t, mut q: *mut quaternion_t) {
    unsafe {
        let mut i: u64 = 0;
        if q.is_null() || r.is_null() {
            return;
        }
        i = 0;
        while i < 4 {
            (*r).q[i as usize] = -(*q).q[i as usize];
            i = i.wrapping_add(1);
            i;
        }
    }
}

#[no_mangle]
pub extern "C" fn quaternion_conj(mut r: *mut quaternion_t, mut q: *mut quaternion_t) {
    unsafe {
        let mut i: u64 = 0;
        if q.is_null() || r.is_null() {
            return;
        };
        (*r).q[0 as usize] = (*q).q[0 as usize];
        i = 1;
        while i < 4 {
            (*r).q[i as usize] = -(*q).q[i as usize];
            i = i.wrapping_add(1);
            i;
        }
    }
}

#[no_mangle]
pub extern "C" fn quaternion_add_d(mut r: *mut quaternion_t, mut q: *mut quaternion_t, mut d: f64) {
    unsafe {
        if q.is_null() || r.is_null() {
            return;
        }
        quaternion_copy(r, q);
        (*r).q[0 as usize] += d;
    }
}

#[no_mangle]
pub extern "C" fn quaternion_add(
    mut r: *mut quaternion_t,
    mut a: *mut quaternion_t,
    mut b: *mut quaternion_t,
) {
    unsafe {
        let mut i: u64 = 0;
        if r.is_null() || a.is_null() || b.is_null() {
            return;
        }
        i = 0;
        while i < 4 {
            (*r).q[i as usize] = (*a).q[i as usize] + (*b).q[i as usize];
            i = i.wrapping_add(1);
            i;
        }
    }
}

#[no_mangle]
pub extern "C" fn quaternion_mul_d(mut r: *mut quaternion_t, mut q: *mut quaternion_t, mut d: f64) {
    unsafe {
        let mut i: u64 = 0;
        if r.is_null() || q.is_null() {
            return;
        }
        i = 0;
        while i < 4 {
            (*r).q[i as usize] = (*q).q[i as usize] * d;
            i = i.wrapping_add(1);
            i;
        }
    }
}

#[no_mangle]
pub extern "C" fn quaternion_equal(mut a: *mut quaternion_t, mut b: *mut quaternion_t) -> bool {
    unsafe {
        let mut i: u64 = 0;
        i = 0;
        while i < 4 {
            if (*a).q[i as usize] != (*b).q[i as usize] {
                return 0 != 0;
            }
            i = i.wrapping_add(1);
            i;
        }
        return 1 != 0;
    }
}

#[no_mangle]
pub extern "C" fn quaternion_mul(
    mut r: *mut quaternion_t,
    mut a: *mut quaternion_t,
    mut b: *mut quaternion_t,
) {
    unsafe {
        let mut i: u64 = 0;
        let mut ri: f64 = 0.0f64;
        if r.is_null() || a.is_null() || b.is_null() {
            return;
        };
        (*r).q[0 as usize] = (*a).q[0 as usize] * (*b).q[0 as usize]
            - (*a).q[1 as usize] * (*b).q[1 as usize]
            - (*a).q[2 as usize] * (*b).q[2 as usize]
            - (*a).q[3 as usize] * (*b).q[3 as usize];
        (*r).q[1 as usize] = (*a).q[0 as usize] * (*b).q[1 as usize]
            + (*a).q[1 as usize] * (*b).q[0 as usize]
            + (*a).q[2 as usize] * (*b).q[3 as usize]
            - (*a).q[3 as usize] * (*b).q[2 as usize];
        (*r).q[2 as usize] = (*a).q[0 as usize] * (*b).q[2 as usize]
            - (*a).q[1 as usize] * (*b).q[3 as usize]
            + (*a).q[2 as usize] * (*b).q[0 as usize]
            + (*a).q[3 as usize] * (*b).q[1 as usize];
        (*r).q[3 as usize] = (*a).q[0 as usize] * (*b).q[3 as usize]
            + (*a).q[1 as usize] * (*b).q[2 as usize]
            - (*a).q[2 as usize] * (*b).q[1 as usize]
            + (*a).q[3 as usize] * (*b).q[0 as usize];
    }
}

#[no_mangle]
pub extern "C" fn quaternion_print(mut q: *mut quaternion_t) {
    unsafe {
        if q.is_null() {
            return;
        }
    }
    print!(
        "({}, {}, {}, {})\n",
        (*q).q[0 as usize],
        (*q).q[1 as usize],
        (*q).q[2 as usize],
        (*q).q[3 as usize]
    );
}

fn main_0() -> i32 {
    unsafe {
        let mut i: u64 = 0;
        let mut d: f64 = 7.0f64;
        let mut q: [*mut quaternion_t; 3] = [0 as *mut quaternion_t; 3];
        let mut r: *mut quaternion_t = quaternion_new();
        let mut qd: *mut quaternion_t = quaternion_new_set(7.0f64, 0.0f64, 0.0f64, 0.0f64);
        q[0 as usize] = quaternion_new_set(1.0f64, 2.0f64, 3.0f64, 4.0f64);
        q[1 as usize] = quaternion_new_set(2.0f64, 3.0f64, 4.0f64, 5.0f64);
        q[2 as usize] = quaternion_new_set(3.0f64, 4.0f64, 5.0f64, 6.0f64);
        print!("r = {}\n", d);
        i = 0;
        while i < 3 {
            print!("q[{}] = ", i);
            quaternion_print(q[i as usize]);
            print!("abs q[{}] = {}\n", i, quaternion_norm(q[i as usize]));
            i = i.wrapping_add(1);
            i;
        }
        print!("-q[0] = ");
        quaternion_neg(r, q[0 as usize]);
        quaternion_print(r);
        print!("conj q[0] = ");
        quaternion_conj(r, q[0 as usize]);
        quaternion_print(r);
        print!("q[1] + q[2] = ");
        quaternion_add(r, q[1 as usize], q[2 as usize]);
        quaternion_print(r);
        print!("q[2] + q[1] = ");
        quaternion_add(r, q[2 as usize], q[1 as usize]);
        quaternion_print(r);
        print!("q[0] * r = ");
        quaternion_mul_d(r, q[0 as usize], d);
        quaternion_print(r);
        print!("q[0] * (r, 0, 0, 0) = ");
        quaternion_mul(r, q[0 as usize], qd);
        quaternion_print(r);
        print!("q[1] * q[2] = ");
        quaternion_mul(r, q[1 as usize], q[2 as usize]);
        quaternion_print(r);
        print!("q[2] * q[1] = ");
        quaternion_mul(r, q[2 as usize], q[1 as usize]);
        quaternion_print(r);
        free(q[0 as usize] as *mut libc::c_void);
        free(q[1 as usize] as *mut libc::c_void);
        free(q[2 as usize] as *mut libc::c_void);
        free(r as *mut libc::c_void);
        return 0;
    }
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
