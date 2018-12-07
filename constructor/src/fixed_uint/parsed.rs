// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Convert the parsed tokens into structs after some checks.

use syn;

use crate::definition;

pub struct UintDefinition {
    pub name: String,
    pub attrs: UintAttributes,
}

impl ::std::convert::From<definition::Definition> for UintDefinition {
    fn from(input: definition::Definition) -> Self {
        let name = input.name.to_string();
        let attrs = input.attrs.into();
        Self { name, attrs }
    }
}

pub struct UintAttributes {
    pub size: u64,
    pub unit_size: u64,
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
            self.unit_size = if self.size % 64 == 0 {
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
        }
    }
}

impl ::std::convert::From<definition::Attributes> for UintAttributes {
    fn from(input: definition::Attributes) -> Self {
        let mut ret = Self::default();
        let mut check = ::std::collections::HashSet::new();
        for attr in input.into_iter() {
            match attr.key.to_string().as_ref() {
                "size" => parse_attr_with_check!(Int, size, attr.value, ret, check),
                "unit_size" => parse_attr_with_check!(Int, unit_size, attr.value, ret, check),
                key => panic!("Unknown attribute `{}`", key),
            }
        }
        ret.refresh_and_check_with_panics(&check);
        ret
    }
}
