// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Parse the input for the macros.

use syn::{self, braced, Token};

pub struct Definitions {
    pub inner: syn::punctuated::Punctuated<Definition, Token![,]>,
}
pub type Attributes = syn::punctuated::Punctuated<Attribute, Token![,]>;

impl syn::parse::Parse for Definitions {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        Ok(Self {
            inner: input.parse_terminated(syn::parse::Parse::parse)?,
        })
    }
}

pub struct Definition {
    pub name: syn::Ident,
    pub attrs: Attributes,
}

impl syn::parse::Parse for Definition {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let name = input.parse()?;
        let content;
        let _ = braced!(content in input);
        let attrs = content.parse_terminated(syn::parse::Parse::parse)?;
        Ok(Self { name, attrs })
    }
}

pub struct Attribute {
    pub key: syn::Ident,
    _eq: Token![=],
    pub value: syn::Lit,
}

impl syn::parse::Parse for Attribute {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        Ok(Self {
            key: input.parse()?,
            _eq: input.parse()?,
            value: input.parse()?,
        })
    }
}
