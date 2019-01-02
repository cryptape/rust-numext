// Copyright 2018-2019 Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use nfuint::U256;

#[test]
fn saturating_add() {
    let one = U256::one();
    let val = U256::max_value() - &one;
    let x = val.saturating_add(&one);
    let y = &val + &one;
    assert_eq!(x, y);
    let x = y.saturating_add(&one);
    assert!(x.is_max());
}

#[test]
fn saturating_sub() {
    let one = U256::one();
    let val = U256::one();
    let x = val.saturating_sub(&one);
    let y = &one - &val;
    assert_eq!(x, y);
    let x = y.saturating_sub(&one);
    assert!(x.is_zero());
}

#[test]
fn saturating_mul_size() {
    let one = U256::one();
    let max = U256::max_value();
    let x = max.saturating_mul(&one);
    let y = &max * &one;
    assert_eq!(x, y);
    let x = y.saturating_mul(&max);
    assert!(x.is_max());
}

#[test]
fn saturating_mul_value() {
    let one = U256::one();
    let two = &one + &one;
    let max = U256::max_value();
    let x = max.saturating_mul(&one);
    let y = &max * &one;
    assert_eq!(x, y);
    let x = y.saturating_mul(&two);
    assert!(x.is_max());
}
