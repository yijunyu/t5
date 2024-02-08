#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
}
pub type size_t = libc::c_ulong;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct item {
    pub w: libc::c_double,
    pub v: libc::c_double,
    pub name: *const libc::c_char,
}
#[no_mangle]
pub static mut items: [item; 9] = [
    {
        let mut init = item {
            w: 3.8f64,
            v: 36 as libc::c_int as libc::c_double,
            name: b"beef\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = item {
            w: 5.4f64,
            v: 43 as libc::c_int as libc::c_double,
            name: b"pork\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = item {
            w: 3.6f64,
            v: 90 as libc::c_int as libc::c_double,
            name: b"ham\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = item {
            w: 2.4f64,
            v: 45 as libc::c_int as libc::c_double,
            name: b"greaves\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = item {
            w: 4.0f64,
            v: 30 as libc::c_int as libc::c_double,
            name: b"flitch\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = item {
            w: 2.5f64,
            v: 56 as libc::c_int as libc::c_double,
            name: b"brawn\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = item {
            w: 3.7f64,
            v: 67 as libc::c_int as libc::c_double,
            name: b"welt\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = item {
            w: 3.0f64,
            v: 95 as libc::c_int as libc::c_double,
            name: b"salami\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = item {
            w: 5.9f64,
            v: 98 as libc::c_int as libc::c_double,
            name: b"sausage\0" as *const u8 as *const libc::c_char,
        };
        init
    },
];
#[no_mangle]
pub unsafe extern "C" fn item_cmp(
    mut aa: *const libc::c_void,
    mut bb: *const libc::c_void,
) -> libc::c_int {
    let mut a: *const item = aa as *const item;
    let mut b: *const item = bb as *const item;
    let mut ua: libc::c_double = (*a).v / (*a).w;
    let mut ub: libc::c_double = (*b).v / (*b).w;
    return if ua < ub { -(1 as libc::c_int) } else { (ua > ub) as libc::c_int };
}
unsafe fn main_0() -> libc::c_int {
    let mut it: *mut item = 0 as *mut item;
    let mut space: libc::c_double = 15 as libc::c_int as libc::c_double;
    qsort(
        items.as_mut_ptr() as *mut libc::c_void,
        9 as libc::c_int as size_t,
        ::core::mem::size_of::<item>() as libc::c_ulong,
        Some(
            item_cmp
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    it = items.as_mut_ptr().offset(9 as libc::c_int as isize);
    loop {
        let fresh0 = it;
        it = it.offset(-1);
        if !(fresh0.offset_from(items.as_mut_ptr()) as libc::c_long != 0
            && space > 0 as libc::c_int as libc::c_double)
        {
            break;
        }
        if space >= (*it).w {
            printf(b"take all %s\n\0" as *const u8 as *const libc::c_char, (*it).name);
        } else {
            printf(
                b"take %gkg of %g kg of %s\n\0" as *const u8 as *const libc::c_char,
                space,
                (*it).w,
                (*it).name,
            );
        }
        space -= (*it).w;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
