#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sHme {
    pub key: *const libc::c_char,
    pub value: libc::c_int,
    pub link: *mut sHme,
}
pub type MapEntry = *mut sHme;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct he {
    pub first: MapEntry,
    pub last: MapEntry,
}
pub type HashElement = he;
pub type KeyCopyF = Option::<
    unsafe extern "C" fn(*mut *const libc::c_char, *const libc::c_char) -> (),
>;
pub type ValCopyF = Option::<unsafe extern "C" fn(*mut libc::c_int, libc::c_int) -> ()>;
pub type KeyHashF = Option::<
    unsafe extern "C" fn(*const libc::c_char, libc::c_int) -> libc::c_uint,
>;
pub type KeyCmprF = Option::<
    unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> libc::c_int,
>;
#[no_mangle]
pub unsafe extern "C" fn strhashkey(
    mut key: *const libc::c_char,
    mut max: libc::c_int,
) -> libc::c_uint {
    let mut h: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut hl: libc::c_uint = 0;
    let mut hr: libc::c_uint = 0;
    while *key != 0 {
        h = h.wrapping_add(*key as libc::c_uint);
        hl = 0x5c5 as libc::c_int as libc::c_uint
            ^ (h & 0xfff00000 as libc::c_uint) >> 18 as libc::c_int;
        hr = h & 0xfffff as libc::c_int as libc::c_uint;
        let fresh0 = key;
        key = key.offset(1);
        h = hl ^ hr ^ *fresh0 as libc::c_uint;
    }
    return h.wrapping_rem(max as libc::c_uint);
}
#[no_mangle]
pub static mut hash: [HashElement; 4096] = [HashElement {
    first: 0 as *const sHme as *mut sHme,
    last: 0 as *const sHme as *mut sHme,
}; 4096];
#[no_mangle]
pub unsafe extern "C" fn HashAddH(
    mut key: *const libc::c_char,
    mut value: libc::c_int,
    mut copyKey: KeyCopyF,
    mut copyVal: ValCopyF,
    mut hashKey: KeyHashF,
    mut keySame: KeyCmprF,
) {
    let mut hix: libc::c_uint = (Some(hashKey.expect("non-null function pointer")))
        .expect("non-null function pointer")(key, 4096 as libc::c_int);
    let mut m_ent: MapEntry = 0 as *mut sHme;
    m_ent = hash[hix as usize].first;
    while !m_ent.is_null()
        && (Some(keySame.expect("non-null function pointer")))
            .expect("non-null function pointer")((*m_ent).key, key) == 0
    {
        m_ent = (*m_ent).link;
    }
    if !m_ent.is_null() {
        (Some(copyVal.expect("non-null function pointer")))
            .expect("non-null function pointer")(&mut (*m_ent).value, value);
    } else {
        let mut last: MapEntry = 0 as *mut sHme;
        let mut hme: MapEntry = malloc(::core::mem::size_of::<sHme>() as libc::c_ulong)
            as MapEntry;
        (Some(copyKey.expect("non-null function pointer")))
            .expect("non-null function pointer")(&mut (*hme).key, key);
        (Some(copyVal.expect("non-null function pointer")))
            .expect("non-null function pointer")(&mut (*hme).value, value);
        (*hme).link = 0 as *mut sHme;
        last = hash[hix as usize].last;
        if !last.is_null() {
            (*last).link = hme;
        } else {
            hash[hix as usize].first = hme;
        }
        hash[hix as usize].last = hme;
    };
}
#[no_mangle]
pub unsafe extern "C" fn HashGetH(
    mut val: *mut libc::c_int,
    mut key: *const libc::c_char,
    mut hashKey: KeyHashF,
    mut keySame: KeyCmprF,
) -> libc::c_int {
    let mut hix: libc::c_uint = (Some(hashKey.expect("non-null function pointer")))
        .expect("non-null function pointer")(key, 4096 as libc::c_int);
    let mut m_ent: MapEntry = 0 as *mut sHme;
    m_ent = hash[hix as usize].first;
    while !m_ent.is_null()
        && (Some(keySame.expect("non-null function pointer")))
            .expect("non-null function pointer")((*m_ent).key, key) == 0
    {
        m_ent = (*m_ent).link;
    }
    if !m_ent.is_null() {
        *val = (*m_ent).value;
    }
    return (m_ent != 0 as *mut libc::c_void as MapEntry) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn copyStr(
    mut dest: *mut *const libc::c_char,
    mut src: *const libc::c_char,
) {
    *dest = strdup(src);
}
#[no_mangle]
pub unsafe extern "C" fn copyInt(mut dest: *mut libc::c_int, mut src: libc::c_int) {
    *dest = src;
}
#[no_mangle]
pub unsafe extern "C" fn strCompare(
    mut key1: *const libc::c_char,
    mut key2: *const libc::c_char,
) -> libc::c_int {
    return (strcmp(key1, key2) == 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn HashAdd(mut key: *const libc::c_char, mut value: libc::c_int) {
    HashAddH(
        key,
        value,
        Some(
            copyStr
                as unsafe extern "C" fn(
                    *mut *const libc::c_char,
                    *const libc::c_char,
                ) -> (),
        ),
        Some(copyInt as unsafe extern "C" fn(*mut libc::c_int, libc::c_int) -> ()),
        Some(
            strhashkey
                as unsafe extern "C" fn(*const libc::c_char, libc::c_int) -> libc::c_uint,
        ),
        Some(
            strCompare
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn HashGet(
    mut val: *mut libc::c_int,
    mut key: *const libc::c_char,
) -> libc::c_int {
    return HashGetH(
        val,
        key,
        Some(
            strhashkey
                as unsafe extern "C" fn(*const libc::c_char, libc::c_int) -> libc::c_uint,
        ),
        Some(
            strCompare
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
    );
}
unsafe fn main_0() -> libc::c_int {
    static mut keyList: [*const libc::c_char; 6] = [
        b"red\0" as *const u8 as *const libc::c_char,
        b"orange\0" as *const u8 as *const libc::c_char,
        b"yellow\0" as *const u8 as *const libc::c_char,
        b"green\0" as *const u8 as *const libc::c_char,
        b"blue\0" as *const u8 as *const libc::c_char,
        b"violet\0" as *const u8 as *const libc::c_char,
    ];
    static mut valuList: [libc::c_int; 6] = [
        1 as libc::c_int,
        43 as libc::c_int,
        640 as libc::c_int,
        747 as libc::c_int,
        42 as libc::c_int,
        42 as libc::c_int,
    ];
    let mut ix: libc::c_int = 0;
    ix = 0 as libc::c_int;
    while ix < 6 as libc::c_int {
        HashAdd(keyList[ix as usize], valuList[ix as usize]);
        ix += 1;
        ix;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
