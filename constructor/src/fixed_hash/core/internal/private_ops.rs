// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Define private methods about operators.

use fixed_hash::HashConstructor;
use utils;

impl HashConstructor {
    pub fn defun_priv_ops(&self) {
        self.defun_priv_bitwise();
        self.defun_priv_not();
        self.defun_priv_shift();
    }

    fn defun_priv_bitwise(&self) {
        let inner_type = &self.ts.inner_type;
        let loop_unit_amount = &utils::pure_uint_list_to_ts(0..self.info.unit_amount);
        let part = if self.info.expand {
            quote!(
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
            )
        } else {
            quote!(
                #[inline]
                fn _bitand(&self, rhs: &Self) -> Self {
                    let mut ret: #inner_type = unsafe { ::std::mem::uninitialized() };
                    let rhs = rhs.inner();
                    for (idx, lhs) in self.inner().iter().enumerate() {
                        ret[idx] = lhs & rhs[idx];
                    }
                    Self::new(ret)
                }
                #[inline]
                fn _bitor(&self, rhs: &Self) -> Self {
                    let mut ret: #inner_type = unsafe { ::std::mem::uninitialized() };
                    let rhs = rhs.inner();
                    for (idx, lhs) in self.inner().iter().enumerate() {
                        ret[idx] = lhs | rhs[idx];
                    }
                    Self::new(ret)
                }
                #[inline]
                fn _bitxor(&self, rhs: &Self) -> Self {
                    let mut ret: #inner_type = unsafe { ::std::mem::uninitialized() };
                    let rhs = rhs.inner();
                    for (idx, lhs) in self.inner().iter().enumerate() {
                        ret[idx] = lhs ^ rhs[idx];
                    }
                    Self::new(ret)
                }
            )
        };
        self.defun(part);
    }

    fn defun_priv_not(&self) {
        let inner_type = &self.ts.inner_type;
        let loop_unit_amount = &utils::pure_uint_list_to_ts(0..self.info.unit_amount);
        let part = if self.info.expand {
            quote!(
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
            )
        } else {
            quote!(
                #[inline]
                fn _not(&self) -> Self {
                    let mut ret: #inner_type = unsafe { ::std::mem::uninitialized() };
                    let inner = self.inner();
                    for (idx, val) in self.inner().iter().enumerate() {
                        ret[idx] = !val;
                    }
                    Self::new(ret)
                }
            )
        };
        self.defun(part);
    }

    fn defun_priv_shift(&self) {
        let bits_size = &self.ts.bits_size;
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
                    let bit_offset = (rhs & 0x8) as usize;
                    let unit_offset = (rhs / 8) as usize;
                    let mut idx = unit_offset as usize;
                    if bit_offset == 0 {
                        ret[idx] = src[0];
                        idx += 1;
                        while idx < #unit_amount {
                            ret[idx] = src[idx - unit_offset];
                            idx += 1;
                        }
                    } else {
                        let bit_cover = 8 - bit_offset;
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
                    let bit_offset = (rhs & 0x8) as usize;
                    let unit_offset = (rhs / 8) as usize;
                    let mut idx = 0;
                    if bit_offset == 0 {
                        while idx < #unit_amount - unit_offset - 1 {
                            ret[idx] = src[idx + unit_offset];
                            idx += 1;
                        }
                        ret[idx] = src[idx + unit_offset];
                    } else {
                        let bit_cover = 8 - bit_offset;
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
