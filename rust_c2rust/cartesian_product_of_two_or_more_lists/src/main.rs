#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cartesianProduct(
    mut sets: *mut *mut libc::c_int,
    mut setLengths: *mut libc::c_int,
    mut currentSet: *mut libc::c_int,
    mut numSets: libc::c_int,
    mut times: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if times == numSets {
        printf(b"(\0" as *const u8 as *const libc::c_char);
        i = 0 as libc::c_int;
        while i < times {
            printf(
                b"%d,\0" as *const u8 as *const libc::c_char,
                *currentSet.offset(i as isize),
            );
            i += 1;
            i;
        }
        printf(b"\x08),\0" as *const u8 as *const libc::c_char);
    } else {
        j = 0 as libc::c_int;
        while j < *setLengths.offset(times as isize) {
            *currentSet
                .offset(
                    times as isize,
                ) = *(*sets.offset(times as isize)).offset(j as isize);
            cartesianProduct(
                sets,
                setLengths,
                currentSet,
                numSets,
                times + 1 as libc::c_int,
            );
            j += 1;
            j;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn printSets(
    mut sets: *mut *mut libc::c_int,
    mut setLengths: *mut libc::c_int,
    mut numSets: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    printf(b"\nNumber of sets : %d\0" as *const u8 as *const libc::c_char, numSets);
    i = 0 as libc::c_int;
    while i < numSets + 1 as libc::c_int {
        printf(
            b"\nSet %d : \0" as *const u8 as *const libc::c_char,
            i + 1 as libc::c_int,
        );
        j = 0 as libc::c_int;
        while j < *setLengths.offset(i as isize) {
            printf(
                b" %d \0" as *const u8 as *const libc::c_char,
                *(*sets.offset(i as isize)).offset(j as isize),
            );
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn processInputString(mut str: *mut libc::c_char) {
    let mut sets: *mut *mut libc::c_int = 0 as *mut *mut libc::c_int;
    let mut currentSet: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut setLengths: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut setLength: libc::c_int = 0;
    let mut numSets: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut start: libc::c_int = 0;
    let mut counter: libc::c_int = 0 as libc::c_int;
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut holder: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut holderToken: *mut libc::c_char = 0 as *mut libc::c_char;
    i = 0 as libc::c_int;
    while *str.offset(i as isize) as libc::c_int != 0 as libc::c_int {
        if *str.offset(i as isize) as libc::c_int == 'x' as i32 {
            numSets += 1;
            numSets;
        }
        i += 1;
        i;
    }
    if numSets == 0 as libc::c_int {
        printf(b"\n%s\0" as *const u8 as *const libc::c_char, str);
        return;
    }
    currentSet = calloc(
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        (numSets + 1 as libc::c_int) as libc::c_ulong,
    ) as *mut libc::c_int;
    setLengths = calloc(
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        (numSets + 1 as libc::c_int) as libc::c_ulong,
    ) as *mut libc::c_int;
    sets = malloc(
        ((numSets + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong),
    ) as *mut *mut libc::c_int;
    token = strtok(str, b"x\0" as *const u8 as *const libc::c_char);
    while !token.is_null() {
        holder = malloc(
            (strlen(token))
                .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        ) as *mut libc::c_char;
        j = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while *token.offset(i as isize) as libc::c_int != 0 as libc::c_int {
            if *token.offset(i as isize) as libc::c_int >= '0' as i32
                && *token.offset(i as isize) as libc::c_int <= '9' as i32
            {
                let fresh0 = j;
                j = j + 1;
                *holder.offset(fresh0 as isize) = *token.offset(i as isize);
            } else if *token.offset(i as isize) as libc::c_int == ',' as i32 {
                let fresh1 = j;
                j = j + 1;
                *holder.offset(fresh1 as isize) = ' ' as i32 as libc::c_char;
            }
            i += 1;
            i;
        }
        *holder.offset(j as isize) = 0 as libc::c_int as libc::c_char;
        setLength = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while *holder.offset(i as isize) as libc::c_int != 0 as libc::c_int {
            if *holder.offset(i as isize) as libc::c_int == ' ' as i32 {
                setLength += 1;
                setLength;
            }
            i += 1;
            i;
        }
        if setLength == 0 as libc::c_int
            && strlen(holder) == 0 as libc::c_int as libc::c_ulong
        {
            printf(b"\n{}\0" as *const u8 as *const libc::c_char);
            return;
        }
        *setLengths.offset(counter as isize) = setLength + 1 as libc::c_int;
        let ref mut fresh2 = *sets.offset(counter as isize);
        *fresh2 = malloc(
            ((1 as libc::c_int + setLength) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        k = 0 as libc::c_int;
        start = 0 as libc::c_int;
        l = 0 as libc::c_int;
        while *holder.offset(l as isize) as libc::c_int != 0 as libc::c_int {
            if *holder.offset((l + 1 as libc::c_int) as isize) as libc::c_int
                == ' ' as i32
                || *holder.offset((l + 1 as libc::c_int) as isize) as libc::c_int
                    == 0 as libc::c_int
            {
                holderToken = malloc(
                    ((l + 1 as libc::c_int - start) as libc::c_ulong)
                        .wrapping_mul(
                            ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        ),
                ) as *mut libc::c_char;
                strncpy(
                    holderToken,
                    holder.offset(start as isize),
                    (l + 1 as libc::c_int - start) as libc::c_ulong,
                );
                let fresh3 = k;
                k = k + 1;
                *(*sets.offset(counter as isize))
                    .offset(fresh3 as isize) = atoi(holderToken);
                start = l + 2 as libc::c_int;
            }
            l += 1;
            l;
        }
        counter += 1;
        counter;
        token = strtok(
            0 as *mut libc::c_char,
            b"x\0" as *const u8 as *const libc::c_char,
        );
    }
    printf(b"\n{\0" as *const u8 as *const libc::c_char);
    cartesianProduct(
        sets,
        setLengths,
        currentSet,
        numSets + 1 as libc::c_int,
        0 as libc::c_int,
    );
    printf(b"\x08}\0" as *const u8 as *const libc::c_char);
}
unsafe fn main_0(
    mut argC: libc::c_int,
    mut argV: *mut *mut libc::c_char,
) -> libc::c_int {
    if argC != 2 as libc::c_int {
        printf(
            b"Usage : %s <Set product expression enclosed in double quotes>\0"
                as *const u8 as *const libc::c_char,
            *argV.offset(0 as libc::c_int as isize),
        );
    } else {
        processInputString(*argV.offset(1 as libc::c_int as isize));
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
