#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use c2rust_out::*;
extern "C" {
    fn perror(__s: *const i8);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn exit(_: i32) -> !;
    fn mmap(
        __addr: *mut libc::c_void,
        __len: u64,
        __prot: i32,
        __flags: i32,
        __fd: i32,
        __offset: i64,
    ) -> *mut libc::c_void;
}
pub type f_int = Option<unsafe extern "C" fn() -> i32>;
#[no_mangle]
pub extern "C" fn _tmpl() -> i32 {
    let mut x: i32 = 0xdeadbeef;
    return x * x;
}

#[no_mangle]
pub extern "C" fn dupf(mut v: i32) -> f_int {
    unsafe {
        let mut len: u64 = (::core::mem::transmute::<
            Option<unsafe extern "C" fn(i32) -> f_int>,
            *mut libc::c_void,
        >(Some(dupf as unsafe extern "C" fn(i32) -> f_int)))
        .offset_from(::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> i32>,
            *mut libc::c_void,
        >(Some(::core::mem::transmute::<
            unsafe extern "C" fn() -> i32,
            unsafe extern "C" fn() -> i32,
        >(_tmpl)))) as u64;
        let mut ret: f_int = ::core::mem::transmute::<*mut libc::c_void, f_int>(mmap(
            0 as *mut libc::c_void,
            len,
            0x4 | 0x2,
            0x2 | 0x20,
            0,
            0,
        ));
        let mut p: *mut i8 = 0 as *mut i8;
        if ret == ::core::mem::transmute::<*mut libc::c_void, f_int>(-1i32 as *mut libc::c_void) {
            perror(b"mmap\0" as *const u8 as *const i8);
            exit(-1);
        }
        memcpy(
            ::core::mem::transmute::<f_int, *mut libc::c_void>(ret),
            ::core::mem::transmute::<Option<unsafe extern "C" fn() -> i32>, *const libc::c_void>(
                Some(::core::mem::transmute::<
                    unsafe extern "C" fn() -> i32,
                    unsafe extern "C" fn() -> i32,
                >(_tmpl)),
            ),
            len,
        );
        p = ::core::mem::transmute::<f_int, *mut i8>(ret);
        while p
            < (::core::mem::transmute::<f_int, *mut i8>(ret))
                .offset(len as isize)
                .offset(-(::core::mem::size_of::<i32>() as u64 as isize))
        {
            if *(p as *mut i32) as u32 == 0xdeadbeef {
                *(p as *mut i32) = v;
            }
            p = p.offset(1);
            p;
        }
        return ret;
    }
}

fn main_0() -> i32 {
    unsafe {
        let mut funcs: [f_int; 10] = [None; 10];
        let mut i: i32 = 0;
        i = 0;
        while i < 10 {
            funcs[i as usize] = dupf(i);
            i += 1;
            i;
        }
        i = 0;
        while i < 9 {
            print!(
                "func[{}]: {}\n",
                i,
                ::core::mem::transmute::<_, fn() -> i32>(
                    (funcs[i as usize]).expect("non-null function pointer"),
                )()
            );
            i += 1;
            i;
        }
    }
    return 0;
}

pub fn main() {
    unsafe {
        ::std::process::exit(main_0() as i32);
    }
}
