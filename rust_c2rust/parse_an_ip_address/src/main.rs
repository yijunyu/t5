#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
}
unsafe extern "C" fn _parseDecimal(
    mut pchCursor: *mut *const libc::c_char,
) -> libc::c_uint {
    let mut nVal: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut chNow: libc::c_char = 0;
    loop {
        chNow = **pchCursor;
        if !(chNow as libc::c_int >= '0' as i32 && chNow as libc::c_int <= '9' as i32) {
            break;
        }
        nVal = nVal.wrapping_mul(10 as libc::c_int as libc::c_uint);
        nVal = nVal.wrapping_add((chNow as libc::c_int - '0' as i32) as libc::c_uint);
        *pchCursor = (*pchCursor).offset(1);
        *pchCursor;
    }
    return nVal;
}
unsafe extern "C" fn _parseHex(mut pchCursor: *mut *const libc::c_char) -> libc::c_uint {
    let mut nVal: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut chNow: libc::c_char = 0;
    loop {
        chNow = (**pchCursor as libc::c_int & 0x5f as libc::c_int) as libc::c_char;
        if !(chNow as libc::c_int >= '0' as i32 & 0x5f as libc::c_int
            && chNow as libc::c_int <= '9' as i32 & 0x5f as libc::c_int
            || chNow as libc::c_int >= 'A' as i32 && chNow as libc::c_int <= 'F' as i32)
        {
            break;
        }
        let mut nybbleValue: libc::c_uchar = 0;
        chNow = (chNow as libc::c_int - 0x10 as libc::c_int) as libc::c_char;
        nybbleValue = (if chNow as libc::c_int > 9 as libc::c_int {
            chNow as libc::c_int - (0x31 as libc::c_int - 0xa as libc::c_int)
        } else {
            chNow as libc::c_int
        }) as libc::c_uchar;
        nVal <<= 4 as libc::c_int;
        nVal = nVal.wrapping_add(nybbleValue as libc::c_uint);
        *pchCursor = (*pchCursor).offset(1);
        *pchCursor;
    }
    return nVal;
}
#[no_mangle]
pub unsafe extern "C" fn ParseIPv4OrIPv6(
    mut ppszText: *mut *const libc::c_char,
    mut abyAddr: *mut libc::c_uchar,
    mut pnPort: *mut libc::c_int,
    mut pbIsIPv6: *mut libc::c_int,
) -> libc::c_int {
    let mut abyAddrLocal: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut abyDummyAddr: [libc::c_uchar; 16] = [0; 16];
    let mut pchColon: *const libc::c_char = strchr(*ppszText, ':' as i32);
    let mut pchDot: *const libc::c_char = strchr(*ppszText, '.' as i32);
    let mut pchOpenBracket: *const libc::c_char = strchr(*ppszText, '[' as i32);
    let mut pchCloseBracket: *const libc::c_char = 0 as *const libc::c_char;
    let mut bIsIPv6local: libc::c_int = (!pchOpenBracket.is_null() || pchDot.is_null()
        || !pchColon.is_null() && (pchDot.is_null() || pchColon < pchDot))
        as libc::c_int;
    if bIsIPv6local != 0 {
        pchCloseBracket = strchr(*ppszText, ']' as i32);
        if !pchOpenBracket.is_null()
            && (pchCloseBracket.is_null() || pchCloseBracket < pchOpenBracket)
        {
            return 0 as libc::c_int;
        }
    } else if pchDot.is_null() || !pchColon.is_null() && pchColon < pchDot {
        return 0 as libc::c_int
    }
    if !pbIsIPv6.is_null() {
        *pbIsIPv6 = bIsIPv6local;
    }
    abyAddrLocal = abyAddr;
    if abyAddrLocal.is_null() {
        abyAddrLocal = abyDummyAddr.as_mut_ptr();
    }
    if bIsIPv6local == 0 {
        let mut pbyAddrCursor: *mut libc::c_uchar = abyAddrLocal;
        let mut nVal: libc::c_uint = 0;
        let mut pszTextBefore: *const libc::c_char = *ppszText;
        nVal = _parseDecimal(ppszText);
        if '.' as i32 != **ppszText as libc::c_int
            || nVal > 255 as libc::c_int as libc::c_uint || pszTextBefore == *ppszText
        {
            return 0 as libc::c_int;
        }
        let fresh0 = pbyAddrCursor;
        pbyAddrCursor = pbyAddrCursor.offset(1);
        *fresh0 = nVal as libc::c_uchar;
        *ppszText = (*ppszText).offset(1);
        *ppszText;
        pszTextBefore = *ppszText;
        nVal = _parseDecimal(ppszText);
        if '.' as i32 != **ppszText as libc::c_int
            || nVal > 255 as libc::c_int as libc::c_uint || pszTextBefore == *ppszText
        {
            return 0 as libc::c_int;
        }
        let fresh1 = pbyAddrCursor;
        pbyAddrCursor = pbyAddrCursor.offset(1);
        *fresh1 = nVal as libc::c_uchar;
        *ppszText = (*ppszText).offset(1);
        *ppszText;
        pszTextBefore = *ppszText;
        nVal = _parseDecimal(ppszText);
        if '.' as i32 != **ppszText as libc::c_int
            || nVal > 255 as libc::c_int as libc::c_uint || pszTextBefore == *ppszText
        {
            return 0 as libc::c_int;
        }
        let fresh2 = pbyAddrCursor;
        pbyAddrCursor = pbyAddrCursor.offset(1);
        *fresh2 = nVal as libc::c_uchar;
        *ppszText = (*ppszText).offset(1);
        *ppszText;
        pszTextBefore = *ppszText;
        nVal = _parseDecimal(ppszText);
        if nVal > 255 as libc::c_int as libc::c_uint || pszTextBefore == *ppszText {
            return 0 as libc::c_int;
        }
        let fresh3 = pbyAddrCursor;
        pbyAddrCursor = pbyAddrCursor.offset(1);
        *fresh3 = nVal as libc::c_uchar;
        if ':' as i32 == **ppszText as libc::c_int && !pnPort.is_null() {
            let mut usPortNetwork: libc::c_ushort = 0;
            *ppszText = (*ppszText).offset(1);
            *ppszText;
            pszTextBefore = *ppszText;
            nVal = _parseDecimal(ppszText);
            if nVal > 65535 as libc::c_int as libc::c_uint || pszTextBefore == *ppszText
            {
                return 0 as libc::c_int;
            }
            *(&mut usPortNetwork as *mut libc::c_ushort as *mut libc::c_uchar)
                .offset(
                    0 as libc::c_int as isize,
                ) = ((nVal & 0xff00 as libc::c_int as libc::c_uint) >> 8 as libc::c_int)
                as libc::c_uchar;
            *(&mut usPortNetwork as *mut libc::c_ushort as *mut libc::c_uchar)
                .offset(
                    1 as libc::c_int as isize,
                ) = (nVal & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
            *pnPort = usPortNetwork as libc::c_int;
            return 1 as libc::c_int;
        } else {
            if !pnPort.is_null() {
                *pnPort = 0 as libc::c_int;
            }
            return 1 as libc::c_int;
        }
    } else {
        let mut pbyAddrCursor_0: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut pbyZerosLoc: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut bIPv4Detected: libc::c_int = 0;
        let mut nIdx: libc::c_int = 0;
        if !pchOpenBracket.is_null() {
            *ppszText = pchOpenBracket.offset(1 as libc::c_int as isize);
        }
        pbyAddrCursor_0 = abyAddrLocal;
        pbyZerosLoc = 0 as *mut libc::c_uchar;
        bIPv4Detected = 0 as libc::c_int;
        nIdx = 0 as libc::c_int;
        while nIdx < 8 as libc::c_int {
            let mut pszTextBefore_0: *const libc::c_char = *ppszText;
            let mut nVal_0: libc::c_uint = _parseHex(ppszText);
            if pszTextBefore_0 == *ppszText {
                if !pbyZerosLoc.is_null() {
                    if pbyZerosLoc == pbyAddrCursor_0 {
                        nIdx -= 1;
                        nIdx;
                        break;
                    } else {
                        return 0 as libc::c_int
                    }
                } else {
                    if ':' as i32 != **ppszText as libc::c_int {
                        return 0 as libc::c_int;
                    }
                    if 0 as libc::c_int == nIdx {
                        *ppszText = (*ppszText).offset(1);
                        *ppszText;
                        if ':' as i32 != **ppszText as libc::c_int {
                            return 0 as libc::c_int;
                        }
                    }
                    pbyZerosLoc = pbyAddrCursor_0;
                    *ppszText = (*ppszText).offset(1);
                    *ppszText;
                }
            } else if '.' as i32 == **ppszText as libc::c_int {
                let mut pszTextlocal: *const libc::c_char = pszTextBefore_0;
                let mut abyAddrlocal: [libc::c_uchar; 16] = [0; 16];
                let mut bIsIPv6local_0: libc::c_int = 0;
                let mut bParseResultlocal: libc::c_int = ParseIPv4OrIPv6(
                    &mut pszTextlocal,
                    abyAddrlocal.as_mut_ptr(),
                    0 as *mut libc::c_int,
                    &mut bIsIPv6local_0,
                );
                *ppszText = pszTextlocal;
                if bParseResultlocal == 0 || bIsIPv6local_0 != 0 {
                    return 0 as libc::c_int;
                }
                let fresh4 = pbyAddrCursor_0;
                pbyAddrCursor_0 = pbyAddrCursor_0.offset(1);
                *fresh4 = abyAddrlocal[0 as libc::c_int as usize];
                let fresh5 = pbyAddrCursor_0;
                pbyAddrCursor_0 = pbyAddrCursor_0.offset(1);
                *fresh5 = abyAddrlocal[1 as libc::c_int as usize];
                let fresh6 = pbyAddrCursor_0;
                pbyAddrCursor_0 = pbyAddrCursor_0.offset(1);
                *fresh6 = abyAddrlocal[2 as libc::c_int as usize];
                let fresh7 = pbyAddrCursor_0;
                pbyAddrCursor_0 = pbyAddrCursor_0.offset(1);
                *fresh7 = abyAddrlocal[3 as libc::c_int as usize];
                nIdx += 1;
                nIdx;
                bIPv4Detected = 1 as libc::c_int;
                break;
            } else {
                if nVal_0 > 65535 as libc::c_int as libc::c_uint {
                    return 0 as libc::c_int;
                }
                let fresh8 = pbyAddrCursor_0;
                pbyAddrCursor_0 = pbyAddrCursor_0.offset(1);
                *fresh8 = (nVal_0 >> 8 as libc::c_int) as libc::c_uchar;
                let fresh9 = pbyAddrCursor_0;
                pbyAddrCursor_0 = pbyAddrCursor_0.offset(1);
                *fresh9 = (nVal_0 & 0xff as libc::c_int as libc::c_uint)
                    as libc::c_uchar;
                if !(':' as i32 == **ppszText as libc::c_int) {
                    break;
                }
                *ppszText = (*ppszText).offset(1);
                *ppszText;
            }
            nIdx += 1;
            nIdx;
        }
        if !pbyZerosLoc.is_null() {
            let mut nHead: libc::c_int = pbyZerosLoc.offset_from(abyAddrLocal)
                as libc::c_long as libc::c_int;
            let mut nTail: libc::c_int = nIdx * 2 as libc::c_int
                - pbyZerosLoc.offset_from(abyAddrLocal) as libc::c_long as libc::c_int;
            let mut nZeros: libc::c_int = 16 as libc::c_int - nTail - nHead;
            memmove(
                &mut *abyAddrLocal.offset((16 as libc::c_int - nTail) as isize)
                    as *mut libc::c_uchar as *mut libc::c_void,
                pbyZerosLoc as *const libc::c_void,
                nTail as libc::c_ulong,
            );
            memset(
                pbyZerosLoc as *mut libc::c_void,
                0 as libc::c_int,
                nZeros as libc::c_ulong,
            );
        }
        if bIPv4Detected != 0 {
            static mut abyPfx: [libc::c_uchar; 12] = [
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0xff as libc::c_int as libc::c_uchar,
                0xff as libc::c_int as libc::c_uchar,
            ];
            if 0 as libc::c_int
                != memcmp(
                    abyAddrLocal as *const libc::c_void,
                    abyPfx.as_ptr() as *const libc::c_void,
                    ::core::mem::size_of::<[libc::c_uchar; 12]>() as libc::c_ulong,
                )
            {
                return 0 as libc::c_int;
            }
        }
        if !pchOpenBracket.is_null() {
            if ']' as i32 != **ppszText as libc::c_int {
                return 0 as libc::c_int;
            }
            *ppszText = (*ppszText).offset(1);
            *ppszText;
        }
        if ':' as i32 == **ppszText as libc::c_int && !pnPort.is_null() {
            let mut pszTextBefore_1: *const libc::c_char = 0 as *const libc::c_char;
            let mut nVal_1: libc::c_uint = 0;
            let mut usPortNetwork_0: libc::c_ushort = 0;
            *ppszText = (*ppszText).offset(1);
            *ppszText;
            pszTextBefore_1 = *ppszText;
            pszTextBefore_1 = *ppszText;
            nVal_1 = _parseDecimal(ppszText);
            if nVal_1 > 65535 as libc::c_int as libc::c_uint
                || pszTextBefore_1 == *ppszText
            {
                return 0 as libc::c_int;
            }
            *(&mut usPortNetwork_0 as *mut libc::c_ushort as *mut libc::c_uchar)
                .offset(
                    0 as libc::c_int as isize,
                ) = ((nVal_1 & 0xff00 as libc::c_int as libc::c_uint)
                >> 8 as libc::c_int) as libc::c_uchar;
            *(&mut usPortNetwork_0 as *mut libc::c_ushort as *mut libc::c_uchar)
                .offset(
                    1 as libc::c_int as isize,
                ) = (nVal_1 & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
            *pnPort = usPortNetwork_0 as libc::c_int;
            return 1 as libc::c_int;
        } else {
            if !pnPort.is_null() {
                *pnPort = 0 as libc::c_int;
            }
            return 1 as libc::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ParseIPv4OrIPv6_2(
    mut pszText: *const libc::c_char,
    mut abyAddr: *mut libc::c_uchar,
    mut pnPort: *mut libc::c_int,
    mut pbIsIPv6: *mut libc::c_int,
) -> libc::c_int {
    let mut pszTextLocal: *const libc::c_char = pszText;
    return ParseIPv4OrIPv6(&mut pszTextLocal, abyAddr, pnPort, pbIsIPv6);
}
#[no_mangle]
pub unsafe extern "C" fn htons(mut us: libc::c_ushort) -> libc::c_ushort {
    return (((*(&mut us as *mut libc::c_ushort as *mut libc::c_uchar)
        .offset(0 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int)
        + *(&mut us as *mut libc::c_ushort as *mut libc::c_uchar)
            .offset(1 as libc::c_int as isize) as libc::c_int) as libc::c_ushort;
}
#[no_mangle]
pub unsafe extern "C" fn dumpbin(mut pbyBin: *mut libc::c_uchar, mut nLen: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < nLen {
        printf(
            b"%02x\0" as *const u8 as *const libc::c_char,
            *pbyBin.offset(i as isize) as libc::c_int,
        );
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn testcase(mut pszTest: *const libc::c_char) {
    let mut abyAddr: [libc::c_uchar; 16] = [0; 16];
    let mut bIsIPv6: libc::c_int = 0;
    let mut nPort: libc::c_int = 0;
    let mut bSuccess: libc::c_int = 0;
    printf(b"Test case '%s'\n\0" as *const u8 as *const libc::c_char, pszTest);
    let mut pszTextCursor: *const libc::c_char = pszTest;
    bSuccess = ParseIPv4OrIPv6(
        &mut pszTextCursor,
        abyAddr.as_mut_ptr(),
        &mut nPort,
        &mut bIsIPv6,
    );
    if bSuccess == 0 {
        printf(
            b"parse failed, at about index %d; rest: '%s'\n\0" as *const u8
                as *const libc::c_char,
            pszTextCursor.offset_from(pszTest) as libc::c_long,
            pszTextCursor,
        );
        return;
    }
    printf(b"addr:  \0" as *const u8 as *const libc::c_char);
    dumpbin(
        abyAddr.as_mut_ptr(),
        if bIsIPv6 != 0 { 16 as libc::c_int } else { 4 as libc::c_int },
    );
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int == nPort {
        printf(b"port absent\0" as *const u8 as *const libc::c_char);
    } else {
        printf(
            b"port:  %d\0" as *const u8 as *const libc::c_char,
            htons(nPort as libc::c_ushort) as libc::c_int,
        );
    }
    printf(b"\n\n\0" as *const u8 as *const libc::c_char);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    testcase(b"127.0.0.1\0" as *const u8 as *const libc::c_char);
    testcase(b"127.0.0.1:80\0" as *const u8 as *const libc::c_char);
    testcase(b"::1\0" as *const u8 as *const libc::c_char);
    testcase(b"[::1]:80\0" as *const u8 as *const libc::c_char);
    testcase(b"2605:2700:0:3::4713:93e3\0" as *const u8 as *const libc::c_char);
    testcase(b"[2605:2700:0:3::4713:93e3]:80\0" as *const u8 as *const libc::c_char);
    testcase(b"::ffff:192.168.173.22\0" as *const u8 as *const libc::c_char);
    testcase(b"[::ffff:192.168.173.22]:80\0" as *const u8 as *const libc::c_char);
    testcase(b"1::\0" as *const u8 as *const libc::c_char);
    testcase(b"[1::]:80\0" as *const u8 as *const libc::c_char);
    testcase(b"::\0" as *const u8 as *const libc::c_char);
    testcase(b"[::]:80\0" as *const u8 as *const libc::c_char);
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
