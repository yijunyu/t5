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
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn qsort(__base: *mut libc::c_void, __nmemb: u64, __size: u64, __compar: __compar_fn_t);
}
pub type __compar_fn_t =
    Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tPoint {
    pub x: i32,
    pub y: i32,
}
pub type Point = tPoint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tNode {
    pub data: Point,
    pub next: *mut tNode,
}
pub type Node = tNode;
#[no_mangle]
pub extern "C" fn ccw(mut a: *const Point, mut b: *const Point, mut c: *const Point) -> bool {
    unsafe {
        return ((*b).x - (*a).x) * ((*c).y - (*a).y) > ((*b).y - (*a).y) * ((*c).x - (*a).x);
    }
}

#[no_mangle]
pub extern "C" fn comp(mut lhs: *const libc::c_void, mut rhs: *const libc::c_void) -> i32 {
    unsafe {
        let mut lp: Point = *(lhs as *mut Point);
        let mut rp: Point = *(rhs as *mut Point);
        if lp.x < rp.x {
            return -1;
        }
        if rp.x < lp.x {
            return 1;
        }
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn freeNode(mut ptr: *mut Node) {
    unsafe {
        if ptr.is_null() {
            return;
        }
        freeNode((*ptr).next);
        (*ptr).next = 0 as *mut tNode;
        free(ptr as *mut libc::c_void);
    }
}

#[no_mangle]
pub extern "C" fn pushBack(mut ptr: *mut Node, mut data: Point) -> *mut Node {
    unsafe {
        let mut tmp: *mut Node = ptr;
        if ptr.is_null() {
            ptr = malloc(::core::mem::size_of::<Node>() as u64) as *mut Node;
            (*ptr).data = data;
            (*ptr).next = 0 as *mut tNode;
            return ptr;
        }
        while !((*tmp).next).is_null() {
            tmp = (*tmp).next;
        }
        (*tmp).next = malloc(::core::mem::size_of::<Node>() as u64) as *mut Node;
        (*(*tmp).next).data = data;
        (*(*tmp).next).next = 0 as *mut tNode;
        return ptr;
    }
}

#[no_mangle]
pub extern "C" fn popBack(mut ptr: *mut Node) -> *mut Node {
    unsafe {
        let mut tmp: *mut Node = ptr;
        if ptr.is_null() {
            return 0 as *mut Node;
        }
        if ((*ptr).next).is_null() {
            free(ptr as *mut libc::c_void);
            return 0 as *mut Node;
        }
        while !((*(*tmp).next).next).is_null() {
            tmp = (*tmp).next;
        }
        free((*tmp).next as *mut libc::c_void);
        (*tmp).next = 0 as *mut tNode;
        return ptr;
    }
}

#[no_mangle]
pub extern "C" fn print(mut ptr: *mut Node) {
    unsafe {
        print!("[");
    }
    if !ptr.is_null() {
        print!("({}, {})", (*ptr).data.x, (*ptr).data.y);
        ptr = (*ptr).next;
    }
    while !ptr.is_null() {
        print!(", ({}, {})", (*ptr).data.x, (*ptr).data.y);
        ptr = (*ptr).next;
    }
    unsafe {
        print!("]");
    }
}

#[no_mangle]
pub extern "C" fn convexHull(mut len: i32, mut p: *mut Point) -> *mut Node {
    unsafe {
        let mut h: *mut Node = 0 as *mut Node;
        let mut hptr: *mut Node = 0 as *mut Node;
        let mut hLen: u64 = 0;
        let mut i: i32 = 0;
        qsort(
            p as *mut libc::c_void,
            len as u64,
            ::core::mem::size_of::<Point>() as u64,
            Some(comp as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32),
        );
        i = 0;
        while i < len {
            while hLen >= 2 {
                hptr = h;
                while !((*(*hptr).next).next).is_null() {
                    hptr = (*hptr).next;
                }
                if ccw(
                    &mut (*hptr).data,
                    &mut (*(*hptr).next).data,
                    &mut *p.offset(i as isize),
                ) {
                    break;
                }
                h = popBack(h);
                hLen = hLen.wrapping_sub(1);
                hLen;
            }
            h = pushBack(h, *p.offset(i as isize));
            hLen = hLen.wrapping_add(1);
            hLen;
            i += 1;
            i;
        }
        i = len - 1;
        while i >= 0 {
            while hLen >= 2 {
                hptr = h;
                while !((*(*hptr).next).next).is_null() {
                    hptr = (*hptr).next;
                }
                if ccw(
                    &mut (*hptr).data,
                    &mut (*(*hptr).next).data,
                    &mut *p.offset(i as isize),
                ) {
                    break;
                }
                h = popBack(h);
                hLen = hLen.wrapping_sub(1);
                hLen;
            }
            h = pushBack(h, *p.offset(i as isize));
            hLen = hLen.wrapping_add(1);
            hLen;
            i -= 1;
            i;
        }
        popBack(h);
        return h;
    }
}

fn main_0() -> i32 {
    unsafe {
        let mut points: [Point; 20] = [
            {
                let mut init = tPoint { x: 16, y: 3 };
                init
            },
            {
                let mut init = tPoint { x: 12, y: 17 };
                init
            },
            {
                let mut init = tPoint { x: 0, y: 6 };
                init
            },
            {
                let mut init = tPoint { x: -4, y: -6 };
                init
            },
            {
                let mut init = tPoint { x: 16, y: 6 };
                init
            },
            {
                let mut init = tPoint { x: 16, y: -7 };
                init
            },
            {
                let mut init = tPoint { x: 16, y: -3 };
                init
            },
            {
                let mut init = tPoint { x: 17, y: -4 };
                init
            },
            {
                let mut init = tPoint { x: 5, y: 19 };
                init
            },
            {
                let mut init = tPoint { x: 19, y: -8 };
                init
            },
            {
                let mut init = tPoint { x: 3, y: 16 };
                init
            },
            {
                let mut init = tPoint { x: 12, y: 13 };
                init
            },
            {
                let mut init = tPoint { x: 3, y: -4 };
                init
            },
            {
                let mut init = tPoint { x: 17, y: 5 };
                init
            },
            {
                let mut init = tPoint { x: -3, y: 15 };
                init
            },
            {
                let mut init = tPoint { x: -3, y: -9 };
                init
            },
            {
                let mut init = tPoint { x: 0, y: 11 };
                init
            },
            {
                let mut init = tPoint { x: -9, y: -3 };
                init
            },
            {
                let mut init = tPoint { x: -4, y: -2 };
                init
            },
            {
                let mut init = tPoint { x: 12, y: 10 };
                init
            },
        ];
        let mut hull: *mut Node = convexHull(
            (::core::mem::size_of::<[Point; 20]>() as u64)
                .wrapping_div(::core::mem::size_of::<Point>() as u64) as i32,
            points.as_mut_ptr(),
        );
        print!("Convex Hull: ");
        print(hull);
        print!("\n");
        freeNode(hull);
        hull = 0 as *mut Node;
        return 0;
    }
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
