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
    fn strncpy(_: *mut i8, _: *const i8, _: u64) -> *mut i8;
    fn strtok(_: *mut i8, _: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn atoi(__nptr: *const i8) -> i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
}
#[no_mangle]
pub extern "C" fn cartesianProduct(
    mut sets: *mut *mut i32,
    mut setLengths: *mut i32,
    mut currentSet: *mut i32,
    mut numSets: i32,
    mut times: i32,
) {
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        if times == numSets {
            print!("(");
            i = 0;
            while i < times {
                print!("{},", *currentSet.offset(i as isize));
                i += 1;
                i;
            }
            print!("\x08),");
        } else {
            j = 0;
            while j < *setLengths.offset(times as isize) {
                *currentSet.offset(times as isize) =
                    *(*sets.offset(times as isize)).offset(j as isize);
                cartesianProduct(sets, setLengths, currentSet, numSets, times + 1);
                j += 1;
                j;
            }
        };
    }
}

#[no_mangle]
pub extern "C" fn printSets(mut sets: *mut *mut i32, mut setLengths: *mut i32, mut numSets: i32) {
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        print!("\nNumber of sets : {}", numSets);
        i = 0;
        while i < numSets + 1 {
            print!("\nSet {} : ", i + 1);
            j = 0;
            while j < *setLengths.offset(i as isize) {
                print!(" {} ", *(*sets.offset(i as isize)).offset(j as isize));
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    }
}

#[no_mangle]
pub extern "C" fn processInputString(mut str: *mut i8) {
    unsafe {
        let mut sets: *mut *mut i32 = 0 as *mut *mut i32;
        let mut currentSet: *mut i32 = 0 as *mut i32;
        let mut setLengths: *mut i32 = 0 as *mut i32;
        let mut setLength: i32 = 0;
        let mut numSets: i32 = 0;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut k: i32 = 0;
        let mut l: i32 = 0;
        let mut start: i32 = 0;
        let mut counter: i32 = 0;
        let mut token: *mut i8 = 0 as *mut i8;
        let mut holder: *mut i8 = 0 as *mut i8;
        let mut holderToken: *mut i8 = 0 as *mut i8;
        i = 0;
        while *str.offset(i as isize) as i32 != 0 {
            if *str.offset(i as isize) as i32 == 'x' as i32 {
                numSets += 1;
                numSets;
            }
            i += 1;
            i;
        }
        if numSets == 0 {
            print!("\n{}", build_str_from_raw_ptr(str as *mut u8));
            return;
        }
        currentSet = calloc(
            ::core::mem::size_of::<i32>() as u64,
            (numSets + 1i32) as u64,
        ) as *mut i32;
        setLengths = calloc(
            ::core::mem::size_of::<i32>() as u64,
            (numSets + 1i32) as u64,
        ) as *mut i32;
        sets = malloc(
            ((numSets + 1i32) as u64).wrapping_mul(::core::mem::size_of::<*mut i32>() as u64),
        ) as *mut *mut i32;
        token = strtok(str, b"x\0" as *const u8 as *const i8);
        while !token.is_null() {
            holder = malloc((strlen(token)).wrapping_mul(::core::mem::size_of::<i8>() as u64))
                as *mut i8;
            j = 0;
            i = 0;
            while *token.offset(i as isize) as i32 != 0 {
                if *token.offset(i as isize) as i32 >= '0' as i32
                    && *token.offset(i as isize) as i32 <= '9' as i32
                {
                    let fresh0 = j;
                    j = j + 1;
                    *holder.offset(fresh0 as isize) = *token.offset(i as isize);
                } else if *token.offset(i as isize) as i32 == ',' as i32 {
                    let fresh1 = j;
                    j = j + 1;
                    *holder.offset(fresh1 as isize) = ' ' as i8;
                }
                i += 1;
                i;
            }
            *holder.offset(j as isize) = 0;
            setLength = 0;
            i = 0;
            while *holder.offset(i as isize) as i32 != 0 {
                if *holder.offset(i as isize) as i32 == ' ' as i32 {
                    setLength += 1;
                    setLength;
                }
                i += 1;
                i;
            }
            if setLength == 0 && strlen(holder) == 0 {
                print!("\n{{}}");
                return;
            };
            *setLengths.offset(counter as isize) = setLength + 1;
            let ref mut fresh2 = *sets.offset(counter as isize);
            *fresh2 =
                malloc(((1 + setLength) as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64))
                    as *mut i32;
            k = 0;
            start = 0;
            l = 0;
            while *holder.offset(l as isize) as i32 != 0 {
                if *holder.offset((l + 1i32) as isize) as i32 == ' ' as i32
                    || *holder.offset((l + 1i32) as isize) as i32 == 0
                {
                    holderToken = malloc(
                        ((l + 1 - start) as u64).wrapping_mul(::core::mem::size_of::<i8>() as u64),
                    ) as *mut i8;
                    strncpy(
                        holderToken,
                        holder.offset(start as isize),
                        (l + 1 - start) as u64,
                    );
                    let fresh3 = k;
                    k = k + 1;
                    *(*sets.offset(counter as isize)).offset(fresh3 as isize) = atoi(holderToken);
                    start = l + 2;
                }
                l += 1;
                l;
            }
            counter += 1;
            counter;
            token = strtok(0 as *mut i8, b"x\0" as *const u8 as *const i8);
        }
        print!("\n{{");
        cartesianProduct(sets, setLengths, currentSet, numSets + 1, 0);
        print!("\x08}}");
    }
}

fn main_0(mut argC: i32, mut argV: *mut *mut i8) -> i32 {
    unsafe {
        if argC != 2 {
            print!(
                "Usage : {} <Set product expression enclosed in double quotes>",
                build_str_from_raw_ptr(*argV.offset(0 as isize) as *mut u8)
            );
        } else {
            processInputString(*argV.offset(1 as isize));
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
