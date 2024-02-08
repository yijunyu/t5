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

use std::time::SystemTime;
pub fn rust_time(ref_result: Option<&mut i64>) -> i64 {
    let result = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    };
    match ref_result {
        Some(r) => *r = result,
        None => {}
    }
    return result as i64;
}

use c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    fn sscanf(_: *const i8, _: *const i8, _: ...) -> i32;
    fn fgets(__s: *mut i8, __n: i32, __stream: *mut FILE) -> *mut i8;
    fn puts(__s: *const i8) -> i32;
    fn rand() -> i32;
    fn srand(__seed: u32);
    fn exit(_: i32) -> !;
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
static mut DESCRIPTION: [i8; 398] = unsafe {
    * :: core :: mem :: transmute :: < & [u8; 398], & mut [i8; 398], > (
b"21 Game                                                          \n                                                                 \n21 is a two player game, the game is played by choosing a number \n(1, 2, or 3) to be added to the running total. The game is won by\nthe player whose chosen number causes the running total to reach \nexactly 21. The running total starts at zero.                    \n\n\0"
    ,)
};
static mut total: i32 = 0;
#[no_mangle]
pub extern "C" fn update(mut player: *mut i8, mut move_0: i32) {
    unsafe {
        print!(
            "{:8}:  {} = {} + {}\n\n",
            build_str_from_raw_ptr(player as *mut u8),
            total + move_0,
            total,
            move_0
        );
        total += move_0;
        if total == 21 {
            print!(
                "The winner is {}.\n\n",
                build_str_from_raw_ptr(player as *mut u8)
            );
        }
    }
}

#[no_mangle]
pub extern "C" fn ai() -> i32 {
    unsafe {
        static mut precomputed: [i32; 31] = [
            1, 1, 3, 2, 1, 1, 3, 2, 1, 1, 3, 2, 1, 1, 3, 2, 1, 1, 3, 2, 1, 1, 3, 2, 1, 1, 3, 2, 1,
            1, 3,
        ];
        update(
            b"ai\0" as *const u8 as *const i8 as *mut i8,
            precomputed[total as usize],
        );
    }
    panic!("Reached end of non-void function without returning");
}

#[no_mangle]
pub extern "C" fn human() {
    let mut buffer: [i8; 256] = [0; 256];
    let mut move_0: i32 = 0;
    unsafe {
        loop {
            print!("enter your move to play (or enter 0 to exit game): ");
            fgets(buffer.as_mut_ptr(), 256, stdin);
            if !(sscanf(
                buffer.as_mut_ptr(),
                b"%d\0" as *const u8 as *const i8,
                &mut move_0 as *mut i32,
            ) != 1
                || move_0 != 0 && (move_0 < 1 || move_0 > 3 || total + move_0 > 21))
            {
                break;
            }
            puts(b"\nYour answer is not a valid choice.\n\0" as *const u8 as *const i8);
        }
    }
    print!("{}", '\n' as i32);
    unsafe {
        if move_0 == 0 {
            exit(0);
        }
    }
    update(b"human\0" as *const u8 as *const i8 as *mut i8, move_0);
}

fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    unsafe {
        srand(rust_time(None) as u32);
        puts(DESCRIPTION.as_mut_ptr());
        loop {
            puts(b"\n---- NEW GAME ----\n\0" as *const u8 as *const i8);
            puts(b"\nThe running total is currently zero.\n\0" as *const u8 as *const i8);
            total = 0;
            if rand() % 2 != 0 {
                puts(b"The first move is AI move.\n\0" as *const u8 as *const i8);
                ai();
            } else {
                puts(b"The first move is human move.\n\0" as *const u8 as *const i8);
            }
            while total < 21 {
                human();
                ai();
            }
        }
    }
}

pub fn main() {
    let mut args: Vec<*mut i8> = Vec::new();
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
            main_0((args.len() - 1) as i32, args.as_mut_ptr() as *mut *mut i8) as i32,
        );
    }
}
