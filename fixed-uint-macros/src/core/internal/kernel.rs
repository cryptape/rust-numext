// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Define the struct and the methods or implement built-in traits to modify the struct directly.

use core::constructor::UintConstructor;
use core::utils;

impl UintConstructor {
    pub fn define_kernel(&self) {
        self.defun_priv_kernel();
        self.defun_pub_kernel();
    }

    fn defun_priv_kernel(&self) {
        let name = &self.ts.name;
        let inner_type = &self.ts.inner_type;
        let part = quote!(
            /// Create a new fixed uint with a provided input.
            #[inline]
            fn new(data: #inner_type) -> Self {
                #name (data)
            }
            /// Get a reference of the inner data of the fixed uint.
            #[inline]
            fn inner<'a>(&'a self) -> &'a #inner_type {
                &self.0
            }
            /// Get a mutable reference of the inner data of the fixed uint.
            #[inline]
            fn mut_inner<'a>(&'a mut self) -> &'a mut #inner_type {
                &mut self.0
            }
            /// Get the inner data of the fixed uint.
            #[inline]
            fn into_inner(self) -> #inner_type {
                self.0
            }
        );
        self.defun(part);
    }

    fn defun_pub_kernel(&self) {
        let unit_amount = &self.ts.unit_amount;
        let loop_unit_amount = &utils::pure_uint_list_to_ts(0..self.info.unit_amount);
        let part = quote!(
            /// Create a new fixed uint and value is zero.
            #[inline]
            pub fn zero() -> Self {
                Self::new([0; #unit_amount])
            }
            /// Create a new fixed uint and value is zero.
            #[inline]
            pub fn one() -> Self {
                let mut ret = [0; #unit_amount];
                ret[0] = 1;
                Self::new(ret)
            }
            /// Test if a fixed uint is zero.
            #[inline]
            pub fn is_zero(&self) -> bool {
                let inner = self.inner();
                #({
                    if inner[#loop_unit_amount] != 0 {
                        return false;
                    }
                })*
                true
            }
            /// Test if a fixed uint is the max value.
            #[inline]
            pub fn is_max(&self) -> bool {
                let inner = self.inner();
                #({
                    if inner[#loop_unit_amount] != !0 {
                        return false;
                    }
                })*
                true
            }
        );
        self.defun(part);
    }
}
