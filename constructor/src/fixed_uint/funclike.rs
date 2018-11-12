// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Parse the input for the macro `construct_uints`.

use syn;

pub struct UintDefinitions {
    pub inner: syn::punctuated::Punctuated<UintDefinition, Token![,]>,
}
pub type UintAttributes = syn::punctuated::Punctuated<UintAttribute, Token![,]>;

impl syn::parse::Parse for UintDefinitions {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        Ok(Self {
            inner: input.parse_terminated(syn::parse::Parse::parse)?,
        })
    }
}

pub struct UintDefinition {
    pub name: syn::Ident,
    pub attrs: UintAttributes,
}

impl syn::parse::Parse for UintDefinition {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let name = input.parse()?;
        let content;
        let _ = braced!(content in input);
        let attrs = content.parse_terminated(syn::parse::Parse::parse)?;
        Ok(Self { name, attrs })
    }
}

pub struct UintAttribute {
    pub key: syn::Ident,
    _eq: Token![=],
    pub value: syn::Lit,
}

impl syn::parse::Parse for UintAttribute {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        Ok(Self {
            key: input.parse()?,
            _eq: input.parse()?,
            value: input.parse()?,
        })
    }
}
