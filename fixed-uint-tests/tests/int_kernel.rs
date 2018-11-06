// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate numext_fixed_uint;
extern crate numext_fixed_uint_tests;

use numext_fixed_uint::{prelude::UintConvert, U128, U256};
use numext_fixed_uint_tests::tools;

#[test]
fn kernel() {
    let x = tools::gen_nonzero::<u64>();

    assert!(U128::zero() == U128::from(0u8));
    assert!(U128::zero() != U128::from(1u8));
    assert!(U128::zero() != U128::from(x));
    assert!(U128::zero().is_zero());

    assert!(U128::one() != U128::from(0u8));
    assert!(U128::one() == U128::from(1u8));
    assert!(U128::one() != U128::from(x));
    assert!(!U128::one().is_zero());

    assert!(!U128::from(x).is_zero());

    assert!(!U128::zero().is_max());
    assert!(!U128::one().is_max());
    assert!(!U128::from(x).is_max());
    assert!(U128::max_value().is_max());
}

#[test]
fn convert() {
    let r = tools::gen_nonzero::<u64>();
    let n_u256 = U256::max_value() - U256::from(r);
    let n_u128 = U128::max_value() - U128::from(r);

    let (x, t): (U128, _) = n_u256.convert_into();
    assert!(t);
    assert_eq!(x, n_u128);
    let (y, t): (U256, _) = x.convert_into();
    assert!(!t);

    let n_diff = &n_u256 - &y;
    let n_u256_lo = &n_u256 - &n_diff;

    let (x, t): (U256, _) = n_u128.convert_into();
    assert!(!t);
    assert_eq!(x, n_u256_lo);
}
