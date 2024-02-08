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
}
fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut len: i32 = 0;
        let mut reference: i8 = 0;
        if argc > 2 {
            print!(
                "Usage : {} <Test String>\n",
                build_str_from_raw_ptr(*argv.offset(0 as isize) as *mut u8)
            );
            return 0;
        }
        if argc == 1 || strlen(*argv.offset(1 as isize)) == 1 {
            if argc == 1 {
                if argc == 1 {
                    print!(
                        "Input string : \"{}\"\nLength : {}\nAll characters are identical.\n",
                        "\0", 0
                    )
                } else {
                    print!(
                        "Input string : \"{}\"\nLength : {}\nAll characters are identical.\n",
                        "\0",
                        strlen(*argv.offset(1 as isize)) as i32
                    )
                }
            } else {
                if argc == 1 {
                    print!(
                        "Input string : \"{}\"\nLength : {}\nAll characters are identical.\n",
                        build_str_from_raw_ptr(*argv.offset(1 as isize) as *const i8 as *mut u8),
                        0
                    )
                } else {
                    print!(
                        "Input string : \"{}\"\nLength : {}\nAll characters are identical.\n",
                        build_str_from_raw_ptr(*argv.offset(1 as isize) as *const i8 as *mut u8),
                        strlen(*argv.offset(1 as isize)) as i32
                    )
                }
            };
            return 0;
        }
        reference = *(*argv.offset(1 as isize)).offset(0 as isize);
        len = strlen(*argv.offset(1 as isize)) as i32;
        i = 1;
        while i < len {
            if *(*argv.offset(1 as isize)).offset(i as isize) as i32 != reference as i32 {
                print! ("Input string : \"{}\"\nLength : {}\nFirst different character : \"{}\"(0x{:x}) at position : {}\n", build_str_from_raw_ptr (* argv.offset (1 as isize) as * mut u8), len, * (* argv.offset (1 as isize)).offset (i as isize) as i32, *
                  (* argv.offset (1 as isize)).offset (i as isize) as i32, i + 1);
                return 0;
            }
            i += 1;
            i;
        }
        print!(
            "Input string : \"{}\"\nLength : {}\nAll characters are identical.\n",
            build_str_from_raw_ptr(*argv.offset(1 as isize) as *mut u8),
            len
        );
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
