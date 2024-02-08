#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
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

use c2rust_out::*;
extern "C" {
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
}
extern "C" fn _parseDecimal(mut pchCursor: *mut *const i8) -> u32 {
    unsafe {
        let mut nVal: u32 = 0;
        let mut chNow: i8 = 0;
        loop {
            chNow = **pchCursor;
            if !(chNow as i32 >= '0' as i32 && chNow as i32 <= '9' as i32) {
                break;
            }
            nVal = nVal.wrapping_mul(10);
            nVal = nVal.wrapping_add((chNow as i32 - '0' as i32) as u32);
            *pchCursor = (*pchCursor).offset(1);
            *pchCursor;
        }
        return nVal;
    }
}

extern "C" fn _parseHex(mut pchCursor: *mut *const i8) -> u32 {
    unsafe {
        let mut nVal: u32 = 0;
        let mut chNow: i8 = 0;
        loop {
            chNow = (**pchCursor as i32 & 0x5fi32) as i8;
            if !(chNow as i32 >= '0' as i32 & 0x5f && chNow as i32 <= '9' as i32 & 0x5f
                || chNow as i32 >= 'A' as i32 && chNow as i32 <= 'F' as i32)
            {
                break;
            }
            let mut nybbleValue: u8 = 0;
            chNow = (chNow as i32 - 0x10i32) as i8;
            nybbleValue = (if chNow as i32 > 9i32 {
                chNow as i32 - (0x31 - 0xa)
            } else {
                chNow as i32
            }) as u8;
            nVal <<= 4;
            nVal = nVal.wrapping_add(nybbleValue as u32);
            *pchCursor = (*pchCursor).offset(1);
            *pchCursor;
        }
        return nVal;
    }
}

#[no_mangle]
pub extern "C" fn ParseIPv4OrIPv6(
    mut ppszText: *mut *const i8,
    mut abyAddr: *mut u8,
    mut pnPort: *mut i32,
    mut pbIsIPv6: *mut i32,
) -> i32 {
    unsafe {
        let mut abyAddrLocal: *mut u8 = 0 as *mut u8;
        let mut abyDummyAddr: [u8; 16] = [0; 16];
        let mut pchColon: *const i8 = strchr(*ppszText, ':' as i32);
        let mut pchDot: *const i8 = strchr(*ppszText, '.' as i32);
        let mut pchOpenBracket: *const i8 = strchr(*ppszText, '[' as i32);
        let mut pchCloseBracket: *const i8 = 0 as *const i8;
        let mut bIsIPv6local: i32 = (!pchOpenBracket.is_null()
            || pchDot.is_null()
            || !pchColon.is_null() && (pchDot.is_null() || pchColon < pchDot))
            as i32;
        if bIsIPv6local != 0 {
            pchCloseBracket = strchr(*ppszText, ']' as i32);
            if !pchOpenBracket.is_null()
                && (pchCloseBracket.is_null() || pchCloseBracket < pchOpenBracket)
            {
                return 0;
            }
        } else if pchDot.is_null() || !pchColon.is_null() && pchColon < pchDot {
            return 0;
        }
        if !pbIsIPv6.is_null() {
            *pbIsIPv6 = bIsIPv6local;
        }
        abyAddrLocal = abyAddr;
        if abyAddrLocal.is_null() {
            abyAddrLocal = abyDummyAddr.as_mut_ptr();
        }
        if bIsIPv6local == 0 {
            let mut pbyAddrCursor: *mut u8 = abyAddrLocal;
            let mut nVal: u32 = 0;
            let mut pszTextBefore: *const i8 = *ppszText;
            nVal = _parseDecimal(ppszText);
            if '.' as i32 != **ppszText as i32 || nVal > 255 || pszTextBefore == *ppszText {
                return 0;
            }
            let fresh0 = pbyAddrCursor;
            pbyAddrCursor = pbyAddrCursor.offset(1);
            *fresh0 = nVal as u8;
            *ppszText = (*ppszText).offset(1);
            *ppszText;
            pszTextBefore = *ppszText;
            nVal = _parseDecimal(ppszText);
            if '.' as i32 != **ppszText as i32 || nVal > 255 || pszTextBefore == *ppszText {
                return 0;
            }
            let fresh1 = pbyAddrCursor;
            pbyAddrCursor = pbyAddrCursor.offset(1);
            *fresh1 = nVal as u8;
            *ppszText = (*ppszText).offset(1);
            *ppszText;
            pszTextBefore = *ppszText;
            nVal = _parseDecimal(ppszText);
            if '.' as i32 != **ppszText as i32 || nVal > 255 || pszTextBefore == *ppszText {
                return 0;
            }
            let fresh2 = pbyAddrCursor;
            pbyAddrCursor = pbyAddrCursor.offset(1);
            *fresh2 = nVal as u8;
            *ppszText = (*ppszText).offset(1);
            *ppszText;
            pszTextBefore = *ppszText;
            nVal = _parseDecimal(ppszText);
            if nVal > 255 || pszTextBefore == *ppszText {
                return 0;
            }
            let fresh3 = pbyAddrCursor;
            pbyAddrCursor = pbyAddrCursor.offset(1);
            *fresh3 = nVal as u8;
            if ':' as i32 == **ppszText as i32 && !pnPort.is_null() {
                let mut usPortNetwork: u16 = 0;
                *ppszText = (*ppszText).offset(1);
                *ppszText;
                pszTextBefore = *ppszText;
                nVal = _parseDecimal(ppszText);
                if nVal > 65535 || pszTextBefore == *ppszText {
                    return 0;
                };
                *(&mut usPortNetwork as *mut u16 as *mut u8).offset(0 as isize) =
                    ((nVal & 0xff00u32) >> 8) as u8;
                *(&mut usPortNetwork as *mut u16 as *mut u8).offset(1 as isize) =
                    (nVal & 0xffu32) as u8;
                *pnPort = usPortNetwork as i32;
                return 1;
            } else {
                if !pnPort.is_null() {
                    *pnPort = 0;
                }
                return 1;
            }
        } else {
            let mut pbyAddrCursor_0: *mut u8 = 0 as *mut u8;
            let mut pbyZerosLoc: *mut u8 = 0 as *mut u8;
            let mut bIPv4Detected: i32 = 0;
            let mut nIdx: i32 = 0;
            if !pchOpenBracket.is_null() {
                *ppszText = pchOpenBracket.offset(1 as isize);
            }
            pbyAddrCursor_0 = abyAddrLocal;
            pbyZerosLoc = 0 as *mut u8;
            bIPv4Detected = 0;
            nIdx = 0;
            while nIdx < 8 {
                let mut pszTextBefore_0: *const i8 = *ppszText;
                let mut nVal_0: u32 = _parseHex(ppszText);
                if pszTextBefore_0 == *ppszText {
                    if !pbyZerosLoc.is_null() {
                        if pbyZerosLoc == pbyAddrCursor_0 {
                            nIdx -= 1;
                            nIdx;
                            break;
                        } else {
                            return 0;
                        }
                    } else {
                        if ':' as i32 != **ppszText as i32 {
                            return 0;
                        }
                        if 0 == nIdx {
                            *ppszText = (*ppszText).offset(1);
                            *ppszText;
                            if ':' as i32 != **ppszText as i32 {
                                return 0;
                            }
                        }
                        pbyZerosLoc = pbyAddrCursor_0;
                        *ppszText = (*ppszText).offset(1);
                        *ppszText;
                    }
                } else if '.' as i32 == **ppszText as i32 {
                    let mut pszTextlocal: *const i8 = pszTextBefore_0;
                    let mut abyAddrlocal: [u8; 16] = [0; 16];
                    let mut bIsIPv6local_0: i32 = 0;
                    let mut bParseResultlocal: i32 = ParseIPv4OrIPv6(
                        &mut pszTextlocal,
                        abyAddrlocal.as_mut_ptr(),
                        0 as *mut i32,
                        &mut bIsIPv6local_0,
                    );
                    *ppszText = pszTextlocal;
                    if bParseResultlocal == 0 || bIsIPv6local_0 != 0 {
                        return 0;
                    }
                    let fresh4 = pbyAddrCursor_0;
                    pbyAddrCursor_0 = pbyAddrCursor_0.offset(1);
                    *fresh4 = abyAddrlocal[0 as usize];
                    let fresh5 = pbyAddrCursor_0;
                    pbyAddrCursor_0 = pbyAddrCursor_0.offset(1);
                    *fresh5 = abyAddrlocal[1 as usize];
                    let fresh6 = pbyAddrCursor_0;
                    pbyAddrCursor_0 = pbyAddrCursor_0.offset(1);
                    *fresh6 = abyAddrlocal[2 as usize];
                    let fresh7 = pbyAddrCursor_0;
                    pbyAddrCursor_0 = pbyAddrCursor_0.offset(1);
                    *fresh7 = abyAddrlocal[3 as usize];
                    nIdx += 1;
                    nIdx;
                    bIPv4Detected = 1;
                    break;
                } else {
                    if nVal_0 > 65535 {
                        return 0;
                    }
                    let fresh8 = pbyAddrCursor_0;
                    pbyAddrCursor_0 = pbyAddrCursor_0.offset(1);
                    *fresh8 = (nVal_0 >> 8i32) as u8;
                    let fresh9 = pbyAddrCursor_0;
                    pbyAddrCursor_0 = pbyAddrCursor_0.offset(1);
                    *fresh9 = (nVal_0 & 0xffu32) as u8;
                    if !(':' as i32 == **ppszText as i32) {
                        break;
                    }
                    *ppszText = (*ppszText).offset(1);
                    *ppszText;
                }
                nIdx += 1;
                nIdx;
            }
            if !pbyZerosLoc.is_null() {
                let mut nHead: i32 = pbyZerosLoc.offset_from(abyAddrLocal) as i32;
                let mut nTail: i32 = nIdx * 2 - pbyZerosLoc.offset_from(abyAddrLocal) as i32;
                let mut nZeros: i32 = 16 - nTail - nHead;
                memmove(
                    &mut *abyAddrLocal.offset((16 - nTail) as isize) as *mut u8
                        as *mut libc::c_void,
                    pbyZerosLoc as *const libc::c_void,
                    nTail as u64,
                );
                memset(pbyZerosLoc as *mut libc::c_void, 0, nZeros as u64);
            }
            if bIPv4Detected != 0 {
                static mut abyPfx: [u8; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0xff];
                if 0 != memcmp(
                    abyAddrLocal as *const libc::c_void,
                    abyPfx.as_ptr() as *const libc::c_void,
                    ::core::mem::size_of::<[u8; 12]>() as u64,
                ) {
                    return 0;
                }
            }
            if !pchOpenBracket.is_null() {
                if ']' as i32 != **ppszText as i32 {
                    return 0;
                }
                *ppszText = (*ppszText).offset(1);
                *ppszText;
            }
            if ':' as i32 == **ppszText as i32 && !pnPort.is_null() {
                let mut pszTextBefore_1: *const i8 = 0 as *const i8;
                let mut nVal_1: u32 = 0;
                let mut usPortNetwork_0: u16 = 0;
                *ppszText = (*ppszText).offset(1);
                *ppszText;
                pszTextBefore_1 = *ppszText;
                pszTextBefore_1 = *ppszText;
                nVal_1 = _parseDecimal(ppszText);
                if nVal_1 > 65535 || pszTextBefore_1 == *ppszText {
                    return 0;
                };
                *(&mut usPortNetwork_0 as *mut u16 as *mut u8).offset(0 as isize) =
                    ((nVal_1 & 0xff00u32) >> 8) as u8;
                *(&mut usPortNetwork_0 as *mut u16 as *mut u8).offset(1 as isize) =
                    (nVal_1 & 0xffu32) as u8;
                *pnPort = usPortNetwork_0 as i32;
                return 1;
            } else {
                if !pnPort.is_null() {
                    *pnPort = 0;
                }
                return 1;
            }
        };
    }
}

#[no_mangle]
pub extern "C" fn ParseIPv4OrIPv6_2(
    mut pszText: *const i8,
    mut abyAddr: *mut u8,
    mut pnPort: *mut i32,
    mut pbIsIPv6: *mut i32,
) -> i32 {
    unsafe {
        let mut pszTextLocal: *const i8 = pszText;
        return ParseIPv4OrIPv6(&mut pszTextLocal, abyAddr, pnPort, pbIsIPv6);
    }
}

#[no_mangle]
pub extern "C" fn htons(mut us: u16) -> u16 {
    unsafe {
        return (((*(&mut us as *mut u16 as *mut u8).offset(0 as isize) as i32) << 8i32)
            + *(&mut us as *mut u16 as *mut u8).offset(1 as isize) as i32) as u16;
    }
}

#[no_mangle]
pub extern "C" fn dumpbin(mut pbyBin: *mut u8, mut nLen: i32) {
    unsafe {
        let mut i: i32 = 0;
        i = 0;
        while i < nLen {
            print!("{:02x}", *pbyBin.offset(i as isize) as i32);
            i += 1;
            i;
        }
    }
}

#[no_mangle]
pub extern "C" fn testcase(mut pszTest: *const i8) {
    unsafe {
        let mut abyAddr: [u8; 16] = [0; 16];
        let mut bIsIPv6: i32 = 0;
        let mut nPort: i32 = 0;
        let mut bSuccess: i32 = 0;
        print!("Test case {}\n", build_str_from_raw_ptr(pszTest as *mut u8));
        let mut pszTextCursor: *const i8 = pszTest;
        bSuccess = ParseIPv4OrIPv6(
            &mut pszTextCursor,
            abyAddr.as_mut_ptr(),
            &mut nPort,
            &mut bIsIPv6,
        );
        if bSuccess == 0 {
            print!(
                "parse failed, at about index {}; rest: {}\n",
                pszTextCursor.offset_from(pszTest) as i64,
                build_str_from_raw_ptr(pszTextCursor as *mut u8)
            );
            return;
        }
        print!("addr:  ");
        dumpbin(abyAddr.as_mut_ptr(), if bIsIPv6 != 0 { 16 } else { 4 });
        print!("\n");
        if 0 == nPort {
            print!("port absent");
        } else {
            print!("port:  {}", htons(nPort as u16) as i32);
        }
        print!("\n\n");
    }
}

fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    unsafe {
        testcase(b"127.0.0.1\0" as *const u8 as *const i8);
        testcase(b"127.0.0.1:80\0" as *const u8 as *const i8);
        testcase(b"::1\0" as *const u8 as *const i8);
        testcase(b"[::1]:80\0" as *const u8 as *const i8);
        testcase(b"2605:2700:0:3::4713:93e3\0" as *const u8 as *const i8);
        testcase(b"[2605:2700:0:3::4713:93e3]:80\0" as *const u8 as *const i8);
        testcase(b"::ffff:192.168.173.22\0" as *const u8 as *const i8);
        testcase(b"[::ffff:192.168.173.22]:80\0" as *const u8 as *const i8);
        testcase(b"1::\0" as *const u8 as *const i8);
        testcase(b"[1::]:80\0" as *const u8 as *const i8);
        testcase(b"::\0" as *const u8 as *const i8);
        testcase(b"[::]:80\0" as *const u8 as *const i8);
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
