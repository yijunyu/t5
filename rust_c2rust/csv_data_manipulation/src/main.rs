#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __errno_location() -> *mut libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CSV {
    pub delim: *mut libc::c_char,
    pub rows: libc::c_uint,
    pub cols: libc::c_uint,
    pub table: *mut *mut libc::c_char,
}
#[no_mangle]
pub unsafe extern "C" fn trim(mut str: *mut *mut libc::c_char) -> libc::c_int {
    let mut trimmed: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    len = strlen(*str) as libc::c_int;
    n = len - 1 as libc::c_int;
    while n >= 0 as libc::c_int
        && *(*__ctype_b_loc()).offset(*(*str).offset(n as isize) as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        *(*str).offset(n as isize) = '\0' as i32 as libc::c_char;
        trimmed += 1 as libc::c_int;
        n -= 1;
        n;
    }
    n = 0 as libc::c_int;
    while n < len
        && *(*__ctype_b_loc())
            .offset(*(*str).offset(0 as libc::c_int as isize) as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        *(*str).offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        *str = (*str).offset(1 as libc::c_int as isize);
        trimmed += 1 as libc::c_int;
        n += 1;
        n;
    }
    return trimmed;
}
#[no_mangle]
pub unsafe extern "C" fn csv_destroy(mut csv: *mut CSV) -> libc::c_int {
    if csv.is_null() {
        return 0 as libc::c_int;
    }
    if !((*csv).table).is_null() {
        free((*csv).table as *mut libc::c_void);
    }
    if !((*csv).delim).is_null() {
        free((*csv).delim as *mut libc::c_void);
    }
    free(csv as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn csv_create(
    mut cols: libc::c_uint,
    mut rows: libc::c_uint,
) -> *mut CSV {
    let mut csv: *mut CSV = 0 as *mut CSV;
    csv = malloc(::core::mem::size_of::<CSV>() as libc::c_ulong) as *mut CSV;
    (*csv).rows = rows;
    (*csv).cols = cols;
    (*csv).delim = strdup(b",\0" as *const u8 as *const libc::c_char);
    (*csv)
        .table = malloc(
        (::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(cols as libc::c_ulong)
            .wrapping_mul(rows as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    if ((*csv).table).is_null() {
        csv_destroy(csv);
        return 0 as *mut CSV;
    } else {
        memset(
            (*csv).table as *mut libc::c_void,
            0 as libc::c_int,
            (::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                .wrapping_mul(cols as libc::c_ulong)
                .wrapping_mul(rows as libc::c_ulong),
        );
        return csv;
    };
}
#[no_mangle]
pub unsafe extern "C" fn csv_get(
    mut csv: *mut CSV,
    mut col: libc::c_uint,
    mut row: libc::c_uint,
) -> *mut libc::c_char {
    let mut idx: libc::c_uint = 0;
    idx = col.wrapping_add(row.wrapping_mul((*csv).cols));
    return *((*csv).table).offset(idx as isize);
}
#[no_mangle]
pub unsafe extern "C" fn csv_set(
    mut csv: *mut CSV,
    mut col: libc::c_uint,
    mut row: libc::c_uint,
    mut value: *mut libc::c_char,
) -> libc::c_int {
    let mut idx: libc::c_uint = 0;
    idx = col.wrapping_add(row.wrapping_mul((*csv).cols));
    let ref mut fresh0 = *((*csv).table).offset(idx as isize);
    *fresh0 = value;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn csv_display(mut csv: *mut CSV) {
    let mut row: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    let mut content: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*csv).rows == 0 as libc::c_int as libc::c_uint
        || (*csv).cols == 0 as libc::c_int as libc::c_uint
    {
        printf(b"[Empty table]\n\0" as *const u8 as *const libc::c_char);
        return;
    }
    printf(
        b"\n[Table cols=%d rows=%d]\n\0" as *const u8 as *const libc::c_char,
        (*csv).cols,
        (*csv).rows,
    );
    row = 0 as libc::c_int;
    while (row as libc::c_uint) < (*csv).rows {
        printf(b"[|\0" as *const u8 as *const libc::c_char);
        col = 0 as libc::c_int;
        while (col as libc::c_uint) < (*csv).cols {
            content = csv_get(csv, col as libc::c_uint, row as libc::c_uint);
            printf(b"%s\t|\0" as *const u8 as *const libc::c_char, content);
            col += 1;
            col;
        }
        printf(b"]\n\0" as *const u8 as *const libc::c_char);
        row += 1;
        row;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn csv_resize(
    mut old_csv: *mut CSV,
    mut new_cols: libc::c_uint,
    mut new_rows: libc::c_uint,
) -> libc::c_int {
    let mut cur_col: libc::c_uint = 0;
    let mut cur_row: libc::c_uint = 0;
    let mut max_cols: libc::c_uint = 0;
    let mut max_rows: libc::c_uint = 0;
    let mut new_csv: *mut CSV = 0 as *mut CSV;
    let mut content: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut in_old: libc::c_int = 0;
    let mut in_new: libc::c_int = 0;
    new_csv = csv_create(new_cols, new_rows);
    if new_csv.is_null() {
        printf(
            b"Unable to resize CSV table: error %d - %s\n\0" as *const u8
                as *const libc::c_char,
            *__errno_location(),
            strerror(*__errno_location()),
        );
        return -(1 as libc::c_int);
    } else {
        (*new_csv).rows = new_rows;
        (*new_csv).cols = new_cols;
        max_cols = if new_cols > (*old_csv).cols { new_cols } else { (*old_csv).cols };
        max_rows = if new_rows > (*old_csv).rows { new_rows } else { (*old_csv).rows };
        cur_col = 0 as libc::c_int as libc::c_uint;
        while cur_col < max_cols {
            cur_row = 0 as libc::c_int as libc::c_uint;
            while cur_row < max_rows {
                in_old = (cur_col < (*old_csv).cols && cur_row < (*old_csv).rows)
                    as libc::c_int;
                in_new = (cur_col < (*new_csv).cols && cur_row < (*new_csv).rows)
                    as libc::c_int;
                if in_old != 0 && in_new != 0 {
                    content = csv_get(old_csv, cur_col, cur_row);
                    csv_set(new_csv, cur_col, cur_row, content);
                } else if in_old != 0 {
                    content = csv_get(old_csv, cur_col, cur_row);
                    free(content as *mut libc::c_void);
                }
                cur_row = cur_row.wrapping_add(1);
                cur_row;
            }
            cur_col = cur_col.wrapping_add(1);
            cur_col;
        }
        free((*old_csv).table as *mut libc::c_void);
        (*old_csv).rows = new_rows;
        (*old_csv).cols = new_cols;
        (*old_csv).table = (*new_csv).table;
        (*new_csv).table = 0 as *mut *mut libc::c_char;
        csv_destroy(new_csv);
        return 0 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn csv_open(
    mut csv: *mut CSV,
    mut filename: *mut libc::c_char,
) -> libc::c_int {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut m_rows: libc::c_uint = 0;
    let mut m_cols: libc::c_uint = 0;
    let mut cols: libc::c_uint = 0;
    let mut line: [libc::c_char; 2048] = [0; 2048];
    let mut lineptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    fp = fopen(filename, b"r\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        fclose(fp);
        printf(
            b"Unable to open %s for reading.\0" as *const u8 as *const libc::c_char,
            filename,
        );
        return -(1 as libc::c_int);
    } else {
        m_rows = 0 as libc::c_int as libc::c_uint;
        m_cols = 0 as libc::c_int as libc::c_uint;
        while !(fgets(
            line.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong
                as libc::c_int,
            fp,
        ))
            .is_null()
        {
            m_rows = m_rows.wrapping_add(1 as libc::c_int as libc::c_uint);
            cols = 0 as libc::c_int as libc::c_uint;
            lineptr = line.as_mut_ptr();
            loop {
                token = strtok(lineptr, (*csv).delim);
                if token.is_null() {
                    break;
                }
                lineptr = 0 as *mut libc::c_char;
                trim(&mut token);
                cols = cols.wrapping_add(1 as libc::c_int as libc::c_uint);
                if cols > m_cols {
                    m_cols = cols;
                }
                csv_resize(csv, m_cols, m_rows);
                csv_set(
                    csv,
                    cols.wrapping_sub(1 as libc::c_int as libc::c_uint),
                    m_rows.wrapping_sub(1 as libc::c_int as libc::c_uint),
                    strdup(token),
                );
            }
        }
        fclose(fp);
        (*csv).rows = m_rows;
        (*csv).cols = m_cols;
        return 0 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn csv_save(
    mut csv: *mut CSV,
    mut filename: *mut libc::c_char,
) -> libc::c_int {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut row: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    let mut content: *mut libc::c_char = 0 as *mut libc::c_char;
    fp = fopen(filename, b"w\0" as *const u8 as *const libc::c_char);
    row = 0 as libc::c_int;
    while (row as libc::c_uint) < (*csv).rows {
        col = 0 as libc::c_int;
        while (col as libc::c_uint) < (*csv).cols {
            content = csv_get(csv, col as libc::c_uint, row as libc::c_uint);
            fprintf(
                fp,
                b"%s%s\0" as *const u8 as *const libc::c_char,
                content,
                if col as libc::c_uint
                    == ((*csv).cols).wrapping_sub(1 as libc::c_int as libc::c_uint)
                {
                    b"\0" as *const u8 as *const libc::c_char
                } else {
                    (*csv).delim as *const libc::c_char
                },
            );
            col += 1;
            col;
        }
        fprintf(fp, b"\n\0" as *const u8 as *const libc::c_char);
        row += 1;
        row;
    }
    fclose(fp);
    return 0 as libc::c_int;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut csv: *mut CSV = 0 as *mut CSV;
    printf(
        b"%s\n%s\n\n\0" as *const u8 as *const libc::c_char,
        b"CSV data manipulation\0" as *const u8 as *const libc::c_char,
        b"http://rosettacode.org/wiki/CSV_data_manipulation\0" as *const u8
            as *const libc::c_char,
    );
    csv = csv_create(0 as libc::c_int as libc::c_uint, 0 as libc::c_int as libc::c_uint);
    csv_open(
        csv,
        b"input.csv\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    csv_display(csv);
    csv_set(
        csv,
        0 as libc::c_int as libc::c_uint,
        0 as libc::c_int as libc::c_uint,
        b"Column0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    csv_set(
        csv,
        1 as libc::c_int as libc::c_uint,
        1 as libc::c_int as libc::c_uint,
        b"100\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    csv_set(
        csv,
        2 as libc::c_int as libc::c_uint,
        2 as libc::c_int as libc::c_uint,
        b"200\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    csv_set(
        csv,
        3 as libc::c_int as libc::c_uint,
        3 as libc::c_int as libc::c_uint,
        b"300\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    csv_set(
        csv,
        4 as libc::c_int as libc::c_uint,
        4 as libc::c_int as libc::c_uint,
        b"400\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    csv_display(csv);
    csv_save(
        csv,
        b"output.csv\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    csv_destroy(csv);
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
