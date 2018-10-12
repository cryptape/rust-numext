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

use core::constructor::UintConstructor;

impl UintConstructor {
    pub fn with_rand(&self) {
        self.with_rand_defun_pub();
    }

    fn with_rand_defun_pub(&self) {
        let inner_type = &self.ts.inner_type;
        let unit_amount = &self.ts.unit_amount;
        let part = quote!(
            /// Create a random fixed uint with a input random core.
            #[inline]
            pub fn random<R: rand::RngCore>(rng: &mut R) -> Self {
                use rand::Rng;
                let mut data: #inner_type = [0; #unit_amount];
                rng.fill(&mut data);
                Self::new(data)
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
