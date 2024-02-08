#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
pub type ubyte = libc::c_uchar;
#[no_mangle]
pub static mut BASE64: [ubyte; 65] = unsafe {
    *::core::mem::transmute::<
        &[u8; 65],
        &[ubyte; 65],
    >(b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/\0")
};
#[no_mangle]
pub unsafe extern "C" fn findIndex(val: ubyte) -> libc::c_int {
    if 'A' as i32 <= val as libc::c_int && val as libc::c_int <= 'Z' as i32 {
        return val as libc::c_int - 'A' as i32;
    }
    if 'a' as i32 <= val as libc::c_int && val as libc::c_int <= 'z' as i32 {
        return val as libc::c_int - 'a' as i32 + 26 as libc::c_int;
    }
    if '0' as i32 <= val as libc::c_int && val as libc::c_int <= '9' as i32 {
        return val as libc::c_int - '0' as i32 + 52 as libc::c_int;
    }
    if val as libc::c_int == '+' as i32 {
        return 62 as libc::c_int;
    }
    if val as libc::c_int == '/' as i32 {
        return 63 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn decode(
    mut source: *const ubyte,
    mut sink: *mut ubyte,
) -> libc::c_int {
    let length: size_t = strlen(source as *const libc::c_char);
    let mut it: *const ubyte = source;
    let mut end: *const ubyte = source.offset(length as isize);
    let mut acc: libc::c_int = 0;
    if length.wrapping_rem(4 as libc::c_int as libc::c_ulong)
        != 0 as libc::c_int as libc::c_ulong
    {
        return 1 as libc::c_int;
    }
    while it != end {
        let fresh0 = it;
        it = it.offset(1);
        let b1: ubyte = *fresh0;
        let fresh1 = it;
        it = it.offset(1);
        let b2: ubyte = *fresh1;
        let fresh2 = it;
        it = it.offset(1);
        let b3: ubyte = *fresh2;
        let fresh3 = it;
        it = it.offset(1);
        let b4: ubyte = *fresh3;
        let i1: libc::c_int = findIndex(b1);
        let i2: libc::c_int = findIndex(b2);
        acc = i1 << 2 as libc::c_int;
        acc |= i2 >> 4 as libc::c_int;
        let fresh4 = sink;
        sink = sink.offset(1);
        *fresh4 = acc as ubyte;
        if b3 as libc::c_int != '=' as i32 {
            let i3: libc::c_int = findIndex(b3);
            acc = (i2 & 0xf as libc::c_int) << 4 as libc::c_int;
            acc += i3 >> 2 as libc::c_int;
            let fresh5 = sink;
            sink = sink.offset(1);
            *fresh5 = acc as ubyte;
            if b4 as libc::c_int != '=' as i32 {
                let i4: libc::c_int = findIndex(b4);
                acc = (i3 & 0x3 as libc::c_int) << 6 as libc::c_int;
                acc |= i4;
                let fresh6 = sink;
                sink = sink.offset(1);
                *fresh6 = acc as ubyte;
            }
        }
    }
    *sink = '\0' as i32 as ubyte;
    return 0 as libc::c_int;
}
unsafe fn main_0() -> libc::c_int {
    let mut data: [ubyte; 117] = *::core::mem::transmute::<
        &[u8; 117],
        &mut [ubyte; 117],
    >(
        b"VG8gZXJyIGlzIGh1bWFuLCBidXQgdG8gcmVhbGx5IGZvdWwgdGhpbmdzIHVwIHlvdSBuZWVkIGEgY29tcHV0ZXIuCiAgICAtLVBhdWwgUi5FaHJsaWNo\0",
    );
    let mut decoded: [ubyte; 1024] = [0; 1024];
    printf(b"%s\n\n\0" as *const u8 as *const libc::c_char, data.as_mut_ptr());
    decode(data.as_mut_ptr() as *const ubyte, decoded.as_mut_ptr());
    printf(b"%s\n\n\0" as *const u8 as *const libc::c_char, decoded.as_mut_ptr());
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
