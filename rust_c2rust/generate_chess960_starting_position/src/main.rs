#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn rand() -> libc::c_int;
    fn srand(__seed: libc::c_uint);
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
}
pub type __time_t = libc::c_long;
pub type time_t = __time_t;
pub type wint_t = libc::c_uint;
#[no_mangle]
pub static mut rank: [libc::c_char; 9] = [0; 9];
#[no_mangle]
pub static mut pos: [libc::c_int; 8] = [0; 8];
#[no_mangle]
pub unsafe extern "C" fn swap(mut i: libc::c_int, mut j: libc::c_int) {
    let mut temp: libc::c_int = pos[i as usize];
    pos[i as usize] = pos[j as usize];
    pos[j as usize] = temp;
}
#[no_mangle]
pub unsafe extern "C" fn generateFirstRank() {
    let mut kPos: libc::c_int = 0;
    let mut qPos: libc::c_int = 0;
    let mut bPos1: libc::c_int = 0;
    let mut bPos2: libc::c_int = 0;
    let mut rPos1: libc::c_int = 0;
    let mut rPos2: libc::c_int = 0;
    let mut nPos1: libc::c_int = 0;
    let mut nPos2: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        rank[i as usize] = 'e' as i32 as libc::c_char;
        pos[i as usize] = i;
        i += 1;
        i;
    }
    loop {
        kPos = rand() % 8 as libc::c_int;
        rPos1 = rand() % 8 as libc::c_int;
        rPos2 = rand() % 8 as libc::c_int;
        if !(rPos1 - kPos <= 0 as libc::c_int && rPos2 - kPos <= 0 as libc::c_int
            || rPos1 - kPos >= 0 as libc::c_int && rPos2 - kPos >= 0 as libc::c_int
            || (rPos1 == rPos2 || kPos == rPos1 || kPos == rPos2))
        {
            break;
        }
    }
    rank[pos[rPos1 as usize] as usize] = 'R' as i32 as libc::c_char;
    rank[pos[kPos as usize] as usize] = 'K' as i32 as libc::c_char;
    rank[pos[rPos2 as usize] as usize] = 'R' as i32 as libc::c_char;
    swap(rPos1, 7 as libc::c_int);
    swap(rPos2, 6 as libc::c_int);
    swap(kPos, 5 as libc::c_int);
    loop {
        bPos1 = rand() % 5 as libc::c_int;
        bPos2 = rand() % 5 as libc::c_int;
        if !((pos[bPos1 as usize] - pos[bPos2 as usize]) % 2 as libc::c_int
            == 0 as libc::c_int || bPos1 == bPos2)
        {
            break;
        }
    }
    rank[pos[bPos1 as usize] as usize] = 'B' as i32 as libc::c_char;
    rank[pos[bPos2 as usize] as usize] = 'B' as i32 as libc::c_char;
    swap(bPos1, 4 as libc::c_int);
    swap(bPos2, 3 as libc::c_int);
    loop {
        qPos = rand() % 3 as libc::c_int;
        nPos1 = rand() % 3 as libc::c_int;
        if !(qPos == nPos1) {
            break;
        }
    }
    rank[pos[qPos as usize] as usize] = 'Q' as i32 as libc::c_char;
    rank[pos[nPos1 as usize] as usize] = 'N' as i32 as libc::c_char;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        if rank[i as usize] as libc::c_int == 'e' as i32 {
            rank[i as usize] = 'N' as i32 as libc::c_char;
            break;
        } else {
            i += 1;
            i;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn printRank() {
    let mut i: libc::c_int = 0;
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        if rank[i as usize] as libc::c_int == 'K' as i32 {
            printf(
                b"%lc\0" as *const u8 as *const libc::c_char,
                9812 as libc::c_int as wint_t,
            );
        } else if rank[i as usize] as libc::c_int == 'Q' as i32 {
            printf(
                b"%lc\0" as *const u8 as *const libc::c_char,
                9813 as libc::c_int as wint_t,
            );
        } else if rank[i as usize] as libc::c_int == 'R' as i32 {
            printf(
                b"%lc\0" as *const u8 as *const libc::c_char,
                9814 as libc::c_int as wint_t,
            );
        } else if rank[i as usize] as libc::c_int == 'B' as i32 {
            printf(
                b"%lc\0" as *const u8 as *const libc::c_char,
                9815 as libc::c_int as wint_t,
            );
        }
        if rank[i as usize] as libc::c_int == 'N' as i32 {
            printf(
                b"%lc\0" as *const u8 as *const libc::c_char,
                9816 as libc::c_int as wint_t,
            );
        }
        i += 1;
        i;
    }
}
unsafe fn main_0() -> libc::c_int {
    let mut i: libc::c_int = 0;
    srand(time(0 as *mut time_t) as libc::c_uint);
    i = 0 as libc::c_int;
    while i < 9 as libc::c_int {
        generateFirstRank();
        printRank();
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
