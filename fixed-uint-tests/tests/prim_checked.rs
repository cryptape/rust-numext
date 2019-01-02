// Copyright 2018-2019 Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use nfuint::U256;
use nfuint_tests::tools;

#[test]
fn checked_add() {
    let one = U256::one();
    let val = U256::max_value() - U256::one();
    let x = val.checked_add(&one);
    let y = &val + &one;
    assert_eq!(x, Some(y.clone()));
    let x = y.checked_add(&one);
    assert!(x.is_none());
}

#[test]
fn checked_sub() {
    let one = U256::one();
    let val = U256::one();
    let x = val.checked_sub(&one);
    let y = &one - &val;
    assert_eq!(x, Some(y.clone()));
    let x = y.checked_sub(&one);
    assert!(x.is_none());
}

#[test]
fn checked_mul_size() {
    let one = U256::one();
    let max = U256::max_value();
    let x = max.checked_mul(&one);
    let y = &max * &one;
    assert_eq!(x, Some(y.clone()));
    let x = y.checked_mul(&max);
    assert!(x.is_none());
}

#[test]
fn checked_mul_value() {
    let one = U256::one();
    let two = &one + &one;
    let max = U256::max_value();
    let x = max.checked_mul(&one);
    let y = &max * &one;
    assert_eq!(x, Some(y.clone()));
    let x = y.checked_mul(&two);
    assert!(x.is_none());
}

#[test]
fn checked_div() {
    let zero = U256::zero();
    let one = U256::one();
    let x = zero.checked_div(&one);
    let y = &zero / &one;
    assert_eq!(x, Some(y.clone()));
    let x = y.checked_div(&zero);
    assert!(x.is_none());
    let val = U256::thread_random();
    let x = val.checked_div(&one);
    let y = &val / &one;
    assert_eq!(x, Some(y.clone()));
    let x = y.checked_div(&zero);
    assert!(x.is_none());
}

#[test]
fn checked_rem() {
    let zero = U256::zero();
    let one = U256::one();
    let x = zero.checked_rem(&one);
    let y = &zero % &one;
    assert_eq!(x, Some(y.clone()));
    let x = y.checked_rem(&zero);
    assert!(x.is_none());
    let val = U256::thread_random();
    let x = val.checked_rem(&one);
    let y = &val % &one;
    assert_eq!(x, Some(y.clone()));
    let x = y.checked_rem(&zero);
    assert!(x.is_none());
}

#[test]
fn checked_shl() {
    let bits = u128::from(tools::gen_nonzero::<u8>());
    let val = &U256::thread_random();
    let x = val.checked_shl(bits);
    assert_eq!(x, Some(val << bits));
    let x = val.checked_shl(255);
    assert_eq!(x, Some(val << 255));
    let x = val.checked_shl(256);
    assert!(x.is_none());
    let x = val.checked_shl(257);
    assert!(x.is_none());
}

#[test]
fn checked_shr() {
    let bits = u128::from(tools::gen_nonzero::<u8>());
    let val = &U256::thread_random();
    let x = val.checked_shr(bits);
    assert_eq!(x, Some(val >> bits));
    let x = val.checked_shr(255);
    assert_eq!(x, Some(val >> 255));
    let x = val.checked_shr(256);
    assert!(x.is_none());
    let x = val.checked_shr(257);
    assert!(x.is_none());
}

#[test]
fn checked_reg() {
    let zero = U256::zero();
    let one = U256::one();
    let val = U256::thread_random();
    let x = zero.checked_neg();
    assert_eq!(x, Some(U256::zero()));
    let x = one.checked_neg();
    assert!(x.is_none());
    let x = val.checked_neg();
    assert!(x.is_none());
}
