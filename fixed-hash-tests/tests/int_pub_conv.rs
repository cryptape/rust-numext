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

macro_rules! from_hex_str {
    ($name:ident, $hash:ident, $regex:expr) => {
        proptest! {
            #[test]
            fn $name(ref s in $regex) {
                let expected: props::H256Bytes = etypes::$hash::from_str(s).unwrap().into();
                let result: props::H256Bytes = nfhash::$hash::from_hex_str(s).unwrap().into();
                assert_eq!(expected, result);
            }
        }
    };
}

macro_rules! from_trimmed_hex_str {
    ($name:ident, $hash:ident, $regex:expr) => {
        proptest! {
            #[test]
            fn $name(mut s in $regex) {
                println!("s = {}", s);
                let expected: props::H256Bytes = nfhash::$hash::from_hex_str(s.as_str()).unwrap().into();
                let mut l = 0;
                for x in s.as_bytes() {
                    if *x == b'0' {
                        l += 1;
                    } else {
                        break;
                    }
                }
                let t = if l != s.len() {
                    s.drain(..l);
                    s
                } else {
                    "0".to_owned()
                };
                println!("t = {}", t);
                let result: props::H256Bytes = nfhash::$hash::from_trimmed_hex_str(t.as_str()).unwrap().into();
                assert_eq!(expected, result);
            }
        }
    };
}

from_hex_str!(h256_from_hex_str, H256, "[0-9a-fA-F]{64}");
from_trimmed_hex_str!(h256_from_trimmed_hex_str, H256, "[0-9a-fA-F]{64}");
