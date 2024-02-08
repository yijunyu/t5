#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use std::io::Read;
fn rust_getchar() -> u8 {
    return std::io::stdin()
        .bytes()
        .next()
        .and_then(|result| result.ok())
        .map(|byte| byte)
        .unwrap();
}

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
    fn rand() -> i32;
    fn srand(__seed: u32);
    fn exit(_: i32) -> !;
}
pub const MOVE_RIGHT: u32 = 3;
pub const MOVE_LEFT: u32 = 2;
pub const MOVE_DOWN: u32 = 1;
pub const MOVE_UP: u32 = 0;
#[no_mangle]
pub static mut holeRow: i32 = 0;
#[no_mangle]
pub static mut holeCollumn: i32 = 0;
#[no_mangle]
pub static mut cells: [[i32; 4]; 4] = [[0; 4]; 4];
#[no_mangle]
pub static mut nShuffles: i32 = 100;
#[no_mangle]
pub extern "C" fn Game_update(mut move_0: u32) -> i32 {
    let dx: [i32; 4] = [0, 0, -1, 1];
    let dy: [i32; 4] = [-1, 1, 0, 0];
    unsafe {
        let mut i: i32 = holeRow + dy[move_0 as usize];
        let mut j: i32 = holeCollumn + dx[move_0 as usize];
        if i >= 0 && i < 4 && j >= 0 && j < 4 {
            cells[holeRow as usize][holeCollumn as usize] = cells[i as usize][j as usize];
            cells[i as usize][j as usize] = 0;
            holeRow = i;
            holeCollumn = j;
            return 1;
        }
    }
    return 0;
}

#[no_mangle]
pub extern "C" fn Game_setup() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    i = 0;
    unsafe {
        while i < 4 {
            j = 0;
            while j < 4 {
                cells[i as usize][j as usize] = i * 4 + j + 1;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
        cells[(4 - 1i32) as usize][(4 - 1i32) as usize] = 0;
        holeRow = 4 - 1;
        holeCollumn = 4 - 1;
    }
    k = 0;
    unsafe {
        while k < nShuffles {
            k += Game_update((rand() % 4i32) as u32);
        }
    }
}

#[no_mangle]
pub extern "C" fn Game_isFinished() -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 1;
    i = 0;
    unsafe {
        while i < 4 {
            j = 0;
            while j < 4 {
                if k < 4 * 4 && {
                    let fresh0 = k;
                    k = k + 1;
                    cells[i as usize][j as usize] != fresh0
                } {
                    return 0;
                }
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    }
    return 1;
}

#[no_mangle]
pub extern "C" fn View_showBoard() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    print!("{}", '\n' as i32);
    i = 0;
    unsafe {
        while i < 4 {
            j = 0;
            while j < 4 {
                if cells[i as usize][j as usize] != 0 {
                    if j != 4 - 1 {
                        print!(" {:2} ", cells[i as usize][j as usize])
                    } else {
                        print!(" {:2} \n", cells[i as usize][j as usize])
                    };
                } else {
                    if j != 4 - 1 {
                        print!(" {:2} ", "\0")
                    } else {
                        print!(" {:2} \n", "\0")
                    };
                }
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    }
    print!("{}", '\n' as i32);
}

#[no_mangle]
pub extern "C" fn View_displayMessage(mut text: *mut i8) {
    unsafe {
        print!("\n{}\n", build_str_from_raw_ptr(text as *mut u8));
    }
}

#[no_mangle]
pub extern "C" fn Controller_getMove() -> u32 {
    let mut c: i32 = 0;
    unsafe {
        loop {
            print!("{}", "enter u/d/l/r : \0");
            c = rust_getchar() as i32;
            while rust_getchar() as i32 != '\n' as i32 {}
            match c {
                27 => {
                    exit(0);
                }
                100 => return MOVE_UP,
                117 => return MOVE_DOWN,
                114 => return MOVE_LEFT,
                108 => return MOVE_RIGHT,
                _ => {}
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn Controller_pause() {
    rust_getchar() as i32;
}

fn main_0() -> i32 {
    unsafe {
        srand(rust_time(None) as u32);
    }
    loop {
        Game_setup();
        if !(Game_isFinished() != 0) {
            break;
        }
    }
    View_showBoard();
    while Game_isFinished() == 0 {
        Game_update(Controller_getMove());
        View_showBoard();
    }
    View_displayMessage(b"You win\0" as *const u8 as *const i8 as *mut i8);
    Controller_pause();
    return 0;
}

pub fn main() {
    unsafe {
        ::std::process::exit(main_0() as i32);
    }
}
