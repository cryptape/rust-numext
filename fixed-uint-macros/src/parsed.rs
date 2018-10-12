// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Convert the parsed tokens into structs after some checks.

use syn;

use funclike;

pub struct UintDefinition {
    pub name: String,
    pub attrs: UintAttributes,
}

impl ::std::convert::From<funclike::UintDefinition> for UintDefinition {
    fn from(input: funclike::UintDefinition) -> Self {
        let name = input.name.to_string();
        let attrs = input.attrs.into();
        Self { name, attrs }
    }
}

impl ::std::convert::From<syn::DeriveInput> for UintDefinition {
    fn from(input: syn::DeriveInput) -> Self {
        let syn::DeriveInput {
            ident,
            generics,
            data,
            ..
        } = input;
        let mut attrs = UintAttributes::default();
        let mut data_is_unit = false;
        attrs.derived = true;
        if let syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Unnamed(fileds),
            ..
        }) = data
        {
            if fileds.unnamed.len() == 1 {
                if let Some(pair) = fileds.unnamed.first() {
                    if let syn::Type::Array(syn::TypeArray {
                        ref elem,
                        len:
                            syn::Expr::Lit(syn::ExprLit {
                                lit: syn::Lit::Int(ref len),
                                ..
                            }),
                        ..
                    }) = pair.value().ty
                    {
                        let unit_suffix = quote!(#elem).to_string();
                        attrs.unit_size = match unit_suffix.as_ref() {
                            "u8" => 8,
                            "u16" => 16,
                            "u32" => 32,
                            "u64" => 64,
                            _ => panic!(
                                "The unit(={}) should be one of `u8`, `u16`, `u32`, `u64`.",
                                unit_suffix
                            ),
                        };
                        let unit_amount = len.value();
                        attrs.size = attrs.unit_size * unit_amount;
                        data_is_unit = true;
                    }
                }
            }
        }
        if !data_is_unit {
            panic!(
                "The definition should be a unit struct like `{}`",
                "pub struct U256([u64; 4]);"
            );
        }
        if generics.lt_token.is_some()
            || generics.gt_token.is_some()
            || generics.where_clause.is_some()
            || !generics.params.is_empty()
        {
            panic!("The definition should not have generics")
        }
        let name = ident.to_string();
        Self { name, attrs }
    }
}

pub struct UintAttributes {
    pub derived: bool,
    pub size: u64,
    pub unit_size: u64,
}

impl UintAttributes {
    pub fn refresh_and_check_with_panics(&mut self) {
        if self.size == 0 {
            panic!("The attribute `size` should not be zero");
        }
        // And: self.size > self.unit_size
        if self.size <= 64 {
            panic!(
                "If attribute `size`(={}) <= 64, please use the primitive type",
                self.size
            );
        }
        // Do NOT use 128 as unit size, since there is no way to get overflow part of multiply.
        match self.unit_size {
            8 | 16 | 32 | 64 => {}
            _ => panic!("The attribute `unit_size` should be in (8, 16, 32, 64)"),
        };
        if self.size % self.unit_size != 0 {
            panic!(
                "The attributes: `size`(={}) % `unit_size`(={}) should be zero",
                self.size, self.unit_size
            );
        }
    }
}

impl ::std::default::Default for UintAttributes {
    fn default() -> Self {
        Self {
            derived: false,
            size: 0,
            unit_size: 64,
        }
    }
}

macro_rules! parse_attr_with_check {
    ($key:expr, $value:expr, $check:ident, $parse:block) => {
        if $check.contains($key) {
            panic!(
                "Error because attribute `{}` has been set more than once",
                $key
            );
        }
        $check.insert($key);
        let parse_is_ok = { $parse };
        if !parse_is_ok {
            let value = $value;
            panic!("Failed to parse attribute `{}`(={})", $key, quote!(#value));
        }
    };
}

impl ::std::convert::From<funclike::UintAttributes> for UintAttributes {
    fn from(input: funclike::UintAttributes) -> Self {
        let mut ret = Self::default();
        let mut check = ::std::collections::HashSet::new();
        for attr in input.into_iter() {
            match attr.key.to_string().as_ref() {
                "size" => {
                    parse_attr_with_check!("size", attr.value, check, {
                        if let syn::Lit::Int(ref value) = attr.value {
                            ret.size = value.value();
                            true
                        } else {
                            false
                        }
                    });
                }
                "unit_size" => {
                    parse_attr_with_check!("unit_size", attr.value, check, {
                        if let syn::Lit::Int(ref value) = attr.value {
                            ret.unit_size = value.value();
                            true
                        } else {
                            false
                        }
                    });
                }
                key => panic!("Unknown attribute `{}`", key),
            }
        }
        if !check.contains("size") {
            panic!("Failed to parse attribute `size`");
        }
        if !check.contains("unit_size") {
            ret.unit_size = if ret.size % 64 == 0 {
                64
            } else if ret.size % 32 == 0 {
                32
            } else if ret.size % 16 == 0 {
                16
            } else if ret.size % 8 == 0 {
                8
            } else {
                panic!(
                    "The attributes: `size`(={}) % `unit_size`(64 or 32 or 16 or 8) should be zero",
                    ret.size
                );
            };
        }
        ret.refresh_and_check_with_panics();
        ret
    }
}
