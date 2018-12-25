// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Define the struct and the methods or implement built-in traits to modify the struct directly.

use crate::fixed_hash::HashConstructor;
use crate::utils;
use proc_macro2::TokenStream;
use quote::quote;

impl HashConstructor {
    pub fn define_kernel(&self) {
        self.defun_priv_kernel();
        self.defun_pub_kernel();
        self.deftrait_hash_convert();
    }

    pub fn convert_into(&self, uc: &Self) -> TokenStream {
        let this_name = &self.ts.name;
        let that_name = &uc.ts.name;
        let stmts = if self.info.bits_size == uc.info.bits_size {
            quote!(
                let inner = self.inner();
                let val = #that_name::new(inner.clone());
                (val, false)
            )
        } else if self.info.bits_size < uc.info.bits_size {
            let this_bytes_size = &self.ts.unit_amount;
            quote!(
                let mut ret = #that_name::zero();
                ret.mut_inner()[..#this_bytes_size].copy_from_slice(&self.inner()[..]);
                (ret, false)
            )
        } else {
            let that_bytes_size = &uc.ts.unit_amount;
            quote!(
                let mut ret = #that_name::zero();
                ret.mut_inner()
                    .copy_from_slice(&self.inner()[..#that_bytes_size]);
                (ret, true)
            )
        };
        quote!(
            impl prelude::HashConvert<#that_name> for #this_name {
                #[inline]
                fn convert_into(&self) -> (#that_name, bool) {
                    #stmts
                }
            }
        )
    }

    fn defun_priv_kernel(&self) {
        let name = &self.ts.name;
        let inner_type = &self.ts.inner_type;
        let part = quote!(
            /// Create a new fixed hash with a provided input.
            #[inline]
            const fn new(data: #inner_type) -> Self {
                #name(data)
            }
            /// Get a reference of the inner data of the fixed hash.
            #[inline]
            fn inner<'a>(&'a self) -> &'a #inner_type {
                &self.0
            }
            /// Get a mutable reference of the inner data of the fixed hash.
            #[inline]
            fn mut_inner<'a>(&'a mut self) -> &'a mut #inner_type {
                &mut self.0
            }
            /// Get the inner data of the fixed hash.
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
            /// Return a new fixed hash where all bytes are set to the given byte.
            #[inline]
            pub const fn repeat_byte(byte: u8) -> Self {
                Self::new([byte; #unit_amount])
            }
            /// Create a new fixed hash and all bits of it are zeros.
            #[inline]
            pub const fn zero() -> Self {
                Self::new([0; #unit_amount])
            }
            /// Test if all bits of a fixed hash are zero.
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
            /// Test if all bits of a fixed hash are one.
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
            /// Test if all bits set in a hash are also set in `self`.
            #[inline]
            pub fn covers(&self, hash: &Self) -> bool {
                let inner = self.inner();
                let rhs = hash.inner();
                #({
                    let idx = #loop_unit_amount;
                    if inner[idx] & rhs[idx] != rhs[idx] {
                        return false;
                    }
                })*
                true
            }
        );
        self.defun(part);
    }

    fn deftrait_hash_convert(&self) {
        let part = quote!(
            pub trait HashConvert<T> {
                /// Convert a fixed hash into another, return the new fixed hash and if it be truncated.
                fn convert_into(&self) -> (T, bool);
            }
        );
        self.prelude(part);
    }
}
