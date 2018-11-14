// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Define public methods about bits and bytes.

use fixed_hash::HashConstructor;
use utils;

impl HashConstructor {
    pub fn defun_pub_basic(&self) {
        self.defun_pub_bits_ops();
        self.defun_pub_bytes_ops();
        self.defun_pub_ptr_ops();
    }

    fn defun_pub_bits_ops(&self) {
        let bits_size = &self.ts.bits_size;
        let loop_unit_amount = &utils::pure_uint_list_to_ts(0..self.info.unit_amount);
        let loop_unit_amount_rev = &utils::pure_uint_list_to_ts((0..self.info.unit_amount).rev());
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
                    let unit_idx = index / 8;
                    let bit_idx = index % 8;
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
                    let unit_idx = index / 8;
                    let bit_idx = index % 8;
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
                        let x = 8 * (idx + 1) - 1;
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
                        let x = 8 * idx;
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
        let bytes_size = &self.ts.unit_amount;
        let loop_unit_amount = &utils::pure_uint_list_to_ts(0..self.info.unit_amount);
        let loop_unit_amount_rev = &utils::pure_uint_list_to_ts((0..self.info.unit_amount).rev());
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
                    Some(inner[index])
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
                    inner[index] = byte;
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
                        return Some(idx);
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
                        return Some(idx);
                    }
                })*
                None
            }
        );
        self.defun(part);
    }

    fn defun_pub_ptr_ops(&self) {
        let inner_type = &self.ts.inner_type;
        let part = quote!(
            /// Get the inner bytes slice of a fixed hash.
            #[inline]
            pub fn as_bytes(&self) -> &[u8] {
                &self.inner()[..]
            }
            /// Get the mutable inner bytes slice of a fixed hash.
            #[inline]
            pub fn as_bytes_mut(&mut self) -> &mut [u8] {
                &mut self.mut_inner()[..]
            }
            /// Get the inner bytes of a fixed hash.
            #[inline]
            pub fn as_fixed_bytes(&self) -> &#inner_type {
                self.inner()
            }
            /// Get the mutable inner bytes of a fixed hash.
            #[inline]
            pub fn as_fixed_bytes_mut(&mut self) -> &#inner_type {
                self.mut_inner()
            }
            /// Get the inner bytes array of a fixed hash.
            #[inline]
            pub fn to_fixed_bytes(self) -> #inner_type {
                self.into_inner()
            }
            /// Get a constant raw pointer to the inner bytes array of a fixed hash.
            #[inline]
            pub fn as_ptr(&self) -> *const u8 {
                self.inner().as_ptr()
            }
            /// Get a mutable raw pointer to the inner bytes array of a fixed hash.
            #[inline]
            pub fn as_mut_ptr(&mut self) -> *mut u8 {
                self.mut_inner().as_mut_ptr()
            }
        );
        self.defun(part);
    }
}
