#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type character = libc::c_uchar;
pub type string = *mut character;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_t {
    pub tag: tag_t,
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
pub type tag_t = libc::c_uint;
pub const NODE_SEQ: tag_t = 2;
pub const NODE_TREE: tag_t = 1;
pub const NODE_LEAF: tag_t = 0;
#[no_mangle]
pub unsafe extern "C" fn allocate_node(mut tag: tag_t) -> *mut node {
    let mut n: *mut node = malloc(::core::mem::size_of::<node>() as libc::c_ulong)
        as *mut node;
    if n.is_null() {
        fprintf(
            stderr,
            b"Failed to allocate node for tag: %d\n\0" as *const u8
                as *const libc::c_char,
            tag as libc::c_uint,
        );
        exit(1 as libc::c_int);
    }
    (*n).tag = tag;
    (*n).next = 0 as *mut node;
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn make_leaf(mut str: string) -> *mut node {
    let mut n: *mut node = allocate_node(NODE_LEAF);
    (*n).data.str_0 = str;
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn make_tree() -> *mut node {
    let mut n: *mut node = allocate_node(NODE_TREE);
    (*n).data.root = 0 as *mut node;
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn make_seq() -> *mut node {
    let mut n: *mut node = allocate_node(NODE_SEQ);
    (*n).data.root = 0 as *mut node;
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn deallocate_node(mut n: *mut node) {
    if n.is_null() {
        return;
    }
    deallocate_node((*n).next);
    (*n).next = 0 as *mut node;
    if (*n).tag as libc::c_uint == NODE_LEAF as libc::c_int as libc::c_uint {
        free((*n).data.str_0 as *mut libc::c_void);
        (*n).data.str_0 = 0 as string;
    } else if (*n).tag as libc::c_uint == NODE_TREE as libc::c_int as libc::c_uint
        || (*n).tag as libc::c_uint == NODE_SEQ as libc::c_int as libc::c_uint
    {
        deallocate_node((*n).data.root);
        (*n).data.root = 0 as *mut node;
    } else {
        fprintf(
            stderr,
            b"Cannot deallocate node with tag: %d\n\0" as *const u8
                as *const libc::c_char,
            (*n).tag as libc::c_uint,
        );
        exit(1 as libc::c_int);
    }
    free(n as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn append(mut root: *mut node, mut elem: *mut node) {
    if root.is_null() {
        fprintf(
            stderr,
            b"Cannot append to uninitialized node.\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    if elem.is_null() {
        return;
    }
    if (*root).tag as libc::c_uint == NODE_SEQ as libc::c_int as libc::c_uint
        || (*root).tag as libc::c_uint == NODE_TREE as libc::c_int as libc::c_uint
    {
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
            b"Cannot append to node with tag: %d\n\0" as *const u8
                as *const libc::c_char,
            (*root).tag as libc::c_uint,
        );
        exit(1 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn count(mut n: *mut node) -> size_t {
    if n.is_null() {
        return 0 as libc::c_int as size_t;
    }
    if (*n).tag as libc::c_uint == NODE_LEAF as libc::c_int as libc::c_uint {
        return 1 as libc::c_int as size_t;
    }
    if (*n).tag as libc::c_uint == NODE_TREE as libc::c_int as libc::c_uint {
        let mut sum: size_t = 0 as libc::c_int as size_t;
        let mut it: *mut node = (*n).data.root;
        while !it.is_null() {
            sum = (sum as libc::c_ulong).wrapping_add(count(it)) as size_t as size_t;
            it = (*it).next;
        }
        return sum;
    }
    if (*n).tag as libc::c_uint == NODE_SEQ as libc::c_int as libc::c_uint {
        let mut prod: size_t = 1 as libc::c_int as size_t;
        let mut it_0: *mut node = (*n).data.root;
        while !it_0.is_null() {
            prod = (prod as libc::c_ulong).wrapping_mul(count(it_0)) as size_t as size_t;
            it_0 = (*it_0).next;
        }
        return prod;
    }
    fprintf(
        stderr,
        b"Cannot count node with tag: %d\n\0" as *const u8 as *const libc::c_char,
        (*n).tag as libc::c_uint,
    );
    exit(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn expand(mut n: *mut node, mut pos: size_t) {
    if n.is_null() {
        return;
    }
    if (*n).tag as libc::c_uint == NODE_LEAF as libc::c_int as libc::c_uint {
        printf((*n).data.str_0 as *const libc::c_char);
    } else if (*n).tag as libc::c_uint == NODE_TREE as libc::c_int as libc::c_uint {
        let mut it: *mut node = (*n).data.root;
        loop {
            let mut cnt: size_t = count(it);
            if pos < cnt {
                expand(it, pos);
                break;
            } else {
                pos = (pos as libc::c_ulong).wrapping_sub(cnt) as size_t as size_t;
                it = (*it).next;
            }
        }
    } else if (*n).tag as libc::c_uint == NODE_SEQ as libc::c_int as libc::c_uint {
        let mut prod: size_t = pos;
        let mut it_0: *mut node = (*n).data.root;
        while !it_0.is_null() {
            let mut cnt_0: size_t = count(it_0);
            let mut rem: size_t = prod.wrapping_rem(cnt_0);
            expand(it_0, rem);
            it_0 = (*it_0).next;
        }
    } else {
        fprintf(
            stderr,
            b"Cannot expand node with tag: %d\n\0" as *const u8 as *const libc::c_char,
            (*n).tag as libc::c_uint,
        );
        exit(1 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn allocate_string(mut src: string) -> string {
    let mut len: size_t = strlen(src as *const libc::c_char);
    let mut out: string = calloc(
        len.wrapping_add(1 as libc::c_int as libc::c_ulong),
        ::core::mem::size_of::<character>() as libc::c_ulong,
    ) as string;
    if out.is_null() {
        fprintf(
            stderr,
            b"Failed to allocate a copy of the string.\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    strcpy(out as *mut libc::c_char, src as *const libc::c_char);
    return out;
}
#[no_mangle]
pub unsafe extern "C" fn parse_tree(
    mut input: string,
    mut pos: *mut size_t,
) -> *mut node {
    let mut root: *mut node = make_tree();
    let mut buffer: [character; 128] = [
        0 as libc::c_int as character,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut bufpos: size_t = 0 as libc::c_int as size_t;
    let mut depth: size_t = 0 as libc::c_int as size_t;
    let mut asSeq: bool = 0 as libc::c_int != 0;
    let mut allow: bool = 0 as libc::c_int != 0;
    while *input.offset(*pos as isize) as libc::c_int != 0 as libc::c_int {
        let fresh0 = *pos;
        *pos = (*pos).wrapping_add(1);
        let mut c: character = *input.offset(fresh0 as isize);
        if c as libc::c_int == '\\' as i32 {
            let fresh1 = *pos;
            *pos = (*pos).wrapping_add(1);
            c = *input.offset(fresh1 as isize);
            if c as libc::c_int == 0 as libc::c_int {
                break;
            }
            let fresh2 = bufpos;
            bufpos = bufpos.wrapping_add(1);
            buffer[fresh2 as usize] = '\\' as i32 as character;
            let fresh3 = bufpos;
            bufpos = bufpos.wrapping_add(1);
            buffer[fresh3 as usize] = c;
            buffer[bufpos as usize] = 0 as libc::c_int as character;
        } else if c as libc::c_int == '{' as i32 {
            let fresh4 = bufpos;
            bufpos = bufpos.wrapping_add(1);
            buffer[fresh4 as usize] = c;
            buffer[bufpos as usize] = 0 as libc::c_int as character;
            asSeq = 1 as libc::c_int != 0;
            depth = depth.wrapping_add(1);
            depth;
        } else if c as libc::c_int == '}' as i32 {
            let fresh5 = depth;
            depth = depth.wrapping_sub(1);
            if fresh5 > 0 as libc::c_int as libc::c_ulong {
                let fresh6 = bufpos;
                bufpos = bufpos.wrapping_add(1);
                buffer[fresh6 as usize] = c;
                buffer[bufpos as usize] = 0 as libc::c_int as character;
            } else {
                if asSeq {
                    let mut new_pos: size_t = 0 as libc::c_int as size_t;
                    let mut seq: *mut node = parse_seq(
                        buffer.as_mut_ptr(),
                        &mut new_pos,
                    );
                    append(root, seq);
                } else {
                    append(root, make_leaf(allocate_string(buffer.as_mut_ptr())));
                }
                break;
            }
        } else if c as libc::c_int == ',' as i32 {
            if depth == 0 as libc::c_int as libc::c_ulong {
                if asSeq {
                    let mut new_pos_0: size_t = 0 as libc::c_int as size_t;
                    let mut seq_0: *mut node = parse_seq(
                        buffer.as_mut_ptr(),
                        &mut new_pos_0,
                    );
                    append(root, seq_0);
                    bufpos = 0 as libc::c_int as size_t;
                    buffer[bufpos as usize] = 0 as libc::c_int as character;
                    asSeq = 0 as libc::c_int != 0;
                } else {
                    append(root, make_leaf(allocate_string(buffer.as_mut_ptr())));
                    bufpos = 0 as libc::c_int as size_t;
                    buffer[bufpos as usize] = 0 as libc::c_int as character;
                }
            } else {
                let fresh7 = bufpos;
                bufpos = bufpos.wrapping_add(1);
                buffer[fresh7 as usize] = c;
                buffer[bufpos as usize] = 0 as libc::c_int as character;
            }
        } else {
            let fresh8 = bufpos;
            bufpos = bufpos.wrapping_add(1);
            buffer[fresh8 as usize] = c;
            buffer[bufpos as usize] = 0 as libc::c_int as character;
        }
    }
    return root;
}
#[no_mangle]
pub unsafe extern "C" fn parse_seq(
    mut input: string,
    mut pos: *mut size_t,
) -> *mut node {
    let mut root: *mut node = make_seq();
    let mut buffer: [character; 128] = [
        0 as libc::c_int as character,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut bufpos: size_t = 0 as libc::c_int as size_t;
    while *input.offset(*pos as isize) as libc::c_int != 0 as libc::c_int {
        let fresh9 = *pos;
        *pos = (*pos).wrapping_add(1);
        let mut c: character = *input.offset(fresh9 as isize);
        if c as libc::c_int == '\\' as i32 {
            let fresh10 = *pos;
            *pos = (*pos).wrapping_add(1);
            c = *input.offset(fresh10 as isize);
            if c as libc::c_int == 0 as libc::c_int {
                break;
            }
            let fresh11 = bufpos;
            bufpos = bufpos.wrapping_add(1);
            buffer[fresh11 as usize] = c;
            buffer[bufpos as usize] = 0 as libc::c_int as character;
        } else if c as libc::c_int == '{' as i32 {
            let mut tree: *mut node = parse_tree(input, pos);
            if bufpos > 0 as libc::c_int as libc::c_ulong {
                append(root, make_leaf(allocate_string(buffer.as_mut_ptr())));
                bufpos = 0 as libc::c_int as size_t;
                buffer[bufpos as usize] = 0 as libc::c_int as character;
            }
            append(root, tree);
        } else {
            let fresh12 = bufpos;
            bufpos = bufpos.wrapping_add(1);
            buffer[fresh12 as usize] = c;
            buffer[bufpos as usize] = 0 as libc::c_int as character;
        }
    }
    if bufpos > 0 as libc::c_int as libc::c_ulong {
        append(root, make_leaf(allocate_string(buffer.as_mut_ptr())));
        bufpos = 0 as libc::c_int as size_t;
        buffer[bufpos as usize] = 0 as libc::c_int as character;
    }
    return root;
}
#[no_mangle]
pub unsafe extern "C" fn test(mut input: string) {
    let mut pos: size_t = 0 as libc::c_int as size_t;
    let mut n: *mut node = parse_seq(input, &mut pos);
    let mut cnt: size_t = count(n);
    let mut i: size_t = 0;
    printf(b"Pattern: %s\n\0" as *const u8 as *const libc::c_char, input);
    i = 0 as libc::c_int as size_t;
    while i < cnt {
        expand(n, i);
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        i = i.wrapping_add(1);
        i;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    deallocate_node(n);
}
unsafe fn main_0() -> libc::c_int {
    test(
        b"~/{Downloads,Pictures}/*.{jpg,gif,png}\0" as *const u8 as *const libc::c_char
            as string,
    );
    test(
        b"It{{em,alic}iz,erat}e{d,}, please.\0" as *const u8 as *const libc::c_char
            as string,
    );
    test(
        b"{,{,gotta have{ ,\\, again\\, }}more }cowbell!\0" as *const u8
            as *const libc::c_char as string,
    );
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
