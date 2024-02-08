#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
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

use c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn printf(_: *const i8, _: ...) -> i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: i32) -> !;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut i8,
    pub _IO_read_end: *mut i8,
    pub _IO_read_base: *mut i8,
    pub _IO_write_base: *mut i8,
    pub _IO_write_ptr: *mut i8,
    pub _IO_write_end: *mut i8,
    pub _IO_buf_base: *mut i8,
    pub _IO_buf_end: *mut i8,
    pub _IO_save_base: *mut i8,
    pub _IO_backup_base: *mut i8,
    pub _IO_save_end: *mut i8,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: i64,
    pub _cur_column: u16,
    pub _vtable_offset: i8,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: i64,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: u64,
    pub _mode: i32,
    pub _unused2: [i8; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type string = *mut u8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_t {
    pub tag: u32,
    pub data: C2RustUnnamed,
    pub next: *mut node,
}
pub type node = node_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub str_0: string,
    pub root: *mut node,
}
pub const NODE_SEQ: u32 = 2;
pub const NODE_TREE: u32 = 1;
pub const NODE_LEAF: u32 = 0;
#[no_mangle]
pub extern "C" fn allocate_node(mut tag: u32) -> *mut node {
    unsafe {
        let mut n: *mut node = malloc(::core::mem::size_of::<node>() as u64) as *mut node;
        if n.is_null() {
            fprintf(
                stderr,
                b"Failed to allocate node for tag: %d\n\0" as *const u8 as *const i8,
                tag as u32,
            );
            exit(1);
        };
        (*n).tag = tag;
        (*n).next = 0 as *mut node;
        return n;
    }
}

#[no_mangle]
pub extern "C" fn make_leaf(mut str: string) -> *mut node {
    unsafe {
        let mut n: *mut node = allocate_node(NODE_LEAF);
        (*n).data.str_0 = str;
        return n;
    }
}

#[no_mangle]
pub extern "C" fn make_tree() -> *mut node {
    unsafe {
        let mut n: *mut node = allocate_node(NODE_TREE);
        (*n).data.root = 0 as *mut node;
        return n;
    }
}

#[no_mangle]
pub extern "C" fn make_seq() -> *mut node {
    unsafe {
        let mut n: *mut node = allocate_node(NODE_SEQ);
        (*n).data.root = 0 as *mut node;
        return n;
    }
}

#[no_mangle]
pub extern "C" fn deallocate_node(mut n: *mut node) {
    unsafe {
        if n.is_null() {
            return;
        }
        deallocate_node((*n).next);
        (*n).next = 0 as *mut node;
        if (*n).tag as u32 == NODE_LEAF as u32 {
            free((*n).data.str_0 as *mut libc::c_void);
            (*n).data.str_0 = 0 as string;
        } else if (*n).tag as u32 == NODE_TREE as u32 || (*n).tag as u32 == NODE_SEQ as u32 {
            deallocate_node((*n).data.root);
            (*n).data.root = 0 as *mut node;
        } else {
            fprintf(
                stderr,
                b"Cannot deallocate node with tag: %d\n\0" as *const u8 as *const i8,
                (*n).tag as u32,
            );
            exit(1);
        }
        free(n as *mut libc::c_void);
    }
}

#[no_mangle]
pub extern "C" fn append(mut root: *mut node, mut elem: *mut node) {
    unsafe {
        if root.is_null() {
            fprintf(
                stderr,
                b"Cannot append to uninitialized node.\0" as *const u8 as *const i8,
            );
            exit(1);
        }
        if elem.is_null() {
            return;
        }
        if (*root).tag as u32 == NODE_SEQ as u32 || (*root).tag as u32 == NODE_TREE as u32 {
            if ((*root).data.root).is_null() {
                (*root).data.root = elem;
            } else {
                let mut it: *mut node = (*root).data.root;
                while !((*it).next).is_null() {
                    it = (*it).next;
                }
                (*it).next = elem;
            }
        } else {
            fprintf(
                stderr,
                b"Cannot append to node with tag: %d\n\0" as *const u8 as *const i8,
                (*root).tag as u32,
            );
            exit(1);
        };
    }
}

#[no_mangle]
pub extern "C" fn count(mut n: *mut node) -> u64 {
    unsafe {
        if n.is_null() {
            return 0;
        }
        if (*n).tag as u32 == NODE_LEAF as u32 {
            return 1;
        }
        if (*n).tag as u32 == NODE_TREE as u32 {
            let mut sum: u64 = 0;
            let mut it: *mut node = (*n).data.root;
            while !it.is_null() {
                sum = (sum).wrapping_add(count(it)) as u64;
                it = (*it).next;
            }
            return sum;
        }
        if (*n).tag as u32 == NODE_SEQ as u32 {
            let mut prod: u64 = 1;
            let mut it_0: *mut node = (*n).data.root;
            while !it_0.is_null() {
                prod = (prod).wrapping_mul(count(it_0)) as u64;
                it_0 = (*it_0).next;
            }
            return prod;
        }
        fprintf(
            stderr,
            b"Cannot count node with tag: %d\n\0" as *const u8 as *const i8,
            (*n).tag as u32,
        );
        exit(1);
    }
}

#[no_mangle]
pub extern "C" fn expand(mut n: *mut node, mut pos: u64) {
    unsafe {
        if n.is_null() {
            return;
        }
        if (*n).tag as u32 == NODE_LEAF as u32 {
            printf((*n).data.str_0 as *const i8);
        } else if (*n).tag as u32 == NODE_TREE as u32 {
            let mut it: *mut node = (*n).data.root;
            loop {
                let mut cnt: u64 = count(it);
                if pos < cnt {
                    expand(it, pos);
                    break;
                } else {
                    pos = (pos as u64).wrapping_sub(cnt) as u64;
                    it = (*it).next;
                }
            }
        } else if (*n).tag as u32 == NODE_SEQ as u32 {
            let mut prod: u64 = pos;
            let mut it_0: *mut node = (*n).data.root;
            while !it_0.is_null() {
                let mut cnt_0: u64 = count(it_0);
                let mut rem: u64 = prod.wrapping_rem(cnt_0);
                expand(it_0, rem);
                it_0 = (*it_0).next;
            }
        } else {
            fprintf(
                stderr,
                b"Cannot expand node with tag: %d\n\0" as *const u8 as *const i8,
                (*n).tag as u32,
            );
            exit(1);
        };
    }
}

#[no_mangle]
pub extern "C" fn allocate_string(mut src: string) -> string {
    unsafe {
        let mut len: u64 = strlen(src as *const i8);
        let mut out: string =
            calloc(len.wrapping_add(1), ::core::mem::size_of::<u8>() as u64) as string;
        if out.is_null() {
            fprintf(
                stderr,
                b"Failed to allocate a copy of the string.\0" as *const u8 as *const i8,
            );
            exit(1);
        }
        strcpy(out as *mut i8, src as *const i8);
        return out;
    }
}

#[no_mangle]
pub extern "C" fn parse_tree(mut input: string, mut pos: *mut u64) -> *mut node {
    unsafe {
        let mut root: *mut node = make_tree();
        let mut buffer: [u8; 128] = [0; 128];
        let mut bufpos: u64 = 0;
        let mut depth: u64 = 0;
        let mut asSeq: bool = 0 != 0;
        let mut allow: bool = 0 != 0;
        while *input.offset(*pos as isize) as i32 != 0 {
            let fresh0 = *pos;
            *pos = (*pos).wrapping_add(1);
            let mut c: u8 = *input.offset(fresh0 as isize);
            if c as i32 == '\\' as i32 {
                let fresh1 = *pos;
                *pos = (*pos).wrapping_add(1);
                c = *input.offset(fresh1 as isize);
                if c as i32 == 0 {
                    break;
                }
                let fresh2 = bufpos;
                bufpos = bufpos.wrapping_add(1);
                buffer[fresh2 as usize] = '\\' as u8;
                let fresh3 = bufpos;
                bufpos = bufpos.wrapping_add(1);
                buffer[fresh3 as usize] = c;
                buffer[bufpos as usize] = 0;
            } else if c as i32 == '{' as i32 {
                let fresh4 = bufpos;
                bufpos = bufpos.wrapping_add(1);
                buffer[fresh4 as usize] = c;
                buffer[bufpos as usize] = 0;
                asSeq = 1 != 0;
                depth = depth.wrapping_add(1);
                depth;
            } else if c as i32 == '}' as i32 {
                let fresh5 = depth;
                depth = depth.wrapping_sub(1);
                if fresh5 > 0 {
                    let fresh6 = bufpos;
                    bufpos = bufpos.wrapping_add(1);
                    buffer[fresh6 as usize] = c;
                    buffer[bufpos as usize] = 0;
                } else {
                    if asSeq {
                        let mut new_pos: u64 = 0;
                        let mut seq: *mut node = parse_seq(buffer.as_mut_ptr(), &mut new_pos);
                        append(root, seq);
                    } else {
                        append(root, make_leaf(allocate_string(buffer.as_mut_ptr())));
                    }
                    break;
                }
            } else if c as i32 == ',' as i32 {
                if depth == 0 {
                    if asSeq {
                        let mut new_pos_0: u64 = 0;
                        let mut seq_0: *mut node = parse_seq(buffer.as_mut_ptr(), &mut new_pos_0);
                        append(root, seq_0);
                        bufpos = 0;
                        buffer[bufpos as usize] = 0;
                        asSeq = 0 != 0;
                    } else {
                        append(root, make_leaf(allocate_string(buffer.as_mut_ptr())));
                        bufpos = 0;
                        buffer[bufpos as usize] = 0;
                    }
                } else {
                    let fresh7 = bufpos;
                    bufpos = bufpos.wrapping_add(1);
                    buffer[fresh7 as usize] = c;
                    buffer[bufpos as usize] = 0;
                }
            } else {
                let fresh8 = bufpos;
                bufpos = bufpos.wrapping_add(1);
                buffer[fresh8 as usize] = c;
                buffer[bufpos as usize] = 0;
            }
        }
        return root;
    }
}

#[no_mangle]
pub extern "C" fn parse_seq(mut input: string, mut pos: *mut u64) -> *mut node {
    unsafe {
        let mut root: *mut node = make_seq();
        let mut buffer: [u8; 128] = [0; 128];
        let mut bufpos: u64 = 0;
        while *input.offset(*pos as isize) as i32 != 0 {
            let fresh9 = *pos;
            *pos = (*pos).wrapping_add(1);
            let mut c: u8 = *input.offset(fresh9 as isize);
            if c as i32 == '\\' as i32 {
                let fresh10 = *pos;
                *pos = (*pos).wrapping_add(1);
                c = *input.offset(fresh10 as isize);
                if c as i32 == 0 {
                    break;
                }
                let fresh11 = bufpos;
                bufpos = bufpos.wrapping_add(1);
                buffer[fresh11 as usize] = c;
                buffer[bufpos as usize] = 0;
            } else if c as i32 == '{' as i32 {
                let mut tree: *mut node = parse_tree(input, pos);
                if bufpos > 0 {
                    append(root, make_leaf(allocate_string(buffer.as_mut_ptr())));
                    bufpos = 0;
                    buffer[bufpos as usize] = 0;
                }
                append(root, tree);
            } else {
                let fresh12 = bufpos;
                bufpos = bufpos.wrapping_add(1);
                buffer[fresh12 as usize] = c;
                buffer[bufpos as usize] = 0;
            }
        }
        if bufpos > 0 {
            append(root, make_leaf(allocate_string(buffer.as_mut_ptr())));
            bufpos = 0;
            buffer[bufpos as usize] = 0;
        }
        return root;
    }
}

#[no_mangle]
pub extern "C" fn test(mut input: string) {
    unsafe {
        let mut pos: u64 = 0;
        let mut n: *mut node = parse_seq(input, &mut pos);
        let mut cnt: u64 = count(n);
        let mut i: u64 = 0;
        print!("Pattern: {}\n", build_str_from_raw_ptr(input as *mut u8));
        i = 0;
        while i < cnt {
            expand(n, i);
            print!("\n");
            i = i.wrapping_add(1);
            i;
        }
        print!("\n");
        deallocate_node(n);
    }
}

fn main_0() -> i32 {
    test(b"~/{Downloads,Pictures}/*.{jpg,gif,png}\0" as *const u8 as *const i8 as string);
    test(b"It{{em,alic}iz,erat}e{d,}, please.\0" as *const u8 as *const i8 as string);
    test(b"{,{,gotta have{ ,\\, again\\, }}more }cowbell!\0" as *const u8 as *const i8 as string);
    return 0;
}

pub fn main() {
    unsafe {
        ::std::process::exit(main_0() as i32);
    }
}
