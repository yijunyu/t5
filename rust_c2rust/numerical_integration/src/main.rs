#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn log(_: libc::c_double) -> libc::c_double;
}
pub type pfunc = Option::<
    unsafe extern "C" fn(
        libc::c_double,
        libc::c_double,
        libc::c_double,
        Option::<unsafe extern "C" fn() -> libc::c_double>,
    ) -> libc::c_double,
>;
pub type rfunc = Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double>;
#[no_mangle]
pub unsafe extern "C" fn int_leftrect(
    mut from: libc::c_double,
    mut to: libc::c_double,
    mut n: libc::c_double,
    mut func: Option::<unsafe extern "C" fn() -> libc::c_double>,
) -> libc::c_double {
    let mut h: libc::c_double = (to - from) / n;
    let mut sum: libc::c_double = 0.0f64;
    let mut x: libc::c_double = 0.;
    x = from;
    while x <= to - h {
        sum
            += ::core::mem::transmute::<
                _,
                fn(_) -> libc::c_double,
            >(func.expect("non-null function pointer"))(x);
        x += h;
    }
    return h * sum;
}
#[no_mangle]
pub unsafe extern "C" fn int_rightrect(
    mut from: libc::c_double,
    mut to: libc::c_double,
    mut n: libc::c_double,
    mut func: Option::<unsafe extern "C" fn() -> libc::c_double>,
) -> libc::c_double {
    let mut h: libc::c_double = (to - from) / n;
    let mut sum: libc::c_double = 0.0f64;
    let mut x: libc::c_double = 0.;
    x = from;
    while x <= to - h {
        sum
            += ::core::mem::transmute::<
                _,
                fn(_) -> libc::c_double,
            >(func.expect("non-null function pointer"))(x + h);
        x += h;
    }
    return h * sum;
}
#[no_mangle]
pub unsafe extern "C" fn int_midrect(
    mut from: libc::c_double,
    mut to: libc::c_double,
    mut n: libc::c_double,
    mut func: Option::<unsafe extern "C" fn() -> libc::c_double>,
) -> libc::c_double {
    let mut h: libc::c_double = (to - from) / n;
    let mut sum: libc::c_double = 0.0f64;
    let mut x: libc::c_double = 0.;
    x = from;
    while x <= to - h {
        sum
            += ::core::mem::transmute::<
                _,
                fn(_) -> libc::c_double,
            >(func.expect("non-null function pointer"))(x + h / 2.0f64);
        x += h;
    }
    return h * sum;
}
#[no_mangle]
pub unsafe extern "C" fn int_trapezium(
    mut from: libc::c_double,
    mut to: libc::c_double,
    mut n: libc::c_double,
    mut func: Option::<unsafe extern "C" fn() -> libc::c_double>,
) -> libc::c_double {
    let mut h: libc::c_double = (to - from) / n;
    let mut sum: libc::c_double = ::core::mem::transmute::<
        _,
        fn(_) -> libc::c_double,
    >(func.expect("non-null function pointer"))(from)
        + ::core::mem::transmute::<
            _,
            fn(_) -> libc::c_double,
        >(func.expect("non-null function pointer"))(to);
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while (i as libc::c_double) < n {
        sum
            += 2.0f64
                * ::core::mem::transmute::<
                    _,
                    fn(_) -> libc::c_double,
                >(
                    func.expect("non-null function pointer"),
                )(from + i as libc::c_double * h);
        i += 1;
        i;
    }
    return h * sum / 2.0f64;
}
#[no_mangle]
pub unsafe extern "C" fn int_simpson(
    mut from: libc::c_double,
    mut to: libc::c_double,
    mut n: libc::c_double,
    mut func: Option::<unsafe extern "C" fn() -> libc::c_double>,
) -> libc::c_double {
    let mut h: libc::c_double = (to - from) / n;
    let mut sum1: libc::c_double = 0.0f64;
    let mut sum2: libc::c_double = 0.0f64;
    let mut i: libc::c_int = 0;
    let mut x: libc::c_double = 0.;
    i = 0 as libc::c_int;
    while (i as libc::c_double) < n {
        sum1
            += ::core::mem::transmute::<
                _,
                fn(_) -> libc::c_double,
            >(
                func.expect("non-null function pointer"),
            )(from + h * i as libc::c_double + h / 2.0f64);
        i += 1;
        i;
    }
    i = 1 as libc::c_int;
    while (i as libc::c_double) < n {
        sum2
            += ::core::mem::transmute::<
                _,
                fn(_) -> libc::c_double,
            >(func.expect("non-null function pointer"))(from + h * i as libc::c_double);
        i += 1;
        i;
    }
    return h / 6.0f64
        * (::core::mem::transmute::<
            _,
            fn(_) -> libc::c_double,
        >(func.expect("non-null function pointer"))(from)
            + ::core::mem::transmute::<
                _,
                fn(_) -> libc::c_double,
            >(func.expect("non-null function pointer"))(to) + 4.0f64 * sum1
            + 2.0f64 * sum2);
}
#[no_mangle]
pub unsafe extern "C" fn f3(mut x: libc::c_double) -> libc::c_double {
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn f3a(mut x: libc::c_double) -> libc::c_double {
    return x * x / 2.0f64;
}
#[no_mangle]
pub unsafe extern "C" fn f2(mut x: libc::c_double) -> libc::c_double {
    return 1.0f64 / x;
}
#[no_mangle]
pub unsafe extern "C" fn f2a(mut x: libc::c_double) -> libc::c_double {
    return log(x);
}
#[no_mangle]
pub unsafe extern "C" fn f1(mut x: libc::c_double) -> libc::c_double {
    return x * x * x;
}
#[no_mangle]
pub unsafe extern "C" fn f1a(mut x: libc::c_double) -> libc::c_double {
    return x * x * x * x / 4.0f64;
}
unsafe fn main_0() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ic: libc::c_double = 0.;
    let mut f: [pfunc; 5] = [
        Some(
            int_leftrect
                as unsafe extern "C" fn(
                    libc::c_double,
                    libc::c_double,
                    libc::c_double,
                    Option::<unsafe extern "C" fn() -> libc::c_double>,
                ) -> libc::c_double,
        ),
        Some(
            int_rightrect
                as unsafe extern "C" fn(
                    libc::c_double,
                    libc::c_double,
                    libc::c_double,
                    Option::<unsafe extern "C" fn() -> libc::c_double>,
                ) -> libc::c_double,
        ),
        Some(
            int_midrect
                as unsafe extern "C" fn(
                    libc::c_double,
                    libc::c_double,
                    libc::c_double,
                    Option::<unsafe extern "C" fn() -> libc::c_double>,
                ) -> libc::c_double,
        ),
        Some(
            int_trapezium
                as unsafe extern "C" fn(
                    libc::c_double,
                    libc::c_double,
                    libc::c_double,
                    Option::<unsafe extern "C" fn() -> libc::c_double>,
                ) -> libc::c_double,
        ),
        Some(
            int_simpson
                as unsafe extern "C" fn(
                    libc::c_double,
                    libc::c_double,
                    libc::c_double,
                    Option::<unsafe extern "C" fn() -> libc::c_double>,
                ) -> libc::c_double,
        ),
    ];
    let mut names: [*const libc::c_char; 5] = [
        b"leftrect\0" as *const u8 as *const libc::c_char,
        b"rightrect\0" as *const u8 as *const libc::c_char,
        b"midrect\0" as *const u8 as *const libc::c_char,
        b"trapezium\0" as *const u8 as *const libc::c_char,
        b"simpson\0" as *const u8 as *const libc::c_char,
    ];
    let mut rf: [rfunc; 4] = [
        Some(f1 as unsafe extern "C" fn(libc::c_double) -> libc::c_double),
        Some(f2 as unsafe extern "C" fn(libc::c_double) -> libc::c_double),
        Some(f3 as unsafe extern "C" fn(libc::c_double) -> libc::c_double),
        Some(f3 as unsafe extern "C" fn(libc::c_double) -> libc::c_double),
    ];
    let mut If: [rfunc; 4] = [
        Some(f1a as unsafe extern "C" fn(libc::c_double) -> libc::c_double),
        Some(f2a as unsafe extern "C" fn(libc::c_double) -> libc::c_double),
        Some(f3a as unsafe extern "C" fn(libc::c_double) -> libc::c_double),
        Some(f3a as unsafe extern "C" fn(libc::c_double) -> libc::c_double),
    ];
    let mut ivals: [libc::c_double; 8] = [
        0.0f64,
        1.0f64,
        1.0f64,
        100.0f64,
        0.0f64,
        5000.0f64,
        0.0f64,
        6000.0f64,
    ];
    let mut approx: [libc::c_double; 4] = [
        100.0f64,
        1000.0f64,
        5000000.0f64,
        6000000.0f64,
    ];
    j = 0 as libc::c_int;
    while (j as libc::c_ulong)
        < (::core::mem::size_of::<[rfunc; 4]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<rfunc>() as libc::c_ulong)
    {
        i = 0 as libc::c_int;
        while i < 5 as libc::c_int {
            ic = (Some(
                (*f.as_mut_ptr().offset(i as isize)).expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                ivals[(2 as libc::c_int * j) as usize],
                ivals[(2 as libc::c_int * j + 1 as libc::c_int) as usize],
                approx[j as usize],
                ::core::mem::transmute::<
                    rfunc,
                    Option::<unsafe extern "C" fn() -> libc::c_double>,
                >(rf[j as usize]),
            );
            printf(
                b"%10s [ 0,1] num: %+lf, an: %lf\n\0" as *const u8
                    as *const libc::c_char,
                names[i as usize],
                ic,
                (Some(
                    (*If.as_mut_ptr().offset(j as isize))
                        .expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(ivals[(2 as libc::c_int * j + 1 as libc::c_int) as usize])
                    - (Some(
                        (*If.as_mut_ptr().offset(j as isize))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(ivals[(2 as libc::c_int * j) as usize]),
            );
            i += 1;
            i;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        j += 1;
        j;
    }
    return 0;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
