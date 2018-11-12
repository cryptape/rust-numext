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

pub struct UintAttributes {
    pub size: u64,
    pub unit_size: u64,
    pub is_hash: bool,
}

impl UintAttributes {
    pub fn refresh_and_check_with_panics(
        &mut self,
        check: &::std::collections::HashSet<&'static str>,
    ) {
        if !check.contains("size") {
            panic!("Failed to parse attribute `size`");
        }
        if self.size == 0 {
            panic!("The attribute `size` should not be zero");
        }
        if self.size <= 64 {
            panic!(
                "If attribute `size`(={}) <= 64, please use the primitive type",
                self.size
            );
        }

        if !check.contains("unit_size") {
            self.unit_size = if self.is_hash {
                8
            } else if self.size % 64 == 0 {
                64
            } else if self.size % 32 == 0 {
                32
            } else if self.size % 16 == 0 {
                16
            } else if self.size % 8 == 0 {
                8
            } else {
                panic!(
                    "The attributes: `size`(={}) % `unit_size`(64 or 32 or 16 or 8) should be zero",
                    self.size
                );
            };
        } else if self.is_hash {
            panic!("The `unit_size` for hashes is fixed (= 8), do not set it manually.");
        } else {
            // Do NOT use 128 as unit size, since there is no way to get overflow part of multiply.
            match self.unit_size {
                8 | 16 | 32 | 64 => {}
                _ => panic!("The attribute `unit_size` should be in (8, 16, 32, 64)"),
            };
        }

        if self.size < self.unit_size {
            panic!(
                "The attributes: `size`(={}) should not be less than `unit_size`(={})",
                self.size, self.unit_size
            );
        }
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
            size: 0,
            unit_size: 64,
            is_hash: false,
        }
    }
}

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

impl ::std::convert::From<funclike::UintAttributes> for UintAttributes {
    fn from(input: funclike::UintAttributes) -> Self {
        let mut ret = Self::default();
        let mut check = ::std::collections::HashSet::new();
        for attr in input.into_iter() {
            match attr.key.to_string().as_ref() {
                "size" => parse_attr_with_check!(Int, size, attr.value, ret, check),
                "unit_size" => parse_attr_with_check!(Int, unit_size, attr.value, ret, check),
                "is_hash" => parse_attr_with_check!(Bool, is_hash, attr.value, ret, check),
                key => panic!("Unknown attribute `{}`", key),
            }
        }
        ret.refresh_and_check_with_panics(&check);
        ret
    }
}
