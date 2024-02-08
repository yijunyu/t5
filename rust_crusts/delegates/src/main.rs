#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
fn build_str_from_raw_ptr(raw_ptr: *mut u8) -> String {
    unsafe {
        let mut str_size: usize = 0;
        while *raw_ptr.offset(str_size as isize) != 0 {
            str_size += 1;
        }
        return std::str::from_utf8_unchecked(std::slice::from_raw_parts(raw_ptr, str_size))
            .to_owned();
    }
}

use c2rust_out::*;
extern "C" {
    fn printf(_: *const i8, _: ...) -> i32;
    fn malloc(_: u64) -> *mut libc::c_void;
}
pub type Responder = Option<unsafe extern "C" fn(i32) -> *const i8>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sDelegate {
    pub operation: Responder,
}
pub type Delegate = *mut sDelegate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sDelegator {
    pub param: i32,
    pub phrase: *mut i8,
    pub delegate: Delegate,
}
pub type Delegator = *mut sDelegator;
#[no_mangle]
pub extern "C" fn NewDelegate(mut rspndr: Responder) -> Delegate {
    unsafe {
        let mut dl: Delegate = malloc(::core::mem::size_of::<sDelegate>() as u64) as Delegate;
        (*dl).operation = rspndr;
        return dl;
    }
}

#[no_mangle]
pub extern "C" fn DelegateThing(mut dl: Delegate, mut p1: i32) -> *const i8 {
    return if ((*dl).operation).is_some() {
        (Some(((*dl).operation).expect("non-null function pointer")))
            .expect("non-null function pointer")(p1)
    } else {
        0 as *const i8
    };
}

#[no_mangle]
pub extern "C" fn defaultResponse(mut p1: i32) -> *const i8 {
    return b"default implementation\0" as *const u8 as *const i8;
}

static mut defaultDel: sDelegate = {
    let mut init = sDelegate {
        operation: Some(defaultResponse as unsafe extern "C" fn(i32) -> *const i8),
    };
    init
};
#[no_mangle]
pub extern "C" fn NewDelegator(mut p: i32, mut phrase: *mut i8) -> Delegator {
    unsafe {
        let mut d: Delegator = malloc(::core::mem::size_of::<sDelegator>() as u64) as Delegator;
        (*d).param = p;
        (*d).phrase = phrase;
        (*d).delegate = &mut defaultDel;
        return d;
    }
}

#[no_mangle]
pub extern "C" fn Delegator_Operation(
    mut theDelegator: Delegator,
    mut p1: i32,
    mut delroy: Delegate,
) -> *const i8 {
    unsafe {
        let mut rtn: *const i8 = 0 as *const i8;
        if !delroy.is_null() {
            rtn = DelegateThing(delroy, p1);
            if rtn.is_null() {
                rtn = DelegateThing((*theDelegator).delegate, p1);
            }
        } else {
            rtn = DelegateThing((*theDelegator).delegate, p1);
        }
        print!(
            "{}\n",
            build_str_from_raw_ptr((*theDelegator).phrase as *mut u8)
        );
        return rtn;
    }
}

#[no_mangle]
pub extern "C" fn thing1(mut p1: i32) -> *const i8 {
    unsafe {
        printf(
            b"We're in thing1 with value %d\n\0" as *const u8 as *const i8,
            p1,
        );
    }
    return b"delegate implementation\0" as *const u8 as *const i8;
}

fn main_0() -> i32 {
    unsafe {
        let mut del1: Delegate =
            NewDelegate(Some(thing1 as unsafe extern "C" fn(i32) -> *const i8));
        let mut del2: Delegate = NewDelegate(None);
        let mut theDelegator: Delegator = NewDelegator(
            14,
            b"A stellar vista, Baby.\0" as *const u8 as *const i8 as *mut i8,
        );
        print!(
            "Delegator returns {}\n\n",
            build_str_from_raw_ptr(Delegator_Operation(theDelegator, 3, 0 as Delegate) as *mut u8)
        );
        print!(
            "Delegator returns {}\n\n",
            build_str_from_raw_ptr(Delegator_Operation(theDelegator, 3, del1) as *mut u8)
        );
        print!(
            "Delegator returns {}\n\n",
            build_str_from_raw_ptr(Delegator_Operation(theDelegator, 3, del2) as *mut u8)
        );
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
