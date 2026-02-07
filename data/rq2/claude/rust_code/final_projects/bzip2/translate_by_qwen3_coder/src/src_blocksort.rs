//! Module: src_blocksort
//!
//! Auto-generated skeleton - function bodies are unimplemented.

#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::types::*;
use crate::globals::*;
use crate::compat::*;

// === C2R_FILE_STATICS_BEGIN ===
// File-scope `static` variables (internal linkage) from the original C TU.
// These are module-local by design (Scheme B).
/// C: static Int32[14] incs
static mut incs: [crate::types::Int32; 14usize] = unsafe { core::mem::MaybeUninit::<[crate::types::Int32; 14usize]>::zeroed().assume_init() };

// === C2R_FILE_STATICS_END ===

fn fallbackSimpleSort(fmap: *mut crate::types::UInt32, eclass: *mut crate::types::UInt32, lo: crate::types::Int32, hi: crate::types::Int32) {
    let mut i: crate::types::Int32;
    let mut j: crate::types::Int32;
    let mut tmp: crate::types::Int32;
    let mut ec_tmp: crate::types::UInt32;

    if lo == hi {
        return;
    }

    if hi - lo > 3 {
        i = hi - 4;
        while i >= lo {
            unsafe {
                tmp = *fmap.offset(i as isize) as crate::types::Int32;
                ec_tmp = *eclass.offset(tmp as isize);
                j = i + 4;
                while j <= hi && ec_tmp > *eclass.offset(*fmap.offset(j as isize) as isize) {
                    *fmap.offset((j - 4) as isize) = *fmap.offset(j as isize);
                    j += 4;
                }
                *fmap.offset((j - 4) as isize) = tmp as crate::types::UInt32;
            }
            i -= 1;
        }
    }

    i = hi - 1;
    while i >= lo {
        unsafe {
            tmp = *fmap.offset(i as isize) as crate::types::Int32;
            ec_tmp = *eclass.offset(tmp as isize);
            j = i + 1;
            while j <= hi && ec_tmp > *eclass.offset(*fmap.offset(j as isize) as isize) {
                *fmap.offset((j - 1) as isize) = *fmap.offset(j as isize);
                j += 1;
            }
            *fmap.offset((j - 1) as isize) = tmp as crate::types::UInt32;
        }
        i -= 1;
    }
}

fn fswap(zz1: *mut crate::types::UInt32, zz2: *mut crate::types::UInt32) {
    unsafe {
        let zztmp: crate::types::UInt32 = *zz1;
        *zz1 = *zz2;
        *zz2 = zztmp;
    }
}

fn fvswap(zzp1: crate::types::Int32, zzp2: crate::types::Int32, zzn: crate::types::Int32, fmap: *mut crate::types::UInt32) {
    let mut yyp1 = zzp1;
    let mut yyp2 = zzp2;
    let mut yyn = zzn;
    while yyn > 0 {
        unsafe {
            crate::src_blocksort::fswap(
                fmap.offset(yyp1 as isize),
                fmap.offset(yyp2 as isize),
            );
        }
        yyp1 += 1;
        yyp2 += 1;
        yyn -= 1;
    }
}

fn fmin(a: crate::types::Int32, b: crate::types::Int32) -> crate::types::Int32 {
    if a < b { a } else { b }
}

fn fpush(stackLo: *mut crate::types::Int32, stackHi: *mut crate::types::Int32, sp: *mut crate::types::Int32, lz: crate::types::Int32, hz: crate::types::Int32) {
    unsafe {
        *stackLo.offset(*sp as isize) = lz;
        *stackHi.offset(*sp as isize) = hz;
        *sp += 1;
    }
}

fn fpop(stackLo: *mut crate::types::Int32, stackHi: *mut crate::types::Int32, sp: *mut crate::types::Int32, lz: *mut crate::types::Int32, hz: *mut crate::types::Int32) {
    unsafe {
        *sp -= 1;
        *lz = *stackLo.offset(*sp as isize);
        *hz = *stackHi.offset(*sp as isize);
    }
}

fn fallbackQSort3(fmap: *mut crate::types::UInt32, eclass: *mut crate::types::UInt32, loSt: crate::types::Int32, hiSt: crate::types::Int32) {
    let mut unLo: crate::types::Int32;
    let mut unHi: crate::types::Int32;
    let mut ltLo: crate::types::Int32;
    let mut gtHi: crate::types::Int32;
    let mut n: crate::types::Int32;
    let mut m: crate::types::Int32;
    let mut sp: crate::types::Int32;
    let mut lo: crate::types::Int32 = 0;
    let mut hi: crate::types::Int32 = 0;
    let mut med: crate::types::UInt32;
    let mut r: crate::types::UInt32;
    let mut r3: crate::types::UInt32;
    let mut stackLo: [crate::types::Int32; 100] = [0; 100];
    let mut stackHi: [crate::types::Int32; 100] = [0; 100];

    r = 0;
    sp = 0;

    unsafe {
        crate::src_blocksort::fpush(stackLo.as_mut_ptr(), stackHi.as_mut_ptr(), &mut sp, loSt, hiSt);

        while sp > 0 {
            if !(sp < FALLBACK_QSORT_STACK_SIZE - 1) {
                crate::src_bzlib::BZ2_bz__AssertH__fail(1004);
            }

            crate::src_blocksort::fpop(stackLo.as_mut_ptr(), stackHi.as_mut_ptr(), &mut sp, &mut lo, &mut hi);

            if hi - lo < FALLBACK_QSORT_SMALL_THRESH {
                crate::src_blocksort::fallbackSimpleSort(fmap, eclass, lo, hi);
                continue;
            }

            r = ((r * 7621) + 1) % 32768;
            r3 = r % 3;
            if r3 == 0 {
                med = *eclass.offset(*fmap.offset(lo as isize) as isize);
            } else if r3 == 1 {
                med = *eclass.offset(*fmap.offset(((lo + hi) >> 1) as isize) as isize);
            } else {
                med = *eclass.offset(*fmap.offset(hi as isize) as isize);
            }

            unLo = lo;
            ltLo = lo;
            unHi = hi;
            gtHi = hi;

            loop {
                loop {
                    if unLo > unHi {
                        break;
                    }
                    n = (*eclass.offset(*fmap.offset(unLo as isize) as isize) as crate::types::Int32) - (med as crate::types::Int32);
                    if n == 0 {
                        crate::src_blocksort::fswap(fmap.offset(unLo as isize), fmap.offset(ltLo as isize));
                        ltLo += 1;
                        unLo += 1;
                        continue;
                    }
                    if n > 0 {
                        break;
                    }
                    unLo += 1;
                }
                loop {
                    if unLo > unHi {
                        break;
                    }
                    n = (*eclass.offset(*fmap.offset(unHi as isize) as isize) as crate::types::Int32) - (med as crate::types::Int32);
                    if n == 0 {
                        crate::src_blocksort::fswap(fmap.offset(unHi as isize), fmap.offset(gtHi as isize));
                        gtHi -= 1;
                        unHi -= 1;
                        continue;
                    }
                    if n < 0 {
                        break;
                    }
                    unHi -= 1;
                }
                if unLo > unHi {
                    break;
                }
                crate::src_blocksort::fswap(fmap.offset(unLo as isize), fmap.offset(unHi as isize));
                unLo += 1;
                unHi -= 1;
            }

            if gtHi < ltLo {
                continue;
            }

            n = crate::src_blocksort::fmin(ltLo - lo, unLo - ltLo);
            crate::src_blocksort::fvswap(lo, unLo - n, n, fmap);
            m = crate::src_blocksort::fmin(hi - gtHi, gtHi - unHi);
            crate::src_blocksort::fvswap(unLo, hi - m + 1, m, fmap);

            n = lo + unLo - ltLo - 1;
            m = hi - (gtHi - unHi) + 1;

            if n - lo > hi - m {
                crate::src_blocksort::fpush(stackLo.as_mut_ptr(), stackHi.as_mut_ptr(), &mut sp, lo, n);
                crate::src_blocksort::fpush(stackLo.as_mut_ptr(), stackHi.as_mut_ptr(), &mut sp, m, hi);
            } else {
                crate::src_blocksort::fpush(stackLo.as_mut_ptr(), stackHi.as_mut_ptr(), &mut sp, m, hi);
                crate::src_blocksort::fpush(stackLo.as_mut_ptr(), stackHi.as_mut_ptr(), &mut sp, lo, n);
            }
        }
    }
}

fn SET_BH(bhtab: *mut crate::types::UInt32, zz: crate::types::Int32) {
    unsafe {
        let index = (zz >> 5) as usize;
        let bit = 1u32 << (zz & 31);
        *bhtab.add(index) |= bit;
    }
}

fn CLEAR_BH(bhtab: *mut crate::types::UInt32, zz: crate::types::Int32) {
    unsafe {
        let index = (zz >> 5) as usize;
        let bit_pos = zz & 31;
        *bhtab.add(index) &= !((1u32) << bit_pos);
    }
}

fn ISSET_BH(bhtab: *mut crate::types::UInt32, zz: crate::types::Int32) -> crate::types::Int32 {
    unsafe {
        let index = (zz >> 5) as usize;
        let shift = zz & 31;
        let mask = 1u32 << shift;
        ((*bhtab.add(index)) & mask) as crate::types::Int32
    }
}

fn WORD_BH(bhtab: *mut crate::types::UInt32, zz: crate::types::Int32) -> crate::types::UInt32 {
    unsafe { *bhtab.offset((zz >> 5) as isize) }
}

fn UNALIGNED_BH(zz: crate::types::Int32) -> crate::types::Int32 {
    zz & 0x01f
}

fn fallbackSort(fmap: *mut crate::types::UInt32, eclass: *mut crate::types::UInt32, bhtab: *mut crate::types::UInt32, nblock: crate::types::Int32, verb: crate::types::Int32) {
    let mut ftab: [crate::types::Int32; 257] = [0; 257];
    let mut ftabCopy: [crate::types::Int32; 256] = [0; 256];
    let mut H: crate::types::Int32;
    let mut i: crate::types::Int32;
    let mut j: crate::types::Int32;
    let mut k: crate::types::Int32;
    let mut l: crate::types::Int32;
    let mut r: crate::types::Int32;
    let mut cc: crate::types::Int32;
    let mut cc1: crate::types::Int32;
    let mut nNotDone: crate::types::Int32;
    let mut nBhtab: crate::types::Int32;
    let eclass8: *mut crate::types::UChar = eclass as *mut crate::types::UChar;

    unsafe {
        let stderr_ptr = crate::compat::stderr;
        if verb >= 4 {
            libc::fprintf(stderr_ptr as *mut libc::FILE, b"        bucket sorting ...\n\0".as_ptr() as *const i8);
        }

        i = 0;
        while i < 257 {
            ftab[i as usize] = 0;
            i += 1;
        }

        i = 0;
        while i < nblock {
            ftab[*eclass8.offset(i as isize) as usize] += 1;
            i += 1;
        }

        i = 0;
        while i < 256 {
            ftabCopy[i as usize] = ftab[i as usize];
            i += 1;
        }

        i = 1;
        while i < 257 {
            ftab[i as usize] += ftab[(i - 1) as usize];
            i += 1;
        }

        i = 0;
        while i < nblock {
            j = *eclass8.offset(i as isize) as crate::types::Int32;
            k = ftab[j as usize] - 1;
            ftab[j as usize] = k;
            *fmap.offset(k as isize) = i as crate::types::UInt32;
            i += 1;
        }

        nBhtab = 2 + (nblock / 32);
        i = 0;
        while i < nBhtab {
            *bhtab.offset(i as isize) = 0;
            i += 1;
        }

        i = 0;
        while i < 256 {
            crate::src_blocksort::SET_BH(bhtab, ftab[i as usize]);
            i += 1;
        }

        i = 0;
        while i < 32 {
            crate::src_blocksort::SET_BH(bhtab, nblock + 2 * i);
            crate::src_blocksort::CLEAR_BH(bhtab, nblock + 2 * i + 1);
            i += 1;
        }

        H = 1;
        loop {
            if verb >= 4 {
                libc::fprintf(stderr_ptr as *mut libc::FILE, b"        depth %6d has \0".as_ptr() as *const i8, H);
            }

            j = 0;
            i = 0;
            while i < nblock {
                if crate::src_blocksort::ISSET_BH(bhtab, i) != 0 {
                    j = i;
                }
                k = *fmap.offset(i as isize) as crate::types::Int32 - H;
                if k < 0 {
                    k += nblock;
                }
                *eclass.offset(k as isize) = j as crate::types::UInt32;
                i += 1;
            }

            nNotDone = 0;
            r = -1;
            loop {
                k = r + 1;
                while crate::src_blocksort::ISSET_BH(bhtab, k) != 0 && crate::src_blocksort::UNALIGNED_BH(k) != 0 {
                    k += 1;
                }
                if crate::src_blocksort::ISSET_BH(bhtab, k) != 0 {
                    while crate::src_blocksort::WORD_BH(bhtab, k) == 0xffffffff {
                        k += 32;
                    }
                    while crate::src_blocksort::ISSET_BH(bhtab, k) != 0 {
                        k += 1;
                    }
                }
                l = k - 1;
                if l >= nblock {
                    break;
                }
                while crate::src_blocksort::ISSET_BH(bhtab, k) == 0 && crate::src_blocksort::UNALIGNED_BH(k) != 0 {
                    k += 1;
                }
                if crate::src_blocksort::ISSET_BH(bhtab, k) == 0 {
                    while crate::src_blocksort::WORD_BH(bhtab, k) == 0x00000000 {
                        k += 32;
                    }
                    while crate::src_blocksort::ISSET_BH(bhtab, k) == 0 {
                        k += 1;
                    }
                }
                r = k - 1;
                if r >= nblock {
                    break;
                }

                if r > l {
                    nNotDone += r - l + 1;
                    crate::src_blocksort::fallbackQSort3(fmap, eclass, l, r);

                    cc = -1;
                    i = l;
                    while i <= r {
                        cc1 = *eclass.offset(*fmap.offset(i as isize) as isize) as crate::types::Int32;
                        if cc != cc1 {
                            crate::src_blocksort::SET_BH(bhtab, i);
                            cc = cc1;
                        }
                        i += 1;
                    }
                }
            }

            if verb >= 4 {
                libc::fprintf(stderr_ptr as *mut libc::FILE, b"%6d unresolved strings\n\0".as_ptr() as *const i8, nNotDone);
            }

            H *= 2;
            if H > nblock || nNotDone == 0 {
                break;
            }
        }

        if verb >= 4 {
            libc::fprintf(stderr_ptr as *mut libc::FILE, b"        reconstructing block ...\n\0".as_ptr() as *const i8);
        }

        j = 0;
        i = 0;
        while i < nblock {
            while ftabCopy[j as usize] == 0 {
                j += 1;
            }
            ftabCopy[j as usize] -= 1;
            *eclass8.offset(*fmap.offset(i as isize) as isize) = j as crate::types::UChar;
            i += 1;
        }

        if !(j < 256) {
            crate::src_bzlib::BZ2_bz__AssertH__fail(1005);
        }
    }
}

fn mainGtU(i1: crate::types::UInt32, i2: crate::types::UInt32, block: *mut crate::types::UChar, quadrant: *mut crate::types::UInt16, nblock: crate::types::UInt32, budget: *mut crate::types::Int32) -> crate::types::Bool {
    let mut i1 = i1;
    let mut i2 = i2;
    
    unsafe {
        let mut c1: crate::types::UChar;
        let mut c2: crate::types::UChar;
        let mut s1: crate::types::UInt16;
        let mut s2: crate::types::UInt16;
        
        // First 12 unrolled comparisons
        c1 = *block.offset(i1 as isize); c2 = *block.offset(i2 as isize);
        if c1 != c2 { return (c1 > c2) as crate::types::Bool; }
        i1 += 1; i2 += 1;
        
        c1 = *block.offset(i1 as isize); c2 = *block.offset(i2 as isize);
        if c1 != c2 { return (c1 > c2) as crate::types::Bool; }
        i1 += 1; i2 += 1;
        
        c1 = *block.offset(i1 as isize); c2 = *block.offset(i2 as isize);
        if c1 != c2 { return (c1 > c2) as crate::types::Bool; }
        i1 += 1; i2 += 1;
        
        c1 = *block.offset(i1 as isize); c2 = *block.offset(i2 as isize);
        if c1 != c2 { return (c1 > c2) as crate::types::Bool; }
        i1 += 1; i2 += 1;
        
        c1 = *block.offset(i1 as isize); c2 = *block.offset(i2 as isize);
        if c1 != c2 { return (c1 > c2) as crate::types::Bool; }
        i1 += 1; i2 += 1;
        
        c1 = *block.offset(i1 as isize); c2 = *block.offset(i2 as isize);
        if c1 != c2 { return (c1 > c2) as crate::types::Bool; }
        i1 += 1; i2 += 1;
        
        c1 = *block.offset(i1 as isize); c2 = *block.offset(i2 as isize);
        if c1 != c2 { return (c1 > c2) as crate::types::Bool; }
        i1 += 1; i2 += 1;
        
        c1 = *block.offset(i1 as isize); c2 = *block.offset(i2 as isize);
        if c1 != c2 { return (c1 > c2) as crate::types::Bool; }
        i1 += 1; i2 += 1;
        
        c1 = *block.offset(i1 as isize); c2 = *block.offset(i2 as isize);
        if c1 != c2 { return (c1 > c2) as crate::types::Bool; }
        i1 += 1; i2 += 1;
        
        c1 = *block.offset(i1 as isize); c2 = *block.offset(i2 as isize);
        if c1 != c2 { return (c1 > c2) as crate::types::Bool; }
        i1 += 1; i2 += 1;
        
        c1 = *block.offset(i1 as isize); c2 = *block.offset(i2 as isize);
        if c1 != c2 { return (c1 > c2) as crate::types::Bool; }
        i1 += 1; i2 += 1;
        
        c1 = *block.offset(i1 as isize); c2 = *block.offset(i2 as isize);
        if c1 != c2 { return (c1 > c2) as crate::types::Bool; }
        i1 += 1; i2 += 1;
        
        let mut k: crate::types::Int32 = nblock as crate::types::Int32 + 8;
        
        loop {
            c1 = *block.offset(i1 as isize); c2 = *block.offset(i2 as isize);
            if c1 != c2 { return (c1 > c2) as crate::types::Bool; }
            s1 = *quadrant.offset(i1 as isize); s2 = *quadrant.offset(i2 as isize);
            if s1 != s2 { return (s1 > s2) as crate::types::Bool; }
            i1 += 1; i2 += 1;
            
            c1 = *block.offset(i1 as isize); c2 = *block.offset(i2 as isize);
            if c1 != c2 { return (c1 > c2) as crate::types::Bool; }
            s1 = *quadrant.offset(i1 as isize); s2 = *quadrant.offset(i2 as isize);
            if s1 != s2 { return (s1 > s2) as crate::types::Bool; }
            i1 += 1; i2 += 1;
            
            c1 = *block.offset(i1 as isize); c2 = *block.offset(i2 as isize);
            if c1 != c2 { return (c1 > c2) as crate::types::Bool; }
            s1 = *quadrant.offset(i1 as isize); s2 = *quadrant.offset(i2 as isize);
            if s1 != s2 { return (s1 > s2) as crate::types::Bool; }
            i1 += 1; i2 += 1;
            
            c1 = *block.offset(i1 as isize); c2 = *block.offset(i2 as isize);
            if c1 != c2 { return (c1 > c2) as crate::types::Bool; }
            s1 = *quadrant.offset(i1 as isize); s2 = *quadrant.offset(i2 as isize);
            if s1 != s2 { return (s1 > s2) as crate::types::Bool; }
            i1 += 1; i2 += 1;
            
            c1 = *block.offset(i1 as isize); c2 = *block.offset(i2 as isize);
            if c1 != c2 { return (c1 > c2) as crate::types::Bool; }
            s1 = *quadrant.offset(i1 as isize); s2 = *quadrant.offset(i2 as isize);
            if s1 != s2 { return (s1 > s2) as crate::types::Bool; }
            i1 += 1; i2 += 1;
            
            c1 = *block.offset(i1 as isize); c2 = *block.offset(i2 as isize);
            if c1 != c2 { return (c1 > c2) as crate::types::Bool; }
            s1 = *quadrant.offset(i1 as isize); s2 = *quadrant.offset(i2 as isize);
            if s1 != s2 { return (s1 > s2) as crate::types::Bool; }
            i1 += 1; i2 += 1;
            
            c1 = *block.offset(i1 as isize); c2 = *block.offset(i2 as isize);
            if c1 != c2 { return (c1 > c2) as crate::types::Bool; }
            s1 = *quadrant.offset(i1 as isize); s2 = *quadrant.offset(i2 as isize);
            if s1 != s2 { return (s1 > s2) as crate::types::Bool; }
            i1 += 1; i2 += 1;
            
            c1 = *block.offset(i1 as isize); c2 = *block.offset(i2 as isize);
            if c1 != c2 { return (c1 > c2) as crate::types::Bool; }
            s1 = *quadrant.offset(i1 as isize); s2 = *quadrant.offset(i2 as isize);
            if s1 != s2 { return (s1 > s2) as crate::types::Bool; }
            i1 += 1; i2 += 1;
            
            if i1 >= nblock { i1 -= nblock; }
            if i2 >= nblock { i2 -= nblock; }
            
            k -= 8;
            *budget -= 1;
            
            if k < 0 { break; }
        }
        
        0 as crate::types::Bool
    }
}

fn mainSimpleSort(ptr: *mut crate::types::UInt32, block: *mut crate::types::UChar, quadrant: *mut crate::types::UInt16, nblock: crate::types::Int32, lo: crate::types::Int32, hi: crate::types::Int32, d: crate::types::Int32, budget: *mut crate::types::Int32) {
    unsafe {
        let bigN: crate::types::Int32 = hi - lo + 1;
        if bigN < 2 {
            return;
        }

        let incs_var: [crate::types::Int32; 14] = [
            1, 4, 13, 40, 121, 364, 1093, 3280, 9841, 29524, 88573, 265720, 797161, 2391484
        ];

        let mut hp: crate::types::Int32 = 0;
        while incs[hp as usize] < bigN {
            hp += 1;
        }
        hp -= 1;

        while hp >= 0 {
            let h: crate::types::Int32 = incs[hp as usize];
            let mut i: crate::types::Int32 = lo + h;

            loop {
                if i > hi {
                    break;
                }
                let mut v: crate::types::UInt32 = *ptr.offset(i as isize);
                let mut j: crate::types::Int32 = i;
                while crate::src_blocksort::mainGtU(
                    (*ptr.offset((j - h) as isize)).wrapping_add(d as u32),
                    v.wrapping_add(d as u32),
                    block,
                    quadrant,
                    nblock as crate::types::UInt32,
                    budget,
                ) != 0 {
                    *ptr.offset(j as isize) = *ptr.offset((j - h) as isize);
                    j = j - h;
                    if j <= (lo + h - 1) {
                        break;
                    }
                }
                *ptr.offset(j as isize) = v;
                i += 1;

                if i > hi {
                    break;
                }
                v = *ptr.offset(i as isize);
                j = i;
                while crate::src_blocksort::mainGtU(
                    (*ptr.offset((j - h) as isize)).wrapping_add(d as u32),
                    v.wrapping_add(d as u32),
                    block,
                    quadrant,
                    nblock as crate::types::UInt32,
                    budget,
                ) != 0 {
                    *ptr.offset(j as isize) = *ptr.offset((j - h) as isize);
                    j = j - h;
                    if j <= (lo + h - 1) {
                        break;
                    }
                }
                *ptr.offset(j as isize) = v;
                i += 1;

                if i > hi {
                    break;
                }
                v = *ptr.offset(i as isize);
                j = i;
                while crate::src_blocksort::mainGtU(
                    (*ptr.offset((j - h) as isize)).wrapping_add(d as u32),
                    v.wrapping_add(d as u32),
                    block,
                    quadrant,
                    nblock as crate::types::UInt32,
                    budget,
                ) != 0 {
                    *ptr.offset(j as isize) = *ptr.offset((j - h) as isize);
                    j = j - h;
                    if j <= (lo + h - 1) {
                        break;
                    }
                }
                *ptr.offset(j as isize) = v;
                i += 1;

                if *budget < 0 {
                    return;
                }
            }
            hp -= 1;
        }
    }
}

fn mswap(zz1: *mut crate::types::UInt32, zz2: *mut crate::types::UInt32) {
    unsafe {
        let zztmp = *zz1;
        *zz1 = *zz2;
        *zz2 = zztmp;
    }
}

fn mvswap(zzp1: crate::types::Int32, zzp2: crate::types::Int32, zzn: crate::types::Int32, ptr: *mut crate::types::UInt32) {
    let mut yyp1 = zzp1;
    let mut yyp2 = zzp2;
    let mut yyn = zzn;
    while yyn > 0 {
        unsafe {
            crate::src_blocksort::mswap(
                ptr.offset(yyp1 as isize),
                ptr.offset(yyp2 as isize),
            );
        }
        yyp1 += 1;
        yyp2 += 1;
        yyn -= 1;
    }
}

fn mmed3(a: crate::types::UChar, b: crate::types::UChar, c: crate::types::UChar) -> crate::types::UChar {
    let mut a = a;
    let mut b = b;
    let c = c;
    
    if a > b {
        let t = a;
        a = b;
        b = t;
    }
    if b > c {
        b = c;
        if a > b {
            b = a;
        }
    }
    b
}

fn mmin(a: crate::types::Int32, b: crate::types::Int32) -> crate::types::Int32 {
    if a < b { a } else { b }
}

fn mpush(lz: crate::types::Int32, hz: crate::types::Int32, dz: crate::types::Int32, stackLo: *mut crate::types::Int32, stackHi: *mut crate::types::Int32, stackD: *mut crate::types::Int32, sp: *mut crate::types::Int32) {
    unsafe {
        let idx = *sp as isize;
        *stackLo.offset(idx) = lz;
        *stackHi.offset(idx) = hz;
        *stackD.offset(idx) = dz;
        *sp += 1;
    }
}

fn mpop(lz: *mut crate::types::Int32, hz: *mut crate::types::Int32, dz: *mut crate::types::Int32, stackLo: *mut crate::types::Int32, stackHi: *mut crate::types::Int32, stackD: *mut crate::types::Int32, sp: *mut crate::types::Int32) {
    unsafe {
        *sp -= 1;
        *lz = *stackLo.offset(*sp as isize);
        *hz = *stackHi.offset(*sp as isize);
        *dz = *stackD.offset(*sp as isize);
    }
}

fn mnextsize(az: crate::types::Int32, nextHi: *mut crate::types::Int32, nextLo: *mut crate::types::Int32) -> crate::types::Int32 {
    unsafe {
        *nextHi.offset(az as isize) - *nextLo.offset(az as isize)
    }
}

fn mnextswap(az: crate::types::Int32, bz: crate::types::Int32, nextLo: *mut crate::types::Int32, nextHi: *mut crate::types::Int32, nextD: *mut crate::types::Int32) {
    unsafe {
        let mut tz: crate::types::Int32;
        
        tz = *nextLo.offset(az as isize);
        *nextLo.offset(az as isize) = *nextLo.offset(bz as isize);
        *nextLo.offset(bz as isize) = tz;
        
        tz = *nextHi.offset(az as isize);
        *nextHi.offset(az as isize) = *nextHi.offset(bz as isize);
        *nextHi.offset(bz as isize) = tz;
        
        tz = *nextD.offset(az as isize);
        *nextD.offset(az as isize) = *nextD.offset(bz as isize);
        *nextD.offset(bz as isize) = tz;
    }
}

fn mainQSort3(ptr: *mut crate::types::UInt32, block: *mut crate::types::UChar, quadrant: *mut crate::types::UInt16, nblock: crate::types::Int32, loSt: crate::types::Int32, hiSt: crate::types::Int32, dSt: crate::types::Int32, budget: *mut crate::types::Int32) {
    unsafe {
        let mut unLo: crate::types::Int32;
        let mut unHi: crate::types::Int32;
        let mut ltLo: crate::types::Int32;
        let mut gtHi: crate::types::Int32;
        let mut n: crate::types::Int32;
        let mut m: crate::types::Int32;
        let mut med: crate::types::Int32;
        let mut sp: crate::types::Int32;
        let mut lo: crate::types::Int32 = 0;
        let mut hi: crate::types::Int32 = 0;
        let mut d: crate::types::Int32 = 0;

        let mut stackLo: [crate::types::Int32; 100] = [0; 100];
        let mut stackHi: [crate::types::Int32; 100] = [0; 100];
        let mut stackD: [crate::types::Int32; 100] = [0; 100];

        let mut nextLo: [crate::types::Int32; 3] = [0; 3];
        let mut nextHi: [crate::types::Int32; 3] = [0; 3];
        let mut nextD: [crate::types::Int32; 3] = [0; 3];

        sp = 0;
        crate::src_blocksort::mpush(loSt, hiSt, dSt, stackLo.as_mut_ptr(), stackHi.as_mut_ptr(), stackD.as_mut_ptr(), &mut sp);

        while sp > 0 {
            if !(sp < MAIN_QSORT_STACK_SIZE - 2) {
                crate::src_bzlib::BZ2_bz__AssertH__fail(1001);
            }

            crate::src_blocksort::mpop(&mut lo, &mut hi, &mut d, stackLo.as_mut_ptr(), stackHi.as_mut_ptr(), stackD.as_mut_ptr(), &mut sp);
            
            if hi - lo < MAIN_QSORT_SMALL_THRESH || d > MAIN_QSORT_DEPTH_THRESH {
                crate::src_blocksort::mainSimpleSort(ptr, block, quadrant, nblock, lo, hi, d, budget);
                if *budget < 0 {
                    return;
                }
                continue;
            }

            let idx_lo = *ptr.offset(lo as isize) as isize + d as isize;
            let idx_hi = *ptr.offset(hi as isize) as isize + d as isize;
            let idx_mid = *ptr.offset(((lo + hi) >> 1) as isize) as isize + d as isize;
            med = crate::src_blocksort::mmed3(
                *block.offset(idx_lo),
                *block.offset(idx_hi),
                *block.offset(idx_mid)
            ) as crate::types::Int32;

            unLo = lo;
            ltLo = lo;
            unHi = hi;
            gtHi = hi;

            loop {
                loop {
                    if unLo > unHi {
                        break;
                    }
                    n = (*block.offset(*ptr.offset(unLo as isize) as isize + d as isize)) as crate::types::Int32 - med;
                    if n == 0 {
                        crate::src_blocksort::mswap(ptr.offset(unLo as isize), ptr.offset(ltLo as isize));
                        ltLo += 1;
                        unLo += 1;
                        continue;
                    }
                    if n > 0 {
                        break;
                    }
                    unLo += 1;
                }
                loop {
                    if unLo > unHi {
                        break;
                    }
                    n = (*block.offset(*ptr.offset(unHi as isize) as isize + d as isize)) as crate::types::Int32 - med;
                    if n == 0 {
                        crate::src_blocksort::mswap(ptr.offset(unHi as isize), ptr.offset(gtHi as isize));
                        gtHi -= 1;
                        unHi -= 1;
                        continue;
                    }
                    if n < 0 {
                        break;
                    }
                    unHi -= 1;
                }
                if unLo > unHi {
                    break;
                }
                crate::src_blocksort::mswap(ptr.offset(unLo as isize), ptr.offset(unHi as isize));
                unLo += 1;
                unHi -= 1;
            }

            if gtHi < ltLo {
                crate::src_blocksort::mpush(lo, hi, d + 1, stackLo.as_mut_ptr(), stackHi.as_mut_ptr(), stackD.as_mut_ptr(), &mut sp);
                continue;
            }

            n = crate::src_blocksort::mmin(ltLo - lo, unLo - ltLo);
            crate::src_blocksort::mvswap(lo, unLo - n, n, ptr);
            m = crate::src_blocksort::mmin(hi - gtHi, gtHi - unHi);
            crate::src_blocksort::mvswap(unLo, hi - m + 1, m, ptr);

            n = lo + unLo - ltLo - 1;
            m = hi - (gtHi - unHi) + 1;

            nextLo[0] = lo;
            nextHi[0] = n;
            nextD[0] = d;
            nextLo[1] = m;
            nextHi[1] = hi;
            nextD[1] = d;
            nextLo[2] = n + 1;
            nextHi[2] = m - 1;
            nextD[2] = d + 1;

            if crate::src_blocksort::mnextsize(0, nextHi.as_mut_ptr(), nextLo.as_mut_ptr()) < crate::src_blocksort::mnextsize(1, nextHi.as_mut_ptr(), nextLo.as_mut_ptr()) {
                crate::src_blocksort::mnextswap(0, 1, nextLo.as_mut_ptr(), nextHi.as_mut_ptr(), nextD.as_mut_ptr());
            }
            if crate::src_blocksort::mnextsize(1, nextHi.as_mut_ptr(), nextLo.as_mut_ptr()) < crate::src_blocksort::mnextsize(2, nextHi.as_mut_ptr(), nextLo.as_mut_ptr()) {
                crate::src_blocksort::mnextswap(1, 2, nextLo.as_mut_ptr(), nextHi.as_mut_ptr(), nextD.as_mut_ptr());
            }
            if crate::src_blocksort::mnextsize(0, nextHi.as_mut_ptr(), nextLo.as_mut_ptr()) < crate::src_blocksort::mnextsize(1, nextHi.as_mut_ptr(), nextLo.as_mut_ptr()) {
                crate::src_blocksort::mnextswap(0, 1, nextLo.as_mut_ptr(), nextHi.as_mut_ptr(), nextD.as_mut_ptr());
            }

            crate::src_blocksort::mpush(nextLo[0], nextHi[0], nextD[0], stackLo.as_mut_ptr(), stackHi.as_mut_ptr(), stackD.as_mut_ptr(), &mut sp);
            crate::src_blocksort::mpush(nextLo[1], nextHi[1], nextD[1], stackLo.as_mut_ptr(), stackHi.as_mut_ptr(), stackD.as_mut_ptr(), &mut sp);
            crate::src_blocksort::mpush(nextLo[2], nextHi[2], nextD[2], stackLo.as_mut_ptr(), stackHi.as_mut_ptr(), stackD.as_mut_ptr(), &mut sp);
        }
    }
}

fn BIGFREQ(b: crate::types::Int32, ftab: *mut crate::types::UInt32) -> crate::types::Int32 {
    unsafe {
        let idx1 = ((b + 1) << 8) as usize;
        let idx2 = (b << 8) as usize;
        (*ftab.add(idx1) as crate::types::Int32) - (*ftab.add(idx2) as crate::types::Int32)
    }
}

fn mainSort(ptr: *mut crate::types::UInt32, block: *mut crate::types::UChar, quadrant: *mut crate::types::UInt16, ftab: *mut crate::types::UInt32, nblock: crate::types::Int32, verb: crate::types::Int32, budget: *mut crate::types::Int32) {
    unsafe {
        let mut i: crate::types::Int32;
        let mut j: crate::types::Int32;
        let mut k: crate::types::Int32;
        let mut ss: crate::types::Int32;
        let mut sb: crate::types::Int32;
        let mut runningOrder: [crate::types::Int32; 256] = [0; 256];
        let mut bigDone: [crate::types::Bool; 256] = [0; 256];
        let mut copyStart: [crate::types::Int32; 256] = [0; 256];
        let mut copyEnd: [crate::types::Int32; 256] = [0; 256];
        let mut c1: crate::types::UChar;
        let mut numQSorted: crate::types::Int32;
        let mut s: crate::types::UInt16;

        // Initialize ftab
        i = 65536;
        while i >= 0 {
            *ftab.offset(i as isize) = 0;
            i -= 1;
        }

        j = (*block.offset(0) as i32) << 8;
        i = nblock - 1;
        while i >= 3 {
            *quadrant.offset(i as isize) = 0;
            j = (j >> 8) | ((*block.offset(i as isize) as crate::types::UInt16 as i32) << 8);
            *ftab.offset(j as isize) += 1;
            *quadrant.offset((i - 1) as isize) = 0;
            j = (j >> 8) | ((*block.offset((i - 1) as isize) as crate::types::UInt16 as i32) << 8);
            *ftab.offset(j as isize) += 1;
            *quadrant.offset((i - 2) as isize) = 0;
            j = (j >> 8) | ((*block.offset((i - 2) as isize) as crate::types::UInt16 as i32) << 8);
            *ftab.offset(j as isize) += 1;
            *quadrant.offset((i - 3) as isize) = 0;
            j = (j >> 8) | ((*block.offset((i - 3) as isize) as crate::types::UInt16 as i32) << 8);
            *ftab.offset(j as isize) += 1;
            i -= 4;
        }
        while i >= 0 {
            *quadrant.offset(i as isize) = 0;
            j = (j >> 8) | ((*block.offset(i as isize) as crate::types::UInt16 as i32) << 8);
            *ftab.offset(j as isize) += 1;
            i -= 1;
        }

        // BZ_N_OVERSHOOT = 2 + 12 + 18 + 2 = 34
        for i in 0..(2 + 12 + 18 + 2) {
            *block.offset((nblock + i) as isize) = *block.offset(i as isize);
            *quadrant.offset((nblock + i) as isize) = 0;
        }

        // Cumulative sum
        for i in 1..=65536 {
            *ftab.offset(i as isize) += *ftab.offset((i - 1) as isize);
        }

        s = ((*block.offset(0) as i32) << 8) as crate::types::UInt16;
        i = nblock - 1;
        while i >= 3 {
            s = ((s >> 8) | ((*block.offset(i as isize) as i32) << 8) as u16) as crate::types::UInt16;
            j = (*ftab.offset(s as isize) as i32) - 1;
            *ftab.offset(s as isize) = j as u32;
            *ptr.offset(j as isize) = i as u32;
            s = ((s >> 8) | ((*block.offset((i - 1) as isize) as i32) << 8) as u16) as crate::types::UInt16;
            j = (*ftab.offset(s as isize) as i32) - 1;
            *ftab.offset(s as isize) = j as u32;
            *ptr.offset(j as isize) = (i - 1) as u32;
            s = ((s >> 8) | ((*block.offset((i - 2) as isize) as i32) << 8) as u16) as crate::types::UInt16;
            j = (*ftab.offset(s as isize) as i32) - 1;
            *ftab.offset(s as isize) = j as u32;
            *ptr.offset(j as isize) = (i - 2) as u32;
            s = ((s >> 8) | ((*block.offset((i - 3) as isize) as i32) << 8) as u16) as crate::types::UInt16;
            j = (*ftab.offset(s as isize) as i32) - 1;
            *ftab.offset(s as isize) = j as u32;
            *ptr.offset(j as isize) = (i - 3) as u32;
            i -= 4;
        }
        while i >= 0 {
            s = ((s >> 8) | ((*block.offset(i as isize) as i32) << 8) as u16) as crate::types::UInt16;
            j = (*ftab.offset(s as isize) as i32) - 1;
            *ftab.offset(s as isize) = j as u32;
            *ptr.offset(j as isize) = i as u32;
            i -= 1;
        }

        for i in 0..=255 {
            bigDone[i as usize] = 0;
            runningOrder[i as usize] = i;
        }

        // Shell sort
        {
            let mut h: crate::types::Int32 = 1;
            while h <= 256 {
                h = 3 * h + 1;
            }
            loop {
                h = h / 3;
                for i in h..=255 {
                    let vv = runningOrder[i as usize];
                    j = i;
                    loop {
                        if BIGFREQ(runningOrder[(j - h) as usize], ftab) > BIGFREQ(vv, ftab) {
                            runningOrder[j as usize] = runningOrder[(j - h) as usize];
                            j = j - h;
                            if j <= (h - 1) {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    runningOrder[j as usize] = vv;
                }
                if h == 1 {
                    break;
                }
            }
        }

        numQSorted = 0;

        for i in 0..=255 {
            ss = runningOrder[i as usize];

            for j in 0..=255 {
                if j != ss {
                    sb = (ss << 8) + j;
                    if (*ftab.offset(sb as isize) as i32 & crate::globals::SETMASK) == 0 {
                        let lo = *ftab.offset(sb as isize) as i32 & crate::globals::CLEARMASK;
                        let hi = (*ftab.offset((sb + 1) as isize) as i32 & crate::globals::CLEARMASK) - 1;
                        if hi > lo {
                            mainQSort3(ptr, block, quadrant, nblock, lo, hi, 2, budget);
                            numQSorted += hi - lo + 1;
                            if *budget < 0 {
                                return;
                            }
                        }
                    }
                    *ftab.offset(sb as isize) |= crate::globals::SETMASK as u32;
                }
            }

            if bigDone[ss as usize] != 0 {
                crate::src_bzlib::BZ2_bz__AssertH__fail(1006);
            }

            {
                for j in 0..=255 {
                    copyStart[j as usize] = *ftab.offset(((j << 8) + ss) as isize) as i32 & crate::globals::CLEARMASK;
                    copyEnd[j as usize] = (*ftab.offset(((j << 8) + ss + 1) as isize) as i32 & crate::globals::CLEARMASK) - 1;
                }
                j = *ftab.offset((ss << 8) as isize) as i32 & crate::globals::CLEARMASK;
                while j < copyStart[ss as usize] {
                    k = *ptr.offset(j as isize) as i32 - 1;
                    if k < 0 {
                        k += nblock;
                    }
                    c1 = *block.offset(k as isize);
                    if bigDone[c1 as usize] == 0 {
                        *ptr.offset(copyStart[c1 as usize] as isize) = k as u32;
                        copyStart[c1 as usize] += 1;
                    }
                    j += 1;
                }
                j = (*ftab.offset(((ss + 1) << 8) as isize) as i32 & crate::globals::CLEARMASK) - 1;
                while j > copyEnd[ss as usize] {
                    k = *ptr.offset(j as isize) as i32 - 1;
                    if k < 0 {
                        k += nblock;
                    }
                    c1 = *block.offset(k as isize);
                    if bigDone[c1 as usize] == 0 {
                        *ptr.offset(copyEnd[c1 as usize] as isize) = k as u32;
                        copyEnd[c1 as usize] -= 1;
                    }
                    j -= 1;
                }
            }

            if !((copyStart[ss as usize] - 1 == copyEnd[ss as usize]) || (copyStart[ss as usize] == 0 && copyEnd[ss as usize] == nblock - 1)) {
                crate::src_bzlib::BZ2_bz__AssertH__fail(1007);
            }

            for j in 0..=255 {
                *ftab.offset(((j << 8) + ss) as isize) |= crate::globals::SETMASK as u32;
            }

            bigDone[ss as usize] = 1;

            if i < 255 {
                let bbStart = *ftab.offset((ss << 8) as isize) as i32 & crate::globals::CLEARMASK;
                let bbSize = (*ftab.offset(((ss + 1) << 8) as isize) as i32 & crate::globals::CLEARMASK) - bbStart;
                let mut shifts: crate::types::Int32 = 0;

                while (bbSize >> shifts) > 65534 {
                    shifts += 1;
                }

                for j in (0..bbSize).rev() {
                    let a2update = *ptr.offset((bbStart + j) as isize) as i32;
                    let qVal = (j >> shifts) as crate::types::UInt16;
                    *quadrant.offset(a2update as isize) = qVal;
                    if a2update < (2 + 12 + 18 + 2) {
                        *quadrant.offset((a2update + nblock) as isize) = qVal;
                    }
                }
                if !(((bbSize - 1) >> shifts) <= 65535) {
                    crate::src_bzlib::BZ2_bz__AssertH__fail(1002);
                }
            }
        }
    }
}

pub extern "C" fn BZ2_blockSort(arg1: *mut crate::types::EState) {
    unsafe {
        let s = arg1;
        
        let ptr_field_ptr = crate::compat::c2r_field_ptr_EState__ptr(s as *mut ::core::ffi::c_void);
        let ptr: *mut crate::types::UInt32 = *(ptr_field_ptr as *const *mut crate::types::UInt32);
        
        let block_field_ptr = crate::compat::c2r_field_ptr_EState__block(s as *mut ::core::ffi::c_void);
        let block: *mut crate::types::UChar = *(block_field_ptr as *const *mut crate::types::UChar);
        
        let ftab_field_ptr = crate::compat::c2r_field_ptr_EState__ftab(s as *mut ::core::ffi::c_void);
        let ftab: *mut crate::types::UInt32 = *(ftab_field_ptr as *const *mut crate::types::UInt32);
        
        let nblock_field_ptr = crate::compat::c2r_field_ptr_EState__nblock(s as *mut ::core::ffi::c_void);
        let nblock: crate::types::Int32 = *(nblock_field_ptr as *const crate::types::Int32);
        
        let verb_field_ptr = crate::compat::c2r_field_ptr_EState__verbosity(s as *mut ::core::ffi::c_void);
        let verb: crate::types::Int32 = *(verb_field_ptr as *const crate::types::Int32);
        
        let wfact_field_ptr = crate::compat::c2r_field_ptr_EState__workFactor(s as *mut ::core::ffi::c_void);
        let mut wfact: crate::types::Int32 = *(wfact_field_ptr as *const crate::types::Int32);
        
        let arr1_field_ptr = crate::compat::c2r_field_ptr_EState__arr1(s as *mut ::core::ffi::c_void);
        let arr1: *mut crate::types::UInt32 = *(arr1_field_ptr as *const *mut crate::types::UInt32);
        
        let arr2_field_ptr = crate::compat::c2r_field_ptr_EState__arr2(s as *mut ::core::ffi::c_void);
        let arr2: *mut crate::types::UInt32 = *(arr2_field_ptr as *const *mut crate::types::UInt32);
        
        if nblock < 10000 {
            crate::src_blocksort::fallbackSort(arr1, arr2, ftab, nblock, verb);
        } else {
            let mut i: crate::types::Int32 = nblock + (2 + 12 + 18 + 2);
            if (i & 1) != 0 {
                i += 1;
            }
            let quadrant: *mut crate::types::UInt16 = block.offset(i as isize) as *mut crate::types::UInt16;
            
            if wfact < 1 {
                wfact = 1;
            }
            if wfact > 100 {
                wfact = 100;
            }
            let budgetInit: crate::types::Int32 = nblock * ((wfact - 1) / 3);
            let mut budget: crate::types::Int32 = budgetInit;
            
            crate::src_blocksort::mainSort(ptr, block, quadrant, ftab, nblock, verb, &mut budget);
            
            if verb >= 3 {
                let _ratio: f32 = (budgetInit - budget) as f32 / (if nblock == 0 { 1 } else { nblock }) as f32;
                extern "C" {
                    static stderr: *mut libc::FILE;
                }
                libc::fprintf(
                    stderr,
                    b"      %d work, %d block, ratio %5.2f\n\0".as_ptr() as *const i8,
                    budgetInit - budget,
                    nblock,
                    _ratio as f64
                );
            }
            
            if budget < 0 {
                if verb >= 2 {
                    extern "C" {
                        static stderr: *mut libc::FILE;
                    }
                    libc::fprintf(
                        stderr,
                        b"    too repetitive; using fallback sorting algorithm\n\0".as_ptr() as *const i8
                    );
                }
                crate::src_blocksort::fallbackSort(arr1, arr2, ftab, nblock, verb);
            }
        }
        
        let origPtr_field_ptr = crate::compat::c2r_field_ptr_EState__origPtr(s as *mut ::core::ffi::c_void);
        *(origPtr_field_ptr as *mut crate::types::Int32) = -1;
        
        let nblock_for_loop: crate::types::Int32 = *(nblock_field_ptr as *const crate::types::Int32);
        for i in 0..nblock_for_loop {
            if *ptr.offset(i as isize) == 0 {
                *(origPtr_field_ptr as *mut crate::types::Int32) = i;
                break;
            }
        }
        
        let origPtr_val: crate::types::Int32 = *(origPtr_field_ptr as *const crate::types::Int32);
        if !(origPtr_val != -1) {
            crate::src_bzlib::BZ2_bz__AssertH__fail(1003);
        }
    }
}
