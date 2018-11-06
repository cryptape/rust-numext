// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use proc_macro2::TokenStream;

pub mod utils;

mod constructor;
pub use self::constructor::UintConstructor;

mod builtin;
mod extension;
mod internal;

impl UintConstructor {
    pub fn construct_all(&self, ucs: &[Self]) -> (TokenStream, TokenStream) {
        self.clear();

        // kernal
        self.define_kernel();

        // minimal
        self.impl_traits_std_clone();
        self.impl_traits_std_default();
        self.defun_pub_basic();

        // div need cmp
        self.impl_traits_std_cmp();
        self.defun_priv_ops();
        self.impl_traits_std_ops();

        self.defun_priv_conv();
        self.defun_pub_conv();
        self.impl_traits_std_convert();

        self.defun_as_prim();
        self.impl_traits_std_fmt();
        self.impl_traits_std_hash();
        self.impl_traits_std_iter();

        // extension
        #[cfg(feature = "rand")]
        self.with_rand();

        self.output(ucs)
    }
}
