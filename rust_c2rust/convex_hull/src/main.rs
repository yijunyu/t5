#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
}
pub type size_t = libc::c_ulong;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tPoint {
    pub x: libc::c_int,
    pub y: libc::c_int,
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
pub unsafe extern "C" fn ccw(
    mut a: *const Point,
    mut b: *const Point,
    mut c: *const Point,
) -> bool {
    return ((*b).x - (*a).x) * ((*c).y - (*a).y) > ((*b).y - (*a).y) * ((*c).x - (*a).x);
}
#[no_mangle]
pub unsafe extern "C" fn comp(
    mut lhs: *const libc::c_void,
    mut rhs: *const libc::c_void,
) -> libc::c_int {
    let mut lp: Point = *(lhs as *mut Point);
    let mut rp: Point = *(rhs as *mut Point);
    if lp.x < rp.x {
        return -(1 as libc::c_int);
    }
    if rp.x < lp.x {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn freeNode(mut ptr: *mut Node) {
    if ptr.is_null() {
        return;
    }
    freeNode((*ptr).next);
    (*ptr).next = 0 as *mut tNode;
    free(ptr as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn pushBack(mut ptr: *mut Node, mut data: Point) -> *mut Node {
    let mut tmp: *mut Node = ptr;
    if ptr.is_null() {
        ptr = malloc(::core::mem::size_of::<Node>() as libc::c_ulong) as *mut Node;
        (*ptr).data = data;
        (*ptr).next = 0 as *mut tNode;
        return ptr;
    }
    while !((*tmp).next).is_null() {
        tmp = (*tmp).next;
    }
    (*tmp).next = malloc(::core::mem::size_of::<Node>() as libc::c_ulong) as *mut Node;
    (*(*tmp).next).data = data;
    (*(*tmp).next).next = 0 as *mut tNode;
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn popBack(mut ptr: *mut Node) -> *mut Node {
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
#[no_mangle]
pub unsafe extern "C" fn print(mut ptr: *mut Node) {
    printf(b"[\0" as *const u8 as *const libc::c_char);
    if !ptr.is_null() {
        printf(
            b"(%d, %d)\0" as *const u8 as *const libc::c_char,
            (*ptr).data.x,
            (*ptr).data.y,
        );
        ptr = (*ptr).next;
    }
    while !ptr.is_null() {
        printf(
            b", (%d, %d)\0" as *const u8 as *const libc::c_char,
            (*ptr).data.x,
            (*ptr).data.y,
        );
        ptr = (*ptr).next;
    }
    printf(b"]\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn convexHull(
    mut len: libc::c_int,
    mut p: *mut Point,
) -> *mut Node {
    let mut h: *mut Node = 0 as *mut Node;
    let mut hptr: *mut Node = 0 as *mut Node;
    let mut hLen: size_t = 0 as libc::c_int as size_t;
    let mut i: libc::c_int = 0;
    qsort(
        p as *mut libc::c_void,
        len as size_t,
        ::core::mem::size_of::<Point>() as libc::c_ulong,
        Some(
            comp
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    i = 0 as libc::c_int;
    while i < len {
        while hLen >= 2 as libc::c_int as libc::c_ulong {
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
    i = len - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        while hLen >= 2 as libc::c_int as libc::c_ulong {
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
unsafe fn main_0() -> libc::c_int {
    let mut points: [Point; 20] = [
        {
            let mut init = tPoint {
                x: 16 as libc::c_int,
                y: 3 as libc::c_int,
            };
            init
        },
        {
            let mut init = tPoint {
                x: 12 as libc::c_int,
                y: 17 as libc::c_int,
            };
            init
        },
        {
            let mut init = tPoint {
                x: 0 as libc::c_int,
                y: 6 as libc::c_int,
            };
            init
        },
        {
            let mut init = tPoint {
                x: -(4 as libc::c_int),
                y: -(6 as libc::c_int),
            };
            init
        },
        {
            let mut init = tPoint {
                x: 16 as libc::c_int,
                y: 6 as libc::c_int,
            };
            init
        },
        {
            let mut init = tPoint {
                x: 16 as libc::c_int,
                y: -(7 as libc::c_int),
            };
            init
        },
        {
            let mut init = tPoint {
                x: 16 as libc::c_int,
                y: -(3 as libc::c_int),
            };
            init
        },
        {
            let mut init = tPoint {
                x: 17 as libc::c_int,
                y: -(4 as libc::c_int),
            };
            init
        },
        {
            let mut init = tPoint {
                x: 5 as libc::c_int,
                y: 19 as libc::c_int,
            };
            init
        },
        {
            let mut init = tPoint {
                x: 19 as libc::c_int,
                y: -(8 as libc::c_int),
            };
            init
        },
        {
            let mut init = tPoint {
                x: 3 as libc::c_int,
                y: 16 as libc::c_int,
            };
            init
        },
        {
            let mut init = tPoint {
                x: 12 as libc::c_int,
                y: 13 as libc::c_int,
            };
            init
        },
        {
            let mut init = tPoint {
                x: 3 as libc::c_int,
                y: -(4 as libc::c_int),
            };
            init
        },
        {
            let mut init = tPoint {
                x: 17 as libc::c_int,
                y: 5 as libc::c_int,
            };
            init
        },
        {
            let mut init = tPoint {
                x: -(3 as libc::c_int),
                y: 15 as libc::c_int,
            };
            init
        },
        {
            let mut init = tPoint {
                x: -(3 as libc::c_int),
                y: -(9 as libc::c_int),
            };
            init
        },
        {
            let mut init = tPoint {
                x: 0 as libc::c_int,
                y: 11 as libc::c_int,
            };
            init
        },
        {
            let mut init = tPoint {
                x: -(9 as libc::c_int),
                y: -(3 as libc::c_int),
            };
            init
        },
        {
            let mut init = tPoint {
                x: -(4 as libc::c_int),
                y: -(2 as libc::c_int),
            };
            init
        },
        {
            let mut init = tPoint {
                x: 12 as libc::c_int,
                y: 10 as libc::c_int,
            };
            init
        },
    ];
    let mut hull: *mut Node = convexHull(
        (::core::mem::size_of::<[Point; 20]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<Point>() as libc::c_ulong)
            as libc::c_int,
        points.as_mut_ptr(),
    );
    printf(b"Convex Hull: \0" as *const u8 as *const libc::c_char);
    print(hull);
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    freeNode(hull);
    hull = 0 as *mut Node;
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
