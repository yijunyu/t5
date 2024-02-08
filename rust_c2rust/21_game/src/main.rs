#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn putchar(__c: libc::c_int) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn rand() -> libc::c_int;
    fn srand(__seed: libc::c_uint);
    fn exit(_: libc::c_int) -> !;
    fn time(__timer: *mut time_t) -> time_t;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
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
pub type time_t = __time_t;
static mut DESCRIPTION: [libc::c_char; 398] = unsafe {
    *::core::mem::transmute::<
        &[u8; 398],
        &mut [libc::c_char; 398],
    >(
        b"21 Game                                                          \n                                                                 \n21 is a two player game, the game is played by choosing a number \n(1, 2, or 3) to be added to the running total. The game is won by\nthe player whose chosen number causes the running total to reach \nexactly 21. The running total starts at zero.                    \n\n\0",
    )
};
static mut total: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn update(mut player: *mut libc::c_char, mut move_0: libc::c_int) {
    printf(
        b"%8s:  %d = %d + %d\n\n\0" as *const u8 as *const libc::c_char,
        player,
        total + move_0,
        total,
        move_0,
    );
    total += move_0;
    if total == 21 as libc::c_int {
        printf(b"The winner is %s.\n\n\0" as *const u8 as *const libc::c_char, player);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ai() -> libc::c_int {
    static mut precomputed: [libc::c_int; 31] = [
        1 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
        2 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
        2 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
        2 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
        2 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
        2 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
        2 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
        2 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
    ];
    update(
        b"ai\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        precomputed[total as usize],
    );
    panic!("Reached end of non-void function without returning");
}
#[no_mangle]
pub unsafe extern "C" fn human() {
    let mut buffer: [libc::c_char; 256] = [0; 256];
    let mut move_0: libc::c_int = 0;
    loop {
        printf(
            b"enter your move to play (or enter 0 to exit game): \0" as *const u8
                as *const libc::c_char,
        );
        fgets(buffer.as_mut_ptr(), 256 as libc::c_int, stdin);
        if !(sscanf(
            buffer.as_mut_ptr(),
            b"%d\0" as *const u8 as *const libc::c_char,
            &mut move_0 as *mut libc::c_int,
        ) != 1 as libc::c_int
            || move_0 != 0
                && (move_0 < 1 as libc::c_int || move_0 > 3 as libc::c_int
                    || total + move_0 > 21 as libc::c_int))
        {
            break;
        }
        puts(
            b"\nYour answer is not a valid choice.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    putchar('\n' as i32);
    if move_0 == 0 {
        exit(0 as libc::c_int);
    }
    update(b"human\0" as *const u8 as *const libc::c_char as *mut libc::c_char, move_0);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    srand(time(0 as *mut time_t) as libc::c_uint);
    puts(DESCRIPTION.as_mut_ptr());
    loop {
        puts(b"\n---- NEW GAME ----\n\0" as *const u8 as *const libc::c_char);
        puts(
            b"\nThe running total is currently zero.\n\0" as *const u8
                as *const libc::c_char,
        );
        total = 0 as libc::c_int;
        if rand() % 2 as libc::c_int != 0 {
            puts(b"The first move is AI move.\n\0" as *const u8 as *const libc::c_char);
            ai();
        } else {
            puts(
                b"The first move is human move.\n\0" as *const u8 as *const libc::c_char,
            );
        }
        while total < 21 as libc::c_int {
            human();
            ai();
        }
    };
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
