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

use std::time::SystemTime;
pub fn rust_time(ref_result: Option<&mut i64>) -> i64 {
    let result = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    };
    match ref_result {
        Some(r) => *r = result,
        None => {}
    }
    return result as i64;
}

use c2rust_out::*;
extern "C" {
    fn atoi(__nptr: *const i8) -> i32;
    fn strtoull(_: *const i8, _: *mut *mut i8, _: i32) -> u64;
    fn rand() -> i32;
    fn srand(__seed: u32);
    fn malloc(_: u64) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct drawer {
    pub cardNum: i32,
    pub hasBeenOpened: bool,
}
#[no_mangle]
pub static mut drawerSet: *mut drawer = 0 as *const drawer as *mut drawer;
#[no_mangle]
pub extern "C" fn initialize(mut prisoners: i32) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut card: i32 = 0;
    let mut unique: bool = false;
    unsafe {
        drawerSet =
            (malloc((prisoners as u64).wrapping_mul(::core::mem::size_of::<drawer>() as u64))
                as *mut drawer)
                .offset(-(1 as isize));
        card = rand() % prisoners + 1;
        *drawerSet.offset(1 as isize) = {
            let mut init = drawer {
                cardNum: card,
                hasBeenOpened: 0 != 0,
            };
            init
        };
        i = 1 + 1;
        while i < prisoners + 1 {
            unique = 0 != 0;
            while unique as i32 == 0 {
                j = 0;
                while j < i {
                    if (*drawerSet.offset(j as isize)).cardNum == card {
                        card = rand() % prisoners + 1;
                        break;
                    } else {
                        j += 1;
                        j;
                    }
                }
                if j == i {
                    unique = 1 != 0;
                }
            }
            *drawerSet.offset(i as isize) = {
                let mut init = drawer {
                    cardNum: card,
                    hasBeenOpened: 0 != 0,
                };
                init
            };
            i += 1;
            i;
        }
    }
}

#[no_mangle]
pub extern "C" fn closeAllDrawers(mut prisoners: i32) {
    let mut i: i32 = 0;
    i = 1;
    unsafe {
        while i < prisoners + 1 {
            (*drawerSet.offset(i as isize)).hasBeenOpened = 0 != 0;
            i += 1;
            i;
        }
    }
}

#[no_mangle]
pub extern "C" fn libertyOrDeathAtRandom(mut prisoners: i32, mut chances: i32) -> bool {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut chosenDrawer: i32 = 0;
    i = 1;
    unsafe {
        while i < prisoners + 1 {
            let mut foundCard: bool = 0 != 0;
            j = 0;
            while j < chances {
                loop {
                    chosenDrawer = rand() % prisoners + 1;
                    if !((*drawerSet.offset(chosenDrawer as isize)).hasBeenOpened as i32 == 1) {
                        break;
                    }
                }
                if (*drawerSet.offset(chosenDrawer as isize)).cardNum == i {
                    foundCard = 1 != 0;
                    break;
                } else {
                    (*drawerSet.offset(chosenDrawer as isize)).hasBeenOpened = 1 != 0;
                    j += 1;
                    j;
                }
            }
            closeAllDrawers(prisoners);
            if foundCard as i32 == 0 {
                return 1 != 0;
            }
            i += 1;
            i;
        }
    }
    return 0 != 0;
}

#[no_mangle]
pub extern "C" fn libertyOrDeathPlanned(mut prisoners: i32, mut chances: i32) -> bool {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut chosenDrawer: i32 = 0;
    i = 1;
    unsafe {
        while i < prisoners + 1 {
            chosenDrawer = i;
            let mut foundCard: bool = 0 != 0;
            j = 0;
            while j < chances {
                (*drawerSet.offset(chosenDrawer as isize)).hasBeenOpened = 1 != 0;
                if (*drawerSet.offset(chosenDrawer as isize)).cardNum == i {
                    foundCard = 1 != 0;
                    break;
                } else {
                    if chosenDrawer == (*drawerSet.offset(chosenDrawer as isize)).cardNum {
                        loop {
                            chosenDrawer = rand() % prisoners + 1;
                            if !((*drawerSet.offset(chosenDrawer as isize)).hasBeenOpened as i32
                                == 1)
                            {
                                break;
                            }
                        }
                    } else {
                        chosenDrawer = (*drawerSet.offset(chosenDrawer as isize)).cardNum;
                    }
                    j += 1;
                    j;
                }
            }
            closeAllDrawers(prisoners);
            if foundCard as i32 == 0 {
                return 1 != 0;
            }
            i += 1;
            i;
        }
    }
    return 0 != 0;
}

fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    unsafe {
        let mut prisoners: i32 = 0;
        let mut chances: i32 = 0;
        let mut trials: u64 = 0;
        let mut i: u64 = 0;
        let mut count: u64 = 0;
        let mut end: *mut i8 = 0 as *mut i8;
        if argc != 4 {
            let str_to_print = format!(
                "Usage : {} <Number of prisoners> <Number of chances> <Number of trials>",
                build_str_from_raw_ptr(*argv.offset(0 as isize) as *mut u8)
            );
            print!("{}", str_to_print);
            return str_to_print.chars().count() as i32;
        }
        prisoners = atoi(*argv.offset(1 as isize));
        chances = atoi(*argv.offset(2 as isize));
        trials = strtoull(*argv.offset(3 as isize), &mut end, 10);
        srand(rust_time(None) as u32);
        print!("Running random trials...");
        i = 0;
        while i < trials {
            initialize(prisoners);
            count = count.wrapping_add(
                (if libertyOrDeathAtRandom(prisoners, chances) as i32 == 1 {
                    0
                } else {
                    1
                }) as u64,
            );
            i = i.wrapping_add(1);
        }
        print!(
            "\n\nGames Played : {}\nGames Won : {}\nChances : {} % \n\n",
            trials,
            count,
            100.0f64 * count as f64 / trials as f64
        );
        count = 0;
        print!("Running strategic trials...");
        i = 0;
        while i < trials {
            initialize(prisoners);
            count = count.wrapping_add(
                (if libertyOrDeathPlanned(prisoners, chances) as i32 == 1 {
                    0
                } else {
                    1
                }) as u64,
            );
            i = i.wrapping_add(1);
        }
        print!(
            "\n\nGames Played : {}\nGames Won : {}\nChances : {} % \n\n",
            trials,
            count,
            100.0f64 * count as f64 / trials as f64
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
