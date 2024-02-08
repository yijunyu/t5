#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sqrt(_: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub static mut tri: [libc::c_int; 171] = [
    55 as libc::c_int,
    94 as libc::c_int,
    48 as libc::c_int,
    95 as libc::c_int,
    30 as libc::c_int,
    96 as libc::c_int,
    77 as libc::c_int,
    71 as libc::c_int,
    26 as libc::c_int,
    67 as libc::c_int,
    97 as libc::c_int,
    13 as libc::c_int,
    76 as libc::c_int,
    38 as libc::c_int,
    45 as libc::c_int,
    7 as libc::c_int,
    36 as libc::c_int,
    79 as libc::c_int,
    16 as libc::c_int,
    37 as libc::c_int,
    68 as libc::c_int,
    48 as libc::c_int,
    7 as libc::c_int,
    9 as libc::c_int,
    18 as libc::c_int,
    70 as libc::c_int,
    26 as libc::c_int,
    6 as libc::c_int,
    18 as libc::c_int,
    72 as libc::c_int,
    79 as libc::c_int,
    46 as libc::c_int,
    59 as libc::c_int,
    79 as libc::c_int,
    29 as libc::c_int,
    90 as libc::c_int,
    20 as libc::c_int,
    76 as libc::c_int,
    87 as libc::c_int,
    11 as libc::c_int,
    32 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    49 as libc::c_int,
    18 as libc::c_int,
    27 as libc::c_int,
    83 as libc::c_int,
    58 as libc::c_int,
    35 as libc::c_int,
    71 as libc::c_int,
    11 as libc::c_int,
    25 as libc::c_int,
    57 as libc::c_int,
    29 as libc::c_int,
    85 as libc::c_int,
    14 as libc::c_int,
    64 as libc::c_int,
    36 as libc::c_int,
    96 as libc::c_int,
    27 as libc::c_int,
    11 as libc::c_int,
    58 as libc::c_int,
    56 as libc::c_int,
    92 as libc::c_int,
    18 as libc::c_int,
    55 as libc::c_int,
    2 as libc::c_int,
    90 as libc::c_int,
    3 as libc::c_int,
    60 as libc::c_int,
    48 as libc::c_int,
    49 as libc::c_int,
    41 as libc::c_int,
    46 as libc::c_int,
    33 as libc::c_int,
    36 as libc::c_int,
    47 as libc::c_int,
    23 as libc::c_int,
    92 as libc::c_int,
    50 as libc::c_int,
    48 as libc::c_int,
    2 as libc::c_int,
    36 as libc::c_int,
    59 as libc::c_int,
    42 as libc::c_int,
    79 as libc::c_int,
    72 as libc::c_int,
    20 as libc::c_int,
    82 as libc::c_int,
    77 as libc::c_int,
    42 as libc::c_int,
    56 as libc::c_int,
    78 as libc::c_int,
    38 as libc::c_int,
    80 as libc::c_int,
    39 as libc::c_int,
    75 as libc::c_int,
    2 as libc::c_int,
    71 as libc::c_int,
    66 as libc::c_int,
    66 as libc::c_int,
    1 as libc::c_int,
    3 as libc::c_int,
    55 as libc::c_int,
    72 as libc::c_int,
    44 as libc::c_int,
    25 as libc::c_int,
    67 as libc::c_int,
    84 as libc::c_int,
    71 as libc::c_int,
    67 as libc::c_int,
    11 as libc::c_int,
    61 as libc::c_int,
    40 as libc::c_int,
    57 as libc::c_int,
    58 as libc::c_int,
    89 as libc::c_int,
    40 as libc::c_int,
    56 as libc::c_int,
    36 as libc::c_int,
    85 as libc::c_int,
    32 as libc::c_int,
    25 as libc::c_int,
    85 as libc::c_int,
    57 as libc::c_int,
    48 as libc::c_int,
    84 as libc::c_int,
    35 as libc::c_int,
    47 as libc::c_int,
    62 as libc::c_int,
    17 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    99 as libc::c_int,
    89 as libc::c_int,
    52 as libc::c_int,
    6 as libc::c_int,
    71 as libc::c_int,
    28 as libc::c_int,
    75 as libc::c_int,
    94 as libc::c_int,
    48 as libc::c_int,
    37 as libc::c_int,
    10 as libc::c_int,
    23 as libc::c_int,
    51 as libc::c_int,
    6 as libc::c_int,
    48 as libc::c_int,
    53 as libc::c_int,
    18 as libc::c_int,
    74 as libc::c_int,
    98 as libc::c_int,
    15 as libc::c_int,
    27 as libc::c_int,
    2 as libc::c_int,
    92 as libc::c_int,
    23 as libc::c_int,
    8 as libc::c_int,
    71 as libc::c_int,
    76 as libc::c_int,
    84 as libc::c_int,
    15 as libc::c_int,
    52 as libc::c_int,
    92 as libc::c_int,
    63 as libc::c_int,
    81 as libc::c_int,
    10 as libc::c_int,
    44 as libc::c_int,
    10 as libc::c_int,
    69 as libc::c_int,
    93 as libc::c_int,
];
unsafe fn main_0() -> libc::c_int {
    let len: libc::c_int = (::core::mem::size_of::<[libc::c_int; 171]>()
        as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
        as libc::c_int;
    let base: libc::c_int = ((sqrt(
        (8 as libc::c_int * len + 1 as libc::c_int) as libc::c_double,
    ) - 1 as libc::c_int as libc::c_double) / 2 as libc::c_int as libc::c_double)
        as libc::c_int;
    let mut step: libc::c_int = base - 1 as libc::c_int;
    let mut stepc: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    i = len - base - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        tri[i as usize]
            += if tri[(i + step) as usize] > tri[(i + step + 1 as libc::c_int) as usize]
            {
                tri[(i + step) as usize]
            } else {
                tri[(i + step + 1 as libc::c_int) as usize]
            };
        stepc += 1;
        if stepc == step {
            step -= 1;
            step;
            stepc = 0 as libc::c_int;
        }
        i -= 1;
        i;
    }
    printf(
        b"%d\n\0" as *const u8 as *const libc::c_char,
        tri[0 as libc::c_int as usize],
    );
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
