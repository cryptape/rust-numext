// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate numext_fixed_uint;
extern crate numext_fixed_uint_tests;

use numext_fixed_uint::U128;
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
