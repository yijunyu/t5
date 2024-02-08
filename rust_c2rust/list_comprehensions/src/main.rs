#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(label_break_value)]
use ::c2rust_out::*;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct List {
    pub nx: *mut List,
    pub val: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Triple {
    pub _1: libc::c_int,
    pub _2: libc::c_int,
    pub _3: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ITERATOR {
    pub l: *mut List,
    pub old: *mut List,
    pub p: *mut ITERATOR,
}
#[no_mangle]
pub static mut FE_var: *mut ITERATOR = 0 as *const ITERATOR as *mut ITERATOR;
#[no_mangle]
pub static mut SEQ_base: ITERATOR = ITERATOR {
    l: 0 as *const List as *mut List,
    old: 0 as *const List as *mut List,
    p: 0 as *const ITERATOR as *mut ITERATOR,
};
#[no_mangle]
pub static mut SEQ_var: *mut ITERATOR = unsafe {
    &SEQ_base as *const ITERATOR as *mut ITERATOR
};
#[no_mangle]
pub unsafe extern "C" fn listNew(
    mut sz: libc::c_int,
    mut val: *mut libc::c_void,
) -> *mut List {
    let mut l: *mut List = malloc(
        (::core::mem::size_of::<List>() as libc::c_ulong)
            .wrapping_add(sz as libc::c_ulong),
    ) as *mut List;
    (*l).nx = 0 as *mut List;
    memcpy(((*l).val).as_mut_ptr() as *mut libc::c_void, val, sz as libc::c_ulong);
    return l;
}
#[no_mangle]
pub unsafe extern "C" fn listAppend(
    mut l: *mut List,
    mut sz: libc::c_int,
    mut val: *mut libc::c_void,
) -> *mut List {
    while !((*l).nx).is_null() {
        l = (*l).nx;
    }
    (*l).nx = listNew(sz, val);
    return l;
}
#[no_mangle]
pub unsafe extern "C" fn intRangeList(
    mut f: libc::c_int,
    mut t: libc::c_int,
) -> *mut List {
    let mut l: *mut List = listNew(
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        &mut f as *mut libc::c_int as *mut libc::c_void,
    );
    let mut e: *mut List = l;
    let mut i: libc::c_int = f + 1 as libc::c_int;
    while i <= t {
        (*e)
            .nx = listNew(
            ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
            &mut i as *mut libc::c_int as *mut libc::c_void,
        );
        e = (*e).nx;
        i += 1;
        i;
    }
    return l;
}
unsafe fn main_0() -> libc::c_int {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    let n: libc::c_int = 20 as libc::c_int;
    SEQ_var = &mut {
        let mut init = ITERATOR {
            l: 0 as *mut List,
            old: 0 as *mut List,
            p: SEQ_var,
        };
        init
    } as *mut ITERATOR;
    FE_var = &mut {
        let mut init = ITERATOR {
            l: (intRangeList
                as unsafe extern "C" fn(
                    libc::c_int,
                    libc::c_int,
                ) -> *mut List)(1 as libc::c_int, n),
            old: 0 as *mut List,
            p: FE_var,
        };
        init
    } as *mut ITERATOR;
    ::core::ptr::write_volatile(
        &mut x as *mut libc::c_int,
        *(&mut (*(*FE_var).l).val as *mut [libc::c_char; 0] as *mut libc::c_int),
    );
    while (if !((*FE_var).l).is_null() {
        ::core::ptr::write_volatile(
            &mut x as *mut libc::c_int,
            *(&mut (*(*FE_var).l).val as *mut [libc::c_char; 0] as *mut libc::c_int),
        );
        1 as libc::c_int
    } else {
        0 as libc::c_int
    }) != 0
    {
        FE_var = &mut {
            let mut init = ITERATOR {
                l: (intRangeList
                    as unsafe extern "C" fn(
                        libc::c_int,
                        libc::c_int,
                    ) -> *mut List)(x, n),
                old: 0 as *mut List,
                p: FE_var,
            };
            init
        } as *mut ITERATOR;
        ::core::ptr::write_volatile(
            &mut y as *mut libc::c_int,
            *(&mut (*(*FE_var).l).val as *mut [libc::c_char; 0] as *mut libc::c_int),
        );
        while (if !((*FE_var).l).is_null() {
            ::core::ptr::write_volatile(
                &mut y as *mut libc::c_int,
                *(&mut (*(*FE_var).l).val as *mut [libc::c_char; 0] as *mut libc::c_int),
            );
            1 as libc::c_int
        } else {
            0 as libc::c_int
        }) != 0
        {
            FE_var = &mut {
                let mut init = ITERATOR {
                    l: (intRangeList
                        as unsafe extern "C" fn(
                            libc::c_int,
                            libc::c_int,
                        ) -> *mut List)(y, n),
                    old: 0 as *mut List,
                    p: FE_var,
                };
                init
            } as *mut ITERATOR;
            ::core::ptr::write_volatile(
                &mut z as *mut libc::c_int,
                *(&mut (*(*FE_var).l).val as *mut [libc::c_char; 0] as *mut libc::c_int),
            );
            while (if !((*FE_var).l).is_null() {
                ::core::ptr::write_volatile(
                    &mut z as *mut libc::c_int,
                    *(&mut (*(*FE_var).l).val as *mut [libc::c_char; 0]
                        as *mut libc::c_int),
                );
                1 as libc::c_int
            } else {
                0 as libc::c_int
            }) != 0
            {
                if x * x + y * y == z * z {
                    if !((*SEQ_var).l).is_null() {
                        listAppend(
                            (*SEQ_var).l,
                            ::core::mem::size_of::<Triple>() as libc::c_ulong
                                as libc::c_int,
                            &mut {
                                let mut init = Triple { _1: x, _2: y, _3: z };
                                init
                            } as *mut Triple as *mut libc::c_void,
                        );
                    } else {
                        (*SEQ_var)
                            .l = listNew(
                            ::core::mem::size_of::<Triple>() as libc::c_ulong
                                as libc::c_int,
                            &mut {
                                let mut init = Triple { _1: x, _2: y, _3: z };
                                init
                            } as *mut Triple as *mut libc::c_void,
                        );
                    };
                } else {};
                (*FE_var).l = (*(*FE_var).l).nx;
            }
            FE_var = (*FE_var).p;
            (*FE_var).l = (*(*FE_var).l).nx;
        }
        FE_var = (*FE_var).p;
        (*FE_var).l = (*(*FE_var).l).nx;
    }
    FE_var = (*FE_var).p;
    (*(*SEQ_var).p).old = (*SEQ_var).l;
    SEQ_var = (*SEQ_var).p;
    let mut pTriples: *mut List = (*SEQ_var).old;
    let mut t: Triple = Triple { _1: 0, _2: 0, _3: 0 };
    FE_var = &mut {
        let mut init = ITERATOR {
            l: pTriples,
            old: 0 as *mut List,
            p: FE_var,
        };
        init
    } as *mut ITERATOR;
    ::core::ptr::write_volatile(
        &mut t as *mut Triple,
        *(&mut (*(*FE_var).l).val as *mut [libc::c_char; 0] as *mut Triple),
    );
    while (if !((*FE_var).l).is_null() {
        ::core::ptr::write_volatile(
            &mut t as *mut Triple,
            *(&mut (*(*FE_var).l).val as *mut [libc::c_char; 0] as *mut Triple),
        );
        1 as libc::c_int
    } else {
        0 as libc::c_int
    }) != 0
    {
        printf(b"%d, %d, %d\n\0" as *const u8 as *const libc::c_char, t._1, t._2, t._3);
        (*FE_var).l = (*(*FE_var).l).nx;
    }
    FE_var = (*FE_var).p;
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
