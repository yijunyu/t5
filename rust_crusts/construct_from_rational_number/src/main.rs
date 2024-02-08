#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use c2rust_out::*;
extern "C" {
    fn printf(_: *const i8, _: ...) -> i32;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fraction {
    pub num: i32,
    pub den: i32,
}
#[no_mangle]
pub static mut examples: [fraction; 6] = [
    {
        let mut init = fraction { num: 1, den: 2 };
        init
    },
    {
        let mut init = fraction { num: 3, den: 1 };
        init
    },
    {
        let mut init = fraction { num: 23, den: 8 };
        init
    },
    {
        let mut init = fraction { num: 13, den: 11 };
        init
    },
    {
        let mut init = fraction { num: 22, den: 7 };
        init
    },
    {
        let mut init = fraction { num: -151, den: 77 };
        init
    },
];
#[no_mangle]
pub static mut sqrt2: [fraction; 4] = [
    {
        let mut init = fraction {
            num: 14142,
            den: 10000,
        };
        init
    },
    {
        let mut init = fraction {
            num: 141421,
            den: 100000,
        };
        init
    },
    {
        let mut init = fraction {
            num: 1414214,
            den: 1000000,
        };
        init
    },
    {
        let mut init = fraction {
            num: 14142136,
            den: 10000000,
        };
        init
    },
];
#[no_mangle]
pub static mut pi: [fraction; 8] = [
    {
        let mut init = fraction { num: 31, den: 10 };
        init
    },
    {
        let mut init = fraction { num: 314, den: 100 };
        init
    },
    {
        let mut init = fraction {
            num: 3142,
            den: 1000,
        };
        init
    },
    {
        let mut init = fraction {
            num: 31428,
            den: 10000,
        };
        init
    },
    {
        let mut init = fraction {
            num: 314285,
            den: 100000,
        };
        init
    },
    {
        let mut init = fraction {
            num: 3142857,
            den: 1000000,
        };
        init
    },
    {
        let mut init = fraction {
            num: 31428571,
            den: 10000000,
        };
        init
    },
    {
        let mut init = fraction {
            num: 314285714,
            den: 100000000,
        };
        init
    },
];
#[no_mangle]
pub extern "C" fn r2cf(mut numerator: *mut i32, mut denominator: *mut i32) -> i32 {
    unsafe {
        let mut quotient: i32 = 0;
        let mut temp: i32 = 0;
        if !denominator.is_null() {
            quotient = *numerator / *denominator;
            temp = *numerator;
            *numerator = *denominator;
            *denominator = temp % *denominator;
        }
        return quotient;
    }
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    print!("Running the examples :");
    i = 0;
    unsafe {
        while (i as u64)
            < (::core::mem::size_of::<[fraction; 6]>() as u64)
                .wrapping_div(::core::mem::size_of::<fraction>() as u64)
        {
            print!(
                "\nFor N = {}, D = {} :",
                examples[i as usize].num, examples[i as usize].den
            );
            while examples[i as usize].den != 0 {
                print!(
                    " {} ",
                    r2cf(
                        &mut (*examples.as_mut_ptr().offset(i as isize)).num,
                        &mut (*examples.as_mut_ptr().offset(i as isize)).den,
                    )
                );
            }
            i += 1;
            i;
        }
        printf(b"\n\nRunning for %c2 :\0" as *const u8 as *const i8, 251);
    }
    i = 0;
    unsafe {
        while (i as u64)
            < (::core::mem::size_of::<[fraction; 4]>() as u64)
                .wrapping_div(::core::mem::size_of::<fraction>() as u64)
        {
            print!(
                "\nFor N = {}, D = {} :",
                sqrt2[i as usize].num, sqrt2[i as usize].den
            );
            while sqrt2[i as usize].den != 0 {
                print!(
                    " {} ",
                    r2cf(
                        &mut (*sqrt2.as_mut_ptr().offset(i as isize)).num,
                        &mut (*sqrt2.as_mut_ptr().offset(i as isize)).den,
                    )
                );
            }
            i += 1;
            i;
        }
    }
    print!("\n\nRunning for {} :", 227);
    i = 0;
    unsafe {
        while (i as u64)
            < (::core::mem::size_of::<[fraction; 8]>() as u64)
                .wrapping_div(::core::mem::size_of::<fraction>() as u64)
        {
            print!(
                "\nFor N = {}, D = {} :",
                pi[i as usize].num, pi[i as usize].den
            );
            while pi[i as usize].den != 0 {
                print!(
                    " {} ",
                    r2cf(
                        &mut (*pi.as_mut_ptr().offset(i as isize)).num,
                        &mut (*pi.as_mut_ptr().offset(i as isize)).den,
                    )
                );
            }
            i += 1;
            i;
        }
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
