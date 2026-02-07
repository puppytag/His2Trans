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
    if lo == hi {
        return;
    }
    unsafe {
        if hi - lo > 3 {
            let mut i = hi - 4;
            while i >= lo {
                let tmp = *fmap.offset(i as isize);
                let ec_tmp = *eclass.offset(tmp as isize);
                let mut j = i + 4;
                while j <= hi && ec_tmp > *eclass.offset(*fmap.offset(j as isize) as isize) {
                    *fmap.offset((j - 4) as isize) = *fmap.offset(j as isize);
                    j += 4;
                }
                *fmap.offset((j - 4) as isize) = tmp;
                i -= 1;
            }
        }
        let mut i = hi - 1;
        while i >= lo {
            let tmp = *fmap.offset(i as isize);
            let ec_tmp = *eclass.offset(tmp as isize);
            let mut j = i + 1;
            while j <= hi && ec_tmp > *eclass.offset(*fmap.offset(j as isize) as isize) {
                *fmap.offset((j - 1) as isize) = *fmap.offset(j as isize);
                j += 1;
            }
            *fmap.offset((j - 1) as isize) = tmp;
            i -= 1;
        }
    }
}

fn fswap(zz1: *mut crate::types::UInt32, zz2: *mut crate::types::UInt32) {
    unsafe {
        let zztmp = *zz1;
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
            let ptr1 = fmap.offset(yyp1 as isize);
            let ptr2 = fmap.offset(yyp2 as isize);
            crate::src_blocksort::fswap(ptr1, ptr2);
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
        let idx = *sp;
        *stackLo.offset(idx as isize) = lz;
        *stackHi.offset(idx as isize) = hz;
        *sp += 1;
    }
}

fn fpop(stackLo: *mut crate::types::Int32, stackHi: *mut crate::types::Int32, sp: *mut crate::types::Int32, lz: *mut crate::types::Int32, hz: *mut crate::types::Int32) {
unsafe {
    *sp -= 1;
    let idx = *sp as usize;
    *lz = *stackLo.offset(idx as isize);
    *hz = *stackHi.offset(idx as isize);
}
}

fn fallbackQSort3(fmap: *mut crate::types::UInt32, eclass: *mut crate::types::UInt32, loSt: crate::types::Int32, hiSt: crate::types::Int32) {
    let mut stackLo: [crate::types::Int32; 100] = [0; 100];
    let mut stackHi: [crate::types::Int32; 100] = [0; 100];
    let mut sp: crate::types::Int32 = 0;
    let mut r: crate::types::UInt32 = 0;
    crate::src_blocksort::fpush(stackLo.as_mut_ptr(), stackHi.as_mut_ptr(), &mut sp, loSt, hiSt);
    while sp > 0 {
        if !(sp < 100 - 1) {
            crate::src_bzlib::BZ2_bz__AssertH__fail(1004);
        }
        let mut lo: crate::types::Int32 = 0;
        let mut hi: crate::types::Int32 = 0;
        crate::src_blocksort::fpop(stackLo.as_mut_ptr(), stackHi.as_mut_ptr(), &mut sp, &mut lo, &mut hi);
        if hi - lo < 10 {
            crate::src_blocksort::fallbackSimpleSort(fmap, eclass, lo, hi);
            continue;
        }
        r = ((r * 7621) + 1) % 32768;
        let r3 = r % 3;
        let med: crate::types::UInt32 = unsafe {
            if r3 == 0 {
                *eclass.offset(*fmap.offset(lo as isize) as isize)
            } else if r3 == 1 {
                *eclass.offset(*fmap.offset(((lo + hi) >> 1) as isize) as isize)
            } else {
                *eclass.offset(*fmap.offset(hi as isize) as isize)
            }
        };
        let mut unLo: crate::types::Int32 = lo;
        let mut ltLo: crate::types::Int32 = lo;
        let mut unHi: crate::types::Int32 = hi;
        let mut gtHi: crate::types::Int32 = hi;
        loop {
            loop {
                if unLo > unHi {
                    break;
                }
                let n: crate::types::Int32 = unsafe {
                    (*eclass.offset(*fmap.offset(unLo as isize) as isize) as crate::types::Int32) - med as crate::types::Int32
                };
                if n == 0 {
                    unsafe {
                        crate::src_blocksort::fswap(fmap.offset(unLo as isize) as *mut crate::types::UInt32, fmap.offset(ltLo as isize) as *mut crate::types::UInt32);
                    }
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
                let n: crate::types::Int32 = unsafe {
                    (*eclass.offset(*fmap.offset(unHi as isize) as isize) as crate::types::Int32) - med as crate::types::Int32
                };
                if n == 0 {
                    unsafe {
                        crate::src_blocksort::fswap(fmap.offset(unHi as isize) as *mut crate::types::UInt32, fmap.offset(gtHi as isize) as *mut crate::types::UInt32);
                    }
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
            unsafe {
                crate::src_blocksort::fswap(fmap.offset(unLo as isize) as *mut crate::types::UInt32, fmap.offset(unHi as isize) as *mut crate::types::UInt32);
            }
            unLo += 1;
            unHi -= 1;
        }
        if gtHi < ltLo {
            continue;
        }
        let n = crate::src_blocksort::fmin(ltLo - lo, unLo - ltLo);
        crate::src_blocksort::fvswap(lo, unLo - n, n, fmap);
        let m = crate::src_blocksort::fmin(hi - gtHi, gtHi - unHi);
        crate::src_blocksort::fvswap(unLo, hi - m + 1, m, fmap);
        let n = lo + unLo - ltLo - 1;
        let m = hi - (gtHi - unHi) + 1;
        if n - lo > hi - m {
            crate::src_blocksort::fpush(stackLo.as_mut_ptr(), stackHi.as_mut_ptr(), &mut sp, lo, n);
            crate::src_blocksort::fpush(stackLo.as_mut_ptr(), stackHi.as_mut_ptr(), &mut sp, m, hi);
        } else {
            crate::src_blocksort::fpush(stackLo.as_mut_ptr(), stackHi.as_mut_ptr(), &mut sp, m, hi);
            crate::src_blocksort::fpush(stackLo.as_mut_ptr(), stackHi.as_mut_ptr(), &mut sp, lo, n);
        }
    }
}

fn SET_BH(bhtab: *mut crate::types::UInt32, zz: crate::types::Int32) {
    unsafe {
        let idx = (zz >> 5) as isize;
        let bit = (zz & 31) as u32;
        *bhtab.offset(idx) |= 1u32 << bit;
    }
}

fn CLEAR_BH(bhtab: *mut crate::types::UInt32, zz: crate::types::Int32) {
    unsafe {
        let idx = (zz >> 5) as isize;
        let bit = 1u32 << (zz & 31);
        (*bhtab.offset(idx)) &= !bit;
    }
}

fn ISSET_BH(bhtab: *mut crate::types::UInt32, zz: crate::types::Int32) -> crate::types::Int32 {
    unsafe {
        let idx = (zz >> 5) as isize;
        let word = *bhtab.offset(idx);
        (word & (1u32 << (zz & 31))) as crate::types::Int32
    }
}

fn WORD_BH(bhtab: *mut crate::types::UInt32, zz: crate::types::Int32) -> crate::types::UInt32 {
    unsafe { *bhtab.offset((zz >> 5) as isize) }
}

fn UNALIGNED_BH(zz: crate::types::Int32) -> crate::types::Int32 {
    zz & 0x1f
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

    if verb >= 4 {
        let _ = verb;
    }
    for i in 0..257 {
        ftab[i as usize] = 0;
    }
    for i in 0..nblock {
        unsafe {
            ftab[*eclass8.offset(i as isize) as usize] += 1;
        }
    }
    for i in 0..256 {
        ftabCopy[i as usize] = ftab[i as usize];
    }
    for i in 1..257 {
        ftab[i as usize] += ftab[(i - 1) as usize];
    }
    for i in 0..nblock {
        unsafe {
            j = *eclass8.offset(i as isize) as crate::types::Int32;
            k = ftab[j as usize] - 1;
            ftab[j as usize] = k;
            *fmap.offset(k as isize) = i as crate::types::UInt32;
        }
    }
    nBhtab = 2 + (nblock / 32);
    for i in 0..nBhtab {
        unsafe {
            *bhtab.offset(i as isize) = 0;
        }
    }
    for i in 0..256 {
        crate::src_blocksort::SET_BH(bhtab, ftab[i as usize]);
    }
    for i in 0..32 {
        crate::src_blocksort::SET_BH(bhtab, nblock + 2 * i);
        crate::src_blocksort::CLEAR_BH(bhtab, nblock + 2 * i + 1);
    }
    H = 1;
    loop {
        if verb >= 4 {
            let _ = H;
        }
        j = 0;
        for i in 0..nblock {
            if crate::src_blocksort::ISSET_BH(bhtab, i) != 0 {
                j = i;
            }
            unsafe {
                k = *fmap.offset(i as isize) as crate::types::Int32 - H;
                if k < 0 {
                    k += nblock;
                }
                *eclass.offset(k as isize) = j as crate::types::UInt32;
            }
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
                nNotDone += (r - l + 1);
                crate::src_blocksort::fallbackQSort3(fmap, eclass, l, r);
                cc = -1;
                for i in l..=r {
                    unsafe {
                        cc1 = *eclass.offset(*fmap.offset(i as isize) as isize) as crate::types::Int32;
                    }
                    if cc != cc1 {
                        crate::src_blocksort::SET_BH(bhtab, i);
                        cc = cc1;
                    }
                }
            }
        }
        if verb >= 4 {
            let _ = nNotDone;
        }
        H *= 2;
        if H > nblock || nNotDone == 0 {
            break;
        }
    }
    if verb >= 4 {
    }
    j = 0;
    for i in 0..nblock {
        while ftabCopy[j as usize] == 0 {
            j += 1;
        }
        ftabCopy[j as usize] -= 1;
        unsafe {
            *eclass8.offset(*fmap.offset(i as isize) as isize) = j as crate::types::UChar;
        }
    }
    if !(j < 256) {
        crate::src_bzlib::BZ2_bz__AssertH__fail(1005);
    }
}

fn mainGtU(i1: crate::types::UInt32, i2: crate::types::UInt32, block: *const crate::types::UChar, quadrant: *mut crate::types::UInt16, nblock: crate::types::UInt32, budget: *mut crate::types::Int32) -> crate::types::Bool {
    let mut i1 = i1;
    let mut i2 = i2;
    let mut c1: crate::types::UChar;
    let mut c2: crate::types::UChar;
    let mut s1: crate::types::UInt16;
    let mut s2: crate::types::UInt16;

    unsafe {
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 != c2 {
            return (c1 > c2) as crate::types::Bool;
        }
        i1 += 1;
        i2 += 1;

        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 != c2 {
            return (c1 > c2) as crate::types::Bool;
        }
        i1 += 1;
        i2 += 1;

        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 != c2 {
            return (c1 > c2) as crate::types::Bool;
        }
        i1 += 1;
        i2 += 1;

        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 != c2 {
            return (c1 > c2) as crate::types::Bool;
        }
        i1 += 1;
        i2 += 1;

        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 != c2 {
            return (c1 > c2) as crate::types::Bool;
        }
        i1 += 1;
        i2 += 1;

        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 != c2 {
            return (c1 > c2) as crate::types::Bool;
        }
        i1 += 1;
        i2 += 1;

        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 != c2 {
            return (c1 > c2) as crate::types::Bool;
        }
        i1 += 1;
        i2 += 1;

        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 != c2 {
            return (c1 > c2) as crate::types::Bool;
        }
        i1 += 1;
        i2 += 1;

        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 != c2 {
            return (c1 > c2) as crate::types::Bool;
        }
        i1 += 1;
        i2 += 1;

        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 != c2 {
            return (c1 > c2) as crate::types::Bool;
        }
        i1 += 1;
        i2 += 1;

        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 != c2 {
            return (c1 > c2) as crate::types::Bool;
        }
        i1 += 1;
        i2 += 1;

        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 != c2 {
            return (c1 > c2) as crate::types::Bool;
        }
        i1 += 1;
        i2 += 1;

        let mut k = (nblock as crate::types::Int32) + 8;

        loop {
            c1 = *block.offset(i1 as isize);
            c2 = *block.offset(i2 as isize);
            if c1 != c2 {
                return (c1 > c2) as crate::types::Bool;
            }
            s1 = *quadrant.offset(i1 as isize);
            s2 = *quadrant.offset(i2 as isize);
            if s1 != s2 {
                return (s1 > s2) as crate::types::Bool;
            }
            i1 += 1;
            i2 += 1;

            c1 = *block.offset(i1 as isize);
            c2 = *block.offset(i2 as isize);
            if c1 != c2 {
                return (c1 > c2) as crate::types::Bool;
            }
            s1 = *quadrant.offset(i1 as isize);
            s2 = *quadrant.offset(i2 as isize);
            if s1 != s2 {
                return (s1 > s2) as crate::types::Bool;
            }
            i1 += 1;
            i2 += 1;

            c1 = *block.offset(i1 as isize);
            c2 = *block.offset(i2 as isize);
            if c1 != c2 {
                return (c1 > c2) as crate::types::Bool;
            }
            s1 = *quadrant.offset(i1 as isize);
            s2 = *quadrant.offset(i2 as isize);
            if s1 != s2 {
                return (s1 > s2) as crate::types::Bool;
            }
            i1 += 1;
            i2 += 1;

            c1 = *block.offset(i1 as isize);
            c2 = *block.offset(i2 as isize);
            if c1 != c2 {
                return (c1 > c2) as crate::types::Bool;
            }
            s1 = *quadrant.offset(i1 as isize);
            s2 = *quadrant.offset(i2 as isize);
            if s1 != s2 {
                return (s1 > s2) as crate::types::Bool;
            }
            i1 += 1;
            i2 += 1;

            c1 = *block.offset(i1 as isize);
            c2 = *block.offset(i2 as isize);
            if c1 != c2 {
                return (c1 > c2) as crate::types::Bool;
            }
            s1 = *quadrant.offset(i1 as isize);
            s2 = *quadrant.offset(i2 as isize);
            if s1 != s2 {
                return (s1 > s2) as crate::types::Bool;
            }
            i1 += 1;
            i2 += 1;

            c1 = *block.offset(i1 as isize);
            c2 = *block.offset(i2 as isize);
            if c1 != c2 {
                return (c1 > c2) as crate::types::Bool;
            }
            s1 = *quadrant.offset(i1 as isize);
            s2 = *quadrant.offset(i2 as isize);
            if s1 != s2 {
                return (s1 > s2) as crate::types::Bool;
            }
            i1 += 1;
            i2 += 1;

            c1 = *block.offset(i1 as isize);
            c2 = *block.offset(i2 as isize);
            if c1 != c2 {
                return (c1 > c2) as crate::types::Bool;
            }
            s1 = *quadrant.offset(i1 as isize);
            s2 = *quadrant.offset(i2 as isize);
            if s1 != s2 {
                return (s1 > s2) as crate::types::Bool;
            }
            i1 += 1;
            i2 += 1;

            c1 = *block.offset(i1 as isize);
            c2 = *block.offset(i2 as isize);
            if c1 != c2 {
                return (c1 > c2) as crate::types::Bool;
            }
            s1 = *quadrant.offset(i1 as isize);
            s2 = *quadrant.offset(i2 as isize);
            if s1 != s2 {
                return (s1 > s2) as crate::types::Bool;
            }
            i1 += 1;
            i2 += 1;

            if i1 >= nblock {
                i1 -= nblock;
            }
            if i2 >= nblock {
                i2 -= nblock;
            }

            k -= 8;
            *budget -= 1;

            if k < 0 {
                break;
            }
        }
    }

    0
}

fn mainSimpleSort(ptr: *mut crate::types::UInt32, block: *mut crate::types::UChar, quadrant: *mut crate::types::UInt16, nblock: crate::types::Int32, lo: crate::types::Int32, hi: crate::types::Int32, d: crate::types::Int32, budget: *mut crate::types::Int32) {
   let mut bigN: crate::types::Int32 = hi - lo + 1;
   if bigN < 2 {
      return;
   }
   let incs_var: [crate::types::Int32; 16] = [1, 4, 13, 40, 121, 364, 1093, 3280, 9841, 29524, 88573, 265720, 797161, 2391484, 7174453, 21523360];
   let mut hp: crate::types::Int32 = 0;
   unsafe {
      while hp < 16 && incs[hp as usize] < bigN {
         hp += 1;
      }
   }
   hp -= 1;
   while hp >= 0 {
      let h: crate::types::Int32;
      unsafe {
         h = incs[hp as usize];
      }
      let mut i: crate::types::Int32 = lo + h;
      loop {
         if i > hi {
            break;
         }
         let mut v: crate::types::UInt32;
         unsafe {
            v = *ptr.offset(i as isize);
         }
         let mut j: crate::types::Int32 = i;
         while crate::src_blocksort::mainGtU(
            (unsafe { *ptr.offset((j - h) as isize) } as crate::types::UInt32).wrapping_add(d as crate::types::UInt32),
            v.wrapping_add(d as crate::types::UInt32),
            block as *const crate::types::UChar,
            quadrant,
            nblock as crate::types::UInt32,
            budget,
         ) != 0 {
            unsafe {
               *ptr.offset(j as isize) = *ptr.offset((j - h) as isize);
            }
            j = j - h;
            if j <= (lo + h - 1) {
               break;
            }
         }
         unsafe {
            *ptr.offset(j as isize) = v;
         }
         i += 1;
         if i > hi {
            break;
         }
         unsafe {
            v = *ptr.offset(i as isize);
         }
         j = i;
         while crate::src_blocksort::mainGtU(
            (unsafe { *ptr.offset((j - h) as isize) } as crate::types::UInt32).wrapping_add(d as crate::types::UInt32),
            v.wrapping_add(d as crate::types::UInt32),
            block as *const crate::types::UChar,
            quadrant,
            nblock as crate::types::UInt32,
            budget,
         ) != 0 {
            unsafe {
               *ptr.offset(j as isize) = *ptr.offset((j - h) as isize);
            }
            j = j - h;
            if j <= (lo + h - 1) {
               break;
            }
         }
         unsafe {
            *ptr.offset(j as isize) = v;
         }
         i += 1;
         if i > hi {
            break;
         }
         unsafe {
            v = *ptr.offset(i as isize);
         }
         j = i;
         while crate::src_blocksort::mainGtU(
            (unsafe { *ptr.offset((j - h) as isize) } as crate::types::UInt32).wrapping_add(d as crate::types::UInt32),
            v.wrapping_add(d as crate::types::UInt32),
            block as *const crate::types::UChar,
            quadrant,
            nblock as crate::types::UInt32,
            budget,
         ) != 0 {
            unsafe {
               *ptr.offset(j as isize) = *ptr.offset((j - h) as isize);
            }
            j = j - h;
            if j <= (lo + h - 1) {
               break;
            }
         }
         unsafe {
            *ptr.offset(j as isize) = v;
         }
         i += 1;
         unsafe {
            if *budget < 0 {
               return;
            }
         }
      }
      hp -= 1;
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
            crate::src_blocksort::mswap(ptr.offset(yyp1 as isize), ptr.offset(yyp2 as isize));
        }
        yyp1 += 1;
        yyp2 += 1;
        yyn -= 1;
    }
}

fn mmed3(a: crate::types::UChar, b: crate::types::UChar, c: crate::types::UChar) -> crate::types::UChar {
    let mut a = a;
    let mut b = b;
    let mut t;
    if a > b {
        t = a;
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
        let idx = *sp;
        *stackLo.offset(idx as isize) = lz;
        *stackHi.offset(idx as isize) = hz;
        *stackD.offset(idx as isize) = dz;
        *sp += 1;
    }
}

fn mpop(lz: *mut crate::types::Int32, hz: *mut crate::types::Int32, dz: *mut crate::types::Int32, stackLo: *mut crate::types::Int32, stackHi: *mut crate::types::Int32, stackD: *mut crate::types::Int32, sp: *mut crate::types::Int32) {
    unsafe {
        *sp -= 1;
        let idx = *sp as usize;
        *lz = *stackLo.add(idx);
        *hz = *stackHi.add(idx);
        *dz = *stackD.add(idx);
    }
}

fn mnextsize(az: crate::types::Int32, nextHi: *mut crate::types::Int32, nextLo: *mut crate::types::Int32) -> crate::types::Int32 {
    unsafe { *nextHi.offset(az as isize) - *nextLo.offset(az as isize) }
}

fn mnextswap(az: crate::types::Int32, bz: crate::types::Int32, nextLo: *mut crate::types::Int32, nextHi: *mut crate::types::Int32, nextD: *mut crate::types::Int32) {
    unsafe {
        let mut tz = *nextLo.offset(az as isize);
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
    let mut stackLo: [crate::types::Int32; 100] = [0; 100];
    let mut stackHi: [crate::types::Int32; 100] = [0; 100];
    let mut stackD: [crate::types::Int32; 100] = [0; 100];
    let mut nextLo: [crate::types::Int32; 3] = [0; 3];
    let mut nextHi: [crate::types::Int32; 3] = [0; 3];
    let mut nextD: [crate::types::Int32; 3] = [0; 3];
    let mut sp: crate::types::Int32 = 0;
    let mut lo: crate::types::Int32 = 0;
    let mut hi: crate::types::Int32 = 0;
    let mut d: crate::types::Int32 = 0;
    let mut unLo: crate::types::Int32 = 0;
    let mut unHi: crate::types::Int32 = 0;
    let mut ltLo: crate::types::Int32 = 0;
    let mut gtHi: crate::types::Int32 = 0;
    let mut n: crate::types::Int32 = 0;
    let mut m: crate::types::Int32 = 0;
    let mut med: crate::types::Int32 = 0;

    crate::src_blocksort::mpush(loSt, hiSt, dSt, stackLo.as_mut_ptr(), stackHi.as_mut_ptr(), stackD.as_mut_ptr(), &mut sp);
    while sp > 0 {
        if !(sp < 100 - 2) {
            crate::src_bzlib::BZ2_bz__AssertH__fail(1001);
        }
        crate::src_blocksort::mpop(&mut lo, &mut hi, &mut d, stackLo.as_mut_ptr(), stackHi.as_mut_ptr(), stackD.as_mut_ptr(), &mut sp);
        if hi - lo < 20 || d > 14 {
            crate::src_blocksort::mainSimpleSort(ptr, block, quadrant, nblock, lo, hi, d, budget);
            unsafe {
                if *budget < 0 {
                    return;
                }
            }
            continue;
        }
        unsafe {
            let idx1 = *ptr.offset(lo as isize) as crate::types::Int32 + d;
            let idx2 = *ptr.offset(hi as isize) as crate::types::Int32 + d;
            let idx3 = *ptr.offset(((lo + hi) >> 1) as isize) as crate::types::Int32 + d;
            med = crate::src_blocksort::mmed3(*block.offset(idx1 as isize), *block.offset(idx2 as isize), *block.offset(idx3 as isize)) as crate::types::Int32;
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
                unsafe {
                    n = *block.offset((*ptr.offset(unLo as isize) as crate::types::Int32 + d) as isize) as crate::types::Int32 - med;
                }
                if n == 0 {
                    unsafe {
                        crate::src_blocksort::mswap(ptr.offset(unLo as isize) as *mut crate::types::UInt32, ptr.offset(ltLo as isize) as *mut crate::types::UInt32);
                    }
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
                unsafe {
                    n = *block.offset((*ptr.offset(unHi as isize) as crate::types::Int32 + d) as isize) as crate::types::Int32 - med;
                }
                if n == 0 {
                    unsafe {
                        crate::src_blocksort::mswap(ptr.offset(unHi as isize) as *mut crate::types::UInt32, ptr.offset(gtHi as isize) as *mut crate::types::UInt32);
                    }
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
            unsafe {
                crate::src_blocksort::mswap(ptr.offset(unLo as isize) as *mut crate::types::UInt32, ptr.offset(unHi as isize) as *mut crate::types::UInt32);
            }
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

fn BIGFREQ(b: crate::types::Int32, ftab: *mut crate::types::UInt32) -> crate::types::Int32 {
    unsafe {
        let idx1 = (b + 1) << 8;
        let idx2 = b << 8;
        (*ftab.offset(idx1 as isize) as crate::types::Int32)
            .wrapping_sub(*ftab.offset(idx2 as isize) as crate::types::Int32)
    }
}

fn mainSort(ptr: *mut crate::types::UInt32, block: *mut crate::types::UChar, quadrant: *mut crate::types::UInt16, ftab: *mut crate::types::UInt32, nblock: crate::types::Int32, verb: crate::types::Int32, budget: *mut crate::types::Int32) {
    use crate::types::{Bool, Int32, UChar, UInt16, UInt32};
    let mut runningOrder: [Int32; 256] = [0; 256];
    let mut bigDone: [Bool; 256] = [0; 256];
    let mut copyStart: [Int32; 256] = [0; 256];
    let mut copyEnd: [Int32; 256] = [0; 256];
    if verb >= 4 {
        unsafe { crate::compat::fprintf(crate::compat::stderr, "        main sort initialise ...\n".as_ptr() as *const i8) };
    }
    unsafe {
        for i in 0..=65536 {
            *ftab.add(i as usize) = 0;
        }
    }
    let mut j = (unsafe { *block } as UInt16) << 8;
    let mut i = nblock - 1;
    while i >= 3 {
        unsafe {
            *quadrant.add(i as usize) = 0;
            j = (j >> 8) | ((*block.add(i as usize) as UInt16) << 8);
            *ftab.add(j as usize) += 1;
            *quadrant.add((i - 1) as usize) = 0;
            j = (j >> 8) | ((*block.add((i - 1) as usize) as UInt16) << 8);
            *ftab.add(j as usize) += 1;
            *quadrant.add((i - 2) as usize) = 0;
            j = (j >> 8) | ((*block.add((i - 2) as usize) as UInt16) << 8);
            *ftab.add(j as usize) += 1;
            *quadrant.add((i - 3) as usize) = 0;
            j = (j >> 8) | ((*block.add((i - 3) as usize) as UInt16) << 8);
            *ftab.add(j as usize) += 1;
        }
        i -= 4;
    }
    while i >= 0 {
        unsafe {
            *quadrant.add(i as usize) = 0;
            j = (j >> 8) | ((*block.add(i as usize) as UInt16) << 8);
            *ftab.add(j as usize) += 1;
        }
        i -= 1;
    }
    let overshoot = 2 + 12 + 18 + 2;
    for i in 0..overshoot {
        unsafe {
            *block.add((nblock + i) as usize) = *block.add(i as usize);
            *quadrant.add((nblock + i) as usize) = 0;
        }
    }
    if verb >= 4 {
        unsafe { crate::compat::fprintf(crate::compat::stderr, "        bucket sorting ...\n".as_ptr() as *const i8) };
    }
    unsafe {
        for i in 1..=65536 {
            *ftab.add(i as usize) += *ftab.add((i - 1) as usize);
        }
    }
    let mut s = (unsafe { *block } as UInt16) << 8;
    let mut i = nblock - 1;
    while i >= 3 {
        unsafe {
            s = (s >> 8) | ((*block.add(i as usize) as UInt16) << 8);
            let mut j = *ftab.add(s as usize) - 1;
            *ftab.add(s as usize) = j;
            *ptr.add(j as usize) = i as UInt32;
            s = (s >> 8) | ((*block.add((i - 1) as usize) as UInt16) << 8);
            j = *ftab.add(s as usize) - 1;
            *ftab.add(s as usize) = j;
            *ptr.add(j as usize) = (i - 1) as UInt32;
            s = (s >> 8) | ((*block.add((i - 2) as usize) as UInt16) << 8);
            j = *ftab.add(s as usize) - 1;
            *ftab.add(s as usize) = j;
            *ptr.add(j as usize) = (i - 2) as UInt32;
            s = (s >> 8) | ((*block.add((i - 3) as usize) as UInt16) << 8);
            j = *ftab.add(s as usize) - 1;
            *ftab.add(s as usize) = j;
            *ptr.add(j as usize) = (i - 3) as UInt32;
        }
        i -= 4;
    }
    while i >= 0 {
        unsafe {
            s = (s >> 8) | ((*block.add(i as usize) as UInt16) << 8);
            let mut j = *ftab.add(s as usize) - 1;
            *ftab.add(s as usize) = j;
            *ptr.add(j as usize) = i as UInt32;
        }
        i -= 1;
    }
    for i in 0..=255 {
        bigDone[i as usize] = 0;
        runningOrder[i as usize] = i as Int32;
    }
    {
        let mut h = 1;
        while h <= 256 {
            h = 3 * h + 1;
        }
        loop {
            h = h / 3;
            for i in h..=255 {
                let vv = runningOrder[i as usize];
                let mut j = i;
                while crate::src_blocksort::BIGFREQ(runningOrder[(j - h) as usize], ftab) > crate::src_blocksort::BIGFREQ(vv, ftab) {
                    runningOrder[j as usize] = runningOrder[(j - h) as usize];
                    j = j - h;
                    if j <= (h - 1) {
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
    let mut numQSorted: Int32 = 0;
    for i in 0..=255 {
        let ss = runningOrder[i as usize];
        for j in 0..=255 {
            if j != ss {
                let sb = (ss << 8) + j;
                unsafe {
                    let ftab_val = *ftab.add(sb as usize);
                    if (ftab_val as Int32 & crate::globals::SETMASK) == 0 {
                        let lo = (ftab_val as Int32 & crate::globals::CLEARMASK) as Int32;
                        let hi = ((*ftab.add((sb + 1) as usize) as Int32 & crate::globals::CLEARMASK) - 1) as Int32;
                        if hi > lo {
                            if verb >= 4 {
                                crate::compat::fprintf(crate::compat::stderr, "        qsort [0x%x, 0x%x]   done %d   this %d\n".as_ptr() as *const i8, ss, j, numQSorted, hi - lo + 1);
                            }
                            crate::src_blocksort::mainQSort3(ptr, block, quadrant, nblock, lo, hi, 2, budget);
                            numQSorted += hi - lo + 1;
                            if *budget < 0 {
                                return;
                            }
                        }
                    }
                    *ftab.add(sb as usize) |= crate::globals::SETMASK as UInt32;
                }
            }
        }
        unsafe {
            if !(!bigDone[ss as usize] != 0) {
                crate::src_bzlib::BZ2_bz__AssertH__fail(1006);
            }
        }
        unsafe {
            for j in 0..=255 {
                copyStart[j as usize] = (*ftab.add(((j << 8) + ss) as usize) as Int32 & crate::globals::CLEARMASK) as Int32;
                copyEnd[j as usize] = ((*ftab.add(((j << 8) + ss + 1) as usize) as Int32 & crate::globals::CLEARMASK) - 1) as Int32;
            }
            let start = (*ftab.add((ss << 8) as usize) as Int32 & crate::globals::CLEARMASK) as Int32;
            for j in start..copyStart[ss as usize] {
                let mut k = *ptr.add(j as usize) as Int32 - 1;
                if k < 0 {
                    k += nblock;
                }
                let c1 = *block.add(k as usize);
                if bigDone[c1 as usize] == 0 {
                    let idx = copyStart[c1 as usize];
                    *ptr.add(idx as usize) = k as UInt32;
                    copyStart[c1 as usize] = idx + 1;
                }
            }
            let end = ((*ftab.add(((ss + 1) << 8) as usize) as Int32 & crate::globals::CLEARMASK) - 1) as Int32;
            let mut j = end;
            while j > copyEnd[ss as usize] {
                let mut k = *ptr.add(j as usize) as Int32 - 1;
                if k < 0 {
                    k += nblock;
                }
                let c1 = *block.add(k as usize);
                if bigDone[c1 as usize] == 0 {
                    let idx = copyEnd[c1 as usize];
                    *ptr.add(idx as usize) = k as UInt32;
                    copyEnd[c1 as usize] = idx - 1;
                }
                j -= 1;
            }
        }
        unsafe {
            if !((copyStart[ss as usize] - 1 == copyEnd[ss as usize]) || (copyStart[ss as usize] == 0 && copyEnd[ss as usize] == nblock - 1)) {
                crate::src_bzlib::BZ2_bz__AssertH__fail(1007);
            }
        }
        unsafe {
            for j in 0..=255 {
                *ftab.add(((j << 8) + ss) as usize) |= crate::globals::SETMASK as UInt32;
            }
        }
        bigDone[ss as usize] = 1;
        if i < 255 {
            unsafe {
                let bbStart = (*ftab.add((ss << 8) as usize) as Int32 & crate::globals::CLEARMASK) as Int32;
                let bbSize = ((*ftab.add(((ss + 1) << 8) as usize) as Int32 & crate::globals::CLEARMASK) - bbStart) as Int32;
                let mut shifts = 0;
                while (bbSize >> shifts) > 65534 {
                    shifts += 1;
                }
                for j in (0..bbSize).rev() {
                    let a2update = *ptr.add((bbStart + j) as usize) as Int32;
                    let qVal = (j >> shifts) as UInt16;
                    *quadrant.add(a2update as usize) = qVal;
                    if a2update < overshoot {
                        *quadrant.add((a2update + nblock) as usize) = qVal;
                    }
                }
                if !(((bbSize - 1) >> shifts) <= 65535) {
                    crate::src_bzlib::BZ2_bz__AssertH__fail(1002);
                }
            }
        }
    }
    if verb >= 4 {
        unsafe { crate::compat::fprintf(crate::compat::stderr, "        %d pointers, %d sorted, %d scanned\n".as_ptr() as *const i8, nblock, numQSorted, nblock - numQSorted) };
    }
}

pub extern "C" fn BZ2_blockSort(arg1: *mut crate::types::EState) {
    let s = arg1;
    unsafe {
        let ptr = crate::compat::c2r_field_ptr_EState__ptr(s as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
        let block = crate::compat::c2r_field_ptr_EState__block(s as *mut ::core::ffi::c_void) as *mut crate::types::UChar;
        let ftab = crate::compat::c2r_field_ptr_EState__ftab(s as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
        let nblock_ptr = crate::compat::c2r_field_ptr_EState__nblock(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
        let nblock = *nblock_ptr;
        let verb_ptr = crate::compat::c2r_field_ptr_EState__verbosity(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
        let verb = *verb_ptr;
        let wfact_ptr = crate::compat::c2r_field_ptr_EState__workFactor(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
        let mut wfact = *wfact_ptr;
        let mut quadrant: *mut crate::types::UInt16 = std::ptr::null_mut();
        let mut budget: crate::types::Int32 = 0;
        let mut budgetInit: crate::types::Int32 = 0;
        let mut i: crate::types::Int32 = 0;

        if nblock < 10000 {
            let arr1 = crate::compat::c2r_field_ptr_EState__arr1(s as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
            let arr2 = crate::compat::c2r_field_ptr_EState__arr2(s as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
            crate::src_blocksort::fallbackSort(arr1, arr2, ftab, nblock, verb);
        } else {
            i = nblock + (2 + 12 + 18 + 2);
            if i & 1 != 0 {
                i += 1;
            }
            quadrant = block.offset(i as isize) as *mut crate::types::UInt16;

            if wfact < 1 {
                wfact = 1;
            }
            if wfact > 100 {
                wfact = 100;
            }
            budgetInit = nblock * ((wfact - 1) / 3);
            budget = budgetInit;

            crate::src_blocksort::mainSort(ptr, block, quadrant, ftab, nblock, verb, &mut budget);
            if verb >= 3 {
                let ratio = if nblock == 0 { 1 } else { nblock };
                crate::compat::fprintf(
                    crate::compat::stderr,
                    b"      %d work, %d block, ratio %5.2f\n\0".as_ptr() as *const i8,
                    budgetInit - budget,
                    nblock,
                    ((budgetInit - budget) as f64) / (ratio as f64),
                );
            }

            if budget < 0 {
                if verb >= 2 {
                    crate::compat::fprintf(
                        crate::compat::stderr,
                        b"    too repetitive; using fallback sorting algorithm\n\0".as_ptr() as *const i8,
                    );
                }
                let arr1 = crate::compat::c2r_field_ptr_EState__arr1(s as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                let arr2 = crate::compat::c2r_field_ptr_EState__arr2(s as *mut ::core::ffi::c_void) as *mut crate::types::UInt32;
                crate::src_blocksort::fallbackSort(arr1, arr2, ftab, nblock, verb);
            }
        }

        let orig_ptr_ptr = crate::compat::c2r_field_ptr_EState__origPtr(s as *mut ::core::ffi::c_void) as *mut crate::types::Int32;
        *orig_ptr_ptr = -1;
        for i in 0..nblock {
            if *ptr.offset(i as isize) == 0 {
                *orig_ptr_ptr = i;
                break;
            }
        }
        if !(*orig_ptr_ptr != -1) {
            crate::src_bzlib::BZ2_bz__AssertH__fail(1003);
        }
    }
}
