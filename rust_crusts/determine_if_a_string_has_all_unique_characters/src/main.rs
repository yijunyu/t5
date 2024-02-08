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
    fn strlen(_: *const i8) -> u64;
    fn malloc(_: u64) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct positionList {
    pub position: i32,
    pub next: *mut positionList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct letterList {
    pub letter: i8,
    pub repititions: i32,
    pub positions: *mut positionList,
    pub next: *mut letterList,
}
#[no_mangle]
pub static mut letterSet: *mut letterList = 0 as *const letterList as *mut letterList;
#[no_mangle]
pub static mut duplicatesFound: bool = 0 != 0;
#[no_mangle]
pub extern "C" fn checkAndUpdateLetterList(mut c: i8, mut pos: i32) {
    unsafe {
        let mut letterOccurs: bool = 0 != 0;
        let mut letterIterator: *mut letterList = 0 as *mut letterList;
        let mut newLetter: *mut letterList = 0 as *mut letterList;
        let mut positionIterator: *mut positionList = 0 as *mut positionList;
        let mut newPosition: *mut positionList = 0 as *mut positionList;
        if letterSet.is_null() {
            letterSet = malloc(::core::mem::size_of::<letterList>() as u64) as *mut letterList;
            (*letterSet).letter = c;
            (*letterSet).repititions = 0;
            (*letterSet).positions =
                malloc(::core::mem::size_of::<positionList>() as u64) as *mut positionList;
            (*(*letterSet).positions).position = pos;
            (*(*letterSet).positions).next = 0 as *mut positionList;
            (*letterSet).next = 0 as *mut letterList;
        } else {
            letterIterator = letterSet;
            while !letterIterator.is_null() {
                if (*letterIterator).letter as i32 == c as i32 {
                    letterOccurs = 1 != 0;
                    duplicatesFound = 1 != 0;
                    (*letterIterator).repititions += 1;
                    (*letterIterator).repititions;
                    positionIterator = (*letterIterator).positions;
                    while !((*positionIterator).next).is_null() {
                        positionIterator = (*positionIterator).next;
                    }
                    newPosition =
                        malloc(::core::mem::size_of::<positionList>() as u64) as *mut positionList;
                    (*newPosition).position = pos;
                    (*newPosition).next = 0 as *mut positionList;
                    (*positionIterator).next = newPosition;
                }
                if letterOccurs as i32 == 0 && ((*letterIterator).next).is_null() {
                    break;
                }
                letterIterator = (*letterIterator).next;
            }
            if letterOccurs as i32 == 0 {
                newLetter = malloc(::core::mem::size_of::<letterList>() as u64) as *mut letterList;
                (*newLetter).letter = c;
                (*newLetter).repititions = 0;
                (*newLetter).positions =
                    malloc(::core::mem::size_of::<positionList>() as u64) as *mut positionList;
                (*(*newLetter).positions).position = pos;
                (*(*newLetter).positions).next = 0 as *mut positionList;
                (*newLetter).next = 0 as *mut letterList;
                (*letterIterator).next = newLetter;
            }
        };
    }
}

#[no_mangle]
pub extern "C" fn printLetterList() {
    unsafe {
        let mut positionIterator: *mut positionList = 0 as *mut positionList;
        let mut letterIterator: *mut letterList = letterSet;
        while !letterIterator.is_null() {
            if (*letterIterator).repititions > 0 {
                print!(
                    "\n{} (0x{:x}) at positions :",
                    (*letterIterator).letter as i32,
                    (*letterIterator).letter as i32
                );
                positionIterator = (*letterIterator).positions;
                while !positionIterator.is_null() {
                    print!("{:3}", (*positionIterator).position + 1);
                    positionIterator = (*positionIterator).next;
                }
            }
            letterIterator = (*letterIterator).next;
        }
        print!("\n");
    }
}

fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut len: i32 = 0;
        if argc > 2 {
            print!(
                "Usage : {} <Test string>\n",
                build_str_from_raw_ptr(*argv.offset(0 as isize) as *mut u8)
            );
            return 0;
        }
        if argc == 1 || strlen(*argv.offset(1 as isize)) == 1 {
            if argc == 1 {
                if argc == 1 {
                    print!(
                        "\"{}\" - Length {} - Contains only unique characters.\n",
                        "\0", 0
                    )
                } else {
                    print!(
                        "\"{}\" - Length {} - Contains only unique characters.\n",
                        "\0", 1
                    )
                }
            } else {
                if argc == 1 {
                    print!(
                        "\"{}\" - Length {} - Contains only unique characters.\n",
                        build_str_from_raw_ptr(*argv.offset(1 as isize) as *const i8 as *mut u8),
                        0
                    )
                } else {
                    print!(
                        "\"{}\" - Length {} - Contains only unique characters.\n",
                        build_str_from_raw_ptr(*argv.offset(1 as isize) as *const i8 as *mut u8),
                        1
                    )
                }
            };
            return 0;
        }
        len = strlen(*argv.offset(1 as isize)) as i32;
        i = 0;
        while i < len {
            checkAndUpdateLetterList(*(*argv.offset(1 as isize)).offset(i as isize), i);
            i += 1;
            i;
        }
        if duplicatesFound as i32 == 0 {
            print!(
                "\"{}\" - Length {} - {}",
                build_str_from_raw_ptr(*argv.offset(1 as isize) as *mut u8),
                len,
                "Contains only unique characters.\n\0"
            )
        } else {
            print!(
                "\"{}\" - Length {} - {}",
                build_str_from_raw_ptr(*argv.offset(1 as isize) as *mut u8),
                len,
                "Contains the following duplicate characters :\0"
            )
        };
        if duplicatesFound as i32 == 1 {
            printLetterList();
        }
        return 0;
    }
}

pub fn main() {
    let mut args: Vec<*mut i8> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    ::std::process::exit(main_0((args.len() - 1) as i32, args.as_mut_ptr() as *mut *mut i8) as i32);
}
