// Copyright 2018-2019 Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Implement built-in traits in [`::std::default`].
//!
//! [`::std::default`]: https://doc.rust-lang.org/std/default/index.html#traits

use crate::fixed_uint::UintConstructor;
use quote::quote;

impl UintConstructor {
    pub fn impl_traits_std_default(&self) {
        let name = &self.ts.name;
        let part = quote!(
            impl ::std::default::Default for #name {
                #[inline]
                fn default() -> Self {
                    Self::zero()
                }
            }
        );
        self.implt(part);
    }
}
