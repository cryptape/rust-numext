// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Use [`rand`] for random number generation.
//!
//! [`rand`]: https://crates.io/crates/rand

use crate::fixed_hash::HashConstructor;
use crate::utils;
use quote::quote;

impl HashConstructor {
    pub fn with_rand(&self) {
        self.with_rand_defun_pub();
    }

    fn with_rand_defun_pub(&self) {
        let name = &self.ts.name;
        let part_core = if self.info.expand {
            let loop_unit_amount = &utils::pure_uint_list_to_ts(0..self.info.unit_amount);
            quote!(
                let inner = self.mut_inner();
                #({
                    let idx = #loop_unit_amount;
                    inner[idx] = inner[idx].to_le();
                })*
            )
        } else {
            quote!(for x in self.mut_inner().iter_mut() {
                *x = x.to_le()
            })
        };
        let part = quote!(
            #[cfg(feature = "support_rand")]
            impl rand::AsByteSliceMut for #name {
                #[inline]
                fn as_byte_slice_mut(&mut self) -> &mut [u8] {
                    &mut self.mut_inner()[..]
                }
                #[inline]
                fn to_le(&mut self) {
                    #part_core
                }
            }
        );
        self.implt(part);
        let part = quote!(
            /// Create a random fixed uint with a input random core.
            #[cfg(feature = "support_rand")]
            #[inline]
            pub fn random<R: rand::RngCore>(rng: &mut R) -> Self {
                use rand::Rng;
                let mut ret = Self::default();
                rng.fill(&mut ret);
                ret
            }
            /// Create a random fixed uint.
            #[cfg(feature = "support_rand")]
            #[inline]
            pub fn thread_random() -> Self {
                let mut rng = rand::thread_rng();
                Self::random(&mut rng)
            }
        );
        self.defun(part);
    }
}
