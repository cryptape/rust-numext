// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Implement built-in traits in [`::std::convert`].
//!
//! [`::std::convert`]: https://doc.rust-lang.org/std/convert/index.html#traits

use fixed_hash::HashConstructor;

impl HashConstructor {
    pub fn impl_traits_std_convert(&self) {
        self.impl_traits_std_convert_from_as();
        self.impl_traits_std_convert_from_array();
    }

    fn impl_traits_std_convert_from_as(&self) {
        let name = &self.ts.name;
        let part = quote!(
            impl ::std::convert::AsRef<[u8]> for #name {
                #[inline]
                fn as_ref(&self) -> &[u8] {
                    &self.inner()[..]
                }
            }
            impl ::std::convert::AsMut<[u8]> for #name {
                #[inline]
                fn as_mut(&mut self) -> &mut [u8] {
                    &mut self.mut_inner()[..]
                }
            }
        );
        self.implt(part);
    }

    fn impl_traits_std_convert_from_array(&self) {
        let name = &self.ts.name;
        let inner_type = &self.ts.inner_type;
        let part = quote!(
            impl ::std::convert::From<#inner_type> for #name {
                #[inline]
                fn from(bytes: #inner_type) -> Self {
                    Self::new(bytes)
                }
            }
            impl<'a> ::std::convert::From<&'a #inner_type> for #name {
                #[inline]
                fn from(bytes: &'a #inner_type) -> Self {
                    Self::new(*bytes)
                }
            }
            impl ::std::convert::From<#name> for #inner_type {
                #[inline]
                fn from(hash: #name) -> Self {
                    hash.into_inner()
                }
            }
        );
        self.implt(part);
    }
}
