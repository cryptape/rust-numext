// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Add [methods] as primitive uint types.
//!
//! For performance, some methods are different to the primitive uint types.
//! Some methods, for example, [`count_ones`], we can use reference of self as input,
//! there is no need to let self to be moved.
//!
//! [methods]: https://doc.rust-lang.org/std/primitive.u64.html#methods
//! [`count_ones`]: https://doc.rust-lang.org/std/primitive.u64.html#method.count_ones

use fixed_hash::HashConstructor;
use utils;

impl HashConstructor {
    pub fn defun_as_prim(&self) {
        self.defun_as_prim_boundary();
        self.defun_as_prim_bits();
        self.defun_as_prim_checked();
        self.defun_as_prim_overflowing();
    }

    fn defun_as_prim_boundary(&self) {
        let unit_amount = &self.ts.unit_amount;
        let part = quote!(
            /// Returns the smallest value that can be represented by this integer type.
            #[inline]
            pub fn min_value() -> Self {
                Self::new([0; #unit_amount])
            }
            /// Returns the largest value that can be represented by this integer type.
            #[inline]
            pub fn max_value() -> Self {
                Self::new([!0; #unit_amount])
            }
        );
        self.defun(part);
    }

    fn defun_as_prim_bits(&self) {
        let bits_size = &self.ts.bits_size;
        let idx_unit_amount = &utils::pure_uint_list_to_ts(0..self.info.unit_amount);
        let idx_unit_amount_rev = &utils::pure_uint_list_to_ts((0..self.info.unit_amount).rev());
        let part = quote!(
            /// Returns the number of ones in the binary representation of self.
            #[inline]
            pub fn count_ones(&self) -> u32 {
                let mut ret = 0u32;
                let inner = self.inner();
                #(
                    ret += inner[#idx_unit_amount].count_ones();
                )*
                ret
            }
            /// Returns the number of zeros in the binary representation of self.
            #[inline]
            pub fn count_zeros(&self) -> u32 {
                let mut ret = 0u32;
                let inner = self.inner();
                #(
                    ret += inner[#idx_unit_amount].count_zeros();
                )*
                ret
            }
            /// Returns the number of leading zeros in the binary representation of self.
            #[inline]
            pub fn leading_zeros(&self) -> u32 {
                let mut ret = 0u32;
                let inner = self.inner();
                #({
                    let v = inner[#idx_unit_amount_rev];
                    if v != 0 {
                        return (8 * #idx_unit_amount) as u32 + v.leading_zeros();
                    }
                })*
                #bits_size
            }
            /// Returns the number of trailing zeros in the binary representation of self.
            #[inline]
            pub fn trailing_zeros(&self) -> u32 {
                let mut ret = 0u32;
                let inner = self.inner();
                #({
                    let idx = #idx_unit_amount;
                    if inner[idx] != 0 {
                        return (8 * idx) as u32 + inner[idx].trailing_zeros();
                    }
                })*
                #bits_size
            }
        );
        self.defun(part);
    }

    fn defun_as_prim_checked(&self) {
        let bits_size = &self.ts.bits_size;
        let part = quote!(
            /// Checked shift left. Computes `self << rhs`,
            /// returning `None` if `rhs` is larger than or equal to the number of bits in `self`.
            #[inline]
            pub fn checked_shl(&self, rhs: u128) -> Option<Self> {
                if rhs >= #bits_size {
                    None
                } else {
                    Some(self._ushl(rhs))
                }
            }
            /// Checked shift right. Computes `self >> rhs`,
            /// returning `None` if `rhs` is larger than or equal to the number of bits in `self`.
            #[inline]
            pub fn checked_shr(&self, rhs: u128) -> Option<Self> {
                if rhs >= #bits_size {
                    None
                } else {
                    Some(self._ushr(rhs))
                }
            }
            /// Checked negation. Computes `-self`, returning `None` unless `self == 0`.
            /// Note that negating any positive integer will overflow.
            #[inline]
            pub fn checked_neg(&self) -> Option<Self> {
                if self.is_zero() {
                    Some(Self::zero())
                } else {
                    None
                }
            }
        );
        self.defun(part);
    }

    fn defun_as_prim_overflowing(&self) {
        let bits_size = &self.ts.bits_size;
        let part = quote!(
            /// Shifts `self` left by `rhs` bits.
            ///
            /// Returns a tuple of the shifted version of `self` along with a boolean indicating
            /// whether the shift value was larger than or equal to the number of bits.
            /// If the shift value is too large, then value is masked (N-1) where N is the number
            /// of bits, and this value is then used to perform the shift.
            #[inline]
            pub fn overflowing_shl(&self, rhs: u128) -> (Self, bool) {
                if rhs >= #bits_size {
                    (self._ushl(rhs % #bits_size), true)
                } else {
                    (self._ushl(rhs), false)
                }
            }
            /// Shifts `self` right by `rhs` bits.
            ///
            /// Returns a tuple of the shifted version of `self` along with a boolean indicating
            /// whether the shift value was larger than or equal to the number of bits.
            /// If the shift value is too large, then value is masked (N-1) where N is the number
            /// of bits, and this value is then used to perform the shift.
            #[inline]
            pub fn overflowing_shr(&self, rhs: u128) -> (Self, bool) {
                if rhs >= #bits_size {
                    (self._ushr(rhs % #bits_size), true)
                } else {
                    (self._ushr(rhs), false)
                }
            }
        );
        self.defun(part);
    }
}
