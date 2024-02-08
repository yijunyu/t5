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
use c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    fn fputc(__c: i32, __stream: *mut FILE) -> i32;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_t {
    pub get: Option<unsafe extern "C" fn(stream) -> i32>,
    pub put: Option<unsafe extern "C" fn(stream, i32) -> i32>,
}
pub type stream = *mut stream_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct string_stream {
    pub get: Option<unsafe extern "C" fn(stream) -> i32>,
    pub put: Option<unsafe extern "C" fn(stream, i32) -> i32>,
    pub string: *mut i8,
    pub pos: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_stream {
    pub get: Option<unsafe extern "C" fn(stream) -> i32>,
    pub put: Option<unsafe extern "C" fn(stream, i32) -> i32>,
    pub fp: *mut FILE,
}
#[no_mangle]
pub extern "C" fn sget(mut in_0: stream) -> i32 {
    unsafe {
        let mut c: i32 = 0;
        let mut s: *mut string_stream = in_0 as *mut string_stream;
        c = *((*s).string).offset((*s).pos as isize) as i32;
        if c == '\0' as i32 {
            return -1;
        };
        (*s).pos += 1;
        (*s).pos;
        return c;
    }
}

#[no_mangle]
pub extern "C" fn sput(mut out: stream, mut c: i32) -> i32 {
    unsafe {
        let mut s: *mut string_stream = out as *mut string_stream;
        let fresh0 = (*s).pos;
        (*s).pos = (*s).pos + 1;
        *((*s).string).offset(fresh0 as isize) = (if c == -1 { '\0' as i32 } else { c }) as i8;
        if c == -1 {
            (*s).pos = 0;
        }
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn file_put(mut out: stream, mut c: i32) -> i32 {
    unsafe {
        let mut f: *mut file_stream = out as *mut file_stream;
        return fputc(c, (*f).fp);
    }
}

#[no_mangle]
pub extern "C" fn output(mut out: stream, mut buf: *mut u8, mut len: i32) {
    unsafe {
        let mut i: i32 = 0;
        ((*out).put).expect("non-null function pointer")(out, 128 + len);
        i = 0;
        while i < len {
            ((*out).put).expect("non-null function pointer")(out, *buf.offset(i as isize) as i32);
            i += 1;
            i;
        }
    }
}

#[no_mangle]
pub extern "C" fn encode(mut in_0: stream, mut out: stream) {
    let mut buf: [u8; 256] = [0; 256];
    let mut len: i32 = 0;
    let mut repeat: i32 = 0;
    let mut end: i32 = 0;
    let mut c: i32 = 0;
    unsafe {
        let mut get: Option<unsafe extern "C" fn(stream) -> i32> = (*in_0).get;
        let mut put: Option<unsafe extern "C" fn(stream, i32) -> i32> = (*out).put;
        while end == 0 {
            c = get.expect("non-null function pointer")(in_0);
            end = (c == -1) as i32;
            if end == 0 {
                let fresh1 = len;
                len = len + 1;
                buf[fresh1 as usize] = c as u8;
                if len <= 1 {
                    continue;
                }
            }
            if repeat != 0 {
                if buf[(len - 1i32) as usize] as i32 != buf[(len - 2i32) as usize] as i32 {
                    repeat = 0;
                }
                if repeat == 0 || len == 129 || end != 0 {
                    put.expect("non-null function pointer")(
                        out,
                        if end != 0 { len } else { len - 1 },
                    );
                    put.expect("non-null function pointer")(out, buf[0 as usize] as i32);
                    buf[0 as usize] = buf[(len - 1i32) as usize];
                    len = 1;
                }
            } else if buf[(len - 1i32) as usize] as i32 == buf[(len - 2i32) as usize] as i32 {
                repeat = 1;
                if len > 2 {
                    output(out, buf.as_mut_ptr(), len - 2);
                    buf[1 as usize] = buf[(len - 1i32) as usize];
                    buf[0 as usize] = buf[1 as usize];
                    len = 2;
                }
            } else if len == 128 || end != 0 {
                output(out, buf.as_mut_ptr(), len);
                len = 0;
                repeat = 0;
            }
        }
        put.expect("non-null function pointer")(out, -1);
    }
}

#[no_mangle]
pub extern "C" fn decode(mut in_0: stream, mut out: stream) {
    let mut c: i32 = 0;
    let mut i: i32 = 0;
    let mut cnt: i32 = 0;
    loop {
        c = ((*in_0).get).expect("non-null function pointer")(in_0);
        if c == -1 {
            return;
        }
        if c > 128 {
            cnt = c - 128;
            i = 0;
            while i < cnt {
                ((*out).put).expect("non-null function pointer")(
                    out,
                    ((*in_0).get).expect("non-null function pointer")(in_0),
                );
                i += 1;
                i;
            }
        } else {
            cnt = c;
            c = ((*in_0).get).expect("non-null function pointer")(in_0);
            i = 0;
            while i < cnt {
                ((*out).put).expect("non-null function pointer")(out, c);
                i += 1;
                i;
            }
        }
    }
}

fn main_0() -> i32 {
    let mut buf: [i8; 256] = [0; 256];
    unsafe {
        let mut str_in: string_stream = {
            let mut init = string_stream {
                get: Some(sget as unsafe extern "C" fn(stream) -> i32),
                put: None,
                string: b"WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWBWWWWWWWWWWWWWW\0"
                    as *const u8 as *const i8 as *mut i8,
                pos: 0,
            };
            init
        };
        let mut str_out: string_stream = {
            let mut init = string_stream {
                get: Some(sget as unsafe extern "C" fn(stream) -> i32),
                put: Some(sput as unsafe extern "C" fn(stream, i32) -> i32),
                string: buf.as_mut_ptr(),
                pos: 0,
            };
            init
        };
        let mut file: file_stream = {
            let mut init = file_stream {
                get: None,
                put: Some(file_put as unsafe extern "C" fn(stream, i32) -> i32),
                fp: stdout,
            };
            init
        };
        encode(
            &mut str_in as *mut string_stream as stream,
            &mut str_out as *mut string_stream as stream,
        );
        decode(
            &mut str_out as *mut string_stream as stream,
            &mut file as *mut file_stream as stream,
        );
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
