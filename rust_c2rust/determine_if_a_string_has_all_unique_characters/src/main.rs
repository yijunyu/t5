#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct positionList {
    pub position: libc::c_int,
    pub next: *mut positionList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct letterList {
    pub letter: libc::c_char,
    pub repititions: libc::c_int,
    pub positions: *mut positionList,
    pub next: *mut letterList,
}
#[no_mangle]
pub static mut letterSet: *mut letterList = 0 as *const letterList as *mut letterList;
#[no_mangle]
pub static mut duplicatesFound: bool = 0 as libc::c_int != 0;
#[no_mangle]
pub unsafe extern "C" fn checkAndUpdateLetterList(
    mut c: libc::c_char,
    mut pos: libc::c_int,
) {
    let mut letterOccurs: bool = 0 as libc::c_int != 0;
    let mut letterIterator: *mut letterList = 0 as *mut letterList;
    let mut newLetter: *mut letterList = 0 as *mut letterList;
    let mut positionIterator: *mut positionList = 0 as *mut positionList;
    let mut newPosition: *mut positionList = 0 as *mut positionList;
    if letterSet.is_null() {
        letterSet = malloc(::core::mem::size_of::<letterList>() as libc::c_ulong)
            as *mut letterList;
        (*letterSet).letter = c;
        (*letterSet).repititions = 0 as libc::c_int;
        (*letterSet)
            .positions = malloc(::core::mem::size_of::<positionList>() as libc::c_ulong)
            as *mut positionList;
        (*(*letterSet).positions).position = pos;
        (*(*letterSet).positions).next = 0 as *mut positionList;
        (*letterSet).next = 0 as *mut letterList;
    } else {
        letterIterator = letterSet;
        while !letterIterator.is_null() {
            if (*letterIterator).letter as libc::c_int == c as libc::c_int {
                letterOccurs = 1 as libc::c_int != 0;
                duplicatesFound = 1 as libc::c_int != 0;
                (*letterIterator).repititions += 1;
                (*letterIterator).repititions;
                positionIterator = (*letterIterator).positions;
                while !((*positionIterator).next).is_null() {
                    positionIterator = (*positionIterator).next;
                }
                newPosition = malloc(
                    ::core::mem::size_of::<positionList>() as libc::c_ulong,
                ) as *mut positionList;
                (*newPosition).position = pos;
                (*newPosition).next = 0 as *mut positionList;
                (*positionIterator).next = newPosition;
            }
            if letterOccurs as libc::c_int == 0 as libc::c_int
                && ((*letterIterator).next).is_null()
            {
                break;
            }
            letterIterator = (*letterIterator).next;
        }
        if letterOccurs as libc::c_int == 0 as libc::c_int {
            newLetter = malloc(::core::mem::size_of::<letterList>() as libc::c_ulong)
                as *mut letterList;
            (*newLetter).letter = c;
            (*newLetter).repititions = 0 as libc::c_int;
            (*newLetter)
                .positions = malloc(
                ::core::mem::size_of::<positionList>() as libc::c_ulong,
            ) as *mut positionList;
            (*(*newLetter).positions).position = pos;
            (*(*newLetter).positions).next = 0 as *mut positionList;
            (*newLetter).next = 0 as *mut letterList;
            (*letterIterator).next = newLetter;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn printLetterList() {
    let mut positionIterator: *mut positionList = 0 as *mut positionList;
    let mut letterIterator: *mut letterList = letterSet;
    while !letterIterator.is_null() {
        if (*letterIterator).repititions > 0 as libc::c_int {
            printf(
                b"\n'%c' (0x%x) at positions :\0" as *const u8 as *const libc::c_char,
                (*letterIterator).letter as libc::c_int,
                (*letterIterator).letter as libc::c_int,
            );
            positionIterator = (*letterIterator).positions;
            while !positionIterator.is_null() {
                printf(
                    b"%3d\0" as *const u8 as *const libc::c_char,
                    (*positionIterator).position + 1 as libc::c_int,
                );
                positionIterator = (*positionIterator).next;
            }
        }
        letterIterator = (*letterIterator).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    if argc > 2 as libc::c_int {
        printf(
            b"Usage : %s <Test string>\n\0" as *const u8 as *const libc::c_char,
            *argv.offset(0 as libc::c_int as isize),
        );
        return 0 as libc::c_int;
    }
    if argc == 1 as libc::c_int
        || strlen(*argv.offset(1 as libc::c_int as isize))
            == 1 as libc::c_int as libc::c_ulong
    {
        printf(
            b"\"%s\" - Length %d - Contains only unique characters.\n\0" as *const u8
                as *const libc::c_char,
            if argc == 1 as libc::c_int {
                b"\0" as *const u8 as *const libc::c_char
            } else {
                *argv.offset(1 as libc::c_int as isize) as *const libc::c_char
            },
            if argc == 1 as libc::c_int { 0 as libc::c_int } else { 1 as libc::c_int },
        );
        return 0 as libc::c_int;
    }
    len = strlen(*argv.offset(1 as libc::c_int as isize)) as libc::c_int;
    i = 0 as libc::c_int;
    while i < len {
        checkAndUpdateLetterList(
            *(*argv.offset(1 as libc::c_int as isize)).offset(i as isize),
            i,
        );
        i += 1;
        i;
    }
    printf(
        b"\"%s\" - Length %d - %s\0" as *const u8 as *const libc::c_char,
        *argv.offset(1 as libc::c_int as isize),
        len,
        if duplicatesFound as libc::c_int == 0 as libc::c_int {
            b"Contains only unique characters.\n\0" as *const u8 as *const libc::c_char
        } else {
            b"Contains the following duplicate characters :\0" as *const u8
                as *const libc::c_char
        },
    );
    if duplicatesFound as libc::c_int == 1 as libc::c_int {
        printLetterList();
    }
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
