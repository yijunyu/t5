#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
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
pub struct stream_t {
    pub get: Option::<unsafe extern "C" fn(stream) -> libc::c_int>,
    pub put: Option::<unsafe extern "C" fn(stream, libc::c_int) -> libc::c_int>,
}
pub type stream = *mut stream_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct string_stream {
    pub get: Option::<unsafe extern "C" fn(stream) -> libc::c_int>,
    pub put: Option::<unsafe extern "C" fn(stream, libc::c_int) -> libc::c_int>,
    pub string: *mut libc::c_char,
    pub pos: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_stream {
    pub get: Option::<unsafe extern "C" fn(stream) -> libc::c_int>,
    pub put: Option::<unsafe extern "C" fn(stream, libc::c_int) -> libc::c_int>,
    pub fp: *mut FILE,
}
#[no_mangle]
pub unsafe extern "C" fn sget(mut in_0: stream) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut s: *mut string_stream = in_0 as *mut string_stream;
    c = *((*s).string).offset((*s).pos as isize) as libc::c_uchar as libc::c_int;
    if c == '\0' as i32 {
        return -(1 as libc::c_int);
    }
    (*s).pos += 1;
    (*s).pos;
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn sput(mut out: stream, mut c: libc::c_int) -> libc::c_int {
    let mut s: *mut string_stream = out as *mut string_stream;
    let fresh0 = (*s).pos;
    (*s).pos = (*s).pos + 1;
    *((*s).string)
        .offset(
            fresh0 as isize,
        ) = (if c == -(1 as libc::c_int) { '\0' as i32 } else { c }) as libc::c_char;
    if c == -(1 as libc::c_int) {
        (*s).pos = 0 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn file_put(mut out: stream, mut c: libc::c_int) -> libc::c_int {
    let mut f: *mut file_stream = out as *mut file_stream;
    return fputc(c, (*f).fp);
}
#[no_mangle]
pub unsafe extern "C" fn output(
    mut out: stream,
    mut buf: *mut libc::c_uchar,
    mut len: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    ((*out).put).expect("non-null function pointer")(out, 128 as libc::c_int + len);
    i = 0 as libc::c_int;
    while i < len {
        ((*out).put)
            .expect(
                "non-null function pointer",
            )(out, *buf.offset(i as isize) as libc::c_int);
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn encode(mut in_0: stream, mut out: stream) {
    let mut buf: [libc::c_uchar; 256] = [0; 256];
    let mut len: libc::c_int = 0 as libc::c_int;
    let mut repeat: libc::c_int = 0 as libc::c_int;
    let mut end: libc::c_int = 0 as libc::c_int;
    let mut c: libc::c_int = 0;
    let mut get: Option::<unsafe extern "C" fn(stream) -> libc::c_int> = (*in_0).get;
    let mut put: Option::<unsafe extern "C" fn(stream, libc::c_int) -> libc::c_int> = (*out)
        .put;
    while end == 0 {
        c = get.expect("non-null function pointer")(in_0);
        end = (c == -(1 as libc::c_int)) as libc::c_int;
        if end == 0 {
            let fresh1 = len;
            len = len + 1;
            buf[fresh1 as usize] = c as libc::c_uchar;
            if len <= 1 as libc::c_int {
                continue;
            }
        }
        if repeat != 0 {
            if buf[(len - 1 as libc::c_int) as usize] as libc::c_int
                != buf[(len - 2 as libc::c_int) as usize] as libc::c_int
            {
                repeat = 0 as libc::c_int;
            }
            if repeat == 0 || len == 129 as libc::c_int || end != 0 {
                put
                    .expect(
                        "non-null function pointer",
                    )(out, if end != 0 { len } else { len - 1 as libc::c_int });
                put
                    .expect(
                        "non-null function pointer",
                    )(out, buf[0 as libc::c_int as usize] as libc::c_int);
                buf[0 as libc::c_int as usize] = buf[(len - 1 as libc::c_int) as usize];
                len = 1 as libc::c_int;
            }
        } else if buf[(len - 1 as libc::c_int) as usize] as libc::c_int
            == buf[(len - 2 as libc::c_int) as usize] as libc::c_int
        {
            repeat = 1 as libc::c_int;
            if len > 2 as libc::c_int {
                output(out, buf.as_mut_ptr(), len - 2 as libc::c_int);
                buf[1 as libc::c_int as usize] = buf[(len - 1 as libc::c_int) as usize];
                buf[0 as libc::c_int as usize] = buf[1 as libc::c_int as usize];
                len = 2 as libc::c_int;
            }
        } else if len == 128 as libc::c_int || end != 0 {
            output(out, buf.as_mut_ptr(), len);
            len = 0 as libc::c_int;
            repeat = 0 as libc::c_int;
        }
    }
    put.expect("non-null function pointer")(out, -(1 as libc::c_int));
}
#[no_mangle]
pub unsafe extern "C" fn decode(mut in_0: stream, mut out: stream) {
    let mut c: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut cnt: libc::c_int = 0;
    loop {
        c = ((*in_0).get).expect("non-null function pointer")(in_0);
        if c == -(1 as libc::c_int) {
            return;
        }
        if c > 128 as libc::c_int {
            cnt = c - 128 as libc::c_int;
            i = 0 as libc::c_int;
            while i < cnt {
                ((*out).put)
                    .expect(
                        "non-null function pointer",
                    )(out, ((*in_0).get).expect("non-null function pointer")(in_0));
                i += 1;
                i;
            }
        } else {
            cnt = c;
            c = ((*in_0).get).expect("non-null function pointer")(in_0);
            i = 0 as libc::c_int;
            while i < cnt {
                ((*out).put).expect("non-null function pointer")(out, c);
                i += 1;
                i;
            }
        }
    };
}
unsafe fn main_0() -> libc::c_int {
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut str_in: string_stream = {
        let mut init = string_stream {
            get: Some(sget as unsafe extern "C" fn(stream) -> libc::c_int),
            put: None,
            string: b"WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWBWWWWWWWWWWWWWW\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            pos: 0 as libc::c_int,
        };
        init
    };
    let mut str_out: string_stream = {
        let mut init = string_stream {
            get: Some(sget as unsafe extern "C" fn(stream) -> libc::c_int),
            put: Some(sput as unsafe extern "C" fn(stream, libc::c_int) -> libc::c_int),
            string: buf.as_mut_ptr(),
            pos: 0 as libc::c_int,
        };
        init
    };
    let mut file: file_stream = {
        let mut init = file_stream {
            get: None,
            put: Some(
                file_put as unsafe extern "C" fn(stream, libc::c_int) -> libc::c_int,
            ),
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
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
