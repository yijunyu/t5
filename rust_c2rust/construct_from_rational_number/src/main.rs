#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fraction {
    pub num: libc::c_int,
    pub den: libc::c_int,
}
#[no_mangle]
pub static mut examples: [fraction; 6] = [
    {
        let mut init = fraction {
            num: 1 as libc::c_int,
            den: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = fraction {
            num: 3 as libc::c_int,
            den: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = fraction {
            num: 23 as libc::c_int,
            den: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = fraction {
            num: 13 as libc::c_int,
            den: 11 as libc::c_int,
        };
        init
    },
    {
        let mut init = fraction {
            num: 22 as libc::c_int,
            den: 7 as libc::c_int,
        };
        init
    },
    {
        let mut init = fraction {
            num: -(151 as libc::c_int),
            den: 77 as libc::c_int,
        };
        init
    },
];
#[no_mangle]
pub static mut sqrt2: [fraction; 4] = [
    {
        let mut init = fraction {
            num: 14142 as libc::c_int,
            den: 10000 as libc::c_int,
        };
        init
    },
    {
        let mut init = fraction {
            num: 141421 as libc::c_int,
            den: 100000 as libc::c_int,
        };
        init
    },
    {
        let mut init = fraction {
            num: 1414214 as libc::c_int,
            den: 1000000 as libc::c_int,
        };
        init
    },
    {
        let mut init = fraction {
            num: 14142136 as libc::c_int,
            den: 10000000 as libc::c_int,
        };
        init
    },
];
#[no_mangle]
pub static mut pi: [fraction; 8] = [
    {
        let mut init = fraction {
            num: 31 as libc::c_int,
            den: 10 as libc::c_int,
        };
        init
    },
    {
        let mut init = fraction {
            num: 314 as libc::c_int,
            den: 100 as libc::c_int,
        };
        init
    },
    {
        let mut init = fraction {
            num: 3142 as libc::c_int,
            den: 1000 as libc::c_int,
        };
        init
    },
    {
        let mut init = fraction {
            num: 31428 as libc::c_int,
            den: 10000 as libc::c_int,
        };
        init
    },
    {
        let mut init = fraction {
            num: 314285 as libc::c_int,
            den: 100000 as libc::c_int,
        };
        init
    },
    {
        let mut init = fraction {
            num: 3142857 as libc::c_int,
            den: 1000000 as libc::c_int,
        };
        init
    },
    {
        let mut init = fraction {
            num: 31428571 as libc::c_int,
            den: 10000000 as libc::c_int,
        };
        init
    },
    {
        let mut init = fraction {
            num: 314285714 as libc::c_int,
            den: 100000000 as libc::c_int,
        };
        init
    },
];
#[no_mangle]
pub unsafe extern "C" fn r2cf(
    mut numerator: *mut libc::c_int,
    mut denominator: *mut libc::c_int,
) -> libc::c_int {
    let mut quotient: libc::c_int = 0 as libc::c_int;
    let mut temp: libc::c_int = 0;
    if !denominator.is_null() {
        quotient = *numerator / *denominator;
        temp = *numerator;
        *numerator = *denominator;
        *denominator = temp % *denominator;
    }
    return quotient;
}
unsafe fn main_0() -> libc::c_int {
    let mut i: libc::c_int = 0;
    printf(b"Running the examples :\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[fraction; 6]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<fraction>() as libc::c_ulong)
    {
        printf(
            b"\nFor N = %d, D = %d :\0" as *const u8 as *const libc::c_char,
            examples[i as usize].num,
            examples[i as usize].den,
        );
        while examples[i as usize].den != 0 as libc::c_int {
            printf(
                b" %d \0" as *const u8 as *const libc::c_char,
                r2cf(
                    &mut (*examples.as_mut_ptr().offset(i as isize)).num,
                    &mut (*examples.as_mut_ptr().offset(i as isize)).den,
                ),
            );
        }
        i += 1;
        i;
    }
    printf(
        b"\n\nRunning for %c2 :\0" as *const u8 as *const libc::c_char,
        251 as libc::c_int,
    );
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[fraction; 4]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<fraction>() as libc::c_ulong)
    {
        printf(
            b"\nFor N = %d, D = %d :\0" as *const u8 as *const libc::c_char,
            sqrt2[i as usize].num,
            sqrt2[i as usize].den,
        );
        while sqrt2[i as usize].den != 0 as libc::c_int {
            printf(
                b" %d \0" as *const u8 as *const libc::c_char,
                r2cf(
                    &mut (*sqrt2.as_mut_ptr().offset(i as isize)).num,
                    &mut (*sqrt2.as_mut_ptr().offset(i as isize)).den,
                ),
            );
        }
        i += 1;
        i;
    }
    printf(
        b"\n\nRunning for %c :\0" as *const u8 as *const libc::c_char,
        227 as libc::c_int,
    );
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[fraction; 8]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<fraction>() as libc::c_ulong)
    {
        printf(
            b"\nFor N = %d, D = %d :\0" as *const u8 as *const libc::c_char,
            pi[i as usize].num,
            pi[i as usize].den,
        );
        while pi[i as usize].den != 0 as libc::c_int {
            printf(
                b" %d \0" as *const u8 as *const libc::c_char,
                r2cf(
                    &mut (*pi.as_mut_ptr().offset(i as isize)).num,
                    &mut (*pi.as_mut_ptr().offset(i as isize)).den,
                ),
            );
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
