// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![recursion_limit = "512"]

extern crate proc_macro;
extern crate proc_macro2;

#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

mod core;

mod funclike;

mod parsed;

#[proc_macro]
pub fn construct_fixed_uints(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let inputs = parse_macro_input!(input as funclike::UintDefinitions);
    let expanded = {
        inputs
            .inner
            .into_iter()
            .map(|input| {
                let parsed: parsed::UintDefinition = input.into();
                core::UintConstructor::new(parsed)
            }).fold((quote!(), Vec::new()), |(uints, mut ucs), uc| {
                let (uint, public) = uc.construct_all(&ucs[..]);
                let uints = quote!(#uints #public #uint);
                ucs.push(uc);
                (uints, ucs)
            }).0
    };
    expanded.into()
}
