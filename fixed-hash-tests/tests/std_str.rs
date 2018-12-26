// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate proptest;

use std::str::FromStr;

use nfhash_tests::props;

macro_rules! std_str {
    ($name:ident, $hash:ident, $regex:expr) => {
        proptest! {
            #[test]
            fn $name(ref s in $regex) {
                let expected: props::H256Bytes = etypes::$hash::from_str(s).unwrap().into();
                let result: props::H256Bytes = nfhash::$hash::from_str(s).unwrap().into();
                assert_eq!(expected, result);
            }
        }
    };
}

std_str!(h256_from_str, H256, "[[:xdigit:]]{64}");
