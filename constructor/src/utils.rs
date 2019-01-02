// Copyright 2018-2019 Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn;

macro_rules! parse_attr_with_check {
    (Int, $key:ident, $input:ident, $output:ident) => {
        $output.$key = $input.value();
    };
    (Bool, $key:ident, $input:ident, $output:ident) => {
        $output.$key = $input.value;
    };
    ($lit_type:ident, $key:ident, $input:expr, $output:ident, $check:ident) => {{
        if $check.contains(stringify!($key)) {
            panic!(
                "Error because attribute `{}` has been set more than once",
                stringify!($key)
            );
        }
        $check.insert(stringify!($key));
        let parse_is_ok = if let syn::Lit::$lit_type(ref value) = $input {
            parse_attr_with_check!($lit_type, $key, value, $output);
            true
        } else {
            false
        };
        if !parse_is_ok {
            let value = $input;
            panic!(
                "Failed to parse attribute `{}`(={})",
                stringify!($key),
                quote!(#value)
            );
        }
    }};
}

/// Get a nonnegative integer literal without type.
pub fn pure_uint_to_ts(val: u64) -> TokenStream {
    syn::LitInt::new(val, syn::IntSuffix::None, Span::call_site()).into_token_stream()
}

/// Get a built-in nonnegative integer type.
pub fn uint_suffix_to_ts(val: u64) -> TokenStream {
    match val {
        8 | 16 | 32 | 64 | 128 => ident_to_ts(format!("u{}", val).as_ref()),
        _ => unreachable!(),
    }
}

/// Get a ident from a string.
pub fn ident_to_ts(val: &str) -> TokenStream {
    syn::Ident::new(val, Span::call_site()).into_token_stream()
}

/// Get a list of nonnegative integer literals without type.
pub fn pure_uint_list_to_ts<T>(vals: T) -> Vec<TokenStream>
where
    T: Iterator<Item = u64>,
{
    vals.map(pure_uint_to_ts).collect()
}
