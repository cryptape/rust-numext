// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Define the struct and the methods or implement built-in traits to modify the struct directly.

use crate::fixed_uint::UintConstructor;
use crate::utils;
use proc_macro2::TokenStream;
use quote::quote;

impl UintConstructor {
    pub fn define_kernel(&self) {
        self.defun_priv_kernel();
        self.defun_pub_kernel();
        self.deftrait_uint_convert();
    }

    pub fn convert_into(&self, uc: &Self) -> TokenStream {
        let this_name = &self.ts.name;
        let that_name = &uc.ts.name;
        let stmts = if self.info.bits_size == uc.info.bits_size {
            if self.info.unit_bits_size == uc.info.unit_bits_size {
                // same inner
                quote!(
                    let inner = self.inner();
                    let val = #that_name::new(inner.clone());
                    (val, false)
                )
            } else if uc.info.unit_bits_size == 8 {
                // into u8 array
                let that_unit_amount = &uc.ts.unit_amount;
                quote!(
                    let mut inner = [0u8; #that_unit_amount];
                    self.into_little_endian(&mut inner[..]).unwrap();
                    let val = #that_name::new(inner);
                    (val, false)
                )
            } else if self.info.unit_bits_size == 8 {
                // from u8 array
                quote!(
                    let inner = self.inner();
                    let val = #that_name::from_little_endian(&inner[..]).unwrap();
                    (val, false)
                )
            } else {
                // same size, diff inner, no u8 array
                let that_bytes_size = &uc.ts.bytes_size;
                quote!(
                    let mut tmp = [0u8; #that_bytes_size];
                    self.into_little_endian(&mut tmp[..]).unwrap();
                    let val = #that_name::from_little_endian(&tmp[..]).unwrap();
                    (val, false)
                )
            }
        } else if self.info.bits_size < uc.info.bits_size {
            let this_bytes_size = &self.ts.bytes_size;
            quote!(
                let mut tmp = [0u8; #this_bytes_size];
                self.into_little_endian(&mut tmp[..]).unwrap();
                let val = #that_name::from_little_endian(&tmp[..]).unwrap();
                (val, false)
            )
        } else {
            let this_bytes_size = &self.ts.bytes_size;
            let that_bytes_size = &uc.ts.bytes_size;
            quote!(
                let mut tmp = [0u8; #this_bytes_size];
                self.into_little_endian(&mut tmp[..]).unwrap();
                let val = #that_name::from_little_endian(&tmp[..#that_bytes_size]).unwrap();
                (val, true)
            )
        };
        quote!(
            impl prelude::UintConvert<#that_name> for #this_name {
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
            /// Create a new fixed uint with a provided input.
            #[inline]
            const fn new(data: #inner_type) -> Self {
                #name(data)
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
        let zero_padding = &utils::pure_uint_list_to_ts(
            ::std::iter::repeat(0).take((self.info.unit_amount - 1) as usize),
        );
        let one = quote!([1, #(#zero_padding),* ]);
        let part = quote!(
            /// Create a new fixed uint and value is zero.
            #[inline]
            pub const fn zero() -> Self {
                Self::new([0; #unit_amount])
            }
            /// Create a new fixed uint and value is zero.
            #[inline]
            pub const fn one() -> Self {
                Self::new( #one )
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

    fn deftrait_uint_convert(&self) {
        let part = quote!(
            pub trait UintConvert<T> {
                /// Convert a fixed uint into another, return the new fixed uint and if it be truncated.
                fn convert_into(&self) -> (T, bool);
            }
        );
        self.prelude(part);
    }
}
