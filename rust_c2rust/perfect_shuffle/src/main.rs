#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
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
#[no_mangle]
pub static mut kDecks: [libc::c_int; 7] = [
    8 as libc::c_int,
    24 as libc::c_int,
    52 as libc::c_int,
    100 as libc::c_int,
    1020 as libc::c_int,
    1024 as libc::c_int,
    10000 as libc::c_int,
];
unsafe fn main_0() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut nCards: libc::c_int = 0;
    let mut nShuffles: libc::c_int = 0;
    let mut deck: *mut libc::c_int = 0 as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < 7 as libc::c_int {
        nCards = kDecks[i as usize];
        if CreateDeck(&mut deck, nCards) == 0 {
            fprintf(
                stderr,
                b"Error: malloc() failed!\n\0" as *const u8 as *const libc::c_char,
            );
            return 1 as libc::c_int;
        }
        InitDeck(deck, nCards);
        nShuffles = 0 as libc::c_int;
        loop {
            ShuffleDeck(deck, nCards);
            nShuffles += 1;
            nShuffles;
            if !(InitedDeck(deck, nCards) == 0) {
                break;
            }
        }
        printf(
            b"Cards count: %d, shuffles required: %d.\n\0" as *const u8
                as *const libc::c_char,
            nCards,
            nShuffles,
        );
        FreeDeck(&mut deck);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CreateDeck(
    mut deck: *mut *mut libc::c_int,
    mut nCards: libc::c_int,
) -> libc::c_int {
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    if !deck.is_null() {
        tmp = malloc(
            (nCards as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
    }
    return if !tmp.is_null() {
        *deck = tmp;
        (*deck != 0 as *mut libc::c_void as *mut libc::c_int) as libc::c_int
    } else {
        0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn InitDeck(mut deck: *mut libc::c_int, mut nCards: libc::c_int) {
    if !deck.is_null() {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < nCards {
            *deck.offset(i as isize) = i;
            i += 1;
            i;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn DuplicateDeck(
    mut dest: *mut *mut libc::c_int,
    mut orig: *const libc::c_int,
    mut nCards: libc::c_int,
) -> libc::c_int {
    if !orig.is_null() && CreateDeck(dest, nCards) != 0 {
        memcpy(
            *dest as *mut libc::c_void,
            orig as *const libc::c_void,
            (nCards as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn InitedDeck(
    mut deck: *mut libc::c_int,
    mut nCards: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < nCards {
        if *deck.offset(i as isize) != i {
            return 0 as libc::c_int;
        }
        i += 1;
        i;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ShuffleDeck(
    mut deck: *mut libc::c_int,
    mut nCards: libc::c_int,
) -> libc::c_int {
    let mut copy: *mut libc::c_int = 0 as *mut libc::c_int;
    if DuplicateDeck(&mut copy, deck, nCards) != 0 {
        let mut i: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        j = 0 as libc::c_int;
        i = j;
        while i < nCards / 2 as libc::c_int {
            *deck.offset(j as isize) = *copy.offset(i as isize);
            *deck
                .offset(
                    (j + 1 as libc::c_int) as isize,
                ) = *copy.offset((i + nCards / 2 as libc::c_int) as isize);
            i += 1;
            i;
            j += 2 as libc::c_int;
        }
        FreeDeck(&mut copy);
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn FreeDeck(mut deck: *mut *mut libc::c_int) {
    if !(*deck).is_null() {
        free(*deck as *mut libc::c_void);
        *deck = 0 as *mut libc::c_int;
    }
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
