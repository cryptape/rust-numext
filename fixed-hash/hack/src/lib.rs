// Copyright 2018-2019 Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This is a internal crate used by [numext-fixed-hash].
//!
//! **Notice:
//! You should NOT use this crate directly.
//! Please use [numext-fixed-hash] instead of this crate.**
//!
//! [numext-fixed-hash]: https://docs.rs/numext-fixed-hash

extern crate nfhash_core;

extern crate proc_macro;

use quote::quote;
use syn::parse_macro_input;

macro_rules! impl_func {
    ($name:ident, $type:ident) =>    {
        #[proc_macro]
        pub fn $name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            let input = parse_macro_input!(input as syn::LitStr);
            let expanded = {
                let input = input.value().replace("_", "");
                if input.len() < 3 || &input[..2] != "0x" {
                    panic!("Input has to be a hexadecimal string with 0x-prefix.");
                };
                let input_str = &input[2..];
                let value = match &input_str[..1] {
                    "0" => {
                        if input_str.len() > 1 {
                            nfhash_core::$type::from_hex_str(input_str)
                        } else {
                            nfhash_core::$type::from_trimmed_hex_str(input_str)
                        }
                    },
                    _ => {
                        nfhash_core::$type::from_trimmed_hex_str(input_str)
                    },
                }
                .unwrap_or_else(|err| {
                    panic!("Failed to parse the input hexadecimal string: {}", err);
                });
                let eval_str = format!("{:?}", value);
                let eval_ts: proc_macro2::TokenStream = eval_str.parse().unwrap_or_else(|_| {
                    panic!("Failed to parse the string \"{}\" to TokenStream.", eval_str);
                });
                quote!(#eval_ts)
            };
            expanded.into()
        }
    };
}

#[cfg(feature = "bits_128")]
impl_func!(h128, H128);
#[cfg(feature = "bits_160")]
impl_func!(h160, H160);
#[cfg(feature = "bits_224")]
impl_func!(h224, H224);
#[cfg(feature = "bits_256")]
impl_func!(h256, H256);
#[cfg(feature = "bits_384")]
impl_func!(h384, H384);
#[cfg(feature = "bits_512")]
impl_func!(h512, H512);
#[cfg(feature = "bits_520")]
impl_func!(h520, H520);
#[cfg(feature = "bits_1024")]
impl_func!(h1024, H1024);
#[cfg(feature = "bits_2048")]
impl_func!(h2048, H2048);
#[cfg(feature = "bits_4096")]
impl_func!(h4096, H4096);
