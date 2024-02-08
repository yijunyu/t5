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
    fn free(_: *mut libc::c_void);
    fn scanf(_: *const i8, _: ...) -> i32;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct charList {
    pub c: i8,
    pub next: *mut charList,
}
#[no_mangle]
pub extern "C" fn strcmpi(mut str1: *mut i8, mut str2: *mut i8) -> i32 {
    unsafe {
        let mut len1: i32 = strlen(str1) as i32;
        let mut len2: i32 = strlen(str2) as i32;
        let mut i: i32 = 0;
        if len1 != len2 {
            return 1;
        } else {
            i = 0;
            while i < len1 {
                if *str1.offset(i as isize) as i32 >= 'A' as i32
                    && *str1.offset(i as isize) as i32 <= 'Z' as i32
                    && (*str2.offset(i as isize) as i32 >= 'a' as i32
                        && *str2.offset(i as isize) as i32 <= 'z' as i32)
                    && *str2.offset(i as isize) as i32 - 65 != *str1.offset(i as isize) as i32
                {
                    return 1;
                } else if *str2.offset(i as isize) as i32 >= 'A' as i32
                    && *str2.offset(i as isize) as i32 <= 'Z' as i32
                    && (*str1.offset(i as isize) as i32 >= 'a' as i32
                        && *str1.offset(i as isize) as i32 <= 'z' as i32)
                    && *str1.offset(i as isize) as i32 - 65 != *str2.offset(i as isize) as i32
                {
                    return 1;
                } else if *str1.offset(i as isize) as i32 != *str2.offset(i as isize) as i32 {
                    return 1;
                }
                i += 1;
                i;
            }
        }
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn strToCharList(mut str: *mut i8) -> *mut charList {
    unsafe {
        let mut len: i32 = strlen(str) as i32;
        let mut i: i32 = 0;
        let mut list: *mut charList = 0 as *mut charList;
        let mut iterator: *mut charList = 0 as *mut charList;
        let mut nextChar: *mut charList = 0 as *mut charList;
        list = malloc(::core::mem::size_of::<charList>() as u64) as *mut charList;
        (*list).c = *str.offset(0 as isize);
        (*list).next = 0 as *mut charList;
        iterator = list;
        i = 1;
        while i < len {
            nextChar = malloc(::core::mem::size_of::<charList>() as u64) as *mut charList;
            (*nextChar).c = *str.offset(i as isize);
            (*nextChar).next = 0 as *mut charList;
            (*iterator).next = nextChar;
            iterator = nextChar;
            i += 1;
            i;
        }
        return list;
    }
}

#[no_mangle]
pub extern "C" fn charListToString(mut list: *mut charList) -> *mut i8 {
    unsafe {
        let mut iterator: *mut charList = list;
        let mut count: i32 = 0;
        let mut i: i32 = 0;
        let mut str: *mut i8 = 0 as *mut i8;
        while !iterator.is_null() {
            count += 1;
            count;
            iterator = (*iterator).next;
        }
        str = malloc(((count + 1i32) as u64).wrapping_mul(::core::mem::size_of::<i8>() as u64))
            as *mut i8;
        iterator = list;
        i = 0;
        while i < count {
            *str.offset(i as isize) = (*iterator).c;
            iterator = (*iterator).next;
            i += 1;
            i;
        }
        free(list as *mut libc::c_void);
        *str.offset(i as isize) = '\0' as i8;
        return str;
    }
}

#[no_mangle]
pub extern "C" fn processString(
    mut str: *mut i8,
    mut operation: i32,
    mut squeezeChar: i8,
) -> *mut i8 {
    unsafe {
        let mut strList: *mut charList = strToCharList(str);
        let mut iterator: *mut charList = strList;
        let mut scout: *mut charList = 0 as *mut charList;
        if operation == 1 {
            while !iterator.is_null() {
                if (*iterator).c as i32 == squeezeChar as i32 {
                    scout = (*iterator).next;
                    while !scout.is_null() && (*scout).c as i32 == squeezeChar as i32 {
                        (*iterator).next = (*scout).next;
                        (*scout).next = 0 as *mut charList;
                        free(scout as *mut libc::c_void);
                        scout = (*iterator).next;
                    }
                }
                iterator = (*iterator).next;
            }
        } else {
            while !iterator.is_null() && !((*iterator).next).is_null() {
                if (*iterator).c as i32 == (*(*iterator).next).c as i32 {
                    scout = (*iterator).next;
                    squeezeChar = (*iterator).c;
                    while !scout.is_null() && (*scout).c as i32 == squeezeChar as i32 {
                        (*iterator).next = (*scout).next;
                        (*scout).next = 0 as *mut charList;
                        free(scout as *mut libc::c_void);
                        scout = (*iterator).next;
                    }
                }
                iterator = (*iterator).next;
            }
        }
        return charListToString(strList);
    }
}

#[no_mangle]
pub extern "C" fn printResults(
    mut originalString: *mut i8,
    mut finalString: *mut i8,
    mut operation: i32,
    mut squeezeChar: i8,
) {
    unsafe {
        if operation == 1 {
            print!(
                "Specified Operation : SQUEEZE\nTarget Character : {}",
                squeezeChar as i32
            );
        } else {
            print!("Specified Operation : COLLAPSE");
        }
        print!(
            "\nOriginal {}{}{}{}{}{}{}\nLength : {}",
            174,
            174,
            174,
            build_str_from_raw_ptr(originalString as *mut u8),
            175,
            175,
            175,
            strlen(originalString as *const i8) as i32
        );
        print!(
            "\nFinal    {}{}{}{}{}{}{}\nLength : {}\n",
            174,
            174,
            174,
            build_str_from_raw_ptr(finalString as *mut u8),
            175,
            175,
            175,
            strlen(finalString as *const i8) as i32
        );
    }
}

fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    unsafe {
        let mut operation: i32 = 0;
        let mut squeezeChar: i8 = 0;
        if argc < 3 || argc > 4 {
            print! ("Usage : {} <SQUEEZE|COLLAPSE> <String to be processed> <Character to be squeezed, if operation is SQUEEZE>\n", build_str_from_raw_ptr (* argv.offset (0 as isize) as * mut u8));
            return 0;
        }
        if strcmpi(
            *argv.offset(1 as isize),
            b"SQUEEZE\0" as *const u8 as *const i8 as *mut i8,
        ) == 0
            && argc != 4
        {
            scanf(
                b"Please enter characted to be squeezed : %c\0" as *const u8 as *const i8,
                &mut squeezeChar as *mut i8,
            );
            operation = 1;
        } else if argc == 4 {
            operation = 1;
            squeezeChar = *(*argv.offset(3 as isize)).offset(0 as isize);
        } else if strcmpi(
            *argv.offset(1 as isize),
            b"COLLAPSE\0" as *const u8 as *const i8 as *mut i8,
        ) == 0
        {
            operation = 0;
        }
        if strlen(*argv.offset(2 as isize)) < 2 {
            printResults(
                *argv.offset(2 as isize),
                *argv.offset(2 as isize),
                operation,
                squeezeChar,
            );
        } else {
            printResults(
                *argv.offset(2 as isize),
                processString(*argv.offset(2 as isize), operation, squeezeChar),
                operation,
                squeezeChar,
            );
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
