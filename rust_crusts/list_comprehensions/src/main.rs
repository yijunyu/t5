#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(label_break_value)]
use c2rust_out::*;
extern "C" {
    fn malloc(_: u64) -> *mut libc::c_void;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct List {
    pub nx: *mut List,
    pub val: [i8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Triple {
    pub _1: i32,
    pub _2: i32,
    pub _3: i32,
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
pub static mut SEQ_var: *mut ITERATOR = unsafe { &SEQ_base as *const ITERATOR as *mut ITERATOR };
#[no_mangle]
pub extern "C" fn listNew(mut sz: i32, mut val: *mut libc::c_void) -> *mut List {
    unsafe {
        let mut l: *mut List =
            malloc((::core::mem::size_of::<List>() as u64).wrapping_add(sz as u64)) as *mut List;
        (*l).nx = 0 as *mut List;
        memcpy(((*l).val).as_mut_ptr() as *mut libc::c_void, val, sz as u64);
        return l;
    }
}

#[no_mangle]
pub extern "C" fn listAppend(
    mut l: *mut List,
    mut sz: i32,
    mut val: *mut libc::c_void,
) -> *mut List {
    unsafe {
        while !((*l).nx).is_null() {
            l = (*l).nx;
        }
        (*l).nx = listNew(sz, val);
        return l;
    }
}

#[no_mangle]
pub extern "C" fn intRangeList(mut f: i32, mut t: i32) -> *mut List {
    unsafe {
        let mut l: *mut List = listNew(
            ::core::mem::size_of::<i32>() as i32,
            &mut f as *mut i32 as *mut libc::c_void,
        );
        let mut e: *mut List = l;
        let mut i: i32 = f + 1;
        while i <= t {
            (*e).nx = listNew(
                ::core::mem::size_of::<i32>() as i32,
                &mut i as *mut i32 as *mut libc::c_void,
            );
            e = (*e).nx;
            i += 1;
            i;
        }
        return l;
    }
}

fn main_0() -> i32 {
    unsafe {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut z: i32 = 0;
        let n: i32 = 20;
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
                l: (intRangeList as unsafe extern "C" fn(i32, i32) -> *mut List)(1, n),
                old: 0 as *mut List,
                p: FE_var,
            };
            init
        } as *mut ITERATOR;
        ::core::ptr::write_volatile(
            &mut x as *mut i32,
            *(&mut (*(*FE_var).l).val as *mut [i8; 0] as *mut i32),
        );
        while (if !((*FE_var).l).is_null() {
            ::core::ptr::write_volatile(
                &mut x as *mut i32,
                *(&mut (*(*FE_var).l).val as *mut [i8; 0] as *mut i32),
            );
            1
        } else {
            0
        }) != 0
        {
            FE_var = &mut {
                let mut init = ITERATOR {
                    l: (intRangeList as unsafe extern "C" fn(i32, i32) -> *mut List)(x, n),
                    old: 0 as *mut List,
                    p: FE_var,
                };
                init
            } as *mut ITERATOR;
            ::core::ptr::write_volatile(
                &mut y as *mut i32,
                *(&mut (*(*FE_var).l).val as *mut [i8; 0] as *mut i32),
            );
            while (if !((*FE_var).l).is_null() {
                ::core::ptr::write_volatile(
                    &mut y as *mut i32,
                    *(&mut (*(*FE_var).l).val as *mut [i8; 0] as *mut i32),
                );
                1
            } else {
                0
            }) != 0
            {
                FE_var = &mut {
                    let mut init = ITERATOR {
                        l: (intRangeList as unsafe extern "C" fn(i32, i32) -> *mut List)(y, n),
                        old: 0 as *mut List,
                        p: FE_var,
                    };
                    init
                } as *mut ITERATOR;
                ::core::ptr::write_volatile(
                    &mut z as *mut i32,
                    *(&mut (*(*FE_var).l).val as *mut [i8; 0] as *mut i32),
                );
                while (if !((*FE_var).l).is_null() {
                    ::core::ptr::write_volatile(
                        &mut z as *mut i32,
                        *(&mut (*(*FE_var).l).val as *mut [i8; 0] as *mut i32),
                    );
                    1
                } else {
                    0
                }) != 0
                {
                    if x * x + y * y == z * z {
                        if !((*SEQ_var).l).is_null() {
                            listAppend(
                                (*SEQ_var).l,
                                ::core::mem::size_of::<Triple>() as i32,
                                &mut {
                                    let mut init = Triple {
                                        _1: x,
                                        _2: y,
                                        _3: z,
                                    };
                                    init
                                } as *mut Triple
                                    as *mut libc::c_void,
                            );
                        } else {
                            (*SEQ_var).l = listNew(::core::mem::size_of::<Triple>() as i32, &mut {
                                let mut init = Triple {
                                    _1: x,
                                    _2: y,
                                    _3: z,
                                };
                                init
                            }
                                as *mut Triple
                                as *mut libc::c_void);
                        };
                    } else {
                    };
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
        let mut t: Triple = Triple {
            _1: 0,
            _2: 0,
            _3: 0,
        };
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
            *(&mut (*(*FE_var).l).val as *mut [i8; 0] as *mut Triple),
        );
        while (if !((*FE_var).l).is_null() {
            ::core::ptr::write_volatile(
                &mut t as *mut Triple,
                *(&mut (*(*FE_var).l).val as *mut [i8; 0] as *mut Triple),
            );
            1
        } else {
            0
        }) != 0
        {
            print!("{}, {}, {}\n", t._1, t._2, t._3);
            (*FE_var).l = (*(*FE_var).l).nx;
        }
        FE_var = (*FE_var).p;
        return 0;
    }
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
