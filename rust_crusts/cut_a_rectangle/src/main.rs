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
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
}
#[no_mangle]
pub static mut grid: *mut u8 = 0 as *const u8 as *mut u8;
#[no_mangle]
pub static mut w: i32 = 0;
#[no_mangle]
pub static mut h: i32 = 0;
#[no_mangle]
pub static mut len: i32 = 0;
#[no_mangle]
pub static mut cnt: u64 = 0;
static mut next: [i32; 4] = [0; 4];
static mut dir: [[i32; 2]; 4] = [[0, -1], [-1, 0], [0, 1], [1, 0]];
#[no_mangle]
pub extern "C" fn walk(mut y: i32, mut x: i32) {
    let mut i: i32 = 0;
    let mut t: i32 = 0;
    unsafe {
        if y == 0 || y == h || x == 0 || x == w {
            cnt = cnt.wrapping_add(2);
            return;
        }
        t = y * (w + 1) + x;
        let ref mut fresh0 = *grid.offset(t as isize);
        *fresh0 = (*fresh0).wrapping_add(1);
        *fresh0;
        let ref mut fresh1 = *grid.offset((len - t) as isize);
        *fresh1 = (*fresh1).wrapping_add(1);
        *fresh1;
    }
    i = 0;
    unsafe {
        while i < 4 {
            if *grid.offset((t + next[i as usize]) as isize) == 0 {
                walk(
                    y + dir[i as usize][0 as usize],
                    x + dir[i as usize][1 as usize],
                );
            }
            i += 1;
            i;
        }
        let ref mut fresh2 = *grid.offset(t as isize);
        *fresh2 = (*fresh2).wrapping_sub(1);
        *fresh2;
        let ref mut fresh3 = *grid.offset((len - t) as isize);
        *fresh3 = (*fresh3).wrapping_sub(1);
        *fresh3;
    }
}

#[no_mangle]
pub extern "C" fn solve(mut hh: i32, mut ww: i32, mut recur: i32) -> u64 {
    let mut t: i32 = 0;
    let mut cx: i32 = 0;
    let mut cy: i32 = 0;
    let mut x: i32 = 0;
    unsafe {
        h = hh;
        w = ww;
        if h & 1 != 0 {
            t = w;
            w = h;
            h = t;
        }
        if h & 1 != 0 {
            return 0;
        }
        if w == 1 {
            return 1;
        }
        if w == 2 {
            return h as u64;
        }
        if h == 2 {
            return w as u64;
        }
        cy = h / 2;
        cx = w / 2;
        len = (h + 1) * (w + 1);
        grid = realloc(grid as *mut libc::c_void, len as u64) as *mut u8;
        let fresh4 = len;
        len = len - 1;
        memset(grid as *mut libc::c_void, 0, fresh4 as u64);
        next[0 as usize] = -1;
        next[1 as usize] = -w - 1;
        next[2 as usize] = 1;
        next[3 as usize] = w + 1;
        if recur != 0 {
            cnt = 0;
        }
    }
    x = cx + 1;
    unsafe {
        while x < w {
            t = cy * (w + 1) + x;
            *grid.offset(t as isize) = 1;
            *grid.offset((len - t) as isize) = 1;
            walk(cy - 1, x);
            x += 1;
            x;
        }
        cnt = cnt.wrapping_add(1);
        cnt;
        if h == w {
            cnt = cnt.wrapping_mul(2);
        } else if w & 1 == 0 && recur != 0 {
            solve(w, h, 0);
        }
        return cnt;
    }
}

fn main_0() -> i32 {
    let mut y: i32 = 0;
    let mut x: i32 = 0;
    y = 1;
    while y <= 10 {
        x = 1;
        while x <= y {
            if x & 1 == 0 || y & 1 == 0 {
                print!("{} x {}: {}\n", y, x, solve(y, x, 1));
            }
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
