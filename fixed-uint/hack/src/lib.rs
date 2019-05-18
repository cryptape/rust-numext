// Copyright 2018-2019 Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This is a internal crate used by [numext-fixed-uint].
//!
//! **Notice:
//! You should NOT use this crate directly.
//! Please use [numext-fixed-uint] instead of this crate.**
//!
//! [numext-fixed-uint]: https://docs.rs/numext-fixed-uint

extern crate proc_macro;

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
                if input.is_empty() {
                    panic!("Input is empty.");
                }
                let (value_result, input_type) = if input.len() < 3 {
                    (nfuint_core::$type::from_dec_str(&input), "decimal")
                } else {
                    match &input[..2] {
                        "0b" => (nfuint_core::$type::from_bin_str(&input[2..]), "binary"),
                        "0o" => (nfuint_core::$type::from_oct_str(&input[2..]), "octal"),
                        "0x" => (nfuint_core::$type::from_hex_str(&input[2..]), "hexadecimal"),
                        _ => (nfuint_core::$type::from_dec_str(&input), "decimal"),
                    }
                };
                let value = value_result.unwrap_or_else(|err| {
                    panic!("Failed to parse the input {} string: {}", input_type, err);
                });
                let eval_str = format!("{:?}", value);
                let eval_ts: proc_macro2::TokenStream = eval_str.parse().unwrap_or_else(|_| {
                    panic!("Failed to parse the string [{}] to TokenStream.", eval_str);
                });
                quote!(#eval_ts)
            };
            expanded.into()
        }
    };
}

impl_hack!(
    (u128, U128),
    (u160, U160),
    (u224, U224),
    (u256, U256),
    (u384, U384),
    (u512, U512),
    (u520, U520),
    (u1024, U1024),
    (u2048, U2048),
    (u4096, U4096),
);
