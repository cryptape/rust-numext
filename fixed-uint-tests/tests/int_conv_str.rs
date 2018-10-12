// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate proptest;

extern crate numext_fixed_uint as nfuint;
extern crate numext_fixed_uint_tests as nfuint_tests;

use nfuint_tests::props;
use proptest::prelude::any;

macro_rules! conv_from_str {
    ($func:ident, $fmt_str:expr) => {
        conv_from_str!($func, $fmt_str, $func);
    };
    ($func:ident, $fmt_str:expr, $name:ident) => {
        proptest! {
            #[test]
            fn $name(ref le in any::<props::U256LeBytes>()) {
                let origin: nfuint::U256 = le.into();
                let origin_str = format!($fmt_str, origin);
                let result = nfuint::U256::$func(origin_str.as_str()).unwrap();
                assert_eq!(origin, result);
            }
        }
    };
}

conv_from_str!(from_bin_str, "{:b}");
conv_from_str!(from_oct_str, "{:o}");
conv_from_str!(from_hex_str, "{:x}", from_lowerhex_str);
conv_from_str!(from_hex_str, "{:X}", from_upperhex_str);
conv_from_str!(from_dec_str, "{}");
