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
    fn scanf(_: *const i8, _: ...) -> i32;
}
fn main_0() -> i32 {
    let mut curr: [[i32; 5]; 5] = [[0; 5]; 5];
    let mut max_claim: [[i32; 5]; 5] = [[0; 5]; 5];
    let mut avl: [i32; 5] = [0; 5];
    let mut alloc: [i32; 5] = [0; 5];
    let mut max_res: [i32; 5] = [0; 5];
    let mut running: [i32; 5] = [0; 5];
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut exec: i32 = 0;
    let mut r: i32 = 0;
    let mut p: i32 = 0;
    let mut count: i32 = 0;
    let mut safe: bool = 0 != 0;
    print!("\nEnter the number of resources: ");
    unsafe {
        scanf(b"%d\0" as *const u8 as *const i8, &mut r as *mut i32);
    }
    print!("\nEnter the number of processes: ");
    unsafe {
        scanf(b"%d\0" as *const u8 as *const i8, &mut p as *mut i32);
    }
    i = 0;
    while i < p {
        running[i as usize] = 1;
        count += 1;
        count;
        i += 1;
        i;
    }
    print!("\nEnter Claim Vector: ");
    i = 0;
    unsafe {
        while i < r {
            scanf(
                b"%d\0" as *const u8 as *const i8,
                &mut *max_res.as_mut_ptr().offset(i as isize) as *mut i32,
            );
            i += 1;
            i;
        }
    }
    print!("\nEnter Allocated Resource Table: ");
    i = 0;
    unsafe {
        while i < p {
            j = 0;
            while j < r {
                scanf(
                    b"%d\0" as *const u8 as *const i8,
                    &mut *(*curr.as_mut_ptr().offset(i as isize))
                        .as_mut_ptr()
                        .offset(j as isize) as *mut i32,
                );
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    }
    print!("\nEnter Maximum Claim table: ");
    i = 0;
    unsafe {
        while i < p {
            j = 0;
            while j < r {
                scanf(
                    b"%d\0" as *const u8 as *const i8,
                    &mut *(*max_claim.as_mut_ptr().offset(i as isize))
                        .as_mut_ptr()
                        .offset(j as isize) as *mut i32,
                );
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    }
    print!("\nThe Claim Vector is: ");
    i = 0;
    while i < r {
        print!("{} ", max_res[i as usize]);
        i += 1;
        i;
    }
    print!("\nThe Allocated Resource Table:\n");
    i = 0;
    while i < p {
        j = 0;
        while j < r {
            print!("	{}", curr[i as usize][j as usize]);
            j += 1;
            j;
        }
        print!("\n");
        i += 1;
        i;
    }
    print!("\nThe Maximum Claim Table:\n");
    i = 0;
    while i < p {
        j = 0;
        while j < r {
            print!("	{}", max_claim[i as usize][j as usize]);
            j += 1;
            j;
        }
        print!("\n");
        i += 1;
        i;
    }
    i = 0;
    while i < p {
        j = 0;
        while j < r {
            alloc[j as usize] += curr[i as usize][j as usize];
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    print!("\nAllocated resources: ");
    i = 0;
    while i < r {
        print!("{} ", alloc[i as usize]);
        i += 1;
        i;
    }
    i = 0;
    while i < r {
        avl[i as usize] = max_res[i as usize] - alloc[i as usize];
        i += 1;
        i;
    }
    print!("\nAvailable resources: ");
    i = 0;
    while i < r {
        print!("{} ", avl[i as usize]);
        i += 1;
        i;
    }
    print!("\n");
    while count != 0 {
        safe = 0 != 0;
        i = 0;
        while i < p {
            if running[i as usize] != 0 {
                exec = 1;
                j = 0;
                while j < r {
                    if max_claim[i as usize][j as usize] - curr[i as usize][j as usize]
                        > avl[j as usize]
                    {
                        exec = 0;
                        break;
                    } else {
                        j += 1;
                        j;
                    }
                }
                if exec != 0 {
                    print!("\nProcess{} is executing.\n", i + 1);
                    running[i as usize] = 0;
                    count -= 1;
                    count;
                    safe = 1 != 0;
                    j = 0;
                    while j < r {
                        avl[j as usize] += curr[i as usize][j as usize];
                        j += 1;
                        j;
                    }
                    break;
                }
            }
            i += 1;
            i;
        }
        if !safe {
            print!("\nThe processes are in unsafe state.");
            break;
        } else {
            if safe {
                print!("\nThe process is in safe state.");
            }
            print!("\nAvailable vector: ");
            i = 0;
            while i < r {
                print!("{} ", avl[i as usize]);
                i += 1;
                i;
            }
        }
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
