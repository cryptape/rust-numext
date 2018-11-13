// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Convert the parsed tokens into structs after some checks.

use syn;

use definition;

pub struct HashDefinition {
    pub name: String,
    pub attrs: HashAttributes,
}

impl ::std::convert::From<definition::Definition> for HashDefinition {
    fn from(input: definition::Definition) -> Self {
        let name = input.name.to_string();
        let attrs = input.attrs.into();
        Self { name, attrs }
    }
}

pub struct HashAttributes {
    pub size: u64,
}

impl HashAttributes {
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
        if self.size < 8 {
            panic!(
                "The attributes: `size`(={}) should not be less than 8",
                self.size
            );
        }
        if self.size % 8 != 0 {
            panic!("The attributes: `size`(={}) % 8 should be zero", self.size);
        }
    }
}

impl ::std::default::Default for HashAttributes {
    fn default() -> Self {
        Self { size: 0 }
    }
}

impl ::std::convert::From<definition::Attributes> for HashAttributes {
    fn from(input: definition::Attributes) -> Self {
        let mut ret = Self::default();
        let mut check = ::std::collections::HashSet::new();
        for attr in input.into_iter() {
            match attr.key.to_string().as_ref() {
                "size" => parse_attr_with_check!(Int, size, attr.value, ret, check),
                key => panic!("Unknown attribute `{}`", key),
            }
        }
        ret.refresh_and_check_with_panics(&check);
        ret
    }
}
