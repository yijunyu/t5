#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct charList {
    pub c: libc::c_char,
    pub next: *mut charList,
}
#[no_mangle]
pub unsafe extern "C" fn strcmpi(
    mut str1: *mut libc::c_char,
    mut str2: *mut libc::c_char,
) -> libc::c_int {
    let mut len1: libc::c_int = strlen(str1 as *const libc::c_char) as libc::c_int;
    let mut len2: libc::c_int = strlen(str2 as *const libc::c_char) as libc::c_int;
    let mut i: libc::c_int = 0;
    if len1 != len2 {
        return 1 as libc::c_int
    } else {
        i = 0 as libc::c_int;
        while i < len1 {
            if *str1.offset(i as isize) as libc::c_int >= 'A' as i32
                && *str1.offset(i as isize) as libc::c_int <= 'Z' as i32
                && (*str2.offset(i as isize) as libc::c_int >= 'a' as i32
                    && *str2.offset(i as isize) as libc::c_int <= 'z' as i32)
                && *str2.offset(i as isize) as libc::c_int - 65 as libc::c_int
                    != *str1.offset(i as isize) as libc::c_int
            {
                return 1 as libc::c_int
            } else if *str2.offset(i as isize) as libc::c_int >= 'A' as i32
                && *str2.offset(i as isize) as libc::c_int <= 'Z' as i32
                && (*str1.offset(i as isize) as libc::c_int >= 'a' as i32
                    && *str1.offset(i as isize) as libc::c_int <= 'z' as i32)
                && *str1.offset(i as isize) as libc::c_int - 65 as libc::c_int
                    != *str2.offset(i as isize) as libc::c_int
            {
                return 1 as libc::c_int
            } else if *str1.offset(i as isize) as libc::c_int
                != *str2.offset(i as isize) as libc::c_int
            {
                return 1 as libc::c_int
            }
            i += 1;
            i;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn strToCharList(mut str: *mut libc::c_char) -> *mut charList {
    let mut len: libc::c_int = strlen(str) as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut list: *mut charList = 0 as *mut charList;
    let mut iterator: *mut charList = 0 as *mut charList;
    let mut nextChar: *mut charList = 0 as *mut charList;
    list = malloc(::core::mem::size_of::<charList>() as libc::c_ulong) as *mut charList;
    (*list).c = *str.offset(0 as libc::c_int as isize);
    (*list).next = 0 as *mut charList;
    iterator = list;
    i = 1 as libc::c_int;
    while i < len {
        nextChar = malloc(::core::mem::size_of::<charList>() as libc::c_ulong)
            as *mut charList;
        (*nextChar).c = *str.offset(i as isize);
        (*nextChar).next = 0 as *mut charList;
        (*iterator).next = nextChar;
        iterator = nextChar;
        i += 1;
        i;
    }
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn charListToString(mut list: *mut charList) -> *mut libc::c_char {
    let mut iterator: *mut charList = list;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    while !iterator.is_null() {
        count += 1;
        count;
        iterator = (*iterator).next;
    }
    str = malloc(
        ((count + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    iterator = list;
    i = 0 as libc::c_int;
    while i < count {
        *str.offset(i as isize) = (*iterator).c;
        iterator = (*iterator).next;
        i += 1;
        i;
    }
    free(list as *mut libc::c_void);
    *str.offset(i as isize) = '\0' as i32 as libc::c_char;
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn processString(
    mut str: *mut libc::c_char,
    mut operation: libc::c_int,
    mut squeezeChar: libc::c_char,
) -> *mut libc::c_char {
    let mut strList: *mut charList = strToCharList(str);
    let mut iterator: *mut charList = strList;
    let mut scout: *mut charList = 0 as *mut charList;
    if operation == 1 as libc::c_int {
        while !iterator.is_null() {
            if (*iterator).c as libc::c_int == squeezeChar as libc::c_int {
                scout = (*iterator).next;
                while !scout.is_null()
                    && (*scout).c as libc::c_int == squeezeChar as libc::c_int
                {
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
            if (*iterator).c as libc::c_int == (*(*iterator).next).c as libc::c_int {
                scout = (*iterator).next;
                squeezeChar = (*iterator).c;
                while !scout.is_null()
                    && (*scout).c as libc::c_int == squeezeChar as libc::c_int
                {
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
#[no_mangle]
pub unsafe extern "C" fn printResults(
    mut originalString: *mut libc::c_char,
    mut finalString: *mut libc::c_char,
    mut operation: libc::c_int,
    mut squeezeChar: libc::c_char,
) {
    if operation == 1 as libc::c_int {
        printf(
            b"Specified Operation : SQUEEZE\nTarget Character : %c\0" as *const u8
                as *const libc::c_char,
            squeezeChar as libc::c_int,
        );
    } else {
        printf(b"Specified Operation : COLLAPSE\0" as *const u8 as *const libc::c_char);
    }
    printf(
        b"\nOriginal %c%c%c%s%c%c%c\nLength : %d\0" as *const u8 as *const libc::c_char,
        174 as libc::c_int,
        174 as libc::c_int,
        174 as libc::c_int,
        originalString,
        175 as libc::c_int,
        175 as libc::c_int,
        175 as libc::c_int,
        strlen(originalString as *const libc::c_char) as libc::c_int,
    );
    printf(
        b"\nFinal    %c%c%c%s%c%c%c\nLength : %d\n\0" as *const u8
            as *const libc::c_char,
        174 as libc::c_int,
        174 as libc::c_int,
        174 as libc::c_int,
        finalString,
        175 as libc::c_int,
        175 as libc::c_int,
        175 as libc::c_int,
        strlen(finalString as *const libc::c_char) as libc::c_int,
    );
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut operation: libc::c_int = 0;
    let mut squeezeChar: libc::c_char = 0;
    if argc < 3 as libc::c_int || argc > 4 as libc::c_int {
        printf(
            b"Usage : %s <SQUEEZE|COLLAPSE> <String to be processed> <Character to be squeezed, if operation is SQUEEZE>\n\0"
                as *const u8 as *const libc::c_char,
            *argv.offset(0 as libc::c_int as isize),
        );
        return 0 as libc::c_int;
    }
    if strcmpi(
        *argv.offset(1 as libc::c_int as isize),
        b"SQUEEZE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0 as libc::c_int && argc != 4 as libc::c_int
    {
        scanf(
            b"Please enter characted to be squeezed : %c\0" as *const u8
                as *const libc::c_char,
            &mut squeezeChar as *mut libc::c_char,
        );
        operation = 1 as libc::c_int;
    } else if argc == 4 as libc::c_int {
        operation = 1 as libc::c_int;
        squeezeChar = *(*argv.offset(3 as libc::c_int as isize))
            .offset(0 as libc::c_int as isize);
    } else if strcmpi(
        *argv.offset(1 as libc::c_int as isize),
        b"COLLAPSE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0 as libc::c_int
    {
        operation = 0 as libc::c_int;
    }
    if strlen(*argv.offset(2 as libc::c_int as isize))
        < 2 as libc::c_int as libc::c_ulong
    {
        printResults(
            *argv.offset(2 as libc::c_int as isize),
            *argv.offset(2 as libc::c_int as isize),
            operation,
            squeezeChar,
        );
    } else {
        printResults(
            *argv.offset(2 as libc::c_int as isize),
            processString(
                *argv.offset(2 as libc::c_int as isize),
                operation,
                squeezeChar,
            ),
            operation,
            squeezeChar,
        );
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
