// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Implement built-in traits in [`::std::convert`].
//!
//! Not implement `AsRef` and `AsMut` traits to reduce confusion.
//!
//! [`::std::convert`]: https://doc.rust-lang.org/std/convert/index.html#traits

use fixed_uint::UintConstructor;
use utils;

impl UintConstructor {
    pub fn impl_traits_std_convert(&self) {
        self.impl_traits_std_convert_from_bool();
        self.impl_traits_std_convert_from_primitive_uint();
    }

    fn impl_traits_std_convert_from_bool(&self) {
        let name = &self.ts.name;
        let part = quote!(
            impl ::std::convert::From<bool> for #name {
                #[inline]
                fn from(val: bool) -> Self {
                    if val {
                        Self::one()
                    } else {
                        Self::zero()
                    }
                }
            }
        );
        self.implt(part);
    }

    fn impl_traits_std_convert_from_primitive_uint(&self) {
        let name = &self.ts.name;
        for prim_bits_size in &[8u64, 16, 32, 64, 128] {
            let prim_type = utils::uint_suffix_to_ts(*prim_bits_size);
            let func_name = utils::ident_to_ts(format!("_from_u{}", prim_bits_size).as_ref());
            let part = quote!(
                impl ::std::convert::From<#prim_type> for #name {
                    #[inline]
                    fn from(prim: #prim_type) -> Self {
                        Self::#func_name(prim)
                    }
                }
                impl<'a> ::std::convert::From<&'a #prim_type> for #name {
                    #[inline]
                    fn from(prim: &#prim_type) -> Self {
                        Self::#func_name(*prim)
                    }
                }
            );
            self.implt(part);
        }
    }
}
