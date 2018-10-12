// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Implement built-in traits in [`::std::clone`].
//!
//! [`::std::clone`]: https://doc.rust-lang.org/std/clone/index.html#traits

use core::constructor::UintConstructor;

impl UintConstructor {
    pub fn impl_traits_std_clone(&self) {
        let name = &self.ts.name;
        let part = quote!(
            impl ::std::clone::Clone for #name {
                #[inline]
                fn clone(&self) -> Self {
                    unsafe {
                        let ret: Self = ::std::mem::transmute_copy(self);
                        ret
                    }
                }
            }
        );
        self.implt(part);
    }
}
