#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
fn build_str_from_raw_ptr(raw_ptr: *mut u8) -> String {
    unsafe {
        let mut str_size: usize = 0;
        while *raw_ptr.offset(str_size as isize) != 0 {
            str_size += 1;
        }
        return std::str::from_utf8_unchecked(std::slice::from_raw_parts(raw_ptr, str_size))
            .to_owned();
    }
}

use std::time::SystemTime;
pub fn rust_time(ref_result: Option<&mut i64>) -> i64 {
    let result = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    };
    match ref_result {
        Some(r) => *r = result,
        None => {}
    }
    return result as i64;
}

use c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    fn printf(_: *const i8, _: ...) -> i32;
    fn fgets(__s: *mut i8, __n: i32, __stream: *mut FILE) -> *mut i8;
    fn __ctype_b_loc() -> *mut *const u16;
    fn rand() -> i32;
    fn srand(__seed: u32);
    fn exit(_: i32) -> !;
    fn _setjmp(_: *mut __jmp_buf_tag) -> i32;
    fn longjmp(_: *mut __jmp_buf_tag, _: i32) -> !;
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [u64; 16],
}
pub type __jmp_buf = [i64; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: i32,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
pub const OP_DIV: u32 = 5;
pub const OP_MUL: u32 = 4;
pub const OP_SUB: u32 = 3;
pub const OP_ADD: u32 = 2;
pub const OP_NUM: u32 = 1;
pub const OP_NONE: u32 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct expr_t {
    pub op: i32,
    pub val: i32,
    pub used: i32,
    pub left: expr,
    pub right: expr,
}
pub type expr = *mut expr_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct frac_t {
    pub denom: i32,
    pub num: i32,
}
pub type frac = *mut frac_t;
#[no_mangle]
pub static mut ctx: jmp_buf = [__jmp_buf_tag {
    __jmpbuf: [0; 8],
    __mask_was_saved: 0,
    __saved_mask: __sigset_t { __val: [0; 16] },
}; 1];
#[no_mangle]
pub static mut msg: *const i8 = 0 as *const i8;
#[no_mangle]
pub static mut digits: [expr_t; 4] = [expr_t {
    op: 0,
    val: 0,
    used: 0,
    left: 0 as *const expr_t as *mut expr_t,
    right: 0 as *const expr_t as *mut expr_t,
}; 4];
#[no_mangle]
pub extern "C" fn gen_digits() {
    let mut i: i32 = 0;
    i = 0;
    unsafe {
        while i < 4 {
            digits[i as usize].val = 1 + rand() % 9;
            i += 1;
            i;
        }
    }
}

#[no_mangle]
pub static mut str: [i8; 64] = [0; 64];
#[no_mangle]
pub static mut pos: i32 = 0;
#[no_mangle]
pub static mut pool: [expr_t; 8] = [expr_t {
    op: 0,
    val: 0,
    used: 0,
    left: 0 as *const expr_t as *mut expr_t,
    right: 0 as *const expr_t as *mut expr_t,
}; 8];
#[no_mangle]
pub static mut pool_ptr: i32 = 0;
#[no_mangle]
pub extern "C" fn reset() {
    let mut i: i32 = 0;
    unsafe {
        msg = 0 as *const i8;
        pos = 0;
        pool_ptr = pos;
    }
    i = 0;
    unsafe {
        while i < 8 {
            pool[i as usize].op = OP_NONE as i32;
            pool[i as usize].right = 0 as expr;
            pool[i as usize].left = pool[i as usize].right;
            i += 1;
            i;
        }
    }
    i = 0;
    unsafe {
        while i < 4 {
            digits[i as usize].used = 0;
            i += 1;
            i;
        }
    }
}

#[no_mangle]
pub extern "C" fn bail(mut s: *const i8) {
    unsafe {
        msg = s;
        longjmp(ctx.as_mut_ptr(), 1);
    }
}

#[no_mangle]
pub extern "C" fn new_expr() -> expr {
    unsafe {
        if pool_ptr < 8 {
            let fresh0 = pool_ptr;
            pool_ptr = pool_ptr + 1;
            return pool.as_mut_ptr().offset(fresh0 as isize);
        }
    }
    return 0 as expr;
}

#[no_mangle]
pub extern "C" fn next_tok() -> i32 {
    unsafe {
        while *(*__ctype_b_loc()).offset(str[pos as usize] as i32 as isize) as i32 & _ISspace as i32
            != 0
        {
            pos += 1;
            pos;
        }
        return str[pos as usize] as i32;
    }
}

#[no_mangle]
pub extern "C" fn take() -> i32 {
    unsafe {
        if str[pos as usize] as i32 != '\0' as i32 {
            pos += 1;
            return pos;
        }
    }
    return 0;
}

#[no_mangle]
pub extern "C" fn get_expr() -> expr {
    let mut c: i32 = 0;
    let mut l: expr = 0 as *mut expr_t;
    let mut r: expr = 0 as *mut expr_t;
    let mut ret: expr = 0 as *mut expr_t;
    ret = get_term();
    if ret.is_null() {
        bail(b"Expected term\0" as *const u8 as *const i8);
    }
    loop {
        c = next_tok();
        if !(c == '+' as i32 || c == '-' as i32) {
            break;
        }
        if take() == 0 {
            bail(b"Unexpected end of input\0" as *const u8 as *const i8);
        }
        r = get_term();
        if r.is_null() {
            bail(b"Expected term\0" as *const u8 as *const i8);
        }
        l = ret;
        ret = new_expr();
        (*ret).op = if c == '+' as i32 {
            OP_ADD as i32
        } else {
            OP_SUB as i32
        };
        (*ret).left = l;
        (*ret).right = r;
    }
    return ret;
}

#[no_mangle]
pub extern "C" fn get_term() -> expr {
    let mut c: i32 = 0;
    let mut l: expr = 0 as *mut expr_t;
    let mut r: expr = 0 as *mut expr_t;
    let mut ret: expr = 0 as *mut expr_t;
    ret = get_fact();
    loop {
        c = next_tok();
        if !(c == '*' as i32 || c == '/' as i32) {
            break;
        }
        if take() == 0 {
            bail(b"Unexpected end of input\0" as *const u8 as *const i8);
        }
        r = get_fact();
        l = ret;
        ret = new_expr();
        (*ret).op = if c == '*' as i32 {
            OP_MUL as i32
        } else {
            OP_DIV as i32
        };
        (*ret).left = l;
        (*ret).right = r;
    }
    return ret;
}

#[no_mangle]
pub extern "C" fn get_digit() -> expr {
    let mut i: i32 = 0;
    let mut c: i32 = next_tok();
    let mut ret: expr = 0 as *mut expr_t;
    unsafe {
        if c >= '0' as i32 && c <= '9' as i32 {
            take();
            ret = new_expr();
            (*ret).op = OP_NUM as i32;
            (*ret).val = c - '0' as i32;
            i = 0;
            while i < 4 {
                if digits[i as usize].val == (*ret).val && digits[i as usize].used == 0 {
                    digits[i as usize].used = 1;
                    return ret;
                }
                i += 1;
                i;
            }
            bail(b"Invalid digit\0" as *const u8 as *const i8);
        }
    }
    return 0 as expr;
}

#[no_mangle]
pub extern "C" fn get_fact() -> expr {
    let mut c: i32 = 0;
    let mut l: expr = get_digit();
    if !l.is_null() {
        return l;
    }
    c = next_tok();
    if c == '(' as i32 {
        take();
        l = get_expr();
        if next_tok() != ')' as i32 {
            bail(b"Unbalanced parens\0" as *const u8 as *const i8);
        }
        take();
        return l;
    }
    return 0 as expr;
}

#[no_mangle]
pub extern "C" fn parse() -> expr {
    let mut i: i32 = 0;
    let mut ret: expr = get_expr();
    if next_tok() != '\0' as i32 {
        bail(b"Trailing garbage\0" as *const u8 as *const i8);
    }
    i = 0;
    unsafe {
        while i < 4 {
            if digits[i as usize].used == 0 {
                bail(b"Not all digits are used\0" as *const u8 as *const i8);
            }
            i += 1;
            i;
        }
    }
    return ret;
}

#[no_mangle]
pub extern "C" fn gcd(mut m: i32, mut n: i32) -> i32 {
    let mut t: i32 = 0;
    while m != 0 {
        t = m;
        m = n % m;
        n = t;
    }
    return n;
}

#[no_mangle]
pub extern "C" fn eval_tree(mut e: expr, mut res: frac) {
    let mut l: frac_t = frac_t { denom: 0, num: 0 };
    let mut r: frac_t = frac_t { denom: 0, num: 0 };
    let mut t: i32 = 0;
    if (*e).op == OP_NUM as i32 {
        (*res).num = (*e).val;
        (*res).denom = 1;
        return;
    }
    eval_tree((*e).left, &mut l);
    eval_tree((*e).right, &mut r);
    match (*e).op {
        2 => {
            (*res).num = l.num * r.denom + l.denom * r.num;
            (*res).denom = l.denom * r.denom;
        }
        3 => {
            (*res).num = l.num * r.denom - l.denom * r.num;
            (*res).denom = l.denom * r.denom;
        }
        4 => {
            (*res).num = l.num * r.num;
            (*res).denom = l.denom * r.denom;
        }
        5 => {
            (*res).num = l.num * r.denom;
            (*res).denom = l.denom * r.num;
        }
        _ => {}
    }
    t = gcd((*res).denom, (*res).num);
    if t != 0 {
        (*res).denom /= t;
        (*res).num /= t;
    }
}

#[no_mangle]
pub extern "C" fn get_input() {
    let mut i: i32 = 0;
    unsafe {
        loop {
            reset();
            print!("\nAvailable digits are:");
            i = 0;
            while i < 4 {
                print!(" {}", digits[i as usize].val);
                i += 1;
                i;
            }
            printf (b". Type an expression and I'll check it for you, or make new numbers.\nYour choice? [Expr/n/q] \0" as * const u8 as * const i8,);
            i = 0;
            while i < 64 {
                str[i as usize] = '\n' as i8;
                i += 1;
                i;
            }
            fgets(str.as_mut_ptr(), 64, stdin);
            if *str.as_mut_ptr() as i32 == '\0' as i32 {
                continue;
            }
            if str[(64 - 1i32) as usize] as i32 != '\n' as i32 {
                bail(b"string too long\0" as *const u8 as *const i8);
            }
            i = 0;
            while i < 64 {
                if str[i as usize] as i32 == '\n' as i32 {
                    str[i as usize] = '\0' as i8;
                }
                i += 1;
                i;
            }
            if str[0 as usize] as i32 == 'q' as i32 {
                print!("Bye\n");
                exit(0);
            }
            if !(str[0 as usize] as i32 == 'n' as i32) {
                break;
            }
            gen_digits();
        }
    }
}

fn main_0() -> i32 {
    let mut f: frac_t = frac_t { denom: 0, num: 0 };
    unsafe {
        srand(rust_time(None) as u32);
    }
    gen_digits();
    unsafe {
        loop {
            get_input();
            _setjmp(ctx.as_mut_ptr());
            if !msg.is_null() {
                print!(
                    "{0:} at {2:.2$}\n",
                    build_str_from_raw_ptr(msg as *mut u8),
                    pos,
                    build_str_from_raw_ptr(str.as_mut_ptr() as *mut u8)
                );
            } else {
                eval_tree(parse(), &mut f);
                if f.denom == 0 {
                    bail(b"Divide by zero\0" as *const u8 as *const i8);
                }
                if f.denom == 1 && f.num == 24 {
                    print!("You got 24.  Very good.\n");
                } else {
                    if f.denom == 1 {
                        print!("Eval to: {}, ", f.num);
                    } else {
                        print!("Eval to: {}/{}, ", f.num, f.denom);
                    }
                    print!("no good.  Try again.\n");
                }
            }
        }
    }
}

pub fn main() {
    unsafe {
        ::std::process::exit(main_0() as i32);
    }
}
