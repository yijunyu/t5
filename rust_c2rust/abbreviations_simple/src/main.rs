#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn toupper(_: libc::c_int) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type size_t = libc::c_ulong;
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
pub struct command_tag {
    pub cmd: *mut libc::c_char,
    pub length: size_t,
    pub min_len: size_t,
    pub next: *mut command_tag,
}
pub type command_t = command_tag;
#[no_mangle]
pub static mut command_table: *const libc::c_char = b"add 1  alter 3  backup 2  bottom 1  Cappend 2  change 1  Schange  Cinsert 2  Clast 3 compress 4 copy 2 count 3 Coverlay 3 cursor 3  delete 3 Cdelete 2  down 1  duplicate 3 xEdit 1 expand 3 extract 3  find 1 Nfind 2 Nfindup 6 NfUP 3 Cfind 2 findUP 3 fUP 2 forward 2  get  help 1 hexType 4  input 1 powerInput 3  join 1 split 2 spltJOIN load locate 1 Clocate 2 lowerCase 3 upperCase 3 Lprefix 2  macro  merge 2 modify 3 move 2 msg  next 1 overlay 1 parse preserve 4 purge 3 put putD query 1 quit  read recover 3 refresh renum 3 repeat 3 replace 1 Creplace 2 reset 3 restore 4 rgtLEFT right 2 left 2  save  set  shift 2  si  sort  sos  stack 3 status 4 top  transfer 3  type 1  up 1\0"
    as *const u8 as *const libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn command_match(
    mut command: *const command_t,
    mut str: *const libc::c_char,
) -> bool {
    let mut olen: size_t = strlen(str);
    return olen >= (*command).min_len && olen <= (*command).length
        && strncmp(str, (*command).cmd, olen) == 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uppercase(
    mut str: *mut libc::c_char,
    mut n: size_t,
) -> *mut libc::c_char {
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < n {
        *str
            .offset(
                i as isize,
            ) = toupper(*str.offset(i as isize) as libc::c_uchar as libc::c_int)
            as libc::c_char;
        i = i.wrapping_add(1);
        i;
    }
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn fatal(mut message: *const libc::c_char) {
    fprintf(stderr, b"%s\n\0" as *const u8 as *const libc::c_char, message);
    exit(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn xmalloc(mut n: size_t) -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = malloc(n);
    if ptr.is_null() {
        fatal(b"Out of memory\0" as *const u8 as *const libc::c_char);
    }
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn xrealloc(
    mut p: *mut libc::c_void,
    mut n: size_t,
) -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = realloc(p, n);
    if ptr.is_null() {
        fatal(b"Out of memory\0" as *const u8 as *const libc::c_char);
    }
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn split_into_words(
    mut str: *const libc::c_char,
    mut count: *mut size_t,
) -> *mut *mut libc::c_char {
    let mut size: size_t = 0 as libc::c_int as size_t;
    let mut capacity: size_t = 16 as libc::c_int as size_t;
    let mut words: *mut *mut libc::c_char = xmalloc(
        capacity
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    let mut len: size_t = strlen(str);
    let mut begin: size_t = 0 as libc::c_int as size_t;
    while begin < len {
        let mut i: size_t = begin;
        while i < len
            && *(*__ctype_b_loc())
                .offset(*str.offset(i as isize) as libc::c_uchar as libc::c_int as isize)
                as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                != 0
        {
            i = i.wrapping_add(1);
            i;
        }
        begin = i;
        while i < len
            && *(*__ctype_b_loc())
                .offset(*str.offset(i as isize) as libc::c_uchar as libc::c_int as isize)
                as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                == 0
        {
            i = i.wrapping_add(1);
            i;
        }
        let mut word_len: size_t = i.wrapping_sub(begin);
        if word_len == 0 as libc::c_int as libc::c_ulong {
            break;
        }
        let mut word: *mut libc::c_char = xmalloc(
            word_len.wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        memcpy(
            word as *mut libc::c_void,
            str.offset(begin as isize) as *const libc::c_void,
            word_len,
        );
        *word.offset(word_len as isize) = 0 as libc::c_int as libc::c_char;
        begin = (begin as libc::c_ulong).wrapping_add(word_len) as size_t as size_t;
        if capacity == size {
            capacity = (capacity as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
            words = xrealloc(
                words as *mut libc::c_void,
                capacity
                    .wrapping_mul(
                        ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                    ),
            ) as *mut *mut libc::c_char;
        }
        let fresh0 = size;
        size = size.wrapping_add(1);
        let ref mut fresh1 = *words.offset(fresh0 as isize);
        *fresh1 = word;
    }
    *count = size;
    return words;
}
#[no_mangle]
pub unsafe extern "C" fn make_command_list(
    mut table: *const libc::c_char,
) -> *mut command_t {
    let mut cmd: *mut command_t = 0 as *mut command_t;
    let mut count: size_t = 0 as libc::c_int as size_t;
    let mut words: *mut *mut libc::c_char = split_into_words(table, &mut count);
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < count {
        let mut word: *mut libc::c_char = *words.offset(i as isize);
        let mut new_cmd: *mut command_t = xmalloc(
            ::core::mem::size_of::<command_t>() as libc::c_ulong,
        ) as *mut command_t;
        let mut word_len: size_t = strlen(word);
        (*new_cmd).length = word_len;
        (*new_cmd).min_len = word_len;
        (*new_cmd).cmd = uppercase(word, word_len);
        if i.wrapping_add(1 as libc::c_int as libc::c_ulong) < count {
            let mut eptr: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut min_len: libc::c_ulong = strtoul(
                *words
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize),
                &mut eptr,
                10 as libc::c_int,
            );
            if min_len > 0 as libc::c_int as libc::c_ulong
                && *eptr as libc::c_int == 0 as libc::c_int
            {
                free(
                    *words
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        ) as *mut libc::c_void,
                );
                (*new_cmd).min_len = min_len;
                i = i.wrapping_add(1);
                i;
            }
        }
        (*new_cmd).next = cmd;
        cmd = new_cmd;
        i = i.wrapping_add(1);
        i;
    }
    free(words as *mut libc::c_void);
    return cmd;
}
#[no_mangle]
pub unsafe extern "C" fn free_command_list(mut cmd: *mut command_t) {
    while !cmd.is_null() {
        let mut next: *mut command_t = (*cmd).next;
        free((*cmd).cmd as *mut libc::c_void);
        free(cmd as *mut libc::c_void);
        cmd = next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn find_command(
    mut commands: *const command_t,
    mut word: *const libc::c_char,
) -> *const command_t {
    let mut cmd: *const command_t = commands;
    while !cmd.is_null() {
        if command_match(cmd, word) {
            return cmd;
        }
        cmd = (*cmd).next;
    }
    return 0 as *const command_t;
}
#[no_mangle]
pub unsafe extern "C" fn test(
    mut commands: *const command_t,
    mut input: *const libc::c_char,
) {
    printf(b" input: %s\n\0" as *const u8 as *const libc::c_char, input);
    printf(b"output:\0" as *const u8 as *const libc::c_char);
    let mut count: size_t = 0 as libc::c_int as size_t;
    let mut words: *mut *mut libc::c_char = split_into_words(input, &mut count);
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < count {
        let mut word: *mut libc::c_char = *words.offset(i as isize);
        uppercase(word, strlen(word));
        let mut cmd_ptr: *const command_t = find_command(commands, word);
        printf(
            b" %s\0" as *const u8 as *const libc::c_char,
            if !cmd_ptr.is_null() {
                (*cmd_ptr).cmd as *const libc::c_char
            } else {
                b"*error*\0" as *const u8 as *const libc::c_char
            },
        );
        free(word as *mut libc::c_void);
        i = i.wrapping_add(1);
        i;
    }
    free(words as *mut libc::c_void);
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
unsafe fn main_0() -> libc::c_int {
    let mut commands: *mut command_t = make_command_list(command_table);
    let mut input: *const libc::c_char = b"riG   rePEAT copies  put mo   rest    types   fup.    6       poweRin\0"
        as *const u8 as *const libc::c_char;
    test(commands, input);
    free_command_list(commands);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
