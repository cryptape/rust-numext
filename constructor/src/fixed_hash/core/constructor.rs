// Copyright 2018-2019 Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Constructor for Hash.

use super::super::parsed;
use crate::utils;
use proc_macro2::TokenStream;
use quote::quote;
use std::cell::Cell;
use std::iter::FromIterator;

pub struct HashInformation {
    pub name: String,
    pub bits_size: u64,
    pub unit_amount: u64,
    pub expand: bool,
}

impl ::std::convert::From<parsed::HashDefinition> for HashInformation {
    fn from(data: parsed::HashDefinition) -> Self {
        let parsed::HashDefinition { name, attrs } = data;
        // bits size for the whole unsigned integer
        let bits_size = attrs.size;
        // how many units in an unsigned integer
        let unit_amount = attrs.size / 8;

        let expand = unit_amount <= 64;

        Self {
            name,
            bits_size,
            unit_amount,
            expand,
        }
    }
}

pub struct HashTokenStreams {
    pub name: TokenStream,
    pub bits_size: TokenStream,
    pub unit_amount: TokenStream,
    pub inner_type: TokenStream,
    pub error_name: TokenStream,
    pub utils_name: TokenStream,
    pub feature: syn::LitStr,
    pub mod_name: syn::Ident,
}

impl<'a> ::std::convert::From<&'a HashInformation> for HashTokenStreams {
    fn from(info: &HashInformation) -> Self {
        let name = utils::ident_to_ts(info.name.as_ref());
        let bits_size = utils::pure_uint_to_ts(info.bits_size);
        let unit_amount = utils::pure_uint_to_ts(info.unit_amount);

        let inner_type = quote!([u8; #unit_amount]);

        let error_name = utils::ident_to_ts("FixedHashError");
        let utils_name = utils::ident_to_ts("utils");

        let feature_string = format!("bits_{}", info.bits_size);
        let feature = syn::LitStr::new(&feature_string, proc_macro2::Span::call_site());

        let mod_name = syn::Ident::new(
            &format!("_mod_{}", info.name),
            proc_macro2::Span::call_site(),
        );

        Self {
            name,
            bits_size,
            unit_amount,
            inner_type,
            error_name,
            utils_name,
            feature,
            mod_name,
        }
    }
}

pub struct HashConstructor {
    // Raw data of hash definition
    pub info: HashInformation,
    // Cache TokenStreams
    pub ts: HashTokenStreams,

    // Outputs (for each)
    hash_common: Cell<Vec<TokenStream>>,
    // Outputs (define methods)
    defuns: Cell<Vec<TokenStream>>,
    // Outputs (implement traits)
    implts: Cell<Vec<TokenStream>>,

    // Outputs (once)
    common: Cell<Vec<TokenStream>>,
    // Outputs (errors)
    errors: Cell<Vec<TokenStream>>,
    // Outputs (utils)
    utils: Cell<Vec<TokenStream>>,
    // Outputs (traits)
    preludes: Cell<Vec<TokenStream>>,
}

impl HashConstructor {
    pub fn new(data: parsed::HashDefinition) -> Self {
        let info: HashInformation = data.into();
        let ts: HashTokenStreams = (&info).into();
        let hash_common = Cell::new(Vec::new());
        let defuns = Cell::new(Vec::new());
        let implts = Cell::new(Vec::new());
        let common = Cell::new(Vec::new());
        let errors = Cell::new(Vec::new());
        let utils = Cell::new(Vec::new());
        let preludes = Cell::new(Vec::new());
        HashConstructor {
            info,
            ts,
            hash_common,
            defuns,
            implts,
            common,
            errors,
            utils,
            preludes,
        }
    }

    fn defstruct(&self) {
        let name = &self.ts.name;
        let feature = &self.ts.feature;
        let inner_type = &self.ts.inner_type;
        let part = quote!(
            /// Fixed hash type.
            #[cfg(feature = #feature)]
            #[derive(Clone)]
            pub struct #name(pub #inner_type);
        );
        self.attach_hash(part);
    }

    fn deferror(&self) {
        let error_name = &self.ts.error_name;
        let part = {
            let errors = self.errors.take();
            if errors.is_empty() {
                quote!()
            } else {
                let errors = TokenStream::from_iter(errors);
                quote!(
                    #[derive(Debug, Error)]
                    pub enum #error_name {
                        #errors
                    }
                )
            }
        };
        self.attach_common(part);
    }

    fn defutils(&self) {
        let utils_name = &self.ts.utils_name;
        let part = {
            let utils = self.utils.take();
            if utils.is_empty() {
                quote!()
            } else {
                let utils = TokenStream::from_iter(utils);
                quote!(
                    pub(crate) mod #utils_name {
                        #utils
                    }
                )
            }
        };
        self.attach_common(part);
    }

    fn deftraits(&self) {
        let part = {
            let preludes = self.preludes.take();
            if preludes.is_empty() {
                quote!()
            } else {
                let preludes = TokenStream::from_iter(preludes);
                quote!(
                    pub mod prelude {
                        #preludes
                    }
                )
            }
        };
        self.attach_common(part);
    }

    pub fn output(&self, ucs: &[Self]) -> (TokenStream, TokenStream) {
        self.defstruct();
        self.deferror();
        self.defutils();
        self.deftraits();
        let name = &self.ts.name;
        let feature = &self.ts.feature;
        let mod_name = &self.ts.mod_name;
        let hash_common = TokenStream::from_iter(self.hash_common.take());
        let defuns = TokenStream::from_iter(self.defuns.take());
        let implts = TokenStream::from_iter(self.implts.take());
        let one_hash = quote!(
            #hash_common

            #[cfg(feature = #feature)]
            #[doc(hide)]
            mod #mod_name {
                use crate::*;
                impl #name {
                    #defuns
                }
                #implts
            }
        );
        let public = if ucs.is_empty() {
            // define common part for all fixed hashes
            TokenStream::from_iter(self.common.take())
        } else {
            // define convert methods (From, Into) between two fixed hashes
            ucs.iter().fold(quote!(), |all, ref uc| {
                let convert_into = self.convert_into(uc);
                let convert_from = uc.convert_into(self);
                quote!(#all #convert_into #convert_from)
            })
        };
        (one_hash, public)
    }

    pub fn clear(&self) {
        let _ = self.hash_common.take();
        let _ = self.defuns.take();
        let _ = self.implts.take();
        let _ = self.common.take();
        let _ = self.errors.take();
        let _ = self.utils.take();
    }

    pub fn attach_hash(&self, part: TokenStream) {
        let mut o = self.hash_common.take();
        o.push(part);
        self.hash_common.set(o);
    }

    pub fn defun(&self, part: TokenStream) {
        let mut o = self.defuns.take();
        o.push(part);
        self.defuns.set(o);
    }

    pub fn implt(&self, part: TokenStream) {
        let mut o = self.implts.take();
        o.push(part);
        self.implts.set(o);
    }

    pub fn attach_common(&self, part: TokenStream) {
        let mut o = self.common.take();
        o.push(part);
        self.common.set(o);
    }

    pub fn error(&self, part: TokenStream) {
        let mut o = self.errors.take();
        o.push(part);
        self.errors.set(o);
    }

    pub fn util(&self, part: TokenStream) {
        let mut o = self.utils.take();
        o.push(part);
        self.utils.set(o);
    }

    pub fn prelude(&self, part: TokenStream) {
        let mut o = self.preludes.take();
        o.push(part);
        self.preludes.set(o);
    }
}
