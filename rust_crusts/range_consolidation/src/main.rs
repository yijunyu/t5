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
    fn qsort(__base: *mut libc::c_void, __nmemb: u64, __size: u64, __compar: __compar_fn_t);
}
pub type __compar_fn_t =
    Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct range_tag {
    pub low: f64,
    pub high: f64,
}
pub type range_t = range_tag;
#[no_mangle]
pub extern "C" fn normalize_range(mut range: *mut range_t) {
    unsafe {
        if (*range).high < (*range).low {
            let mut tmp: f64 = (*range).low;
            (*range).low = (*range).high;
            (*range).high = tmp;
        }
    }
}

#[no_mangle]
pub extern "C" fn range_compare(mut p1: *const libc::c_void, mut p2: *const libc::c_void) -> i32 {
    unsafe {
        let mut r1: *const range_t = p1 as *const range_t;
        let mut r2: *const range_t = p2 as *const range_t;
        if (*r1).low < (*r2).low {
            return -1;
        }
        if (*r1).low > (*r2).low {
            return 1;
        }
        if (*r1).high < (*r2).high {
            return -1;
        }
        if (*r1).high > (*r2).high {
            return 1;
        }
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn normalize_ranges(mut ranges: *mut range_t, mut count: u64) {
    unsafe {
        let mut i: u64 = 0;
        while i < count {
            normalize_range(&mut *ranges.offset(i as isize));
            i = i.wrapping_add(1);
            i;
        }
        qsort(
            ranges as *mut libc::c_void,
            count,
            ::core::mem::size_of::<range_t>() as u64,
            Some(
                range_compare
                    as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
            ),
        );
    }
}

#[no_mangle]
pub extern "C" fn consolidate_ranges(mut ranges: *mut range_t, mut count: u64) -> u64 {
    unsafe {
        normalize_ranges(ranges, count);
        let mut out_index: u64 = 0;
        let mut i: u64 = 0;
        while i < count {
            let mut j: u64 = i;
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
}

#[no_mangle]
pub extern "C" fn print_range(mut range: *const range_t) {
    print!("[{}, {}]", (*range).low, (*range).high);
}

#[no_mangle]
pub extern "C" fn print_ranges(mut ranges: *const range_t, mut count: u64) {
    unsafe {
        if count == 0 {
            return;
        }
        print_range(&*ranges.offset(0 as isize));
        let mut i: u64 = 1;
        while i < count {
            print!(", ");
            print_range(&*ranges.offset(i as isize));
            i = i.wrapping_add(1);
            i;
        }
    }
}

#[no_mangle]
pub extern "C" fn test_consolidate_ranges(mut ranges: *mut range_t, mut count: u64) {
    unsafe {
        print_ranges(ranges, count);
        print!(" -> ");
        count = consolidate_ranges(ranges, count);
        print_ranges(ranges, count);
        print!("\n");
    }
}

fn main_0() -> i32 {
    let mut test1: [range_t; 1] = [{
        let mut init = range_tag {
            low: 1.1f64,
            high: 2.2f64,
        };
        init
    }];
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
                low: 4 as f64,
                high: 3 as f64,
            };
            init
        },
        {
            let mut init = range_tag {
                low: 2 as f64,
                high: 1 as f64,
            };
            init
        },
    ];
    let mut test4: [range_t; 4] = [
        {
            let mut init = range_tag {
                low: 4 as f64,
                high: 3 as f64,
            };
            init
        },
        {
            let mut init = range_tag {
                low: 2 as f64,
                high: 1 as f64,
            };
            init
        },
        {
            let mut init = range_tag {
                low: -1i32 as f64,
                high: -2i32 as f64,
            };
            init
        },
        {
            let mut init = range_tag {
                low: 3.9f64,
                high: 10 as f64,
            };
            init
        },
    ];
    let mut test5: [range_t; 5] = [
        {
            let mut init = range_tag {
                low: 1 as f64,
                high: 3 as f64,
            };
            init
        },
        {
            let mut init = range_tag {
                low: -6i32 as f64,
                high: -1i32 as f64,
            };
            init
        },
        {
            let mut init = range_tag {
                low: -4i32 as f64,
                high: -5i32 as f64,
            };
            init
        },
        {
            let mut init = range_tag {
                low: 8 as f64,
                high: 2 as f64,
            };
            init
        },
        {
            let mut init = range_tag {
                low: -6i32 as f64,
                high: -6i32 as f64,
            };
            init
        },
    ];
    test_consolidate_ranges(
        test1.as_mut_ptr(),
        (::core::mem::size_of::<[range_t; 1]>() as u64)
            .wrapping_div(::core::mem::size_of::<range_t>() as u64),
    );
    test_consolidate_ranges(
        test2.as_mut_ptr(),
        (::core::mem::size_of::<[range_t; 2]>() as u64)
            .wrapping_div(::core::mem::size_of::<range_t>() as u64),
    );
    test_consolidate_ranges(
        test3.as_mut_ptr(),
        (::core::mem::size_of::<[range_t; 2]>() as u64)
            .wrapping_div(::core::mem::size_of::<range_t>() as u64),
    );
    test_consolidate_ranges(
        test4.as_mut_ptr(),
        (::core::mem::size_of::<[range_t; 4]>() as u64)
            .wrapping_div(::core::mem::size_of::<range_t>() as u64),
    );
    test_consolidate_ranges(
        test5.as_mut_ptr(),
        (::core::mem::size_of::<[range_t; 5]>() as u64)
            .wrapping_div(::core::mem::size_of::<range_t>() as u64),
    );
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
