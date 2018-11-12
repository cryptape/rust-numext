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

use super::super::constructor::UintConstructor;
use super::super::utils;

impl UintConstructor {
    pub fn with_rand(&self) {
        self.with_rand_defun_pub();
    }

    fn with_rand_defun_pub(&self) {
        let name = &self.ts.name;
        let bytes_size = &self.ts.bytes_size;
        let inner_type = &self.ts.inner_type;
        let loop_unit_amount = &utils::pure_uint_list_to_ts(0..self.info.unit_amount);
        let part = quote!(
            impl rand::AsByteSliceMut for #name {
                #[inline]
                fn as_byte_slice_mut(&mut self) -> &mut [u8] {
                    let inner = self.mut_inner();
                    unsafe {
                        &mut *(inner as *mut #inner_type as *mut [u8; #bytes_size])
                    }
                }
                #[inline]
                fn to_le(&mut self) {
                    let inner = self.mut_inner();
                    #({
                        let idx = #loop_unit_amount;
                        inner[idx] = inner[idx].to_le();
                    })*
                }
            }
        );
        self.implt(part);
        let part = quote!(
            /// Create a random fixed uint with a input random core.
            #[inline]
            pub fn random<R: rand::RngCore>(rng: &mut R) -> Self {
                use rand::Rng;
                let mut ret = Self::default();
                rng.fill(&mut ret);
                ret
            }
            /// Create a random fixed uint.
            #[inline]
            pub fn thread_random() -> Self {
                let mut rng = rand::thread_rng();
                Self::random(&mut rng)
            }
        );
        self.defun(part);
    }
}
