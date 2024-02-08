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
    fn puts(__s: *const i8) -> i32;
    fn rand() -> i32;
    fn srand(__seed: u32);
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn abort() -> !;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node {
    pub payload: i32,
    pub height: i32,
    pub kid: [*mut node; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct trunk {
    pub prev: *mut trunk,
    pub str_0: *mut i8,
}
#[no_mangle]
pub static mut dummy: node = unsafe {
    {
        let mut init = node {
            payload: 0,
            height: 0,
            kid: [
                &dummy as *const node as *mut node,
                &dummy as *const node as *mut node,
            ],
        };
        init
    }
};
#[no_mangle]
pub static mut nnil: *mut node = unsafe { &dummy as *const node as *mut node };
#[no_mangle]
pub extern "C" fn new_node(mut value: i32) -> *mut node {
    unsafe {
        let mut n: *mut node = calloc(1, ::core::mem::size_of::<node>() as u64) as *mut node;
        *n = {
            let mut init = node {
                payload: value,
                height: 1,
                kid: [nnil, nnil],
            };
            init
        };
        return n;
    }
}

#[no_mangle]
pub extern "C" fn max(mut a: i32, mut b: i32) -> i32 {
    return if a > b { a } else { b };
}

#[no_mangle]
pub extern "C" fn set_height(mut n: *mut node) {
    unsafe {
        (*n).height = 1 + max(
            (*(*n).kid[0 as usize]).height,
            (*(*n).kid[1 as usize]).height,
        );
    }
}

#[no_mangle]
pub extern "C" fn ballance(mut n: *mut node) -> i32 {
    unsafe {
        return (*(*n).kid[0 as usize]).height - (*(*n).kid[1 as usize]).height;
    }
}

#[no_mangle]
pub extern "C" fn rotate(mut rootp: *mut *mut node, mut dir: i32) -> *mut node {
    unsafe {
        let mut old_r: *mut node = *rootp;
        let mut new_r: *mut node = (*old_r).kid[dir as usize];
        *rootp = new_r;
        if nnil == *rootp {
            free(old_r as *mut libc::c_void);
        } else {
            (*old_r).kid[dir as usize] = (*new_r).kid[(dir == 0) as i32 as usize];
            set_height(old_r);
            (*new_r).kid[(dir == 0) as i32 as usize] = old_r;
        }
        return new_r;
    }
}

#[no_mangle]
pub extern "C" fn adjust_balance(mut rootp: *mut *mut node) {
    unsafe {
        let mut root: *mut node = *rootp;
        let mut b: i32 = ballance(root) / 2;
        if b != 0 {
            let mut dir: i32 = (1 - b) / 2;
            if ballance((*root).kid[dir as usize]) == -b {
                rotate(
                    &mut *((*root).kid).as_mut_ptr().offset(dir as isize),
                    (dir == 0) as i32,
                );
            }
            root = rotate(rootp, dir);
        }
        if root != nnil {
            set_height(root);
        }
    }
}

#[no_mangle]
pub extern "C" fn query(mut root: *mut node, mut value: i32) -> *mut node {
    unsafe {
        return if root == nnil {
            0 as *mut node
        } else if (*root).payload == value {
            root
        } else {
            query(
                (*root).kid[(value > (*root).payload) as i32 as usize],
                value,
            )
        };
    }
}

#[no_mangle]
pub extern "C" fn insert(mut rootp: *mut *mut node, mut value: i32) {
    unsafe {
        let mut root: *mut node = *rootp;
        if root == nnil {
            *rootp = new_node(value);
        } else if value != (*root).payload {
            insert(
                &mut *((*root).kid)
                    .as_mut_ptr()
                    .offset((value > (*root).payload) as i32 as isize),
                value,
            );
            adjust_balance(rootp);
        }
    }
}

#[no_mangle]
pub extern "C" fn delete(mut rootp: *mut *mut node, mut value: i32) {
    unsafe {
        let mut root: *mut node = *rootp;
        if root == nnil {
            return;
        }
        if (*root).payload == value {
            root = rotate(rootp, (ballance(root) < 0) as i32);
            if nnil == root {
                return;
            }
        }
        delete(
            &mut *((*root).kid)
                .as_mut_ptr()
                .offset((value > (*root).payload) as i32 as isize),
            value,
        );
        adjust_balance(rootp);
    }
}

#[no_mangle]
pub extern "C" fn show_trunks(mut p: *mut trunk) {
    unsafe {
        if p.is_null() {
            return;
        }
        show_trunks((*p).prev);
        print!("{}", build_str_from_raw_ptr((*p).str_0 as *mut u8));
    }
}

#[no_mangle]
pub extern "C" fn show_tree(mut root: *mut node, mut prev: *mut trunk, mut is_left: i32) {
    unsafe {
        if root == nnil {
            return;
        }
        let mut this_disp: trunk = {
            let mut init = trunk {
                prev: prev,
                str_0: b"    \0" as *const u8 as *const i8 as *mut i8,
            };
            init
        };
        let mut prev_str: *mut i8 = this_disp.str_0;
        show_tree((*root).kid[0 as usize], &mut this_disp, 1);
        if prev.is_null() {
            this_disp.str_0 = b"---\0" as *const u8 as *const i8 as *mut i8;
        } else if is_left != 0 {
            this_disp.str_0 = b".--\0" as *const u8 as *const i8 as *mut i8;
            prev_str = b"   |\0" as *const u8 as *const i8 as *mut i8;
        } else {
            this_disp.str_0 = b"`--\0" as *const u8 as *const i8 as *mut i8;
            (*prev).str_0 = prev_str;
        }
        show_trunks(&mut this_disp);
        print!("{}\n", (*root).payload);
        if !prev.is_null() {
            (*prev).str_0 = prev_str;
        }
        this_disp.str_0 = b"   |\0" as *const u8 as *const i8 as *mut i8;
        show_tree((*root).kid[1 as usize], &mut this_disp, 0);
        if prev.is_null() {
            puts(b"\0" as *const u8 as *const i8);
        }
    }
}

#[no_mangle]
pub extern "C" fn verify(mut p: *mut node) -> i32 {
    unsafe {
        if p == nnil {
            return 1;
        }
        let mut h0: i32 = (*(*p).kid[0 as usize]).height;
        let mut h1: i32 = (*(*p).kid[1 as usize]).height;
        let mut b: i32 = h0 - h1;
        if (*p).height != 1 + max(h0, h1) || b < -1 || b > 1 {
            print!("node {} bad, balance {}\n", (*p).payload, b);
            show_tree(p, 0 as *mut trunk, 0);
            abort();
        }
        return (verify((*p).kid[0 as usize]) != 0 && verify((*p).kid[1 as usize]) != 0) as i32;
    }
}

fn main_0() -> i32 {
    unsafe {
        let mut x: i32 = 0;
        let mut root: *mut node = nnil;
        srand(rust_time(None) as u32);
        x = 0;
        while x < 10 * 32 {
            if rand() & 1 != 0 {
                insert(&mut root, rand() % 32);
            } else {
                delete(&mut root, rand() % 32);
            }
            verify(root);
            x += 1;
            x;
        }
        puts(b"Tree is:\0" as *const u8 as *const i8);
        show_tree(root, 0 as *mut trunk, 0);
        puts(b"\nQuerying values:\0" as *const u8 as *const i8);
        x = 0;
        while x < 32 {
            let mut p: *mut node = query(root, x);
            if !p.is_null() {
                print!("{:2} found: {:p} {}\n", x, p, (*p).payload);
            }
            x += 1;
            x;
        }
        x = 0;
        while x < 32 {
            delete(&mut root, x);
            verify(root);
            x += 1;
            x;
        }
        puts(b"\nAfter deleting all values, tree is:\0" as *const u8 as *const i8);
        show_tree(root, 0 as *mut trunk, 0);
        return 0;
    }
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
