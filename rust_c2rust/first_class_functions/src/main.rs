#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn acos(_: libc::c_double) -> libc::c_double;
    fn asin(_: libc::c_double) -> libc::c_double;
    fn atan(_: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn tan(_: libc::c_double) -> libc::c_double;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
}
pub type Class2Func = Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sComposition {
    pub f1: Class2Func,
    pub f2: Class2Func,
}
pub type Composition = *mut sComposition;
#[no_mangle]
pub unsafe extern "C" fn functionA(mut v: libc::c_double) -> libc::c_double {
    return v * v * v;
}
#[no_mangle]
pub unsafe extern "C" fn functionB(mut v: libc::c_double) -> libc::c_double {
    return exp(log(v) / 3 as libc::c_int as libc::c_double);
}
#[no_mangle]
pub unsafe extern "C" fn Function1(
    mut f2: Class2Func,
    mut val: libc::c_double,
) -> libc::c_double {
    return f2.expect("non-null function pointer")(val);
}
#[no_mangle]
pub unsafe extern "C" fn WhichFunc(mut idx: libc::c_int) -> Class2Func {
    return if idx < 4 as libc::c_int {
        Some(functionA as unsafe extern "C" fn(libc::c_double) -> libc::c_double)
    } else {
        Some(functionB as unsafe extern "C" fn(libc::c_double) -> libc::c_double)
    };
}
#[no_mangle]
pub static mut funcListA: [Class2Func; 4] = [
    Some(functionA as unsafe extern "C" fn(libc::c_double) -> libc::c_double),
    Some(sin as unsafe extern "C" fn(libc::c_double) -> libc::c_double),
    Some(cos as unsafe extern "C" fn(libc::c_double) -> libc::c_double),
    Some(tan as unsafe extern "C" fn(libc::c_double) -> libc::c_double),
];
#[no_mangle]
pub static mut funcListB: [Class2Func; 4] = [
    Some(functionB as unsafe extern "C" fn(libc::c_double) -> libc::c_double),
    Some(asin as unsafe extern "C" fn(libc::c_double) -> libc::c_double),
    Some(acos as unsafe extern "C" fn(libc::c_double) -> libc::c_double),
    Some(atan as unsafe extern "C" fn(libc::c_double) -> libc::c_double),
];
#[no_mangle]
pub unsafe extern "C" fn InvokeComposed(
    mut f1: Class2Func,
    mut f2: Class2Func,
    mut val: libc::c_double,
) -> libc::c_double {
    return f1
        .expect(
            "non-null function pointer",
        )(f2.expect("non-null function pointer")(val));
}
#[no_mangle]
pub unsafe extern "C" fn Compose(mut f1: Class2Func, mut f2: Class2Func) -> Composition {
    let mut comp: Composition = malloc(
        ::core::mem::size_of::<sComposition>() as libc::c_ulong,
    ) as Composition;
    (*comp).f1 = f1;
    (*comp).f2 = f2;
    return comp;
}
#[no_mangle]
pub unsafe extern "C" fn CallComposed(
    mut comp: Composition,
    mut val: libc::c_double,
) -> libc::c_double {
    return ((*comp).f1)
        .expect(
            "non-null function pointer",
        )(((*comp).f2).expect("non-null function pointer")(val));
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut ix: libc::c_int = 0;
    let mut c: Composition = 0 as *mut sComposition;
    printf(
        b"Function1(functionA, 3.0) = %f\n\0" as *const u8 as *const libc::c_char,
        Function1(WhichFunc(0 as libc::c_int), 3.0f64),
    );
    ix = 0 as libc::c_int;
    while ix < 4 as libc::c_int {
        c = Compose(funcListA[ix as usize], funcListB[ix as usize]);
        printf(
            b"Compostion %d(0.9) = %f\n\0" as *const u8 as *const libc::c_char,
            ix,
            CallComposed(c, 0.9f64),
        );
        ix += 1;
        ix;
    }
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
