// Copyright 2018-2019 Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use proc_macro2::TokenStream;

mod constructor;
pub use self::constructor::HashConstructor;

mod builtin;
mod extension;
mod internal;

impl HashConstructor {
    pub fn construct_all(&self, ucs: &[Self]) -> (TokenStream, TokenStream) {
        self.clear();

        self.define_kernel();

        self.impl_traits_std_default();
        self.defun_pub_basic();

        self.defun_pub_conv();
        self.impl_traits_std_convert();

        self.impl_traits_std_cmp();
        self.defun_priv_ops();
        self.impl_traits_std_ops();

        self.defun_as_prim();
        self.impl_traits_std_fmt();
        self.impl_traits_std_hash();
        self.impl_traits_std_str();

        self.with_rand();
        self.with_heapsize();
        self.with_serde();

        self.output(ucs)
    }
}
