#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn tolower(_: libc::c_int) -> libc::c_int;
}
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
#[no_mangle]
pub unsafe extern "C" fn rot(mut c: libc::c_int, mut str: *mut libc::c_char) {
    let mut l: libc::c_int = strlen(str) as libc::c_int;
    let mut alpha: [*const libc::c_char; 2] = [
        b"abcdefghijklmnopqrstuvwxyz\0" as *const u8 as *const libc::c_char,
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\0" as *const u8 as *const libc::c_char,
    ];
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < l {
        if !(*(*__ctype_b_loc()).offset(*str.offset(i as isize) as libc::c_int as isize)
            as libc::c_int & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
            == 0)
        {
            if *(*__ctype_b_loc())
                .offset(*str.offset(i as isize) as libc::c_int as isize) as libc::c_int
                & _ISupper as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                *str
                    .offset(
                        i as isize,
                    ) = *(alpha[1 as libc::c_int as usize])
                    .offset(
                        ((tolower(*str.offset(i as isize) as libc::c_int) - 'a' as i32
                            + c) % 26 as libc::c_int) as isize,
                    );
            } else {
                *str
                    .offset(
                        i as isize,
                    ) = *(alpha[0 as libc::c_int as usize])
                    .offset(
                        ((tolower(*str.offset(i as isize) as libc::c_int) - 'a' as i32
                            + c) % 26 as libc::c_int) as isize,
                    );
            }
        }
        i += 1;
        i;
    }
}
unsafe fn main_0() -> libc::c_int {
    let mut str: [libc::c_char; 35] = *::core::mem::transmute::<
        &[u8; 35],
        &mut [libc::c_char; 35],
    >(b"This is a top secret text message!\0");
    printf(b"Original: %s\n\0" as *const u8 as *const libc::c_char, str.as_mut_ptr());
    rot(13 as libc::c_int, str.as_mut_ptr());
    printf(b"Encrypted: %s\n\0" as *const u8 as *const libc::c_char, str.as_mut_ptr());
    rot(13 as libc::c_int, str.as_mut_ptr());
    printf(b"Decrypted: %s\n\0" as *const u8 as *const libc::c_char, str.as_mut_ptr());
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
