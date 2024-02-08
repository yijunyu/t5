#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sDictWord {
    pub word: *const libc::c_char,
    pub next: DictWord,
}
pub type DictWord = *mut sDictWord;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sHashEntry {
    pub key: *const libc::c_char,
    pub next: HashEntry,
    pub words: DictWord,
    pub link: HashEntry,
    pub wordCount: libc::c_short,
}
pub type HashEntry = *mut sHashEntry;
#[no_mangle]
pub unsafe extern "C" fn sortedWord(
    mut word: *const libc::c_char,
    mut wbuf: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut p1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut endwrd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: libc::c_char = 0;
    let mut swaps: libc::c_int = 0;
    strcpy(wbuf, word);
    endwrd = wbuf.offset(strlen(wbuf) as isize);
    loop {
        swaps = 0 as libc::c_int;
        p1 = wbuf;
        p2 = endwrd.offset(-(1 as libc::c_int as isize));
        while p1 < p2 {
            if *p2 as libc::c_int > *p1 as libc::c_int {
                t = *p2;
                *p2 = *p1;
                *p1 = t;
                swaps = 1 as libc::c_int;
            }
            p1 = p1.offset(1);
            p1;
            p2 = p2.offset(-1);
            p2;
        }
        p1 = wbuf;
        p2 = p1.offset(1 as libc::c_int as isize);
        while p2 < endwrd {
            if *p2 as libc::c_int > *p1 as libc::c_int {
                t = *p2;
                *p2 = *p1;
                *p1 = t;
                swaps = 1 as libc::c_int;
            }
            p1 = p1.offset(1);
            p1;
            p2 = p2.offset(1);
            p2;
        }
        if !(swaps != 0) {
            break;
        }
    }
    return wbuf;
}
static mut cxmap: [libc::c_short; 96] = [
    0x6 as libc::c_int as libc::c_short,
    0x1f as libc::c_int as libc::c_short,
    0x4d as libc::c_int as libc::c_short,
    0xc as libc::c_int as libc::c_short,
    0x5c as libc::c_int as libc::c_short,
    0x28 as libc::c_int as libc::c_short,
    0x5d as libc::c_int as libc::c_short,
    0xe as libc::c_int as libc::c_short,
    0x9 as libc::c_int as libc::c_short,
    0x33 as libc::c_int as libc::c_short,
    0x31 as libc::c_int as libc::c_short,
    0x56 as libc::c_int as libc::c_short,
    0x52 as libc::c_int as libc::c_short,
    0x19 as libc::c_int as libc::c_short,
    0x29 as libc::c_int as libc::c_short,
    0x53 as libc::c_int as libc::c_short,
    0x32 as libc::c_int as libc::c_short,
    0x48 as libc::c_int as libc::c_short,
    0x35 as libc::c_int as libc::c_short,
    0x55 as libc::c_int as libc::c_short,
    0x5e as libc::c_int as libc::c_short,
    0x14 as libc::c_int as libc::c_short,
    0x27 as libc::c_int as libc::c_short,
    0x24 as libc::c_int as libc::c_short,
    0x2 as libc::c_int as libc::c_short,
    0x3e as libc::c_int as libc::c_short,
    0x18 as libc::c_int as libc::c_short,
    0x4a as libc::c_int as libc::c_short,
    0x3f as libc::c_int as libc::c_short,
    0x4c as libc::c_int as libc::c_short,
    0x45 as libc::c_int as libc::c_short,
    0x30 as libc::c_int as libc::c_short,
    0x8 as libc::c_int as libc::c_short,
    0x2c as libc::c_int as libc::c_short,
    0x1a as libc::c_int as libc::c_short,
    0x3 as libc::c_int as libc::c_short,
    0xb as libc::c_int as libc::c_short,
    0xd as libc::c_int as libc::c_short,
    0x4f as libc::c_int as libc::c_short,
    0x7 as libc::c_int as libc::c_short,
    0x20 as libc::c_int as libc::c_short,
    0x1d as libc::c_int as libc::c_short,
    0x51 as libc::c_int as libc::c_short,
    0x3b as libc::c_int as libc::c_short,
    0x11 as libc::c_int as libc::c_short,
    0x58 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0x49 as libc::c_int as libc::c_short,
    0x15 as libc::c_int as libc::c_short,
    0x2d as libc::c_int as libc::c_short,
    0x41 as libc::c_int as libc::c_short,
    0x17 as libc::c_int as libc::c_short,
    0x5f as libc::c_int as libc::c_short,
    0x39 as libc::c_int as libc::c_short,
    0x16 as libc::c_int as libc::c_short,
    0x42 as libc::c_int as libc::c_short,
    0x37 as libc::c_int as libc::c_short,
    0x22 as libc::c_int as libc::c_short,
    0x1c as libc::c_int as libc::c_short,
    0xf as libc::c_int as libc::c_short,
    0x43 as libc::c_int as libc::c_short,
    0x5b as libc::c_int as libc::c_short,
    0x46 as libc::c_int as libc::c_short,
    0x4b as libc::c_int as libc::c_short,
    0xa as libc::c_int as libc::c_short,
    0x26 as libc::c_int as libc::c_short,
    0x2e as libc::c_int as libc::c_short,
    0x40 as libc::c_int as libc::c_short,
    0x12 as libc::c_int as libc::c_short,
    0x21 as libc::c_int as libc::c_short,
    0x3c as libc::c_int as libc::c_short,
    0x36 as libc::c_int as libc::c_short,
    0x38 as libc::c_int as libc::c_short,
    0x1e as libc::c_int as libc::c_short,
    0x1 as libc::c_int as libc::c_short,
    0x1b as libc::c_int as libc::c_short,
    0x5 as libc::c_int as libc::c_short,
    0x4e as libc::c_int as libc::c_short,
    0x44 as libc::c_int as libc::c_short,
    0x3d as libc::c_int as libc::c_short,
    0x4 as libc::c_int as libc::c_short,
    0x10 as libc::c_int as libc::c_short,
    0x5a as libc::c_int as libc::c_short,
    0x2a as libc::c_int as libc::c_short,
    0x23 as libc::c_int as libc::c_short,
    0x34 as libc::c_int as libc::c_short,
    0x25 as libc::c_int as libc::c_short,
    0x2f as libc::c_int as libc::c_short,
    0x2b as libc::c_int as libc::c_short,
    0x50 as libc::c_int as libc::c_short,
    0x3a as libc::c_int as libc::c_short,
    0x54 as libc::c_int as libc::c_short,
    0x47 as libc::c_int as libc::c_short,
    0x59 as libc::c_int as libc::c_short,
    0x13 as libc::c_int as libc::c_short,
    0x57 as libc::c_int as libc::c_short,
];
#[no_mangle]
pub unsafe extern "C" fn Str_Hash(
    mut key: *const libc::c_char,
    mut ix_max: libc::c_int,
) -> libc::c_int {
    let mut cp: *const libc::c_char = 0 as *const libc::c_char;
    let mut mash: libc::c_short = 0;
    let mut hash: libc::c_int = 33501551 as libc::c_int;
    cp = key;
    while *cp != 0 {
        mash = cxmap[(*cp as libc::c_ulong)
            .wrapping_rem(
                (::core::mem::size_of::<[libc::c_short; 96]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                    ),
            ) as usize];
        hash = hash >> 4 as libc::c_int ^ 0x5c5cf5c as libc::c_int
            ^ (hash << 1 as libc::c_int) + ((mash as libc::c_int) << 5 as libc::c_int);
        hash &= 0x3fffffff as libc::c_int;
        cp = cp.offset(1);
        cp;
    }
    return hash % ix_max;
}
#[no_mangle]
pub static mut hashTable: [HashEntry; 8192] = [0 as *const sHashEntry
    as *mut sHashEntry; 8192];
#[no_mangle]
pub static mut mostPerms: HashEntry = 0 as *const sHashEntry as HashEntry;
#[no_mangle]
pub unsafe extern "C" fn buildAnagrams(mut fin: *mut FILE) -> libc::c_int {
    let mut buffer: [libc::c_char; 40] = [0; 40];
    let mut bufr2: [libc::c_char; 40] = [0; 40];
    let mut hkey: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut hix: libc::c_int = 0;
    let mut he: HashEntry = 0 as *mut sHashEntry;
    let mut hep: *mut HashEntry = 0 as *mut HashEntry;
    let mut we: DictWord = 0 as *mut sDictWord;
    let mut maxPC: libc::c_int = 2 as libc::c_int;
    let mut numWords: libc::c_int = 0 as libc::c_int;
    while !(fgets(buffer.as_mut_ptr(), 40 as libc::c_int, fin)).is_null() {
        hkey = buffer.as_mut_ptr();
        while *hkey as libc::c_int != 0 && *hkey as libc::c_int != '\n' as i32 {
            hkey = hkey.offset(1);
            hkey;
        }
        *hkey = 0 as libc::c_int as libc::c_char;
        hkey = sortedWord(buffer.as_mut_ptr(), bufr2.as_mut_ptr());
        hix = Str_Hash(hkey, 8192 as libc::c_int);
        he = hashTable[hix as usize];
        hep = &mut *hashTable.as_mut_ptr().offset(hix as isize) as *mut HashEntry;
        while !he.is_null() && strcmp((*he).key, hkey) != 0 {
            hep = &mut (*he).next;
            he = (*he).next;
        }
        if he.is_null() {
            he = malloc(::core::mem::size_of::<sHashEntry>() as libc::c_ulong)
                as HashEntry;
            (*he).next = 0 as HashEntry;
            (*he).key = strdup(hkey);
            (*he).wordCount = 0 as libc::c_int as libc::c_short;
            (*he).words = 0 as DictWord;
            (*he).link = 0 as HashEntry;
            *hep = he;
        }
        we = malloc(::core::mem::size_of::<sDictWord>() as libc::c_ulong) as DictWord;
        (*we).word = strdup(buffer.as_mut_ptr());
        (*we).next = (*he).words;
        (*he).words = we;
        (*he).wordCount += 1;
        (*he).wordCount;
        if maxPC < (*he).wordCount as libc::c_int {
            maxPC = (*he).wordCount as libc::c_int;
            mostPerms = he;
            (*he).link = 0 as HashEntry;
        } else if maxPC == (*he).wordCount as libc::c_int {
            (*he).link = mostPerms;
            mostPerms = he;
        }
        numWords += 1;
        numWords;
    }
    printf(
        b"%d words in dictionary max ana=%d\n\0" as *const u8 as *const libc::c_char,
        numWords,
        maxPC,
    );
    return maxPC;
}
unsafe fn main_0() -> libc::c_int {
    let mut he: HashEntry = 0 as *mut sHashEntry;
    let mut we: DictWord = 0 as *mut sDictWord;
    let mut f1: *mut FILE = 0 as *mut FILE;
    f1 = fopen(
        b"unixdict.txt\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    buildAnagrams(f1);
    fclose(f1);
    f1 = fopen(
        b"anaout.txt\0" as *const u8 as *const libc::c_char,
        b"w\0" as *const u8 as *const libc::c_char,
    );
    he = mostPerms;
    while !he.is_null() {
        fprintf(
            f1,
            b"%d:\0" as *const u8 as *const libc::c_char,
            (*he).wordCount as libc::c_int,
        );
        we = (*he).words;
        while !we.is_null() {
            fprintf(f1, b"%s, \0" as *const u8 as *const libc::c_char, (*we).word);
            we = (*we).next;
        }
        fprintf(f1, b"\n\0" as *const u8 as *const libc::c_char);
        he = (*he).link;
    }
    fclose(f1);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
