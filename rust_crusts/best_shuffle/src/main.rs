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
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: i32) -> !;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strlen(_: *const i8) -> u64;
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
}
#[no_mangle]
pub extern "C" fn best_shuffle(mut txt: *const i8, mut result: *mut i8) {
    unsafe {
        let len: u64 = strlen(txt);
        if len == 0 {
            return;
        }
        if len == strlen(result) {
        } else {
            __assert_fail(
                b"len == strlen(result)\0" as *const u8 as *const i8,
                b"main.c\0" as *const u8 as *const i8,
                16,
                (*::core::mem::transmute::<&[u8; 40], &[i8; 40]>(
                    b"void best_shuffle(const char *, char *)\0",
                ))
                .as_ptr(),
            );
        }
        'c_2313: {
            if len == strlen(result) {
            } else {
                __assert_fail(
                    b"len == strlen(result)\0" as *const u8 as *const i8,
                    b"main.c\0" as *const u8 as *const i8,
                    16,
                    (*::core::mem::transmute::<&[u8; 40], &[i8; 40]>(
                        b"void best_shuffle(const char *, char *)\0",
                    ))
                    .as_ptr(),
                );
            }
        };
        let mut counts: [u64; 255] = [0; 255];
        memset(
            counts.as_mut_ptr() as *mut libc::c_void,
            '\0' as i32,
            ((127 * 2 + 1i32) as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64),
        );
        let mut fmax: u64 = 0;
        let mut i: u64 = 0;
        while i < len {
            counts[*txt.offset(i as isize) as u8 as usize] =
                (counts[*txt.offset(i as isize) as u8 as usize]).wrapping_add(1);
            counts[*txt.offset(i as isize) as u8 as usize];
            let fnew: u64 = counts[*txt.offset(i as isize) as u8 as usize];
            if fmax < fnew {
                fmax = fnew;
            }
            i = i.wrapping_add(1);
            i;
        }
        if fmax > 0 && fmax <= len {
        } else {
            __assert_fail(
                b"fmax > 0 && fmax <= len\0" as *const u8 as *const i8,
                b"main.c\0" as *const u8 as *const i8,
                29,
                (*::core::mem::transmute::<&[u8; 40], &[i8; 40]>(
                    b"void best_shuffle(const char *, char *)\0",
                ))
                .as_ptr(),
            );
        }
        'c_2197: {
            if fmax > 0 && fmax <= len {
            } else {
                __assert_fail(
                    b"fmax > 0 && fmax <= len\0" as *const u8 as *const i8,
                    b"main.c\0" as *const u8 as *const i8,
                    29,
                    (*::core::mem::transmute::<&[u8; 40], &[i8; 40]>(
                        b"void best_shuffle(const char *, char *)\0",
                    ))
                    .as_ptr(),
                );
            }
        };
        let mut ndx1: *mut u64 =
            malloc(len.wrapping_mul(::core::mem::size_of::<u64>() as u64)) as *mut u64;
        if ndx1.is_null() {
            exit(1);
        }
        let mut ch: u64 = 0;
        let mut i_0: u64 = 0;
        while ch < (127 * 2 + 1i32) as u64 {
            if counts[ch as usize] != 0 {
                let mut j: u64 = 0;
                while j < len {
                    if ch == *txt.offset(j as isize) as u64 {
                        *ndx1.offset(i_0 as isize) = j;
                        i_0 = i_0.wrapping_add(1);
                        i_0;
                    }
                    j = j.wrapping_add(1);
                    j;
                }
            }
            ch = ch.wrapping_add(1);
            ch;
        }
        let mut ndx2: *mut u64 =
            malloc(len.wrapping_mul(::core::mem::size_of::<u64>() as u64)) as *mut u64;
        if ndx2.is_null() {
            exit(1);
        }
        let mut i_1: u64 = 0;
        let mut n: u64 = 0;
        let mut m: u64 = 0;
        while i_1 < len {
            *ndx2.offset(i_1 as isize) = *ndx1.offset(n as isize);
            n = (n).wrapping_add(fmax) as u64;
            if n >= len {
                m = m.wrapping_add(1);
                m;
                n = m;
            }
            i_1 = i_1.wrapping_add(1);
            i_1;
        }
        let grp: u64 = 1u64.wrapping_add(len.wrapping_sub(1).wrapping_div(fmax));
        if grp > 0 && grp <= len {
        } else {
            __assert_fail(
                b"grp > 0 && grp <= len\0" as *const u8 as *const i8,
                b"main.c\0" as *const u8 as *const i8,
                58,
                (*::core::mem::transmute::<&[u8; 40], &[i8; 40]>(
                    b"void best_shuffle(const char *, char *)\0",
                ))
                .as_ptr(),
            );
        }
        'c_2002: {
            if grp > 0 && grp <= len {
            } else {
                __assert_fail(
                    b"grp > 0 && grp <= len\0" as *const u8 as *const i8,
                    b"main.c\0" as *const u8 as *const i8,
                    58,
                    (*::core::mem::transmute::<&[u8; 40], &[i8; 40]>(
                        b"void best_shuffle(const char *, char *)\0",
                    ))
                    .as_ptr(),
                );
            }
        };
        let lng: u64 = 1u64.wrapping_add(len.wrapping_sub(1).wrapping_rem(fmax));
        if lng > 0 && lng <= len {
        } else {
            __assert_fail(
                b"lng > 0 && lng <= len\0" as *const u8 as *const i8,
                b"main.c\0" as *const u8 as *const i8,
                62,
                (*::core::mem::transmute::<&[u8; 40], &[i8; 40]>(
                    b"void best_shuffle(const char *, char *)\0",
                ))
                .as_ptr(),
            );
        }
        'c_1947: {
            if lng > 0 && lng <= len {
            } else {
                __assert_fail(
                    b"lng > 0 && lng <= len\0" as *const u8 as *const i8,
                    b"main.c\0" as *const u8 as *const i8,
                    62,
                    (*::core::mem::transmute::<&[u8; 40], &[i8; 40]>(
                        b"void best_shuffle(const char *, char *)\0",
                    ))
                    .as_ptr(),
                );
            }
        };
        let mut i_2: u64 = 0;
        let mut j_0: u64 = 0;
        while i_2 < fmax {
            let first: u64 = *ndx2.offset(j_0 as isize);
            let glen: u64 = grp.wrapping_sub((if i_2 < lng { 0 } else { 1 }) as u64);
            let mut k: u64 = 1;
            while k < glen {
                *ndx1.offset(j_0.wrapping_add(k).wrapping_sub(1) as isize) =
                    *ndx2.offset(j_0.wrapping_add(k) as isize);
                k = k.wrapping_add(1);
                k;
            }
            *ndx1.offset(j_0.wrapping_add(glen).wrapping_sub(1) as isize) = first;
            j_0 = (j_0).wrapping_add(glen) as u64;
            i_2 = i_2.wrapping_add(1);
            i_2;
        }
        *result.offset(len as isize) = '\0' as i8;
        let mut i_3: u64 = 0;
        while i_3 < len {
            *result.offset(*ndx2.offset(i_3 as isize) as isize) =
                *txt.offset(*ndx1.offset(i_3 as isize) as isize);
            i_3 = i_3.wrapping_add(1);
            i_3;
        }
        free(ndx1 as *mut libc::c_void);
        free(ndx2 as *mut libc::c_void);
    }
}

#[no_mangle]
pub extern "C" fn display(mut txt1: *const i8, mut txt2: *const i8) {
    unsafe {
        let len: u64 = strlen(txt1);
        if len == strlen(txt2) {
        } else {
            __assert_fail(
                b"len == strlen(txt2)\0" as *const u8 as *const i8,
                b"main.c\0" as *const u8 as *const i8,
                85,
                (*::core::mem::transmute::<&[u8; 41], &[i8; 41]>(
                    b"void display(const char *, const char *)\0",
                ))
                .as_ptr(),
            );
        }
        'c_2427: {
            if len == strlen(txt2) {
            } else {
                __assert_fail(
                    b"len == strlen(txt2)\0" as *const u8 as *const i8,
                    b"main.c\0" as *const u8 as *const i8,
                    85,
                    (*::core::mem::transmute::<&[u8; 41], &[i8; 41]>(
                        b"void display(const char *, const char *)\0",
                    ))
                    .as_ptr(),
                );
            }
        };
        let mut score: i32 = 0;
        let mut i: u64 = 0;
        while i < len {
            if *txt1.offset(i as isize) as i32 == *txt2.offset(i as isize) as i32 {
                score += 1;
                score;
            }
            i = i.wrapping_add(1);
            i;
        }
        print!(
            "{}, {}, ({})\n",
            build_str_from_raw_ptr(txt1 as *mut u8),
            build_str_from_raw_ptr(txt2 as *mut u8),
            score
        );
    }
}

fn main_0() -> i32 {
    let mut data: [*const i8; 9] = [
        b"abracadabra\0" as *const u8 as *const i8,
        b"seesaw\0" as *const u8 as *const i8,
        b"elk\0" as *const u8 as *const i8,
        b"grrrrrr\0" as *const u8 as *const i8,
        b"up\0" as *const u8 as *const i8,
        b"a\0" as *const u8 as *const i8,
        b"aabbbbaa\0" as *const u8 as *const i8,
        b"\0" as *const u8 as *const i8,
        b"xxxxx\0" as *const u8 as *const i8,
    ];
    let data_len: u64 = (::core::mem::size_of::<[*const i8; 9]>() as u64)
        .wrapping_div(::core::mem::size_of::<*const i8>() as u64);
    let mut i: u64 = 0;
    unsafe {
        while i < data_len {
            let shuf_len: u64 = (strlen(data[i as usize])).wrapping_add(1);
            let vla = shuf_len as usize;
            let mut shuf: Vec<i8> = ::std::vec::from_elem(0, vla);
            memset(
                shuf.as_mut_ptr() as *mut libc::c_void,
                0xff,
                (vla * ::core::mem::size_of::<i8>()) as u64,
            );
            *shuf.as_mut_ptr().offset(shuf_len.wrapping_sub(1) as isize) = '\0' as i8;
            best_shuffle(data[i as usize], shuf.as_mut_ptr());
            display(data[i as usize], shuf.as_mut_ptr());
            i = i.wrapping_add(1);
            i;
        }
    }
    return 0;
}

pub fn main() {
    unsafe {
        ::std::process::exit(main_0() as i32);
    }
}
