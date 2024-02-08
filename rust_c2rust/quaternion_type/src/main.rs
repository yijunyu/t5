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
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn sqrt(_: libc::c_double) -> libc::c_double;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quaternion {
    pub q: [libc::c_double; 4],
}
pub type quaternion_t = quaternion;
#[no_mangle]
pub unsafe extern "C" fn quaternion_new() -> *mut quaternion_t {
    return malloc(::core::mem::size_of::<quaternion_t>() as libc::c_ulong)
        as *mut quaternion_t;
}
#[no_mangle]
pub unsafe extern "C" fn quaternion_new_set(
    mut q1: libc::c_double,
    mut q2: libc::c_double,
    mut q3: libc::c_double,
    mut q4: libc::c_double,
) -> *mut quaternion_t {
    let mut q: *mut quaternion_t = malloc(
        ::core::mem::size_of::<quaternion_t>() as libc::c_ulong,
    ) as *mut quaternion_t;
    if !q.is_null() {
        (*q).q[0 as libc::c_int as usize] = q1;
        (*q).q[1 as libc::c_int as usize] = q2;
        (*q).q[2 as libc::c_int as usize] = q3;
        (*q).q[3 as libc::c_int as usize] = q4;
    }
    return q;
}
#[no_mangle]
pub unsafe extern "C" fn quaternion_copy(
    mut r: *mut quaternion_t,
    mut q: *mut quaternion_t,
) {
    let mut i: size_t = 0;
    if r.is_null() || q.is_null() {
        return;
    }
    i = 0 as libc::c_int as size_t;
    while i < 4 as libc::c_int as libc::c_ulong {
        (*r).q[i as usize] = (*q).q[i as usize];
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn quaternion_norm(mut q: *mut quaternion_t) -> libc::c_double {
    let mut i: size_t = 0;
    let mut r: libc::c_double = 0.0f64;
    if q.is_null() {
        fprintf(
            stderr,
            b"NULL quaternion in norm\n\0" as *const u8 as *const libc::c_char,
        );
        return 0.0f64;
    }
    i = 0 as libc::c_int as size_t;
    while i < 4 as libc::c_int as libc::c_ulong {
        r += (*q).q[i as usize] * (*q).q[i as usize];
        i = i.wrapping_add(1);
        i;
    }
    return sqrt(r);
}
#[no_mangle]
pub unsafe extern "C" fn quaternion_neg(
    mut r: *mut quaternion_t,
    mut q: *mut quaternion_t,
) {
    let mut i: size_t = 0;
    if q.is_null() || r.is_null() {
        return;
    }
    i = 0 as libc::c_int as size_t;
    while i < 4 as libc::c_int as libc::c_ulong {
        (*r).q[i as usize] = -(*q).q[i as usize];
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn quaternion_conj(
    mut r: *mut quaternion_t,
    mut q: *mut quaternion_t,
) {
    let mut i: size_t = 0;
    if q.is_null() || r.is_null() {
        return;
    }
    (*r).q[0 as libc::c_int as usize] = (*q).q[0 as libc::c_int as usize];
    i = 1 as libc::c_int as size_t;
    while i < 4 as libc::c_int as libc::c_ulong {
        (*r).q[i as usize] = -(*q).q[i as usize];
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn quaternion_add_d(
    mut r: *mut quaternion_t,
    mut q: *mut quaternion_t,
    mut d: libc::c_double,
) {
    if q.is_null() || r.is_null() {
        return;
    }
    quaternion_copy(r, q);
    (*r).q[0 as libc::c_int as usize] += d;
}
#[no_mangle]
pub unsafe extern "C" fn quaternion_add(
    mut r: *mut quaternion_t,
    mut a: *mut quaternion_t,
    mut b: *mut quaternion_t,
) {
    let mut i: size_t = 0;
    if r.is_null() || a.is_null() || b.is_null() {
        return;
    }
    i = 0 as libc::c_int as size_t;
    while i < 4 as libc::c_int as libc::c_ulong {
        (*r).q[i as usize] = (*a).q[i as usize] + (*b).q[i as usize];
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn quaternion_mul_d(
    mut r: *mut quaternion_t,
    mut q: *mut quaternion_t,
    mut d: libc::c_double,
) {
    let mut i: size_t = 0;
    if r.is_null() || q.is_null() {
        return;
    }
    i = 0 as libc::c_int as size_t;
    while i < 4 as libc::c_int as libc::c_ulong {
        (*r).q[i as usize] = (*q).q[i as usize] * d;
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn quaternion_equal(
    mut a: *mut quaternion_t,
    mut b: *mut quaternion_t,
) -> bool {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < 4 as libc::c_int as libc::c_ulong {
        if (*a).q[i as usize] != (*b).q[i as usize] {
            return 0 as libc::c_int != 0;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn quaternion_mul(
    mut r: *mut quaternion_t,
    mut a: *mut quaternion_t,
    mut b: *mut quaternion_t,
) {
    let mut i: size_t = 0;
    let mut ri: libc::c_double = 0.0f64;
    if r.is_null() || a.is_null() || b.is_null() {
        return;
    }
    (*r)
        .q[0 as libc::c_int
        as usize] = (*a).q[0 as libc::c_int as usize] * (*b).q[0 as libc::c_int as usize]
        - (*a).q[1 as libc::c_int as usize] * (*b).q[1 as libc::c_int as usize]
        - (*a).q[2 as libc::c_int as usize] * (*b).q[2 as libc::c_int as usize]
        - (*a).q[3 as libc::c_int as usize] * (*b).q[3 as libc::c_int as usize];
    (*r)
        .q[1 as libc::c_int
        as usize] = (*a).q[0 as libc::c_int as usize] * (*b).q[1 as libc::c_int as usize]
        + (*a).q[1 as libc::c_int as usize] * (*b).q[0 as libc::c_int as usize]
        + (*a).q[2 as libc::c_int as usize] * (*b).q[3 as libc::c_int as usize]
        - (*a).q[3 as libc::c_int as usize] * (*b).q[2 as libc::c_int as usize];
    (*r)
        .q[2 as libc::c_int
        as usize] = (*a).q[0 as libc::c_int as usize] * (*b).q[2 as libc::c_int as usize]
        - (*a).q[1 as libc::c_int as usize] * (*b).q[3 as libc::c_int as usize]
        + (*a).q[2 as libc::c_int as usize] * (*b).q[0 as libc::c_int as usize]
        + (*a).q[3 as libc::c_int as usize] * (*b).q[1 as libc::c_int as usize];
    (*r)
        .q[3 as libc::c_int
        as usize] = (*a).q[0 as libc::c_int as usize] * (*b).q[3 as libc::c_int as usize]
        + (*a).q[1 as libc::c_int as usize] * (*b).q[2 as libc::c_int as usize]
        - (*a).q[2 as libc::c_int as usize] * (*b).q[1 as libc::c_int as usize]
        + (*a).q[3 as libc::c_int as usize] * (*b).q[0 as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn quaternion_print(mut q: *mut quaternion_t) {
    if q.is_null() {
        return;
    }
    printf(
        b"(%lf, %lf, %lf, %lf)\n\0" as *const u8 as *const libc::c_char,
        (*q).q[0 as libc::c_int as usize],
        (*q).q[1 as libc::c_int as usize],
        (*q).q[2 as libc::c_int as usize],
        (*q).q[3 as libc::c_int as usize],
    );
}
unsafe fn main_0() -> libc::c_int {
    let mut i: size_t = 0;
    let mut d: libc::c_double = 7.0f64;
    let mut q: [*mut quaternion_t; 3] = [0 as *mut quaternion_t; 3];
    let mut r: *mut quaternion_t = quaternion_new();
    let mut qd: *mut quaternion_t = quaternion_new_set(7.0f64, 0.0f64, 0.0f64, 0.0f64);
    q[0 as libc::c_int as usize] = quaternion_new_set(1.0f64, 2.0f64, 3.0f64, 4.0f64);
    q[1 as libc::c_int as usize] = quaternion_new_set(2.0f64, 3.0f64, 4.0f64, 5.0f64);
    q[2 as libc::c_int as usize] = quaternion_new_set(3.0f64, 4.0f64, 5.0f64, 6.0f64);
    printf(b"r = %lf\n\0" as *const u8 as *const libc::c_char, d);
    i = 0 as libc::c_int as size_t;
    while i < 3 as libc::c_int as libc::c_ulong {
        printf(b"q[%u] = \0" as *const u8 as *const libc::c_char, i);
        quaternion_print(q[i as usize]);
        printf(
            b"abs q[%u] = %lf\n\0" as *const u8 as *const libc::c_char,
            i,
            quaternion_norm(q[i as usize]),
        );
        i = i.wrapping_add(1);
        i;
    }
    printf(b"-q[0] = \0" as *const u8 as *const libc::c_char);
    quaternion_neg(r, q[0 as libc::c_int as usize]);
    quaternion_print(r);
    printf(b"conj q[0] = \0" as *const u8 as *const libc::c_char);
    quaternion_conj(r, q[0 as libc::c_int as usize]);
    quaternion_print(r);
    printf(b"q[1] + q[2] = \0" as *const u8 as *const libc::c_char);
    quaternion_add(r, q[1 as libc::c_int as usize], q[2 as libc::c_int as usize]);
    quaternion_print(r);
    printf(b"q[2] + q[1] = \0" as *const u8 as *const libc::c_char);
    quaternion_add(r, q[2 as libc::c_int as usize], q[1 as libc::c_int as usize]);
    quaternion_print(r);
    printf(b"q[0] * r = \0" as *const u8 as *const libc::c_char);
    quaternion_mul_d(r, q[0 as libc::c_int as usize], d);
    quaternion_print(r);
    printf(b"q[0] * (r, 0, 0, 0) = \0" as *const u8 as *const libc::c_char);
    quaternion_mul(r, q[0 as libc::c_int as usize], qd);
    quaternion_print(r);
    printf(b"q[1] * q[2] = \0" as *const u8 as *const libc::c_char);
    quaternion_mul(r, q[1 as libc::c_int as usize], q[2 as libc::c_int as usize]);
    quaternion_print(r);
    printf(b"q[2] * q[1] = \0" as *const u8 as *const libc::c_char);
    quaternion_mul(r, q[2 as libc::c_int as usize], q[1 as libc::c_int as usize]);
    quaternion_print(r);
    free(q[0 as libc::c_int as usize] as *mut libc::c_void);
    free(q[1 as libc::c_int as usize] as *mut libc::c_void);
    free(q[2 as libc::c_int as usize] as *mut libc::c_void);
    free(r as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
