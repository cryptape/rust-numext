// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Define public methods about bits and bytes.

use super::super::constructor::UintConstructor;
use super::super::utils;

impl UintConstructor {
    pub fn defun_pub_basic(&self) {
        self.defun_pub_bits_ops();
        self.defun_pub_bytes_ops();
        self.defun_pub_arith_ops();
    }

    fn defun_pub_bits_ops(&self) {
        let bits_size = &self.ts.bits_size;
        let unit_bits_size = &self.ts.unit_bits_size;
        let loop_unit_amount = &utils::pure_uint_list_to_ts(0..self.info.unit_amount);
        let loop_unit_amount_rev = &utils::pure_uint_list_to_ts((0..self.info.unit_amount).rev());
        let loop_unit_bits_size = &vec![unit_bits_size; self.info.unit_amount as usize];
        let part = quote!(
            /// Return the count of bits.
            #[inline]
            pub fn count_bits() -> u64 {
                #bits_size
            }
            /// Return a specific bit, or return None when overlows.
            #[inline]
            pub fn bit(&self, index: usize) -> Option<bool> {
                if index >= #bits_size {
                    None
                } else {
                    let inner = self.inner();
                    let unit_idx = index / #unit_bits_size;
                    let bit_idx = index % #unit_bits_size;
                    Some(inner[unit_idx] & (1 << bit_idx) != 0)
                }
            }
            /// Set a specific bit.
            /// Return false when overflows.
            #[inline]
            pub fn set_bit(&mut self, index: usize, value: bool) -> bool {
                if index >= #bits_size {
                    false
                } else {
                    let inner = self.mut_inner();
                    let unit_idx = index / #unit_bits_size;
                    let bit_idx = index % #unit_bits_size;
                    if value {
                        inner[unit_idx] |= 1 << bit_idx;
                    } else {
                        inner[unit_idx] &= !(1 << bit_idx);
                    }
                    true
                }
            }
            /// Return the highest bit which is one.
            #[inline]
            pub fn highest_one(&self) -> Option<usize> {
                let inner = self.inner();
                #({
                    let idx = #loop_unit_amount_rev;
                    let v = inner[idx];
                    if v != 0 {
                        let x = #loop_unit_bits_size * (idx + 1) - 1;
                        let y = v.leading_zeros() as usize;
                        return Some(x - y);
                    }
                })*
                None
            }
            /// Return the lowest bit which is one.
            #[inline]
            pub fn lowest_one(&self) -> Option<usize> {
                let inner = self.inner();
                #({
                    let idx = #loop_unit_amount;
                    let v = inner[idx];
                    if v != 0 {
                        let x = #loop_unit_bits_size * idx;
                        let y = v.trailing_zeros() as usize;
                        return Some(x + y);
                    }
                })*
                None
            }
        );
        self.defun(part);
    }

    fn defun_pub_bytes_ops(&self) {
        let bytes_size = &self.ts.bytes_size;
        let unit_bytes_size = &self.ts.unit_bytes_size;
        let unit_suffix = &self.ts.unit_suffix;
        let loop_unit_amount = &utils::pure_uint_list_to_ts(0..self.info.unit_amount);
        let loop_unit_amount_rev = &utils::pure_uint_list_to_ts((0..self.info.unit_amount).rev());
        let loop_unit_bytes_size = &vec![unit_bytes_size; self.info.unit_amount as usize];
        let part = quote!(
            /// Return the count of bytes.
            #[inline]
            pub fn count_bytes() -> u64 {
                #bytes_size
            }
            /// Return a specific byte, or return None when overlows.
            #[inline]
            pub fn byte(&self, index: usize) -> Option<u8> {
                if index >= #bytes_size {
                    None
                } else {
                    let inner = self.inner();
                    let unit_bytes_size = #unit_bytes_size;
                    let unit_idx = index / unit_bytes_size;
                    let byte_idx = index % unit_bytes_size;
                    let v = (inner[unit_idx] >> (8 * byte_idx)) & 0xff;
                    Some(v as u8)
                }
            }
            /// Set a specific byte.
            /// Return false when overflows;
            #[inline]
            pub fn set_byte(&mut self, index: usize, byte: u8) -> bool {
                if index >= #bytes_size {
                    false
                } else {
                    let inner = self.mut_inner();
                    let unit_bytes_size = #unit_bytes_size;
                    let unit_idx = index / unit_bytes_size;
                    let byte_idx = index % unit_bytes_size;
                    inner[unit_idx] &= !((0xff as #unit_suffix) << (8 * byte_idx));
                    inner[unit_idx] |= (byte as #unit_suffix) << (8 * byte_idx);
                    true
                }
            }
            /// Return the highest byte which is nonzero.
            #[inline]
            pub fn highest_nonzero_byte(&self) -> Option<usize> {
                let inner = self.inner();
                #({
                    let idx: usize = #loop_unit_amount_rev;
                    let v = inner[idx];
                    if v != 0 {
                        let x = #loop_unit_bytes_size * (idx + 1) - 1;
                        let y = v.leading_zeros() as usize;
                        return Some(x - y / 8);
                    }
                })*
                None
            }
            /// Return the lowest byte which is nonzero.
            #[inline]
            pub fn lowest_nonzero_byte(&self) -> Option<usize> {
                let inner = self.inner();
                #({
                    let idx: usize = #loop_unit_amount;
                    let v = inner[idx];
                    if v != 0 {
                        let x = #loop_unit_bytes_size * idx;
                        let y = v.trailing_zeros() as usize;
                        return Some(x + y / 8);
                    }
                })*
                None
            }
        );
        self.defun(part);
    }

    fn defun_pub_arith_ops(&self) {
        let name = &self.ts.name;
        let part = quote!(
            /// Calculates the multiplication of `self` and `other`.
            ///
            /// Returns a tuple: `(low, high)`,
            /// `low` is the low part of the multiplication,
            /// `high` is the low part of the multiplication.
            ///
            /// The multiplication is equal to `(high << Self::count_bits()) + low`.
            #[inline]
            pub fn complete_mul(&self, other: &Self) -> (Self, Self) {
                self._full_mul(other)
            }
            /// Calculates both the quotient and the remainder when `self` is divided by `other`.
            ///
            /// Returns a tuple: `(quotient, remainder)`.
            ///
            /// The `self` is equal to `quotient * other + remainder`.
            #[inline]
            pub fn complete_div(&self, other: &Self) -> (Self, Self) {
                if let Some(ret) = self._div_with_rem(other) {
                    ret
                } else {
                    panic!("{}: the divisor is zero", stringify!(#name));
                }
            }
        );
        self.defun(part);
    }
}
