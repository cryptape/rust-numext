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
extern crate uint;

use nfuint_tests::{props, tools};
use proptest::prelude::any_with;

macro_rules! prim_overflowing {
    ($func:ident, $param:ident) => {
        proptest! {
            #[test]
            fn $func(ref pair in any_with::<props::U256Pair>(props::U256PairParameters::$param)) {
                let expected: (props::U256LeBytes, bool) = {
                    let (lhs, rhs): (uint::U256, uint::U256) = pair.into();
                    let (ret, of) = lhs.$func(rhs);
                    (ret.into(), of)
                };
                let result: (props::U256LeBytes, bool) = {
                    let (lhs, rhs): (nfuint::U256, nfuint::U256) = pair.into();
                    let (ret, of) = lhs.$func(&rhs);
                    assert!(!of);
                    (ret.into(), of)
                };
                assert_eq!(expected, result);

                let expected: (props::U256LeBytes, bool) = {
                    let (lhs, rhs): (uint::U256, uint::U256) = pair.into();
                    let (ret, of) = (!lhs).$func(!rhs);
                    (ret.into(), of)
                };
                let result: (props::U256LeBytes, bool) = {
                    let (lhs, rhs): (nfuint::U256, nfuint::U256) = pair.into();
                    let (ret, of) = (!lhs).$func(&(!rhs));
                    assert!(of);
                    (ret.into(), of)
                };
                assert_eq!(expected, result);
            }
        }
    };
}

prim_overflowing!(overflowing_add, CanAdd);
prim_overflowing!(overflowing_sub, CanSub);
prim_overflowing!(overflowing_mul, CanMul);

#[test]
#[should_panic]
fn overflowing_div_zero() {
    let val = nfuint::U256::thread_random();
    let zero = nfuint::U256::zero();
    let _ = val.overflowing_div(&zero);
}

#[test]
#[should_panic]
fn overflowing_rem_zero() {
    let val = nfuint::U256::thread_random();
    let zero = nfuint::U256::zero();
    let _ = val.overflowing_rem(&zero);
}

#[test]
fn overflowing_shl() {
    let bits = u128::from(tools::gen_nonzero::<u8>());
    let val = &nfuint::U256::thread_random();
    let (x, of) = val.overflowing_shl(bits);
    assert!(!of);
    assert_eq!(x, val << bits);
    let (x, of) = val.overflowing_shl(255);
    assert!(!of);
    assert_eq!(x, val << 255);
    let (x, of) = val.overflowing_shl(256);
    assert!(of);
    assert_eq!(x, val.clone());
    let times = u128::from(tools::gen_nonzero::<u64>());
    let bits_of = bits + times * 256;
    let (x, of) = val.overflowing_shl(bits_of);
    assert!(of);
    assert_eq!(x, val << bits);
}

#[test]
fn overflowing_shr() {
    let bits = u128::from(tools::gen_nonzero::<u8>());
    let val = &nfuint::U256::thread_random();
    let (x, of) = val.overflowing_shr(bits);
    assert!(!of);
    assert_eq!(x, val >> bits);
    let (x, of) = val.overflowing_shr(255);
    assert!(!of);
    assert_eq!(x, val >> 255);
    let (x, of) = val.overflowing_shr(256);
    assert!(of);
    assert_eq!(x, val.clone());
    let times = u128::from(tools::gen_nonzero::<u64>());
    let bits_of = bits + times * 256;
    let (x, of) = val.overflowing_shr(bits_of);
    assert!(of);
    assert_eq!(x, val >> bits);
}

#[test]
fn overflowing_reg() {
    let zero = nfuint::U256::zero();
    let (x, of) = zero.overflowing_neg();
    assert!(!of);
    assert_eq!(x, zero);
    let val = {
        let mut ret = nfuint::U256::thread_random();
        while ret.is_zero() {
            ret = nfuint::U256::thread_random();
        }
        ret
    };
    let (x, of) = val.overflowing_neg();
    let y = (!val) + nfuint::U256::from(1u8);
    assert!(of);
    assert_eq!(x, y);
}
