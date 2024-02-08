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
    fn malloc(_: u64) -> *mut libc::c_void;
    fn acos(_: f64) -> f64;
    fn asin(_: f64) -> f64;
    fn atan(_: f64) -> f64;
    fn cos(_: f64) -> f64;
    fn sin(_: f64) -> f64;
    fn tan(_: f64) -> f64;
    fn exp(_: f64) -> f64;
    fn log(_: f64) -> f64;
}
pub type Class2Func = Option<unsafe extern "C" fn(f64) -> f64>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sComposition {
    pub f1: Class2Func,
    pub f2: Class2Func,
}
pub type Composition = *mut sComposition;
#[no_mangle]
pub extern "C" fn functionA(mut v: f64) -> f64 {
    return v * v * v;
}

#[no_mangle]
pub extern "C" fn functionB(mut v: f64) -> f64 {
    unsafe {
        return exp(log(v) / 3 as f64);
    }
}

#[no_mangle]
pub extern "C" fn Function1(mut f2: Class2Func, mut val: f64) -> f64 {
    unsafe {
        return f2.expect("non-null function pointer")(val);
    }
}

#[no_mangle]
pub extern "C" fn WhichFunc(mut idx: i32) -> Class2Func {
    return if idx < 4 {
        Some(functionA as unsafe extern "C" fn(f64) -> f64)
    } else {
        Some(functionB as unsafe extern "C" fn(f64) -> f64)
    };
}

#[no_mangle]
pub static mut funcListA: [Class2Func; 4] = [
    Some(functionA as unsafe extern "C" fn(f64) -> f64),
    Some(sin as unsafe extern "C" fn(f64) -> f64),
    Some(cos as unsafe extern "C" fn(f64) -> f64),
    Some(tan as unsafe extern "C" fn(f64) -> f64),
];
#[no_mangle]
pub static mut funcListB: [Class2Func; 4] = [
    Some(functionB as unsafe extern "C" fn(f64) -> f64),
    Some(asin as unsafe extern "C" fn(f64) -> f64),
    Some(acos as unsafe extern "C" fn(f64) -> f64),
    Some(atan as unsafe extern "C" fn(f64) -> f64),
];
#[no_mangle]
pub extern "C" fn InvokeComposed(mut f1: Class2Func, mut f2: Class2Func, mut val: f64) -> f64 {
    unsafe {
        return f1.expect("non-null function pointer")(f2.expect("non-null function pointer")(val));
    }
}

#[no_mangle]
pub extern "C" fn Compose(mut f1: Class2Func, mut f2: Class2Func) -> Composition {
    unsafe {
        let mut comp: Composition =
            malloc(::core::mem::size_of::<sComposition>() as u64) as Composition;
        (*comp).f1 = f1;
        (*comp).f2 = f2;
        return comp;
    }
}

#[no_mangle]
pub extern "C" fn CallComposed(mut comp: Composition, mut val: f64) -> f64 {
    return ((*comp).f1).expect("non-null function pointer")(((*comp).f2)
        .expect("non-null function pointer")(
        val
    ));
}

fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    unsafe {
        let mut ix: i32 = 0;
        let mut c: Composition = 0 as *mut sComposition;
        print!(
            "Function1(functionA, 3.0) = {}\n",
            Function1(WhichFunc(0), 3.0f64)
        );
        ix = 0;
        while ix < 4 {
            c = Compose(funcListA[ix as usize], funcListB[ix as usize]);
            print!("Compostion {}(0.9) = {}\n", ix, CallComposed(c, 0.9f64));
            ix += 1;
            ix;
        }
        return 0;
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
    ::std::process::exit(main_0((args.len() - 1) as i32, args.as_mut_ptr() as *mut *mut i8) as i32);
}
