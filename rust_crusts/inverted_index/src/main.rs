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

use c2rust_out::*;
extern "C" {
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct trie_t {
    pub next: [trie; 41],
    pub eow: i32,
}
pub type trie = *mut trie_t;
#[no_mangle]
pub static mut chr_legal: [i8; 41] = unsafe {
    *::core::mem::transmute::<&[u8; 41], &mut [i8; 41]>(
        b"abcdefghijklmnopqrstuvwxyz0123456789_-./\0",
    )
};
#[no_mangle]
pub static mut chr_idx: [i32; 256] = [0; 256];
#[no_mangle]
pub static mut idx_chr: [i8; 256] = [0; 256];
#[no_mangle]
pub extern "C" fn trie_new() -> trie {
    unsafe {
        return calloc(::core::mem::size_of::<trie_t>() as u64, 1) as trie;
    }
}

#[no_mangle]
pub extern "C" fn trie_trav(mut root: trie, mut str: *const i8, mut no_create: i32) -> trie {
    unsafe {
        let mut c: i32 = 0;
        while !root.is_null() {
            c = *str.offset(0 as isize) as i32;
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
                        };
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
}

#[no_mangle]
pub extern "C" fn trie_all(
    mut root: trie,
    mut path: *mut i8,
    mut depth: i32,
    mut callback: Option<unsafe extern "C" fn(*mut i8) -> i32>,
) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        if (*root).eow != 0 && callback.expect("non-null function pointer")(path) == 0 {
            return 0;
        }
        i = 1;
        while (i as u64) < ::core::mem::size_of::<[i8; 41]>() as u64 {
            if !((*root).next[i as usize]).is_null() {
                *path.offset(depth as isize) = idx_chr[i as usize];
                *path.offset((depth + 1i32) as isize) = '\0' as i8;
                if trie_all((*root).next[i as usize], path, depth + 1, callback) == 0 {
                    return 0;
                }
            }
            i += 1;
            i;
        }
        return 1;
    }
}

#[no_mangle]
pub extern "C" fn add_index(mut root: trie, mut word: *const i8, mut fname: *const i8) {
    unsafe {
        let mut x: trie = trie_trav(root, word, 0);
        (*x).eow = 1;
        if ((*x).next[0 as usize]).is_null() {
            (*x).next[0 as usize] = trie_new();
        }
        x = trie_trav((*x).next[0 as usize], fname, 0);
        (*x).eow = 1;
    }
}

#[no_mangle]
pub extern "C" fn print_path(mut path: *mut i8) -> i32 {
    unsafe {
        print!(" {}", build_str_from_raw_ptr(path as *mut u8));
        return 1;
    }
}

#[no_mangle]
pub static mut files: [*const i8; 3] = [
    b"f1.txt\0" as *const u8 as *const i8,
    b"source/f2.txt\0" as *const u8 as *const i8,
    b"other_file\0" as *const u8 as *const i8,
];
#[no_mangle]
pub static mut text: [[*const i8; 5]; 3] = [
    [
        b"it\0" as *const u8 as *const i8,
        b"is\0" as *const u8 as *const i8,
        b"what\0" as *const u8 as *const i8,
        b"it\0" as *const u8 as *const i8,
        b"is\0" as *const u8 as *const i8,
    ],
    [
        b"what\0" as *const u8 as *const i8,
        b"is\0" as *const u8 as *const i8,
        b"it\0" as *const u8 as *const i8,
        0 as *const i8,
        0 as *const i8,
    ],
    [
        b"it\0" as *const u8 as *const i8,
        b"is\0" as *const u8 as *const i8,
        b"a\0" as *const u8 as *const i8,
        b"banana\0" as *const u8 as *const i8,
        0 as *const i8,
    ],
];
#[no_mangle]
pub extern "C" fn init_tables() -> trie {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut root: trie = trie_new();
    i = 0;
    unsafe {
        while (i as u64) < ::core::mem::size_of::<[i8; 41]>() as u64 {
            chr_idx[chr_legal[i as usize] as i32 as usize] = i + 1;
            idx_chr[(i + 1i32) as usize] = chr_legal[i as usize];
            i += 1;
            i;
        }
    }
    i = 0;
    unsafe {
        while i < 3 {
            j = 0;
            while j < 5 {
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
    }
    return root;
}

#[no_mangle]
pub extern "C" fn search_index(mut root: trie, mut word: *const i8) {
    unsafe {
        let mut path: [i8; 1024] = [0; 1024];
        print!(
            "Search for \"{}\": ",
            build_str_from_raw_ptr(word as *mut u8)
        );
        let mut found: trie = trie_trav(root, word, 1);
        if found.is_null() {
            print!("not found\n");
        } else {
            trie_all(
                (*found).next[0 as usize],
                path.as_mut_ptr(),
                0,
                Some(print_path as unsafe extern "C" fn(*mut i8) -> i32),
            );
            print!("\n");
        };
    }
}

fn main_0() -> i32 {
    let mut root: trie = init_tables();
    search_index(root, b"what\0" as *const u8 as *const i8);
    search_index(root, b"is\0" as *const u8 as *const i8);
    search_index(root, b"banana\0" as *const u8 as *const i8);
    search_index(root, b"boo\0" as *const u8 as *const i8);
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
