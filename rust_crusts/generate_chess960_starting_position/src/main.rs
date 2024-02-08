#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
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
    fn rand() -> i32;
    fn srand(__seed: u32);
    fn setlocale(__category: i32, __locale: *const i8) -> *mut i8;
}
#[no_mangle]
pub static mut rank: [i8; 9] = [0; 9];
#[no_mangle]
pub static mut pos: [i32; 8] = [0; 8];
#[no_mangle]
pub extern "C" fn swap(mut i: i32, mut j: i32) {
    unsafe {
        let mut temp: i32 = pos[i as usize];
        pos[i as usize] = pos[j as usize];
        pos[j as usize] = temp;
    }
}

#[no_mangle]
pub extern "C" fn generateFirstRank() {
    let mut kPos: i32 = 0;
    let mut qPos: i32 = 0;
    let mut bPos1: i32 = 0;
    let mut bPos2: i32 = 0;
    let mut rPos1: i32 = 0;
    let mut rPos2: i32 = 0;
    let mut nPos1: i32 = 0;
    let mut nPos2: i32 = 0;
    let mut i: i32 = 0;
    i = 0;
    unsafe {
        while i < 8 {
            rank[i as usize] = 'e' as i8;
            pos[i as usize] = i;
            i += 1;
            i;
        }
        loop {
            kPos = rand() % 8;
            rPos1 = rand() % 8;
            rPos2 = rand() % 8;
            if !(rPos1 - kPos <= 0 && rPos2 - kPos <= 0
                || rPos1 - kPos >= 0 && rPos2 - kPos >= 0
                || (rPos1 == rPos2 || kPos == rPos1 || kPos == rPos2))
            {
                break;
            }
        }
        rank[pos[rPos1 as usize] as usize] = 'R' as i8;
        rank[pos[kPos as usize] as usize] = 'K' as i8;
        rank[pos[rPos2 as usize] as usize] = 'R' as i8;
    }
    swap(rPos1, 7);
    swap(rPos2, 6);
    swap(kPos, 5);
    unsafe {
        loop {
            bPos1 = rand() % 5;
            bPos2 = rand() % 5;
            if !((pos[bPos1 as usize] - pos[bPos2 as usize]) % 2 == 0 || bPos1 == bPos2) {
                break;
            }
        }
        rank[pos[bPos1 as usize] as usize] = 'B' as i8;
        rank[pos[bPos2 as usize] as usize] = 'B' as i8;
    }
    swap(bPos1, 4);
    swap(bPos2, 3);
    unsafe {
        loop {
            qPos = rand() % 3;
            nPos1 = rand() % 3;
            if !(qPos == nPos1) {
                break;
            }
        }
        rank[pos[qPos as usize] as usize] = 'Q' as i8;
        rank[pos[nPos1 as usize] as usize] = 'N' as i8;
    }
    i = 0;
    unsafe {
        while i < 8 {
            if rank[i as usize] as i32 == 'e' as i32 {
                rank[i as usize] = 'N' as i8;
                break;
            } else {
                i += 1;
                i;
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn printRank() {
    let mut i: i32 = 0;
    unsafe {
        setlocale(6, b"\0" as *const u8 as *const i8);
    }
    print!("\n");
    i = 0;
    unsafe {
        while i < 8 {
            if rank[i as usize] as i32 == 'K' as i32 {
                print!("{}", 9812);
            } else if rank[i as usize] as i32 == 'Q' as i32 {
                print!("{}", 9813);
            } else if rank[i as usize] as i32 == 'R' as i32 {
                print!("{}", 9814);
            } else if rank[i as usize] as i32 == 'B' as i32 {
                print!("{}", 9815);
            }
            if rank[i as usize] as i32 == 'N' as i32 {
                print!("{}", 9816);
            }
            i += 1;
            i;
        }
    }
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    unsafe {
        srand(rust_time(None) as u32);
    }
    i = 0;
    while i < 9 {
        generateFirstRank();
        printRank();
        i += 1;
        i;
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
