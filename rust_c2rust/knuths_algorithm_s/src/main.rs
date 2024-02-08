#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn rand() -> libc::c_int;
    fn srand(__seed: libc::c_uint);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn time(__timer: *mut time_t) -> time_t;
}
pub type size_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct s_env {
    pub n: libc::c_uint,
    pub i: libc::c_uint,
    pub size: size_t,
    pub sample: *mut libc::c_void,
}
#[no_mangle]
pub unsafe extern "C" fn s_of_n_init(
    mut s_env: *mut s_env,
    mut size: size_t,
    mut n: libc::c_uint,
) {
    (*s_env).i = 0 as libc::c_int as libc::c_uint;
    (*s_env).n = n;
    (*s_env).size = size;
    (*s_env).sample = malloc((n as libc::c_ulong).wrapping_mul(size));
}
#[no_mangle]
pub unsafe extern "C" fn sample_set_i(
    mut s_env: *mut s_env,
    mut i: libc::c_uint,
    mut item: *mut libc::c_void,
) {
    memcpy(
        ((*s_env).sample)
            .offset((i as libc::c_ulong).wrapping_mul((*s_env).size) as isize),
        item,
        (*s_env).size,
    );
}
#[no_mangle]
pub unsafe extern "C" fn s_of_n(
    mut s_env: *mut s_env,
    mut item: *mut libc::c_void,
) -> *mut libc::c_void {
    (*s_env).i = ((*s_env).i).wrapping_add(1);
    (*s_env).i;
    if (*s_env).i <= (*s_env).n {
        sample_set_i(
            s_env,
            ((*s_env).i).wrapping_sub(1 as libc::c_int as libc::c_uint),
            item,
        );
    } else if (rand() as libc::c_uint).wrapping_rem((*s_env).i) < (*s_env).n {
        sample_set_i(s_env, (rand() as libc::c_uint).wrapping_rem((*s_env).n), item);
    }
    return (*s_env).sample;
}
#[no_mangle]
pub unsafe extern "C" fn test(
    mut n: libc::c_uint,
    mut items_set: *mut libc::c_int,
    mut num_items: libc::c_uint,
) -> *mut libc::c_int {
    let mut i: libc::c_int = 0;
    let mut s_env: s_env = s_env {
        n: 0,
        i: 0,
        size: 0,
        sample: 0 as *mut libc::c_void,
    };
    s_of_n_init(&mut s_env, ::core::mem::size_of::<libc::c_int>() as libc::c_ulong, n);
    i = 0 as libc::c_int;
    while (i as libc::c_uint) < num_items {
        s_of_n(
            &mut s_env,
            &mut *items_set.offset(i as isize) as *mut libc::c_int as *mut libc::c_void,
        );
        i += 1;
        i;
    }
    return s_env.sample as *mut libc::c_int;
}
unsafe fn main_0() -> libc::c_int {
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut n: libc::c_uint = 3 as libc::c_int as libc::c_uint;
    let mut num_items: libc::c_uint = 10 as libc::c_int as libc::c_uint;
    let mut frequencies: *mut libc::c_uint = 0 as *mut libc::c_uint;
    let mut items_set: *mut libc::c_int = 0 as *mut libc::c_int;
    srand(time(0 as *mut time_t) as libc::c_uint);
    items_set = malloc(
        (num_items as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    frequencies = malloc(
        (num_items as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_uint;
    i = 0 as libc::c_int as libc::c_uint;
    while i < num_items {
        *items_set.offset(i as isize) = i as libc::c_int;
        *frequencies.offset(i as isize) = 0 as libc::c_int as libc::c_uint;
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < 100000 as libc::c_int as libc::c_uint {
        let mut res: *mut libc::c_int = test(n, items_set, num_items);
        j = 0 as libc::c_int as libc::c_uint;
        while j < n {
            let ref mut fresh0 = *frequencies.offset(*res.offset(j as isize) as isize);
            *fresh0 = (*fresh0).wrapping_add(1);
            *fresh0;
            j = j.wrapping_add(1);
            j;
        }
        free(res as *mut libc::c_void);
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < num_items {
        printf(
            b" %d\0" as *const u8 as *const libc::c_char,
            *frequencies.offset(i as isize),
        );
        i = i.wrapping_add(1);
        i;
    }
    puts(b"\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
