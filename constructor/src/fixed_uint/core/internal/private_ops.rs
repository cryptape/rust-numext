// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Define private methods about operators.

use crate::fixed_uint::UintConstructor;
use crate::utils;
use quote::quote;

impl UintConstructor {
    pub fn defun_priv_ops(&self) {
        self.defun_priv_unit_ops();
        self.defun_priv_add();
        self.defun_priv_sub();
        self.defun_priv_mul();
        self.defun_priv_full_mul();
        self.defun_priv_div_and_rem();
        self.defun_priv_bitwise();
        self.defun_priv_not();
        self.defun_priv_shift();
    }

    fn defun_priv_unit_ops(&self) {
        let unit_bits_size = &self.ts.unit_bits_size;
        let unit_amount = &self.ts.unit_amount;
        let unit_suffix = &self.ts.unit_suffix;
        let double_unit_suffix = &self.ts.double_unit_suffix;
        let inner_type = &self.ts.inner_type;
        let loop_unit_bits_size = &vec![unit_bits_size; self.info.unit_amount as usize];
        let loop_unit_amount = &utils::pure_uint_list_to_ts(0..self.info.unit_amount);
        let loop_unit_amount_rev = &utils::pure_uint_list_to_ts((0..self.info.unit_amount).rev());
        let loop_unit_suffix = &vec![unit_suffix; self.info.unit_amount as usize];
        let loop_unit_suffix_clone = &loop_unit_suffix.clone();
        let loop_double_unit_suffix = &vec![double_unit_suffix; self.info.unit_amount as usize];
        let part = quote!(
            #[inline]
            fn _unit(&self, index: usize) -> Option<#unit_suffix> {
                if index >= #unit_amount {
                    None
                } else {
                    let inner = self.inner();
                    Some(inner[index])
                }
            }
            #[inline]
            fn _set_unit(&mut self, index: usize, value: #unit_suffix) -> bool {
                if index >= #unit_amount {
                    false
                } else {
                    let inner = self.mut_inner();
                    inner[index] = value;
                    true
                }
            }
            #[inline]
            fn _highest_nonzero_unit(&self) -> Option<usize> {
                let inner = self.inner();
                #({
                    let idx: usize = #loop_unit_amount_rev;
                    let v = inner[idx];
                    if v != 0 {
                        return Some(idx);
                    }
                })*
                None
            }
            #[inline]
            fn _lowest_nonzero_unit(&self) -> Option<usize> {
                let inner = self.inner();
                #({
                    let idx: usize = #loop_unit_amount;
                    let v = inner[idx];
                    if v != 0 {
                        return Some(idx);
                    }
                })*
                None
            }
            #[inline]
            fn _mul_unit(&self, other: #unit_suffix) -> (Self, bool) {
                let mut ret: #inner_type = [0; #unit_amount];
                let inner = self.inner();
                let other = other as #double_unit_suffix;
                let mut hi: #unit_suffix = 0;
                #({
                    let idx: usize = #loop_unit_amount;
                    let val = inner[idx];
                    if val == 0 {
                        let ret_val = &mut ret[idx];
                        *ret_val = hi;
                        hi = 0;
                    } else {
                        let prod = (val as #loop_double_unit_suffix) * other;
                        let lo = prod as #loop_unit_suffix;
                        let ret_val = &mut ret[idx];
                        let (lo, of) = lo.overflowing_add(hi);
                        *ret_val = lo;
                        hi = (prod >> #loop_unit_bits_size) as #loop_unit_suffix_clone;
                        if of {
                            hi += 1;
                        }
                    }
                })*
                (Self::new(ret), hi > 0)
            }
            #[inline]
            fn _div_unit_with_rem(&self, other: #unit_suffix) -> (Self, #unit_suffix) {
                // check this condition before call this function
                if other == 0 {
                    unreachable!();
                };
                let mut lhs_idx = if let Some(idx) = self._highest_nonzero_unit() {
                    idx
                } else {
                    return (Self::zero(), 0);
                };
                if lhs_idx == 0 {
                    return if self.inner()[0] < other {
                        (Self::zero(), self.inner()[0])
                    } else {
                        let mut z = Self::zero();
                        z._set_unit(0, self.inner()[0] / other);
                        (z, self.inner()[0] % other)
                    };
                }

                let mut ret: #inner_type = [0; #unit_amount];

                let mut copy = self.clone();
                let mut ret_idx = lhs_idx;
                let divisor = other as #double_unit_suffix;

                loop {
                    let lhs_highest = copy.inner()[lhs_idx] as #double_unit_suffix;
                    // if lhs highest byte is ZERO, the skip it
                    if lhs_highest == 0 {
                        if ret_idx == 0 {
                            break;
                        }
                        ret_idx -= 1;
                        lhs_idx -= 1;
                        continue;
                    }
                    // estimate highest byte of quotient
                    let dividend = if lhs_highest < divisor {
                        if ret_idx == 0 {
                            break;
                        }
                        lhs_idx -= 1;
                        ret_idx -= 1;
                        (lhs_highest << #unit_bits_size) + copy.inner()[lhs_idx] as #double_unit_suffix
                    } else {
                        lhs_highest
                    };
                    let quotient = (dividend / divisor) as #unit_suffix;
                    ret[ret_idx] = quotient;
                    let remainder = (dividend % divisor) as #unit_suffix;
                    // the copy[lhs_idx+1] is dirty
                    copy.mut_inner()[lhs_idx] = remainder;
                }
                (Self::new(ret), copy.inner()[0])
            }
        );
        self.defun(part);
    }

    fn defun_priv_add(&self) {
        let unit_suffix = &self.ts.unit_suffix;
        let inner_type = &self.ts.inner_type;
        let mut loop_part = quote!();
        for i in 0..self.info.unit_amount {
            let i = utils::pure_uint_to_ts(i);
            let loop_part_tmp = quote!(
                of = if of {
                    let (val_n, of_n) = #unit_suffix::overflowing_add(lhs[#i], rhs[#i]);
                    // The carry only can be one.
                    let (val_o, of_o) = #unit_suffix::overflowing_add(val_n, 1);
                    unsafe { ::std::ptr::write(ret_ptr.offset(#i), val_o); }
                    // Can not overflow twice.
                    of_n || of_o
                } else {
                    let (val_n, of_n) = #unit_suffix::overflowing_add(lhs[#i], rhs[#i]);
                    unsafe { ::std::ptr::write(ret_ptr.offset(#i), val_n); }
                    of_n
                };
            );
            loop_part = quote!(#loop_part #loop_part_tmp);
        }
        let part = quote!(
            #[inline]
            fn _add(&self, other: &Self) -> (Self, bool) {
                let lhs = self.inner();
                let rhs = other.inner();
                let mut ret: #inner_type = unsafe { ::std::mem::uninitialized() };
                let ret_ptr = &mut ret as *mut #inner_type as *mut #unit_suffix;
                let mut of = false;
                #loop_part
                (Self::new(ret), of)
            }
        );
        self.defun(part);
    }

    fn defun_priv_sub(&self) {
        let inner_type = &self.ts.inner_type;
        let unit_suffix = &self.ts.unit_suffix;
        let mut loop_part = quote!();
        for i in 0..self.info.unit_amount {
            let i = utils::pure_uint_to_ts(i);
            let loop_part_tmp = quote!(
                of = if of {
                    let (val_n, of_n) = #unit_suffix::overflowing_sub(lhs[#i], rhs[#i]);
                    // The carry only can be one.
                    let (val_o, of_o) = #unit_suffix::overflowing_sub(val_n, 1);
                    unsafe { ::std::ptr::write(ret_ptr.offset(#i), val_o); }
                    // Can not overflow twice.
                    of_n || of_o
                } else {
                    let (val_n, of_n) = #unit_suffix::overflowing_sub(lhs[#i], rhs[#i]);
                    unsafe { ::std::ptr::write(ret_ptr.offset(#i), val_n); }
                    of_n
                };
            );
            loop_part = quote!(#loop_part #loop_part_tmp);
        }
        let part = quote!(
            #[inline]
            fn _sub(&self, other: &Self) -> (Self, bool) {
                let lhs = self.inner();
                let rhs = other.inner();
                let mut ret: #inner_type = unsafe { ::std::mem::uninitialized() };
                let ret_ptr = &mut ret as *mut #inner_type as *mut #unit_suffix;
                let mut of = false;
                #loop_part
                (Self::new(ret), of)
            }
        );
        self.defun(part);
    }

    fn defun_priv_mul(&self) {
        let inner_type = &self.ts.inner_type;
        let unit_bits_size = &self.ts.unit_bits_size;
        let unit_amount = &self.ts.unit_amount;
        let unit_suffix = &self.ts.unit_suffix;
        let double_unit_suffix = &self.ts.double_unit_suffix;
        let part = quote!(
            #[inline]
            fn _mul(&self, other: &Self) -> (Self, bool) {
                let (lidx_max, ridx_max) = {
                    let lidx_max_opt = self._highest_nonzero_unit();
                    let ridx_max_opt = other._highest_nonzero_unit();
                    if lidx_max_opt.is_none() || ridx_max_opt.is_none() {
                        return (Self::zero(), false)
                    }
                    (lidx_max_opt.unwrap(), ridx_max_opt.unwrap())
                };
                let mut ret: #inner_type = [0; #unit_amount];
                let lhs = self.inner();
                let rhs = other.inner();
                let mut of = (lidx_max + ridx_max) >= #unit_amount;
                let mut lidx = 0;
                while lidx <= lidx_max {
                    if lhs[lidx] == 0 {
                        lidx += 1;
                        continue;
                    }
                    let mut ridx = 0;
                    while ridx <= ridx_max {
                        if rhs[ridx] == 0 {
                            ridx += 1;
                            continue;
                        }

                        let mut k = lidx + ridx;

                        // store low part of current result
                        if k >= #unit_amount { break; }

                        let lx = lhs[lidx] as #double_unit_suffix;
                        let rx = rhs[ridx] as #double_unit_suffix;
                        let val = lx * rx;

                        let val_lo = val as #unit_suffix;
                        let val_hi = (val >> #unit_bits_size) as #unit_suffix;

                        let of_lo = {
                            let ret_lo = &mut ret[k];
                            let (val_lo, of_lo) = val_lo.overflowing_add(*ret_lo);
                            *ret_lo = val_lo;
                            of_lo
                        };

                        if (!of_lo) && val_hi == 0 {
                            ridx += 1;
                            continue;
                        }

                        // store high part of current result
                        k += 1;
                        if k == #unit_amount {
                            of = true;
                            break;
                        }

                        let mut of_hi = {
                            let ret_hi = &mut ret[k];
                            let (mut val_hi, mut of_hi) = val_hi.overflowing_add(*ret_hi);
                            if of_lo {
                                let (val_tmp, of_tmp) = val_hi.overflowing_add(1);
                                val_hi = val_tmp;
                                of_hi = of_hi || of_tmp;
                            }
                            *ret_hi = val_hi;
                            of_hi
                        };

                        // store the overflow part
                        k += 1;
                        while (of_hi && k != #unit_amount) {
                            let ret_hi = &mut ret[k];
                            let (val_tmp, of_tmp) = (*ret_hi).overflowing_add(1);
                            *ret_hi = val_tmp;
                            of_hi = of_tmp;
                            k += 1;
                        }

                        if k == #unit_amount {
                            of = of || of_hi;
                        }
                        ridx += 1;
                    }
                    lidx += 1;
                }
                (Self::new(ret), of)
            }
        );
        self.defun(part);
    }

    fn defun_priv_full_mul(&self) {
        let inner_type = &self.ts.inner_type;
        let unit_bits_size = &self.ts.unit_bits_size;
        let unit_amount = &self.ts.unit_amount;
        let unit_suffix = &self.ts.unit_suffix;
        let double_unit_suffix = &self.ts.double_unit_suffix;
        let loop_unit_amount = &utils::pure_uint_list_to_ts(0..self.info.unit_amount);
        let part = quote!(
            #[inline]
            fn _full_mul(&self, other: &Self) -> (Self, Self) {
                let (lidx_max, ridx_max) = {
                    let lidx_max_opt = self._highest_nonzero_unit();
                    let ridx_max_opt = other._highest_nonzero_unit();
                    if lidx_max_opt.is_none() || ridx_max_opt.is_none() {
                        return (Self::zero(), Self::zero())
                    }
                    (lidx_max_opt.unwrap(), ridx_max_opt.unwrap())
                };
                if lidx_max + ridx_max > #unit_amount * 2 {
                    unreachable!();
                }
                let mut ret: [#unit_suffix; #unit_amount * 2] = [0; #unit_amount * 2];
                let lhs = self.inner();
                let rhs = other.inner();
                let mut lidx = 0;
                while lidx <= lidx_max {
                    if lhs[lidx] == 0 {
                        lidx += 1;
                        continue;
                    }
                    let mut ridx = 0;
                    while ridx <= ridx_max {
                        if rhs[ridx] == 0 {
                            ridx += 1;
                            continue;
                        }

                        let mut k = lidx + ridx;

                        let lx = lhs[lidx] as #double_unit_suffix;
                        let rx = rhs[ridx] as #double_unit_suffix;
                        let val = lx * rx;

                        let val_lo = val as #unit_suffix;
                        let val_hi = (val >> #unit_bits_size) as #unit_suffix;

                        let of_lo = {
                            let ret_lo = &mut ret[k];
                            let (val_lo, of_lo) = val_lo.overflowing_add(*ret_lo);
                            *ret_lo = val_lo;
                            of_lo
                        };

                        if (!of_lo) && val_hi == 0 {
                            ridx += 1;
                            continue;
                        }

                        k += 1;

                        let mut of_hi = {
                            let ret_hi = &mut ret[k];
                            let (mut val_hi, mut of_hi) = val_hi.overflowing_add(*ret_hi);
                            if of_lo {
                                let (val_tmp, of_tmp) = val_hi.overflowing_add(1);
                                val_hi = val_tmp;
                                of_hi = of_hi || of_tmp;
                            }
                            *ret_hi = val_hi;
                            of_hi
                        };

                        k += 1;
                        while of_hi {
                            let ret_hi = &mut ret[k];
                            let (val_tmp, of_tmp) = (*ret_hi).overflowing_add(1);
                            *ret_hi = val_tmp;
                            of_hi = of_tmp;
                            k += 1;
                        }

                        ridx += 1;
                    }
                    lidx += 1;
                }
                let mut low: #inner_type = [0; #unit_amount];
                let mut high: #inner_type = [0; #unit_amount];
                unsafe {
                    let mut ret_ptr = ret.as_ptr();
                    let mut low_ptr = low.as_mut_ptr();
                    #(
                        let _ = #loop_unit_amount;
                        *low_ptr = *ret_ptr;
                        ret_ptr = ret_ptr.offset(1);
                        low_ptr = low_ptr.offset(1);
                    )*
                    let mut high_ptr = high.as_mut_ptr();
                    #(
                        let _ = #loop_unit_amount;
                        *high_ptr = *ret_ptr;
                        ret_ptr = ret_ptr.offset(1);
                        high_ptr = high_ptr.offset(1);
                    )*
                }
                (Self::new(low), Self::new(high))
            }
        );
        self.defun(part);
    }

    fn defun_priv_div_and_rem(&self) {
        let unit_amount = &self.ts.unit_amount;
        let unit_bits_size = &self.ts.unit_bits_size;
        let unit_suffix = &self.ts.unit_suffix;
        let double_unit_suffix = &self.ts.double_unit_suffix;
        let inner_type = &self.ts.inner_type;
        let part = quote!(
            #[inline]
            fn _div_with_rem(&self, other: &Self) -> Option<(Self, Self)> {
                if self < other {
                    // other > self >= zero
                    return Some((Self::zero(), self.clone()));
                }
                // below: self >= other
                let rhs_idx = if let Some(idx) = other._highest_nonzero_unit() {
                    idx
                } else {
                    return None;
                };
                // below: self >= other > zero
                let mut lhs_idx = if let Some(idx) = self._highest_nonzero_unit() {
                    idx
                } else {
                    unreachable!();
                };
                if lhs_idx < rhs_idx {
                    unreachable!();
                }

                let mut ret: #inner_type = [0; #unit_amount];

                let mut copy = self.clone();
                let mut ret_idx = lhs_idx - rhs_idx;
                // lhs_idx >= ret_idx since rhs_idx >= 0
                let rhs = other.inner();
                let rhs_highest = rhs[rhs_idx] as #double_unit_suffix;

                loop {
                    let lhs_highest = copy.inner()[lhs_idx] as #double_unit_suffix;
                    // if lhs highest byte is ZERO, the skip it
                    if lhs_highest == 0 {
                        if ret_idx == 0 { break; }
                        ret_idx -= 1;
                        lhs_idx -= 1;
                        continue;
                    }
                    // below: ret_idx > 0
                    // estimate highest byte of quotient
                    let divisor = rhs_highest + 1;
                    let dividend = if lhs_highest >= divisor {
                        lhs_highest
                    } else {
                        if ret_idx == 0 { break; }
                        lhs_idx -= 1;
                        ret_idx -= 1;
                        (lhs_highest << #unit_bits_size) + copy.inner()[lhs_idx] as #double_unit_suffix
                    };
                    let quotient = (dividend / divisor) as #unit_suffix;
                    let of = {
                        let ret_tmp = &mut ret[ret_idx];
                        let (tmp, of) = quotient.overflowing_add(*ret_tmp);
                        *ret_tmp = tmp;
                        of
                    };
                    if of {
                        // `ret[ret_idx+1]+1` could not overflow
                        ret[ret_idx+1] += 1;
                    }
                    let minuend = {
                        let (mut minuend_tmp, _) = other._mul_unit(quotient);
                        // left shift
                        let mut idx = #unit_amount - 1;
                        while idx > ret_idx {
                            minuend_tmp.mut_inner()[idx] = minuend_tmp.inner()[idx-ret_idx];
                            idx -= 1;
                        }
                        minuend_tmp.mut_inner()[ret_idx] = minuend_tmp.inner()[0];
                        let mut idx = 0;
                        while idx < ret_idx {
                            minuend_tmp.mut_inner()[idx] = 0;
                            idx += 1;
                        }
                        minuend_tmp
                    };
                    let (copy_new, _) = copy._sub(&minuend);
                    copy = copy_new;
                }
                let mut more: #unit_suffix = 0;
                while copy >= *other {
                    let (copy_new, _) = copy._sub(other);
                    copy = copy_new;
                    more += 1;
                }
                {
                    let mut of = {
                        let ret_tmp = &mut ret[0];
                        let (tmp, of) = more.overflowing_add(*ret_tmp);
                        *ret_tmp = tmp;
                        of
                    };
                    let mut idx = 1;
                    while idx < #unit_amount {
                        if of {
                            let ret_tmp = &mut ret[idx];
                            let (tmp, of_tmp) = ret_tmp.overflowing_add(1);
                            *ret_tmp = tmp;
                            of = of_tmp;
                            idx += 1;
                        } else {
                            break;
                        }
                    }
                }
                Some((Self::new(ret), copy))
            }

            #[inline]
            fn _div(&self, other: &Self) -> (Self, bool) {
                if let Some((q, _r)) = self._div_with_rem(other) {
                    (q, false)
                } else {
                    (Self::default(), true)
                }
            }

            #[inline]
            fn _rem(&self, other: &Self) -> (Self, bool) {
                if let Some((_q, r)) = self._div_with_rem(other) {
                    (r, false)
                } else {
                    (Self::default(), true)
                }
            }
        );
        self.defun(part);
    }

    fn defun_priv_bitwise(&self) {
        let inner_type = &self.ts.inner_type;
        let loop_unit_amount = &utils::pure_uint_list_to_ts(0..self.info.unit_amount);
        let part = quote!(
            #[inline]
            fn _bitand(&self, rhs: &Self) -> Self {
                let mut ret: #inner_type = unsafe { ::std::mem::uninitialized() };
                let inner = self.inner();
                let rhs = rhs.inner();
                #({
                    let idx = #loop_unit_amount;
                    ret[idx] = inner[idx] & rhs[idx];
                })*
                Self::new(ret)
            }
            #[inline]
            fn _bitor(&self, rhs: &Self) -> Self {
                let mut ret: #inner_type = unsafe { ::std::mem::uninitialized() };
                let inner = self.inner();
                let rhs = rhs.inner();
                #({
                    let idx = #loop_unit_amount;
                    ret[idx] = inner[idx] | rhs[idx];
                })*
                Self::new(ret)
            }
            #[inline]
            fn _bitxor(&self, rhs: &Self) -> Self {
                let mut ret: #inner_type = unsafe { ::std::mem::uninitialized() };
                let inner = self.inner();
                let rhs = rhs.inner();
                #({
                    let idx = #loop_unit_amount;
                    ret[idx] = inner[idx] ^ rhs[idx];
                })*
                Self::new(ret)
            }
        );
        self.defun(part);
    }

    fn defun_priv_not(&self) {
        let inner_type = &self.ts.inner_type;
        let loop_unit_amount = &utils::pure_uint_list_to_ts(0..self.info.unit_amount);
        let part = quote!(
            #[inline]
            fn _not(&self) -> Self {
                let mut ret: #inner_type = unsafe { ::std::mem::uninitialized() };
                let inner = self.inner();
                #({
                    let idx = #loop_unit_amount;
                    ret[idx] = !inner[idx];
                })*
                Self::new(ret)
            }
        );
        self.defun(part);
    }

    fn defun_priv_shift(&self) {
        let bits_size = &self.ts.bits_size;
        let unit_bits_size = &self.ts.unit_bits_size;
        let unit_amount = &self.ts.unit_amount;
        let inner_type = &self.ts.inner_type;
        let part = quote!(
            #[inline]
            fn _ishl(&self, rhs: i128) -> Self {
                match rhs {
                    val if val > 0 => self._ushl(val as u128),
                    val if val < 0 => self._ushr((-val) as u128),
                    _ => self.clone(),
                }
            }
            #[inline]
            fn _ishr(&self, rhs: i128) -> Self {
                match rhs {
                    val if val > 0 => self._ushr(val as u128),
                    val if val < 0 => self._ushl((-val) as u128),
                    _ => self.clone(),
                }
            }
            #[inline]
            fn _ushl(&self, rhs: u128) -> Self {
                if rhs == 0 {
                    return self.clone();
                }
                if rhs < #bits_size {
                    let mut ret: #inner_type = [0; #unit_amount];
                    let src = self.inner();
                    let bit_offset = (rhs % #unit_bits_size) as usize;
                    let unit_offset = (rhs / #unit_bits_size) as usize;
                    let mut idx = unit_offset as usize;
                    if bit_offset == 0 {
                        ret[idx] = src[0];
                        idx += 1;
                        while idx < #unit_amount {
                            ret[idx] = src[idx - unit_offset];
                            idx += 1;
                        }
                    } else {
                        let bit_cover = #unit_bits_size - bit_offset;
                        ret[idx] = src[0] << bit_offset;
                        idx += 1;
                        while idx < #unit_amount {
                            ret[idx] = (src[idx - unit_offset] << bit_offset)
                                | (src[idx - unit_offset - 1] >> bit_cover);
                            idx += 1;
                        }
                    }
                    Self::new(ret)
                } else {
                    Self::zero()
                }
            }
            #[inline]
            fn _ushr(&self, rhs: u128) -> Self {
                if rhs == 0 {
                    return self.clone();
                }
                if rhs < #bits_size {
                    let mut ret: #inner_type = [0; #unit_amount];
                    let src = self.inner();
                    let bit_offset = (rhs % #unit_bits_size) as usize;
                    let unit_offset = (rhs / #unit_bits_size) as usize;
                    let mut idx = 0;
                    if bit_offset == 0 {
                        while idx < #unit_amount - unit_offset - 1 {
                            ret[idx] = src[idx + unit_offset];
                            idx += 1;
                        }
                        ret[idx] = src[idx + unit_offset];
                    } else {
                        let bit_cover = #unit_bits_size - bit_offset;
                        while idx < #unit_amount - unit_offset - 1 {
                            ret[idx] = (src[idx + unit_offset] >> bit_offset)
                                | (src[idx + unit_offset + 1] << bit_cover);
                            idx += 1;
                        }
                        ret[idx] = src[idx + unit_offset] >> bit_offset;
                    }
                    Self::new(ret)
                } else {
                    Self::zero()
                }
            }
        );
        self.defun(part);
    }
}
