// Copyright 2018-2019 Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Define public methods for mathematics.

use crate::fixed_uint::UintConstructor;
use quote::quote;

impl UintConstructor {
    pub fn defun_pub_math(&self) {
        self.defun_pub_gcd();
    }

    fn defun_pub_gcd(&self) {
        let part = quote!(
            /// Calculates the Greatest Common Divisor (GCD).
            #[inline]
            pub fn gcd(&self, other: &Self) -> Self {
                // Stein's algorithm
                if self.is_zero() {
                    return other.clone();
                }
                if other.is_zero() {
                    return self.clone();
                }
                let mut m = self.clone();
                let mut n = other.clone();

                // find common factors of 2
                let shift = ::std::cmp::min(m.trailing_zeros(), n.trailing_zeros());

                // divide m and n by 2 until odd
                // m inside loop
                n >>= n.trailing_zeros();

                while !m.is_zero() {
                    m >>= m.trailing_zeros();
                    if n > m {
                        ::std::mem::swap(&mut n, &mut m)
                    }
                    m -= &n;
                }

                n << shift
            }
        );
        self.defun(part);
    }
}
