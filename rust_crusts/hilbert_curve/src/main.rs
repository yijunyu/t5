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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct point {
    pub x: i32,
    pub y: i32,
}
#[no_mangle]
pub extern "C" fn rot(mut n: i32, mut p: *mut point, mut rx: i32, mut ry: i32) {
    unsafe {
        let mut t: i32 = 0;
        if ry == 0 {
            if rx == 1 {
                (*p).x = n - 1 - (*p).x;
                (*p).y = n - 1 - (*p).y;
            }
            t = (*p).x;
            (*p).x = (*p).y;
            (*p).y = t;
        }
    }
}

#[no_mangle]
pub extern "C" fn d2pt(mut n: i32, mut d: i32, mut p: *mut point) {
    unsafe {
        let mut s: i32 = 1;
        let mut t: i32 = d;
        let mut rx: i32 = 0;
        let mut ry: i32 = 0;
        (*p).x = 0;
        (*p).y = 0;
        while s < n {
            rx = 1 & t / 2;
            ry = 1 & (t ^ rx);
            rot(s, p, rx, ry);
            (*p).x += s * rx;
            (*p).y += s * ry;
            t /= 4;
            s *= 2;
        }
    }
}

fn main_0() -> i32 {
    let mut d: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut cx: i32 = 0;
    let mut cy: i32 = 0;
    let mut px: i32 = 0;
    let mut py: i32 = 0;
    let mut pts: [[i8; 96]; 96] = [[0; 96]; 96];
    let mut curr: point = point { x: 0, y: 0 };
    let mut prev: point = point { x: 0, y: 0 };
    x = 0;
    while x < 32 * 3 {
        y = 0;
        while y < 32 * 3 {
            pts[x as usize][y as usize] = ' ' as i8;
            y += 1;
            y;
        }
        x += 1;
        x;
    }
    prev.y = 0;
    prev.x = prev.y;
    pts[0 as usize][0 as usize] = '.' as i8;
    d = 1;
    while d < 32 * 32 {
        d2pt(32, d, &mut curr);
        cx = curr.x * 3;
        cy = curr.y * 3;
        px = prev.x * 3;
        py = prev.y * 3;
        pts[cx as usize][cy as usize] = '.' as i8;
        if cx == px {
            if py < cy {
                y = py + 1;
                while y < cy {
                    pts[cx as usize][y as usize] = '|' as i8;
                    y += 1;
                    y;
                }
            } else {
                y = cy + 1;
                while y < py {
                    pts[cx as usize][y as usize] = '|' as i8;
                    y += 1;
                    y;
                }
            }
        } else if px < cx {
            x = px + 1;
            while x < cx {
                pts[x as usize][cy as usize] = '_' as i8;
                x += 1;
                x;
            }
        } else {
            x = cx + 1;
            while x < px {
                pts[x as usize][cy as usize] = '_' as i8;
                x += 1;
                x;
            }
        }
        prev = curr;
        d += 1;
        d;
    }
    x = 0;
    while x < 32 * 3 {
        y = 0;
        while y < 32 * 3 {
            print!("{}", pts[y as usize][x as usize] as i32);
            y += 1;
            y;
        }
        print!("\n");
        x += 1;
        x;
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
