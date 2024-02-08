#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn perror(__s: *const libc::c_char);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn mmap(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: libc::c_int,
        __flags: libc::c_int,
        __fd: libc::c_int,
        __offset: __off_t,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type f_int = Option::<unsafe extern "C" fn() -> libc::c_int>;
#[no_mangle]
pub unsafe extern "C" fn _tmpl() -> libc::c_int {
    let mut x: libc::c_int = 0xdeadbeef as libc::c_uint as libc::c_int;
    return x * x;
}
#[no_mangle]
pub unsafe extern "C" fn dupf(mut v: libc::c_int) -> f_int {
    let mut len: size_t = (::core::mem::transmute::<
        Option::<unsafe extern "C" fn(libc::c_int) -> f_int>,
        *mut libc::c_void,
    >(Some(dupf as unsafe extern "C" fn(libc::c_int) -> f_int)))
        .offset_from(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn() -> libc::c_int>,
                *mut libc::c_void,
            >(
                Some(
                    ::core::mem::transmute::<
                        unsafe extern "C" fn() -> libc::c_int,
                        unsafe extern "C" fn() -> libc::c_int,
                    >(_tmpl),
                ),
            ),
        ) as libc::c_long as size_t;
    let mut ret: f_int = ::core::mem::transmute::<
        *mut libc::c_void,
        f_int,
    >(
        mmap(
            0 as *mut libc::c_void,
            len,
            0x4 as libc::c_int | 0x2 as libc::c_int,
            0x2 as libc::c_int | 0x20 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int as __off_t,
        ),
    );
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if ret
        == ::core::mem::transmute::<
            *mut libc::c_void,
            f_int,
        >(-(1 as libc::c_int) as *mut libc::c_void)
    {
        perror(b"mmap\0" as *const u8 as *const libc::c_char);
        exit(-(1 as libc::c_int));
    }
    memcpy(
        ::core::mem::transmute::<f_int, *mut libc::c_void>(ret),
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> libc::c_int>,
            *const libc::c_void,
        >(
            Some(
                ::core::mem::transmute::<
                    unsafe extern "C" fn() -> libc::c_int,
                    unsafe extern "C" fn() -> libc::c_int,
                >(_tmpl),
            ),
        ),
        len,
    );
    p = ::core::mem::transmute::<f_int, *mut libc::c_char>(ret);
    while p
        < (::core::mem::transmute::<f_int, *mut libc::c_char>(ret))
            .offset(len as isize)
            .offset(-(::core::mem::size_of::<libc::c_int>() as libc::c_ulong as isize))
    {
        if *(p as *mut libc::c_int) as libc::c_uint == 0xdeadbeef as libc::c_uint {
            *(p as *mut libc::c_int) = v;
        }
        p = p.offset(1);
        p;
    }
    return ret;
}
unsafe fn main_0() -> libc::c_int {
    let mut funcs: [f_int; 10] = [None; 10];
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        funcs[i as usize] = dupf(i);
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < 9 as libc::c_int {
        printf(
            b"func[%d]: %d\n\0" as *const u8 as *const libc::c_char,
            i,
            ::core::mem::transmute::<
                _,
                fn() -> libc::c_int,
            >((funcs[i as usize]).expect("non-null function pointer"))(),
        );
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
