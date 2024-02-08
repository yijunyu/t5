#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct point {
    pub x: libc::c_int,
    pub y: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn rot(
    mut n: libc::c_int,
    mut p: *mut point,
    mut rx: libc::c_int,
    mut ry: libc::c_int,
) {
    let mut t: libc::c_int = 0;
    if ry == 0 {
        if rx == 1 as libc::c_int {
            (*p).x = n - 1 as libc::c_int - (*p).x;
            (*p).y = n - 1 as libc::c_int - (*p).y;
        }
        t = (*p).x;
        (*p).x = (*p).y;
        (*p).y = t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn d2pt(
    mut n: libc::c_int,
    mut d: libc::c_int,
    mut p: *mut point,
) {
    let mut s: libc::c_int = 1 as libc::c_int;
    let mut t: libc::c_int = d;
    let mut rx: libc::c_int = 0;
    let mut ry: libc::c_int = 0;
    (*p).x = 0 as libc::c_int;
    (*p).y = 0 as libc::c_int;
    while s < n {
        rx = 1 as libc::c_int & t / 2 as libc::c_int;
        ry = 1 as libc::c_int & (t ^ rx);
        rot(s, p, rx, ry);
        (*p).x += s * rx;
        (*p).y += s * ry;
        t /= 4 as libc::c_int;
        s *= 2 as libc::c_int;
    }
}
unsafe fn main_0() -> libc::c_int {
    let mut d: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut cx: libc::c_int = 0;
    let mut cy: libc::c_int = 0;
    let mut px: libc::c_int = 0;
    let mut py: libc::c_int = 0;
    let mut pts: [[libc::c_char; 96]; 96] = [[0; 96]; 96];
    let mut curr: point = point { x: 0, y: 0 };
    let mut prev: point = point { x: 0, y: 0 };
    x = 0 as libc::c_int;
    while x < 32 as libc::c_int * 3 as libc::c_int {
        y = 0 as libc::c_int;
        while y < 32 as libc::c_int * 3 as libc::c_int {
            pts[x as usize][y as usize] = ' ' as i32 as libc::c_char;
            y += 1;
            y;
        }
        x += 1;
        x;
    }
    prev.y = 0 as libc::c_int;
    prev.x = prev.y;
    pts[0 as libc::c_int
        as usize][0 as libc::c_int as usize] = '.' as i32 as libc::c_char;
    d = 1 as libc::c_int;
    while d < 32 as libc::c_int * 32 as libc::c_int {
        d2pt(32 as libc::c_int, d, &mut curr);
        cx = curr.x * 3 as libc::c_int;
        cy = curr.y * 3 as libc::c_int;
        px = prev.x * 3 as libc::c_int;
        py = prev.y * 3 as libc::c_int;
        pts[cx as usize][cy as usize] = '.' as i32 as libc::c_char;
        if cx == px {
            if py < cy {
                y = py + 1 as libc::c_int;
                while y < cy {
                    pts[cx as usize][y as usize] = '|' as i32 as libc::c_char;
                    y += 1;
                    y;
                }
            } else {
                y = cy + 1 as libc::c_int;
                while y < py {
                    pts[cx as usize][y as usize] = '|' as i32 as libc::c_char;
                    y += 1;
                    y;
                }
            }
        } else if px < cx {
            x = px + 1 as libc::c_int;
            while x < cx {
                pts[x as usize][cy as usize] = '_' as i32 as libc::c_char;
                x += 1;
                x;
            }
        } else {
            x = cx + 1 as libc::c_int;
            while x < px {
                pts[x as usize][cy as usize] = '_' as i32 as libc::c_char;
                x += 1;
                x;
            }
        }
        prev = curr;
        d += 1;
        d;
    }
    x = 0 as libc::c_int;
    while x < 32 as libc::c_int * 3 as libc::c_int {
        y = 0 as libc::c_int;
        while y < 32 as libc::c_int * 3 as libc::c_int {
            printf(
                b"%c\0" as *const u8 as *const libc::c_char,
                pts[y as usize][x as usize] as libc::c_int,
            );
            y += 1;
            y;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        x += 1;
        x;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
