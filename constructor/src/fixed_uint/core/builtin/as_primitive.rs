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

use super::super::constructor::UintConstructor;
use super::super::utils;

impl UintConstructor {
    pub fn defun_as_prim(&self) {
        self.defun_as_prim_boundary();
        self.defun_as_prim_bits();
        self.defun_as_prim_checked();
        self.defun_as_prim_saturating();
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
        let unit_bits_size = &self.ts.unit_bits_size;
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
                let ubs = #unit_bits_size;
                #({
                    let v = inner[#idx_unit_amount_rev];
                    if v != 0 {
                        return (ubs * #idx_unit_amount) as u32 + v.leading_zeros();
                    }
                })*
                #bits_size
            }
            /// Returns the number of trailing zeros in the binary representation of self.
            #[inline]
            pub fn trailing_zeros(&self) -> u32 {
                let mut ret = 0u32;
                let inner = self.inner();
                let ubs = #unit_bits_size;
                #({
                    let idx = #idx_unit_amount;
                    if inner[idx] != 0 {
                        return (ubs * idx) as u32 + inner[idx].trailing_zeros();
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
            /// Checked integer addition. Computes `self + rhs`,
            /// returning `None` if overflow occurred.
            #[inline]
            pub fn checked_add(&self, rhs: &Self) -> Option<Self> {
                let (ret, of) = self._add(rhs);
                if of { None } else { Some(ret) }
            }
            /// Checked integer subtraction. Computes `self - rhs`,
            /// returning `None` if overflow occurred.
            #[inline]
            pub fn checked_sub(&self, rhs: &Self) -> Option<Self> {
                let (ret, of) = self._sub(rhs);
                if of { None } else { Some(ret) }
            }
            /// Checked integer multiplication. Computes `self * rhs`,
            /// returning `None` if overflow occurred.
            #[inline]
            pub fn checked_mul(&self, rhs: &Self) -> Option<Self> {
                let (ret, of) = self._mul(rhs);
                if of { None } else { Some(ret) }
            }
            /// Checked integer division. Computes `self / rhs`, returning `None` if `rhs == 0`.
            #[inline]
            pub fn checked_div(&self, rhs: &Self) -> Option<Self> {
                let (ret, of) = self._div(rhs);
                if of { None } else { Some(ret) }
            }
            /// Checked integer remainder. Computes `self % rhs`, returning `None` if `rhs == 0`.
            #[inline]
            pub fn checked_rem(&self, rhs: &Self) -> Option<Self> {
                let (ret, of) = self._rem(rhs);
                if of { None } else { Some(ret) }
            }
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

    fn defun_as_prim_saturating(&self) {
        let part = quote!(
            /// Saturating integer addition. Computes `self + rhs`,
            /// saturating at the numeric bounds instead of overflowing.
            #[inline]
            pub fn saturating_add(&self, rhs: &Self) -> Self {
                let (ret, of) = self._add(rhs);
                if of { Self::max_value() } else { ret }
            }
            /// Checked integer subtraction. Computes `self - rhs`,
            /// returning `None` if overflow occurred.
            #[inline]
            pub fn saturating_sub(&self, rhs: &Self) -> Self {
                let (ret, of) = self._sub(rhs);
                if of { Self::zero() } else { ret }
            }
            /// Checked integer multiplication. Computes `self * rhs`,
            /// returning `None` if overflow occurred.
            #[inline]
            pub fn saturating_mul(&self, rhs: &Self) -> Self {
                let (ret, of) = self._mul(rhs);
                if of { Self::max_value() } else { ret }
            }
        );
        self.defun(part);
    }

    fn defun_as_prim_overflowing(&self) {
        let bits_size = &self.ts.bits_size;
        let part = quote!(
            /// Calculates `self + rhs`.
            ///
            /// Returns a tuple of the addition along with a boolean indicating
            /// whether an arithmetic overflow would occur.
            /// If an overflow would have occurred then the wrapped value is returned.
            #[inline]
            pub fn overflowing_add(&self, rhs: &Self) -> (Self, bool) {
                self._add(rhs)
            }
            /// Calculates `self - rhs`.
            ///
            /// Returns a tuple of the subtraction along with a boolean indicating
            /// whether an arithmetic overflow would occur.
            /// If an overflow would have occurred then the wrapped value is returned.
            #[inline]
            pub fn overflowing_sub(&self, rhs: &Self) -> (Self, bool) {
                self._sub(rhs)
            }
            /// Calculates the multiplication of `self` and `rhs`.
            ///
            /// Returns a tuple of the multiplication along with a boolean indicating
            /// whether an arithmetic overflow would occur.
            /// If an overflow would have occurred then the wrapped value is returned.
            #[inline]
            pub fn overflowing_mul(&self, rhs: &Self) -> (Self, bool) {
                self._mul(rhs)
            }
            /// Calculates the divisor when `self` is divided by `rhs`.
            ///
            /// Returns a tuple of the divisor along with a boolean indicating
            /// whether an arithmetic overflow would occur.
            /// Note that for unsigned integers overflow never occurs,
            /// so the second value is always `false`.
            ///
            /// # Panics
            ///
            /// This function will panic if `rhs` is `0`.
            #[inline]
            pub fn overflowing_div(&self, rhs: &Self) -> (Self, bool) {
                (self / rhs, false)
            }
            /// Calculates the remainder when `self` is divided by `rhs`.
            ///
            /// Returns a tuple of the remainder after dividing along with a boolean indicating
            /// whether an arithmetic overflow would occur.
            /// Note that for unsigned integers overflow never occurs,
            /// so the second value is always `false`.
            ///
            /// # Panics
            ///
            /// This function will panic if `rhs` is `0`.
            #[inline]
            pub fn overflowing_rem(&self, rhs: &Self) -> (Self, bool) {
                (self % rhs, false)
            }
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
            /// Negates `self` in an overflowing fashion.
            ///
            /// Returns `!self + 1` using wrapping operations to return the value that represents
            /// the negation of this unsigned value.
            /// Note that for positive unsigned values overflow always occurs,
            /// but negating `0` does not overflow.
            #[inline]
            pub fn overflowing_neg(&self) -> (Self, bool) {
                if self.is_zero() {
                    (Self::zero(), false)
                } else {
                    let (val, of) = self._not()._add(&Self::one());
                    if of {
                        unreachable!();
                    }
                    (val , true)
                }
            }
        );
        self.defun(part);
    }
}
