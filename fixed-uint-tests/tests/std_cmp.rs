// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate proptest;

extern crate ethereum_types as uint;
extern crate numext_fixed_uint as nfuint;
extern crate numext_fixed_uint_tests as nfuint_tests;

use nfuint_tests::props;
use proptest::prelude::{any, any_with};

proptest! {
    #[test]
    fn random(ref pair in any_with::<props::U256Pair>(props::U256PairParameters::Random)) {
        let result_uint = {
            let (ref lhs, ref rhs): (uint::U256, uint::U256) = pair.into();
            (lhs > rhs, lhs >= rhs, lhs == rhs, lhs <= rhs, lhs < rhs)
        };
        let result_nfuint = {
            let (ref lhs, ref rhs): (nfuint::U256, nfuint::U256) = pair.into();
            (lhs > rhs, lhs >= rhs, lhs == rhs, lhs <= rhs, lhs < rhs)
        };
        assert_eq!(result_uint, result_nfuint);
        assert_eq!(result_nfuint.1, result_nfuint.0 || result_nfuint.2);
    }
}

proptest! {
    #[test]
    fn same(ref le in any::<props::U256LeBytes>()) {
        let result_nfuint = {
            let lhs: &nfuint::U256 = &le.into();
            let rhs: &nfuint::U256 = &le.into();
            (lhs > rhs, lhs >= rhs, lhs == rhs, lhs <= rhs, lhs < rhs)
        };
        assert_eq!(result_nfuint, (false, true, true, true, false));
    }
}
