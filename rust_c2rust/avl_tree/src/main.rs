#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn rand() -> libc::c_int;
    fn srand(__seed: libc::c_uint);
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn abort() -> !;
    fn time(__timer: *mut time_t) -> time_t;
}
pub type __time_t = libc::c_long;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node {
    pub payload: libc::c_int,
    pub height: libc::c_int,
    pub kid: [*mut node; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct trunk {
    pub prev: *mut trunk,
    pub str_0: *mut libc::c_char,
}
#[no_mangle]
pub static mut dummy: node = unsafe {
    {
        let mut init = node {
            payload: 0 as libc::c_int,
            height: 0 as libc::c_int,
            kid: [&dummy as *const node as *mut node, &dummy as *const node as *mut node],
        };
        init
    }
};
#[no_mangle]
pub static mut nnil: *mut node = unsafe { &dummy as *const node as *mut node };
#[no_mangle]
pub unsafe extern "C" fn new_node(mut value: libc::c_int) -> *mut node {
    let mut n: *mut node = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<node>() as libc::c_ulong,
    ) as *mut node;
    *n = {
        let mut init = node {
            payload: value,
            height: 1 as libc::c_int,
            kid: [nnil, nnil],
        };
        init
    };
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn max(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a > b { a } else { b };
}
#[no_mangle]
pub unsafe extern "C" fn set_height(mut n: *mut node) {
    (*n)
        .height = 1 as libc::c_int
        + max(
            (*(*n).kid[0 as libc::c_int as usize]).height,
            (*(*n).kid[1 as libc::c_int as usize]).height,
        );
}
#[no_mangle]
pub unsafe extern "C" fn ballance(mut n: *mut node) -> libc::c_int {
    return (*(*n).kid[0 as libc::c_int as usize]).height
        - (*(*n).kid[1 as libc::c_int as usize]).height;
}
#[no_mangle]
pub unsafe extern "C" fn rotate(
    mut rootp: *mut *mut node,
    mut dir: libc::c_int,
) -> *mut node {
    let mut old_r: *mut node = *rootp;
    let mut new_r: *mut node = (*old_r).kid[dir as usize];
    *rootp = new_r;
    if nnil == *rootp {
        free(old_r as *mut libc::c_void);
    } else {
        (*old_r).kid[dir as usize] = (*new_r).kid[(dir == 0) as libc::c_int as usize];
        set_height(old_r);
        (*new_r).kid[(dir == 0) as libc::c_int as usize] = old_r;
    }
    return new_r;
}
#[no_mangle]
pub unsafe extern "C" fn adjust_balance(mut rootp: *mut *mut node) {
    let mut root: *mut node = *rootp;
    let mut b: libc::c_int = ballance(root) / 2 as libc::c_int;
    if b != 0 {
        let mut dir: libc::c_int = (1 as libc::c_int - b) / 2 as libc::c_int;
        if ballance((*root).kid[dir as usize]) == -b {
            rotate(
                &mut *((*root).kid).as_mut_ptr().offset(dir as isize),
                (dir == 0) as libc::c_int,
            );
        }
        root = rotate(rootp, dir);
    }
    if root != nnil {
        set_height(root);
    }
}
#[no_mangle]
pub unsafe extern "C" fn query(
    mut root: *mut node,
    mut value: libc::c_int,
) -> *mut node {
    return if root == nnil {
        0 as *mut node
    } else if (*root).payload == value {
        root
    } else {
        query((*root).kid[(value > (*root).payload) as libc::c_int as usize], value)
    };
}
#[no_mangle]
pub unsafe extern "C" fn insert(mut rootp: *mut *mut node, mut value: libc::c_int) {
    let mut root: *mut node = *rootp;
    if root == nnil {
        *rootp = new_node(value);
    } else if value != (*root).payload {
        insert(
            &mut *((*root).kid)
                .as_mut_ptr()
                .offset((value > (*root).payload) as libc::c_int as isize),
            value,
        );
        adjust_balance(rootp);
    }
}
#[no_mangle]
pub unsafe extern "C" fn delete(mut rootp: *mut *mut node, mut value: libc::c_int) {
    let mut root: *mut node = *rootp;
    if root == nnil {
        return;
    }
    if (*root).payload == value {
        root = rotate(rootp, (ballance(root) < 0 as libc::c_int) as libc::c_int);
        if nnil == root {
            return;
        }
    }
    delete(
        &mut *((*root).kid)
            .as_mut_ptr()
            .offset((value > (*root).payload) as libc::c_int as isize),
        value,
    );
    adjust_balance(rootp);
}
#[no_mangle]
pub unsafe extern "C" fn show_trunks(mut p: *mut trunk) {
    if p.is_null() {
        return;
    }
    show_trunks((*p).prev);
    printf(b"%s\0" as *const u8 as *const libc::c_char, (*p).str_0);
}
#[no_mangle]
pub unsafe extern "C" fn show_tree(
    mut root: *mut node,
    mut prev: *mut trunk,
    mut is_left: libc::c_int,
) {
    if root == nnil {
        return;
    }
    let mut this_disp: trunk = {
        let mut init = trunk {
            prev: prev,
            str_0: b"    \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    };
    let mut prev_str: *mut libc::c_char = this_disp.str_0;
    show_tree((*root).kid[0 as libc::c_int as usize], &mut this_disp, 1 as libc::c_int);
    if prev.is_null() {
        this_disp
            .str_0 = b"---\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    } else if is_left != 0 {
        this_disp
            .str_0 = b".--\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        prev_str = b"   |\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    } else {
        this_disp
            .str_0 = b"`--\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        (*prev).str_0 = prev_str;
    }
    show_trunks(&mut this_disp);
    printf(b"%d\n\0" as *const u8 as *const libc::c_char, (*root).payload);
    if !prev.is_null() {
        (*prev).str_0 = prev_str;
    }
    this_disp.str_0 = b"   |\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    show_tree((*root).kid[1 as libc::c_int as usize], &mut this_disp, 0 as libc::c_int);
    if prev.is_null() {
        puts(b"\0" as *const u8 as *const libc::c_char);
    }
}
#[no_mangle]
pub unsafe extern "C" fn verify(mut p: *mut node) -> libc::c_int {
    if p == nnil {
        return 1 as libc::c_int;
    }
    let mut h0: libc::c_int = (*(*p).kid[0 as libc::c_int as usize]).height;
    let mut h1: libc::c_int = (*(*p).kid[1 as libc::c_int as usize]).height;
    let mut b: libc::c_int = h0 - h1;
    if (*p).height != 1 as libc::c_int + max(h0, h1) || b < -(1 as libc::c_int)
        || b > 1 as libc::c_int
    {
        printf(
            b"node %d bad, balance %d\n\0" as *const u8 as *const libc::c_char,
            (*p).payload,
            b,
        );
        show_tree(p, 0 as *mut trunk, 0 as libc::c_int);
        abort();
    }
    return (verify((*p).kid[0 as libc::c_int as usize]) != 0
        && verify((*p).kid[1 as libc::c_int as usize]) != 0) as libc::c_int;
}
unsafe fn main_0() -> libc::c_int {
    let mut x: libc::c_int = 0;
    let mut root: *mut node = nnil;
    srand(time(0 as *mut time_t) as libc::c_uint);
    x = 0 as libc::c_int;
    while x < 10 as libc::c_int * 32 as libc::c_int {
        if rand() & 1 as libc::c_int != 0 {
            insert(&mut root, rand() % 32 as libc::c_int);
        } else {
            delete(&mut root, rand() % 32 as libc::c_int);
        }
        verify(root);
        x += 1;
        x;
    }
    puts(b"Tree is:\0" as *const u8 as *const libc::c_char);
    show_tree(root, 0 as *mut trunk, 0 as libc::c_int);
    puts(b"\nQuerying values:\0" as *const u8 as *const libc::c_char);
    x = 0 as libc::c_int;
    while x < 32 as libc::c_int {
        let mut p: *mut node = query(root, x);
        if !p.is_null() {
            printf(
                b"%2d found: %p %d\n\0" as *const u8 as *const libc::c_char,
                x,
                p,
                (*p).payload,
            );
        }
        x += 1;
        x;
    }
    x = 0 as libc::c_int;
    while x < 32 as libc::c_int {
        delete(&mut root, x);
        verify(root);
        x += 1;
        x;
    }
    puts(b"\nAfter deleting all values, tree is:\0" as *const u8 as *const libc::c_char);
    show_tree(root, 0 as *mut trunk, 0 as libc::c_int);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
