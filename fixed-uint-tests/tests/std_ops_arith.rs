// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate proptest;

extern crate ethereum_types as etypes;
extern crate numext_fixed_uint as nfuint;
extern crate numext_fixed_uint_tests as nfuint_tests;

use nfuint_tests::props;
use proptest::prelude::{any, any_with};

macro_rules! std_ops_binary {
    ($opr:tt, $name:ident, $param:ident) => {
        proptest! {
            #[test]
            fn $name(ref pair in any_with::<props::U256Pair>(props::U256PairParameters::$param)) {
                let le_etypes: props::U256LeBytes = {
                    let (lhs, rhs): (etypes::U256, etypes::U256) = pair.into();
                    let ret = lhs $opr rhs;
                    ret.into()
                };
                let le_nfuint: props::U256LeBytes = {
                    let (lhs, rhs): (nfuint::U256, nfuint::U256) = pair.into();
                    let ret = lhs $opr rhs;
                    ret.into()
                };
                assert_eq!(le_etypes, le_nfuint);
            }
        }
    };
}

macro_rules! std_ops_unary {
    ($opr:tt, $name:ident) => {
        proptest! {
            #[test]
            fn $name(ref le in any::<props::U256LeBytes>()) {
                let le_etypes: props::U256LeBytes = {
                    let val: etypes::U256 = le.into();
                    let ret = $opr val;
                    ret.into()
                };
                let le_nfuint: props::U256LeBytes = {
                    let val: nfuint::U256 = le.into();
                    let ret = $opr val;
                    ret.into()
                };
                assert_eq!(le_etypes, le_nfuint);
            }
        }
    };
}

std_ops_binary!(+, add, CanAdd);
std_ops_binary!(-, sub, CanSub);
std_ops_binary!(*, mul, CanMul);
std_ops_binary!(/, div, CanDiv);
std_ops_binary!(%, rem, CanRem);
std_ops_binary!(&, bitand, Random);
std_ops_binary!(|, bitor, Random);
std_ops_binary!(^, bitxor, Random);

std_ops_unary!(!, not);
