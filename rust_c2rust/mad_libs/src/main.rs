#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
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
pub struct dstr {
    pub data: *mut libc::c_char,
    pub alloc: size_t,
    pub length: size_t,
}
#[no_mangle]
pub unsafe extern "C" fn dstr_space(
    mut s: *mut dstr,
    mut grow_amount: size_t,
) -> libc::c_int {
    return (((*s).length).wrapping_add(grow_amount) < (*s).alloc) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dstr_grow(mut s: *mut dstr) -> libc::c_int {
    (*s)
        .alloc = ((*s).alloc as libc::c_ulong)
        .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
    let mut attempt: *mut libc::c_char = realloc(
        (*s).data as *mut libc::c_void,
        (*s).alloc,
    ) as *mut libc::c_char;
    if attempt.is_null() {
        return 0 as libc::c_int
    } else {
        (*s).data = attempt;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dstr_init(to_allocate: size_t) -> *mut dstr {
    let mut s: *mut dstr = malloc(::core::mem::size_of::<dstr>() as libc::c_ulong)
        as *mut dstr;
    if !s.is_null() {
        (*s).length = 0 as libc::c_int as size_t;
        (*s).alloc = to_allocate;
        (*s).data = malloc((*s).alloc) as *mut libc::c_char;
        if !((*s).data).is_null() {
            return s;
        }
    }
    if !((*s).data).is_null() {
        free((*s).data as *mut libc::c_void);
    }
    if !s.is_null() {
        free(s as *mut libc::c_void);
    }
    return 0 as *mut dstr;
}
#[no_mangle]
pub unsafe extern "C" fn dstr_delete(mut s: *mut dstr) {
    if !((*s).data).is_null() {
        free((*s).data as *mut libc::c_void);
    }
    if !s.is_null() {
        free(s as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn readinput(mut fd: *mut FILE) -> *mut dstr {
    let mut current_block: u64;
    static mut buffer_size: size_t = 4096 as libc::c_int as size_t;
    let vla = buffer_size as usize;
    let mut buffer: Vec::<libc::c_char> = ::std::vec::from_elem(0, vla);
    let mut s: *mut dstr = dstr_init(buffer_size);
    if s.is_null() {
        current_block = 17156671859070965445;
    } else {
        current_block = 8258075665625361029;
    }
    '_failure: loop {
        match current_block {
            17156671859070965445 => {
                dstr_delete(s);
                return 0 as *mut dstr;
            }
            _ => {
                if !(fgets(buffer.as_mut_ptr(), buffer_size as libc::c_int, fd))
                    .is_null()
                {
                    while dstr_space(s, buffer_size) == 0 {
                        if dstr_grow(s) == 0 {
                            current_block = 17156671859070965445;
                            continue '_failure;
                        }
                    }
                    strncpy(
                        ((*s).data).offset((*s).length as isize),
                        buffer.as_mut_ptr(),
                        buffer_size,
                    );
                    (*s)
                        .length = ((*s).length as libc::c_ulong)
                        .wrapping_add(strlen(buffer.as_mut_ptr())) as size_t as size_t;
                    current_block = 8258075665625361029;
                } else {
                    return s
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn dstr_replace_all(
    mut story: *mut dstr,
    mut replace: *const libc::c_char,
    mut insert: *const libc::c_char,
) {
    let replace_l: size_t = strlen(replace);
    let insert_l: size_t = strlen(insert);
    let mut start: *mut libc::c_char = (*story).data;
    loop {
        start = strstr(start, replace);
        if start.is_null() {
            break;
        }
        if dstr_space(story, insert_l.wrapping_sub(replace_l)) == 0 {
            if dstr_grow(story) == 0 {
                fprintf(
                    stderr,
                    b"Failed to allocate memory\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
        }
        if insert_l != replace_l {
            memmove(
                start.offset(insert_l as isize) as *mut libc::c_void,
                start.offset(replace_l as isize) as *const libc::c_void,
                ((*story).length)
                    .wrapping_sub(
                        start.offset(replace_l as isize).offset_from((*story).data)
                            as libc::c_long as libc::c_ulong,
                    ),
            );
            (*story)
                .length = ((*story).length as libc::c_ulong)
                .wrapping_add(insert_l.wrapping_sub(replace_l)) as size_t as size_t;
            *((*story).data)
                .offset((*story).length as isize) = 0 as libc::c_int as libc::c_char;
        }
        memmove(start as *mut libc::c_void, insert as *const libc::c_void, insert_l);
    };
}
#[no_mangle]
pub unsafe extern "C" fn madlibs(mut story: *mut dstr) {
    static mut buffer_size: size_t = 128 as libc::c_int as size_t;
    let vla = buffer_size as usize;
    let mut insert: Vec::<libc::c_char> = ::std::vec::from_elem(0, vla);
    let vla_0 = buffer_size as usize;
    let mut replace: Vec::<libc::c_char> = ::std::vec::from_elem(0, vla_0);
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = (*story).data;
    loop {
        start = strchr(end, '<' as i32);
        if start.is_null() {
            break;
        }
        end = strchr(start, '>' as i32);
        if end.is_null() {
            fprintf(
                stderr,
                b"Malformed brackets in input\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        strncpy(
            replace.as_mut_ptr(),
            start,
            (end.offset_from(start) as libc::c_long + 1 as libc::c_int as libc::c_long)
                as libc::c_ulong,
        );
        *replace
            .as_mut_ptr()
            .offset(
                (end.offset_from(start) as libc::c_long
                    + 1 as libc::c_int as libc::c_long) as isize,
            ) = '\0' as i32 as libc::c_char;
        printf(
            b"Enter value for field %s: \0" as *const u8 as *const libc::c_char,
            replace.as_mut_ptr(),
        );
        fgets(insert.as_mut_ptr(), buffer_size as libc::c_int, stdin);
        let il: size_t = (strlen(insert.as_mut_ptr()))
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        if *insert.as_mut_ptr().offset(il as isize) as libc::c_int == '\n' as i32 {
            *insert.as_mut_ptr().offset(il as isize) = '\0' as i32 as libc::c_char;
        }
        dstr_replace_all(story, replace.as_mut_ptr(), insert.as_mut_ptr());
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    if argc < 2 as libc::c_int {
        return 0 as libc::c_int;
    }
    let mut fd: *mut FILE = fopen(
        *argv.offset(1 as libc::c_int as isize),
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if fd.is_null() {
        fprintf(
            stderr,
            b"Could not open file: '%s\n\0" as *const u8 as *const libc::c_char,
            *argv.offset(1 as libc::c_int as isize),
        );
        exit(1 as libc::c_int);
    }
    let mut story: *mut dstr = readinput(fd);
    fclose(fd);
    if story.is_null() {
        fprintf(
            stderr,
            b"Failed to allocate memory\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    madlibs(story);
    printf(b"%s\n\0" as *const u8 as *const libc::c_char, (*story).data);
    dstr_delete(story);
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
