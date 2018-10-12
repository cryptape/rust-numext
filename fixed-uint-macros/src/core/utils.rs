// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn;

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
