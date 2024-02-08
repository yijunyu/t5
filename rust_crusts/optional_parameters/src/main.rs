#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(c_variadic, extern_types)]
use c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn qsort(__base: *mut libc::c_void, __nmemb: u64, __size: u64, __compar: __compar_fn_t);
    static mut stdout: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn __ctype_b_loc() -> *mut *const u16;
    fn tolower(_: i32) -> i32;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: u32,
    pub fp_offset: u32,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __compar_fn_t =
    Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32>;
pub type va_list = __builtin_va_list;
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
pub const _ISalnum: u32 = 8;
pub const _ISpunct: u32 = 4;
pub const _IScntrl: u32 = 2;
pub const _ISblank: u32 = 1;
pub const _ISgraph: u32 = 32768;
pub const _ISprint: u32 = 16384;
pub const _ISspace: u32 = 8192;
pub const _ISxdigit: u32 = 4096;
pub const _ISdigit: u32 = 2048;
pub const _ISalpha: u32 = 1024;
pub const _ISlower: u32 = 512;
pub const _ISupper: u32 = 256;
pub type String_0 = *const i8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sTable {
    pub rows: *mut *mut String_0,
    pub n_rows: i32,
    pub n_cols: i32,
}
pub type Table = *mut sTable;
pub type CompareFctn = Option<unsafe extern "C" fn(String_0, String_0) -> i32>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub compare: CompareFctn,
    pub column: i32,
    pub reversed: i32,
}
#[no_mangle]
pub static mut sortSpec: C2RustUnnamed_0 = C2RustUnnamed_0 {
    compare: None,
    column: 0,
    reversed: 0,
};
#[no_mangle]
pub extern "C" fn CmprRows(mut aa: *const libc::c_void, mut bb: *const libc::c_void) -> i32 {
    unsafe {
        let mut rA: *mut String_0 = *(aa as *const *mut String_0);
        let mut rB: *mut String_0 = *(bb as *const *mut String_0);
        let mut sortCol: i32 = sortSpec.column;
        let mut left: String_0 = if sortSpec.reversed != 0 {
            *rB.offset(sortCol as isize)
        } else {
            *rA.offset(sortCol as isize)
        };
        let mut right: String_0 = if sortSpec.reversed != 0 {
            *rA.offset(sortCol as isize)
        } else {
            *rB.offset(sortCol as isize)
        };
        return (sortSpec.compare).expect("non-null function pointer")(left, right);
    }
}

#[no_mangle]
pub unsafe extern "C" fn sortTable(mut tbl: Table, mut argSpec: *const i8, mut args: ...) -> i32 {
    let mut vl: ::core::ffi::VaListImpl;
    let mut p: *const i8 = 0 as *const i8;
    let mut c: i32 = 0;
    sortSpec.compare = Some(strcmp as unsafe extern "C" fn(*const i8, *const i8) -> i32);
    sortSpec.column = 0;
    sortSpec.reversed = 0;
    vl = args.clone();
    if !argSpec.is_null() {
        p = argSpec;
        while *p != 0 {
            match *p as i32 {
                111 => {
                    sortSpec.compare = ::core::mem::transmute(vl.arg::<*mut unsafe extern "C" fn(
                        String_0,
                        String_0,
                    )
                        -> i32>());
                }
                99 => {
                    c = vl.arg::<i32>();
                    if 0 <= c && c < (*tbl).n_cols {
                        sortSpec.column = c;
                    }
                }
                114 => {
                    sortSpec.reversed = (0 != vl.arg::<i32>()) as i32;
                }
                _ => {}
            }
            p = p.offset(1);
            p;
        }
    }
    qsort(
        (*tbl).rows as *mut libc::c_void,
        (*tbl).n_rows as u64,
        ::core::mem::size_of::<*mut String_0>() as u64,
        Some(CmprRows as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32),
    );
    return 0;
}

#[no_mangle]
pub extern "C" fn printTable(mut tbl: Table, mut fout: *mut FILE, mut colFmts: *mut *const i8) {
    unsafe {
        let mut row: i32 = 0;
        let mut col: i32 = 0;
        row = 0;
        while row < (*tbl).n_rows {
            fprintf(fout, b"   \0" as *const u8 as *const i8);
            col = 0;
            while col < (*tbl).n_cols {
                fprintf(
                    fout,
                    *colFmts.offset(col as isize),
                    *(*((*tbl).rows).offset(row as isize)).offset(col as isize),
                );
                col += 1;
                col;
            }
            fprintf(fout, b"\n\0" as *const u8 as *const i8);
            row += 1;
            row;
        }
        fprintf(fout, b"\n\0" as *const u8 as *const i8);
    }
}

#[no_mangle]
pub extern "C" fn ord(mut v: i8) -> i32 {
    return v as i32 - '0' as i32;
}

#[no_mangle]
pub extern "C" fn cmprStrgs(mut s1: String_0, mut s2: String_0) -> i32 {
    unsafe {
        let mut p1: *const i8 = s1;
        let mut p2: *const i8 = s2;
        let mut mrk1: *const i8 = 0 as *const i8;
        let mut mrk2: *const i8 = 0 as *const i8;
        while tolower(*p1 as i32) == tolower(*p2 as i32) && *p1 as i32 != 0 {
            p1 = p1.offset(1);
            p1;
            p2 = p2.offset(1);
            p2;
        }
        if *(*__ctype_b_loc()).offset(*p1 as i32 as isize) as i32 & _ISdigit as i32 != 0
            && *(*__ctype_b_loc()).offset(*p2 as i32 as isize) as i32 & _ISdigit as i32 != 0
        {
            let mut v1: i64 = 0;
            let mut v2: i64 = 0;
            if *p1 as i32 == '0' as i32 || *p2 as i32 == '0' as i32 {
                while p1 > s1 {
                    p1 = p1.offset(-1);
                    p1;
                    p2 = p2.offset(-1);
                    p2;
                    if *p1 as i32 != '0' as i32 {
                        break;
                    }
                }
                if *(*__ctype_b_loc()).offset(*p1 as i32 as isize) as i32 & _ISdigit as i32 == 0 {
                    p1 = p1.offset(1);
                    p1;
                    p2 = p2.offset(1);
                    p2;
                }
            }
            mrk1 = p1;
            mrk2 = p2;
            v1 = 0;
            while *(*__ctype_b_loc()).offset(*p1 as i32 as isize) as i32 & _ISdigit as i32 != 0 {
                v1 = 10 * v1 + ord(*p1) as i64;
                p1 = p1.offset(1);
                p1;
            }
            v2 = 0;
            while *(*__ctype_b_loc()).offset(*p2 as i32 as isize) as i32 & _ISdigit as i32 != 0 {
                v2 = 10 * v2 + ord(*p2) as i64;
                p2 = p2.offset(1);
                p2;
            }
            if v1 == v2 {
                return (p2.offset_from(mrk2) as i64 - p1.offset_from(mrk1) as i64) as i32;
            }
            return (v1 - v2) as i32;
        }
        if tolower(*p1 as i32) != tolower(*p2 as i32) {
            return tolower(*p1 as i32) - tolower(*p2 as i32);
        }
        p1 = s1;
        p2 = s2;
        while *p1 as i32 == *p2 as i32 && *p1 as i32 != 0 {
            p1 = p1.offset(1);
            p1;
            p2 = p2.offset(1);
            p2;
        }
        return *p1 as i32 - *p2 as i32;
    }
}

fn main_0() -> i32 {
    let mut colFmts: [*const i8; 3] = [
        b" %-5.5s\0" as *const u8 as *const i8,
        b" %-5.5s\0" as *const u8 as *const i8,
        b" %-9.9s\0" as *const u8 as *const i8,
    ];
    let mut r1: [String_0; 3] = [
        b"a101\0" as *const u8 as *const i8,
        b"red\0" as *const u8 as *const i8,
        b"Java\0" as *const u8 as *const i8,
    ];
    let mut r2: [String_0; 3] = [
        b"ab40\0" as *const u8 as *const i8,
        b"gren\0" as *const u8 as *const i8,
        b"Smalltalk\0" as *const u8 as *const i8,
    ];
    let mut r3: [String_0; 3] = [
        b"ab9\0" as *const u8 as *const i8,
        b"blue\0" as *const u8 as *const i8,
        b"Fortran\0" as *const u8 as *const i8,
    ];
    let mut r4: [String_0; 3] = [
        b"ab09\0" as *const u8 as *const i8,
        b"ylow\0" as *const u8 as *const i8,
        b"Python\0" as *const u8 as *const i8,
    ];
    let mut r5: [String_0; 3] = [
        b"ab1a\0" as *const u8 as *const i8,
        b"blak\0" as *const u8 as *const i8,
        b"Factor\0" as *const u8 as *const i8,
    ];
    let mut r6: [String_0; 3] = [
        b"ab1b\0" as *const u8 as *const i8,
        b"brwn\0" as *const u8 as *const i8,
        b"C Sharp\0" as *const u8 as *const i8,
    ];
    let mut r7: [String_0; 3] = [
        b"Ab1b\0" as *const u8 as *const i8,
        b"pink\0" as *const u8 as *const i8,
        b"Ruby\0" as *const u8 as *const i8,
    ];
    let mut r8: [String_0; 3] = [
        b"ab1\0" as *const u8 as *const i8,
        b"orng\0" as *const u8 as *const i8,
        b"Scheme\0" as *const u8 as *const i8,
    ];
    let mut rows: [*mut String_0; 8] = [
        r1.as_mut_ptr(),
        r2.as_mut_ptr(),
        r3.as_mut_ptr(),
        r4.as_mut_ptr(),
        r5.as_mut_ptr(),
        r6.as_mut_ptr(),
        r7.as_mut_ptr(),
        r8.as_mut_ptr(),
    ];
    let mut table: sTable = sTable {
        rows: 0 as *mut *mut String_0,
        n_rows: 0,
        n_cols: 0,
    };
    table.rows = rows.as_mut_ptr();
    table.n_rows = 8;
    table.n_cols = 3;
    unsafe {
        sortTable(&mut table as *mut sTable, b"\0" as *const u8 as *const i8);
    }
    print!("sort on col 0, ascending\n");
    unsafe {
        printTable(&mut table, stdout, colFmts.as_mut_ptr());
        sortTable(
            &mut table as *mut sTable,
            b"ro\0" as *const u8 as *const i8,
            1,
            Some(cmprStrgs as unsafe extern "C" fn(String_0, String_0) -> i32),
        );
    }
    print!("sort on col 0, reverse.special\n");
    unsafe {
        printTable(&mut table, stdout, colFmts.as_mut_ptr());
        sortTable(
            &mut table as *mut sTable,
            b"c\0" as *const u8 as *const i8,
            1,
        );
    }
    print!("sort on col 1, ascending\n");
    unsafe {
        printTable(&mut table, stdout, colFmts.as_mut_ptr());
        sortTable(
            &mut table as *mut sTable,
            b"cr\0" as *const u8 as *const i8,
            2,
            1,
        );
    }
    print!("sort on col 2, reverse\n");
    unsafe {
        printTable(&mut table, stdout, colFmts.as_mut_ptr());
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
