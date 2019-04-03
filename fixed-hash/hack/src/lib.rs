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

extern crate proc_macro;

use numext_fixed_hash_core as nfhash;
use proc_macro_hack::proc_macro_hack;
use quote::quote;
use syn::parse_macro_input;

macro_rules! impl_hack {
    ($(($name:ident, $type:ident),)+) => {
        $(impl_hack!($name, $type);)+
    };
    ($(($name:ident, $type:ident)),+) => {
        $(impl_hack!($name, $type);)+
    };
    ($name:ident, $type:ident) =>    {
        #[proc_macro_hack]
        pub fn $name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            let input = parse_macro_input!(input as syn::LitStr);
            let expanded = {
                let input = input.value().replace("_", "");
                if &input[..2] != "0x" {
                    panic!("Input has to be a hexadecimal string with 0x-prefix.");
                };
                let input_str = &input[2..];
                let value = match &input_str[..1] {
                    "0" => {
                        nfhash::$type::from_hex_str(input_str)
                    },
                    _ => {
                        nfhash::$type::from_trimmed_hex_str(input_str)
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

impl_hack!(
    (h128, H128),
    (h160, H160),
    (h224, H224),
    (h256, H256),
    (h384, H384),
    (h512, H512),
    (h520, H520),
    (h1024, H1024),
    (h2048, H2048),
    (h4096, H4096),
);
