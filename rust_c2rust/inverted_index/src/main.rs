#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct trie_t {
    pub next: [trie; 41],
    pub eow: libc::c_int,
}
pub type trie = *mut trie_t;
#[no_mangle]
pub static mut chr_legal: [libc::c_char; 41] = unsafe {
    *::core::mem::transmute::<
        &[u8; 41],
        &mut [libc::c_char; 41],
    >(b"abcdefghijklmnopqrstuvwxyz0123456789_-./\0")
};
#[no_mangle]
pub static mut chr_idx: [libc::c_int; 256] = [
    0 as libc::c_int,
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
    0,
];
#[no_mangle]
pub static mut idx_chr: [libc::c_char; 256] = [
    0 as libc::c_int as libc::c_char,
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
    0,
];
#[no_mangle]
pub unsafe extern "C" fn trie_new() -> trie {
    return calloc(
        ::core::mem::size_of::<trie_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
    ) as trie;
}
#[no_mangle]
pub unsafe extern "C" fn trie_trav(
    mut root: trie,
    mut str: *const libc::c_char,
    mut no_create: libc::c_int,
) -> trie {
    let mut c: libc::c_int = 0;
    while !root.is_null() {
        c = *str.offset(0 as libc::c_int as isize) as libc::c_int;
        if c == '\0' as i32 {
            if (*root).eow == 0 && no_create != 0 {
                return 0 as trie;
            }
            break;
        } else {
            c = chr_idx[c as usize];
            if c == 0 {
                str = str.offset(1);
                str;
            } else {
                if ((*root).next[c as usize]).is_null() {
                    if no_create != 0 {
                        return 0 as trie;
                    }
                    (*root).next[c as usize] = trie_new();
                }
                root = (*root).next[c as usize];
                str = str.offset(1);
                str;
            }
        }
    }
    return root;
}
#[no_mangle]
pub unsafe extern "C" fn trie_all(
    mut root: trie,
    mut path: *mut libc::c_char,
    mut depth: libc::c_int,
    mut callback: Option::<unsafe extern "C" fn(*mut libc::c_char) -> libc::c_int>,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if (*root).eow != 0 && callback.expect("non-null function pointer")(path) == 0 {
        return 0 as libc::c_int;
    }
    i = 1 as libc::c_int;
    while (i as libc::c_ulong)
        < ::core::mem::size_of::<[libc::c_char; 41]>() as libc::c_ulong
    {
        if !((*root).next[i as usize]).is_null() {
            *path.offset(depth as isize) = idx_chr[i as usize];
            *path
                .offset(
                    (depth + 1 as libc::c_int) as isize,
                ) = '\0' as i32 as libc::c_char;
            if trie_all(
                (*root).next[i as usize],
                path,
                depth + 1 as libc::c_int,
                callback,
            ) == 0
            {
                return 0 as libc::c_int;
            }
        }
        i += 1;
        i;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn add_index(
    mut root: trie,
    mut word: *const libc::c_char,
    mut fname: *const libc::c_char,
) {
    let mut x: trie = trie_trav(root, word, 0 as libc::c_int);
    (*x).eow = 1 as libc::c_int;
    if ((*x).next[0 as libc::c_int as usize]).is_null() {
        (*x).next[0 as libc::c_int as usize] = trie_new();
    }
    x = trie_trav((*x).next[0 as libc::c_int as usize], fname, 0 as libc::c_int);
    (*x).eow = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn print_path(mut path: *mut libc::c_char) -> libc::c_int {
    printf(b" %s\0" as *const u8 as *const libc::c_char, path);
    return 1 as libc::c_int;
}
#[no_mangle]
pub static mut files: [*const libc::c_char; 3] = [
    b"f1.txt\0" as *const u8 as *const libc::c_char,
    b"source/f2.txt\0" as *const u8 as *const libc::c_char,
    b"other_file\0" as *const u8 as *const libc::c_char,
];
#[no_mangle]
pub static mut text: [[*const libc::c_char; 5]; 3] = [
    [
        b"it\0" as *const u8 as *const libc::c_char,
        b"is\0" as *const u8 as *const libc::c_char,
        b"what\0" as *const u8 as *const libc::c_char,
        b"it\0" as *const u8 as *const libc::c_char,
        b"is\0" as *const u8 as *const libc::c_char,
    ],
    [
        b"what\0" as *const u8 as *const libc::c_char,
        b"is\0" as *const u8 as *const libc::c_char,
        b"it\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
    ],
    [
        b"it\0" as *const u8 as *const libc::c_char,
        b"is\0" as *const u8 as *const libc::c_char,
        b"a\0" as *const u8 as *const libc::c_char,
        b"banana\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ],
];
#[no_mangle]
pub unsafe extern "C" fn init_tables() -> trie {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut root: trie = trie_new();
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < ::core::mem::size_of::<[libc::c_char; 41]>() as libc::c_ulong
    {
        chr_idx[chr_legal[i as usize] as libc::c_int as usize] = i + 1 as libc::c_int;
        idx_chr[(i + 1 as libc::c_int) as usize] = chr_legal[i as usize];
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 5 as libc::c_int {
            if (text[i as usize][j as usize]).is_null() {
                break;
            }
            add_index(root, text[i as usize][j as usize], files[i as usize]);
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    return root;
}
#[no_mangle]
pub unsafe extern "C" fn search_index(mut root: trie, mut word: *const libc::c_char) {
    let mut path: [libc::c_char; 1024] = [0; 1024];
    printf(b"Search for \"%s\": \0" as *const u8 as *const libc::c_char, word);
    let mut found: trie = trie_trav(root, word, 1 as libc::c_int);
    if found.is_null() {
        printf(b"not found\n\0" as *const u8 as *const libc::c_char);
    } else {
        trie_all(
            (*found).next[0 as libc::c_int as usize],
            path.as_mut_ptr(),
            0 as libc::c_int,
            Some(print_path as unsafe extern "C" fn(*mut libc::c_char) -> libc::c_int),
        );
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    };
}
unsafe fn main_0() -> libc::c_int {
    let mut root: trie = init_tables();
    search_index(root, b"what\0" as *const u8 as *const libc::c_char);
    search_index(root, b"is\0" as *const u8 as *const libc::c_char);
    search_index(root, b"banana\0" as *const u8 as *const libc::c_char);
    search_index(root, b"boo\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
