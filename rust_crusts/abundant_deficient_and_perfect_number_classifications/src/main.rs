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
extern "C" {}
fn main_0() -> i32 {
    let mut sum: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut try_max: i32 = 0;
    let mut count_list: [i32; 3] = [1, 0, 0];
    i = 2;
    while i <= 20000 {
        try_max = i / 2;
        sum = 1;
        j = 2;
        while j < try_max {
            if !(i % j != 0) {
                try_max = i / j;
                sum += j;
                if j != try_max {
                    sum += try_max;
                }
            }
            j += 1;
            j;
        }
        if sum < i {
            count_list[0 as usize] += 1;
            count_list[0 as usize];
        } else if sum > i {
            count_list[2 as usize] += 1;
            count_list[2 as usize];
        } else {
            count_list[1 as usize] += 1;
            count_list[1 as usize];
        }
        i += 1;
        i;
    }
    print!("\nThere are {} deficient,", count_list[0 as usize]);
    print!(" {} perfect,", count_list[1 as usize]);
    print!(
        " {} abundant numbers between 1 and 20000.\n",
        count_list[2 as usize]
    );
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
