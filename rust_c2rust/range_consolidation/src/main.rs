#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
}
pub type size_t = libc::c_ulong;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct range_tag {
    pub low: libc::c_double,
    pub high: libc::c_double,
}
pub type range_t = range_tag;
#[no_mangle]
pub unsafe extern "C" fn normalize_range(mut range: *mut range_t) {
    if (*range).high < (*range).low {
        let mut tmp: libc::c_double = (*range).low;
        (*range).low = (*range).high;
        (*range).high = tmp;
    }
}
#[no_mangle]
pub unsafe extern "C" fn range_compare(
    mut p1: *const libc::c_void,
    mut p2: *const libc::c_void,
) -> libc::c_int {
    let mut r1: *const range_t = p1 as *const range_t;
    let mut r2: *const range_t = p2 as *const range_t;
    if (*r1).low < (*r2).low {
        return -(1 as libc::c_int);
    }
    if (*r1).low > (*r2).low {
        return 1 as libc::c_int;
    }
    if (*r1).high < (*r2).high {
        return -(1 as libc::c_int);
    }
    if (*r1).high > (*r2).high {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn normalize_ranges(mut ranges: *mut range_t, mut count: size_t) {
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < count {
        normalize_range(&mut *ranges.offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    qsort(
        ranges as *mut libc::c_void,
        count,
        ::core::mem::size_of::<range_t>() as libc::c_ulong,
        Some(
            range_compare
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn consolidate_ranges(
    mut ranges: *mut range_t,
    mut count: size_t,
) -> size_t {
    normalize_ranges(ranges, count);
    let mut out_index: size_t = 0 as libc::c_int as size_t;
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < count {
        let mut j: size_t = i;
        loop {
            j = j.wrapping_add(1);
            if !(j < count
                && (*ranges.offset(j as isize)).low <= (*ranges.offset(i as isize)).high)
            {
                break;
            }
            if (*ranges.offset(i as isize)).high < (*ranges.offset(j as isize)).high {
                (*ranges.offset(i as isize)).high = (*ranges.offset(j as isize)).high;
            }
        }
        let fresh0 = out_index;
        out_index = out_index.wrapping_add(1);
        *ranges.offset(fresh0 as isize) = *ranges.offset(i as isize);
        i = j;
    }
    return out_index;
}
#[no_mangle]
pub unsafe extern "C" fn print_range(mut range: *const range_t) {
    printf(
        b"[%g, %g]\0" as *const u8 as *const libc::c_char,
        (*range).low,
        (*range).high,
    );
}
#[no_mangle]
pub unsafe extern "C" fn print_ranges(mut ranges: *const range_t, mut count: size_t) {
    if count == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    print_range(&*ranges.offset(0 as libc::c_int as isize));
    let mut i: size_t = 1 as libc::c_int as size_t;
    while i < count {
        printf(b", \0" as *const u8 as *const libc::c_char);
        print_range(&*ranges.offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn test_consolidate_ranges(
    mut ranges: *mut range_t,
    mut count: size_t,
) {
    print_ranges(ranges, count);
    printf(b" -> \0" as *const u8 as *const libc::c_char);
    count = consolidate_ranges(ranges, count);
    print_ranges(ranges, count);
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
unsafe fn main_0() -> libc::c_int {
    let mut test1: [range_t; 1] = [
        {
            let mut init = range_tag {
                low: 1.1f64,
                high: 2.2f64,
            };
            init
        },
    ];
    let mut test2: [range_t; 2] = [
        {
            let mut init = range_tag {
                low: 6.1f64,
                high: 7.2f64,
            };
            init
        },
        {
            let mut init = range_tag {
                low: 7.2f64,
                high: 8.3f64,
            };
            init
        },
    ];
    let mut test3: [range_t; 2] = [
        {
            let mut init = range_tag {
                low: 4 as libc::c_int as libc::c_double,
                high: 3 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = range_tag {
                low: 2 as libc::c_int as libc::c_double,
                high: 1 as libc::c_int as libc::c_double,
            };
            init
        },
    ];
    let mut test4: [range_t; 4] = [
        {
            let mut init = range_tag {
                low: 4 as libc::c_int as libc::c_double,
                high: 3 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = range_tag {
                low: 2 as libc::c_int as libc::c_double,
                high: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = range_tag {
                low: -(1 as libc::c_int) as libc::c_double,
                high: -(2 as libc::c_int) as libc::c_double,
            };
            init
        },
        {
            let mut init = range_tag {
                low: 3.9f64,
                high: 10 as libc::c_int as libc::c_double,
            };
            init
        },
    ];
    let mut test5: [range_t; 5] = [
        {
            let mut init = range_tag {
                low: 1 as libc::c_int as libc::c_double,
                high: 3 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = range_tag {
                low: -(6 as libc::c_int) as libc::c_double,
                high: -(1 as libc::c_int) as libc::c_double,
            };
            init
        },
        {
            let mut init = range_tag {
                low: -(4 as libc::c_int) as libc::c_double,
                high: -(5 as libc::c_int) as libc::c_double,
            };
            init
        },
        {
            let mut init = range_tag {
                low: 8 as libc::c_int as libc::c_double,
                high: 2 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = range_tag {
                low: -(6 as libc::c_int) as libc::c_double,
                high: -(6 as libc::c_int) as libc::c_double,
            };
            init
        },
    ];
    test_consolidate_ranges(
        test1.as_mut_ptr(),
        (::core::mem::size_of::<[range_t; 1]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<range_t>() as libc::c_ulong),
    );
    test_consolidate_ranges(
        test2.as_mut_ptr(),
        (::core::mem::size_of::<[range_t; 2]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<range_t>() as libc::c_ulong),
    );
    test_consolidate_ranges(
        test3.as_mut_ptr(),
        (::core::mem::size_of::<[range_t; 2]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<range_t>() as libc::c_ulong),
    );
    test_consolidate_ranges(
        test4.as_mut_ptr(),
        (::core::mem::size_of::<[range_t; 4]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<range_t>() as libc::c_ulong),
    );
    test_consolidate_ranges(
        test5.as_mut_ptr(),
        (::core::mem::size_of::<[range_t; 5]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<range_t>() as libc::c_ulong),
    );
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
