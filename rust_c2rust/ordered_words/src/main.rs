#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types, label_break_value)]
use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
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
pub type TWord = [libc::c_char; 100];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Node {
    pub word: TWord,
    pub next: *mut Node,
}
#[no_mangle]
pub unsafe extern "C" fn is_ordered_word(mut word: *const libc::c_char) -> libc::c_int {
    if !word.is_null() {} else {
        __assert_fail(
            b"word != NULL\0" as *const u8 as *const libc::c_char,
            b"main.c\0" as *const u8 as *const libc::c_char,
            18 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"int is_ordered_word(const char *)\0"))
                .as_ptr(),
        );
    }
    'c_1788: {
        if !word.is_null() {} else {
            __assert_fail(
                b"word != NULL\0" as *const u8 as *const libc::c_char,
                b"main.c\0" as *const u8 as *const libc::c_char,
                18 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"int is_ordered_word(const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while *word.offset(i as isize) as libc::c_int != '\0' as i32 {
        if *word.offset(i as isize) as libc::c_int
            > *word.offset((i + 1 as libc::c_int) as isize) as libc::c_int
            && *word.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                != '\0' as i32
        {
            return 0 as libc::c_int;
        }
        i += 1;
        i;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn list_prepend(
    mut words_list: *mut Node,
    mut new_word: *const libc::c_char,
) -> *mut Node {
    if !new_word.is_null() {} else {
        __assert_fail(
            b"new_word != NULL\0" as *const u8 as *const libc::c_char,
            b"main.c\0" as *const u8 as *const libc::c_char,
            30 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"Node *list_prepend(Node *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_1888: {
        if !new_word.is_null() {} else {
            __assert_fail(
                b"new_word != NULL\0" as *const u8 as *const libc::c_char,
                b"main.c\0" as *const u8 as *const libc::c_char,
                30 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 41],
                    &[libc::c_char; 41],
                >(b"Node *list_prepend(Node *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut new_node: *mut Node = malloc(::core::mem::size_of::<Node>() as libc::c_ulong)
        as *mut Node;
    if new_node.is_null() {
        exit(1 as libc::c_int);
    }
    strcpy(((*new_node).word).as_mut_ptr(), new_word);
    (*new_node).next = words_list;
    return new_node;
}
#[no_mangle]
pub unsafe extern "C" fn list_destroy(mut words_list: *mut Node) -> *mut Node {
    while !words_list.is_null() {
        let mut temp: *mut Node = words_list;
        words_list = (*words_list).next;
        free(temp as *mut libc::c_void);
    }
    return words_list;
}
#[no_mangle]
pub unsafe extern "C" fn list_print(mut words_list: *mut Node) {
    while !words_list.is_null() {
        printf(
            b"\n%s\0" as *const u8 as *const libc::c_char,
            ((*words_list).word).as_mut_ptr(),
        );
        words_list = (*words_list).next;
    }
}
unsafe fn main_0() -> libc::c_int {
    let mut fp: *mut FILE = fopen(
        b"unixdict.txt\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if fp.is_null() {
        return 1 as libc::c_int;
    }
    let mut words: *mut Node = 0 as *mut Node;
    let mut line: TWord = [0; 100];
    let mut max_len: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while fscanf(fp, b"%99s\n\0" as *const u8 as *const libc::c_char, line.as_mut_ptr())
        != -(1 as libc::c_int)
    {
        if strlen(line.as_mut_ptr()) > max_len as libc::c_ulong
            && is_ordered_word(line.as_mut_ptr() as *const libc::c_char) != 0
        {
            max_len = strlen(line.as_mut_ptr()) as libc::c_uint;
            words = list_destroy(words);
            words = list_prepend(words, line.as_mut_ptr() as *const libc::c_char);
        } else if strlen(line.as_mut_ptr()) == max_len as libc::c_ulong
            && is_ordered_word(line.as_mut_ptr() as *const libc::c_char) != 0
        {
            words = list_prepend(words, line.as_mut_ptr() as *const libc::c_char);
        }
    }
    fclose(fp);
    list_print(words);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
