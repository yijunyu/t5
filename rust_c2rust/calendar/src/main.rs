#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn putchar(__c: libc::c_int) -> libc::c_int;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct months {
    pub name: *const libc::c_char,
    pub days: libc::c_int,
    pub start_wday: libc::c_int,
    pub at: libc::c_int,
}
#[no_mangle]
pub static mut width: libc::c_int = 80 as libc::c_int;
#[no_mangle]
pub static mut year: libc::c_int = 1969 as libc::c_int;
#[no_mangle]
pub static mut cols: libc::c_int = 0;
#[no_mangle]
pub static mut lead: libc::c_int = 0;
#[no_mangle]
pub static mut gap: libc::c_int = 0;
#[no_mangle]
pub static mut wdays: [*const libc::c_char; 7] = [
    b"Su\0" as *const u8 as *const libc::c_char,
    b"Mo\0" as *const u8 as *const libc::c_char,
    b"Tu\0" as *const u8 as *const libc::c_char,
    b"We\0" as *const u8 as *const libc::c_char,
    b"Th\0" as *const u8 as *const libc::c_char,
    b"Fr\0" as *const u8 as *const libc::c_char,
    b"Sa\0" as *const u8 as *const libc::c_char,
];
#[no_mangle]
pub static mut months: [months; 12] = [
    {
        let mut init = months {
            name: b"January\0" as *const u8 as *const libc::c_char,
            days: 31 as libc::c_int,
            start_wday: 0 as libc::c_int,
            at: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = months {
            name: b"February\0" as *const u8 as *const libc::c_char,
            days: 28 as libc::c_int,
            start_wday: 0 as libc::c_int,
            at: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = months {
            name: b"March\0" as *const u8 as *const libc::c_char,
            days: 31 as libc::c_int,
            start_wday: 0 as libc::c_int,
            at: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = months {
            name: b"April\0" as *const u8 as *const libc::c_char,
            days: 30 as libc::c_int,
            start_wday: 0 as libc::c_int,
            at: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = months {
            name: b"May\0" as *const u8 as *const libc::c_char,
            days: 31 as libc::c_int,
            start_wday: 0 as libc::c_int,
            at: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = months {
            name: b"June\0" as *const u8 as *const libc::c_char,
            days: 30 as libc::c_int,
            start_wday: 0 as libc::c_int,
            at: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = months {
            name: b"July\0" as *const u8 as *const libc::c_char,
            days: 31 as libc::c_int,
            start_wday: 0 as libc::c_int,
            at: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = months {
            name: b"August\0" as *const u8 as *const libc::c_char,
            days: 31 as libc::c_int,
            start_wday: 0 as libc::c_int,
            at: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = months {
            name: b"September\0" as *const u8 as *const libc::c_char,
            days: 30 as libc::c_int,
            start_wday: 0 as libc::c_int,
            at: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = months {
            name: b"October\0" as *const u8 as *const libc::c_char,
            days: 31 as libc::c_int,
            start_wday: 0 as libc::c_int,
            at: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = months {
            name: b"November\0" as *const u8 as *const libc::c_char,
            days: 30 as libc::c_int,
            start_wday: 0 as libc::c_int,
            at: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = months {
            name: b"December\0" as *const u8 as *const libc::c_char,
            days: 31 as libc::c_int,
            start_wday: 0 as libc::c_int,
            at: 0 as libc::c_int,
        };
        init
    },
];
#[no_mangle]
pub unsafe extern "C" fn space(mut n: libc::c_int) {
    loop {
        let fresh0 = n;
        n = n - 1;
        if !(fresh0 > 0 as libc::c_int) {
            break;
        }
        putchar(' ' as i32);
    };
}
#[no_mangle]
pub unsafe extern "C" fn init_months() {
    let mut i: libc::c_int = 0;
    if year % 4 as libc::c_int == 0 && year % 100 as libc::c_int != 0
        || year % 400 as libc::c_int == 0
    {
        months[1 as libc::c_int as usize].days = 29 as libc::c_int;
    }
    year -= 1;
    year;
    months[0 as libc::c_int as usize]
        .start_wday = (year * 365 as libc::c_int + year / 4 as libc::c_int
        - year / 100 as libc::c_int + year / 400 as libc::c_int + 1 as libc::c_int)
        % 7 as libc::c_int;
    i = 1 as libc::c_int;
    while i < 12 as libc::c_int {
        months[i as usize]
            .start_wday = (months[(i - 1 as libc::c_int) as usize].start_wday
            + months[(i - 1 as libc::c_int) as usize].days) % 7 as libc::c_int;
        i += 1;
        i;
    }
    cols = (width + 2 as libc::c_int) / 22 as libc::c_int;
    while 12 as libc::c_int % cols != 0 {
        cols -= 1;
        cols;
    }
    gap = if cols - 1 as libc::c_int != 0 {
        (width - 20 as libc::c_int * cols) / (cols - 1 as libc::c_int)
    } else {
        0 as libc::c_int
    };
    if gap > 4 as libc::c_int {
        gap = 4 as libc::c_int;
    }
    lead = (width - (20 as libc::c_int + gap) * cols + gap + 1 as libc::c_int)
        / 2 as libc::c_int;
    year += 1;
    year;
}
#[no_mangle]
pub unsafe extern "C" fn print_row(mut row: libc::c_int) {
    let mut c: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut from: libc::c_int = row * cols;
    let mut to: libc::c_int = from + cols;
    space(lead);
    c = from;
    while c < to {
        i = strlen(months[c as usize].name) as libc::c_int;
        space((20 as libc::c_int - i) / 2 as libc::c_int);
        printf(b"%s\0" as *const u8 as *const libc::c_char, months[c as usize].name);
        space(
            20 as libc::c_int - i - (20 as libc::c_int - i) / 2 as libc::c_int
                + (if c == to - 1 as libc::c_int { 0 as libc::c_int } else { gap }),
        );
        c += 1;
        c;
    }
    putchar('\n' as i32);
    space(lead);
    c = from;
    while c < to {
        i = 0 as libc::c_int;
        while i < 7 as libc::c_int {
            printf(
                b"%s%s\0" as *const u8 as *const libc::c_char,
                wdays[i as usize],
                if i == 6 as libc::c_int {
                    b"\0" as *const u8 as *const libc::c_char
                } else {
                    b" \0" as *const u8 as *const libc::c_char
                },
            );
            i += 1;
            i;
        }
        if c < to - 1 as libc::c_int {
            space(gap);
        } else {
            putchar('\n' as i32);
        }
        c += 1;
        c;
    }
    loop {
        c = from;
        while c < to {
            if months[c as usize].at < months[c as usize].days {
                break;
            }
            c += 1;
            c;
        }
        if c == to {
            break;
        }
        space(lead);
        c = from;
        while c < to {
            i = 0 as libc::c_int;
            while i < months[c as usize].start_wday {
                space(3 as libc::c_int);
                i += 1;
                i;
            }
            loop {
                let fresh1 = i;
                i = i + 1;
                if !(fresh1 < 7 as libc::c_int
                    && months[c as usize].at < months[c as usize].days)
                {
                    break;
                }
                months[c as usize].at += 1;
                printf(
                    b"%2d\0" as *const u8 as *const libc::c_char,
                    months[c as usize].at,
                );
                if i < 7 as libc::c_int || c < to - 1 as libc::c_int {
                    putchar(' ' as i32);
                }
            }
            loop {
                let fresh2 = i;
                i = i + 1;
                if !(fresh2 <= 7 as libc::c_int && c < to - 1 as libc::c_int) {
                    break;
                }
                space(3 as libc::c_int);
            }
            if c < to - 1 as libc::c_int {
                space(gap - 1 as libc::c_int);
            }
            months[c as usize].start_wday = 0 as libc::c_int;
            c += 1;
            c;
        }
        putchar('\n' as i32);
    }
    putchar('\n' as i32);
}
#[no_mangle]
pub unsafe extern "C" fn print_year() {
    let mut row: libc::c_int = 0;
    let mut buf: [libc::c_char; 32] = [0; 32];
    sprintf(buf.as_mut_ptr(), b"%d\0" as *const u8 as *const libc::c_char, year);
    space(
        (width as libc::c_ulong)
            .wrapping_sub(strlen(buf.as_mut_ptr()))
            .wrapping_div(2 as libc::c_int as libc::c_ulong) as libc::c_int,
    );
    printf(b"%s\n\n\0" as *const u8 as *const libc::c_char, buf.as_mut_ptr());
    row = 0 as libc::c_int;
    while row * cols < 12 as libc::c_int {
        print_row(row);
        row += 1;
        row;
    }
}
unsafe fn main_0(mut c: libc::c_int, mut v: *mut *mut libc::c_char) -> libc::c_int {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut year_set: libc::c_int = 0 as libc::c_int;
    i = 1 as libc::c_int;
    loop {
        if !(i < c) {
            current_block = 1394248824506584008;
            break;
        }
        if strcmp(*v.offset(i as isize), b"-w\0" as *const u8 as *const libc::c_char)
            == 0
        {
            i += 1;
            if i == c
                || {
                    width = atoi(*v.offset(i as isize));
                    width < 20 as libc::c_int
                }
            {
                current_block = 2002819764153645144;
                break;
            }
        } else {
            if !(year_set == 0) {
                current_block = 2002819764153645144;
                break;
            }
            if sscanf(
                *v.offset(i as isize),
                b"%d\0" as *const u8 as *const libc::c_char,
                &mut year as *mut libc::c_int,
            ) == 0 || year <= 0 as libc::c_int
            {
                year = 1969 as libc::c_int;
            }
            year_set = 1 as libc::c_int;
        }
        i += 1;
        i;
    }
    match current_block {
        2002819764153645144 => {
            fprintf(
                stderr,
                b"bad args\nUsage: %s year [-w width (>= 20)]\n\0" as *const u8
                    as *const libc::c_char,
                *v.offset(0 as libc::c_int as isize),
            );
            exit(1 as libc::c_int);
        }
        _ => {
            init_months();
            print_year();
            return 0 as libc::c_int;
        }
    };
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
