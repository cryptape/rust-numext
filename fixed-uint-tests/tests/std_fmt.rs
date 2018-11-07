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
use proptest::prelude::any;

macro_rules! check_fmt {
    ($fmt_str:expr, $val:ident, $expected:expr) => {
        assert_eq!(format!($fmt_str, $val).as_str(), $expected);
    };
    ($fmt_str:expr, $val:expr, $expected:expr) => {
        assert_eq!(format!($fmt_str, $val).as_str(), $expected);
    };
}

macro_rules! std_fmt {
    ($name:ident, $fmt_str:expr) => {
        proptest! {
            #[test]
            fn $name(ref le in any::<props::U256LeBytes>()) {
                let str_uint = {
                    let val: uint::U256 = le.into();
                    format!($fmt_str, val)
                };
                let str_nfuint = {
                    let val: nfuint::U256 = le.into();
                    format!($fmt_str, val)
                };
                assert_eq!(str_uint, str_nfuint);
            }
        }
    };
}

std_fmt!(display_ramdom, "{}");

#[test]
fn debug() {
    let x = nfuint::U256([0x10, 0x0, 0x0, 0x0]);
    let y = nfuint::U256::one() << 4u8;
    assert_eq!(x, y);
    check_fmt!("{:?}", nfuint::U128::from(0u128), "U128 ( [ 0x0, 0x0 ] )");
    check_fmt!(
        "{:?}",
        nfuint::U256::from(0u128),
        "U256 ( [ 0x0, 0x0, 0x0, 0x0 ] )"
    );
    check_fmt!(
        "{:?}",
        nfuint::U256::from(1u128),
        "U256 ( [ 0x1, 0x0, 0x0, 0x0 ] )"
    );
    check_fmt!(
        "{:?}",
        nfuint::U256::from(0x0001_0000_0000_0000_0000u128),
        "U256 ( [ 0x0, 0x1, 0x0, 0x0 ] )"
    );
    check_fmt!(
        "{:?}",
        nfuint::U256::from(0x0001_0000_0000_0000_0001u128),
        "U256 ( [ 0x1, 0x1, 0x0, 0x0 ] )"
    );
}

#[test]
fn binary() {
    check_fmt!("{:b}", nfuint::U256::from(1u128), "1");
    check_fmt!("{:b}", nfuint::U256::from(2u128), "10");
    check_fmt!("{:b}", nfuint::U256::from(3u128), "11");
    check_fmt!(
        "{:b}",
        nfuint::U256::from(0x1234_5678_90ab_cdef_fedc_ba09_8765_4321u128),
        "1001000110100010101100111100010010000101010111100110111101111111\
         1111011011100101110100000100110000111011001010100001100100001"
    );
}

#[test]
fn octal() {
    check_fmt!("{:o}", nfuint::U256::from(7u128), "7");
    check_fmt!("{:o}", nfuint::U256::from(8u128), "10");
    check_fmt!("{:o}", nfuint::U256::from(9u128), "11");
    check_fmt!(
        "{:o}",
        nfuint::U256::from(0x1234_5678_90ab_cdef_fedc_ba09_8765_4321u128),
        "221505317044125715737773345640460731241441"
    );
}

#[test]
fn lowerhex() {
    check_fmt!("{:x}", nfuint::U256::from(15u128), "f");
    check_fmt!("{:x}", nfuint::U256::from(16u128), "10");
    check_fmt!("{:x}", nfuint::U256::from(17u128), "11");
    check_fmt!(
        "{:x}",
        nfuint::U256::from(0x1234_5678_90ab_cdef_fedc_ba09_8765_4321u128),
        "1234567890abcdeffedcba0987654321"
    );
}

#[test]
fn upperhex() {
    check_fmt!("{:X}", nfuint::U256::from(15u128), "F");
    check_fmt!("{:X}", nfuint::U256::from(16u128), "10");
    check_fmt!("{:X}", nfuint::U256::from(17u128), "11");
    check_fmt!(
        "{:X}",
        nfuint::U256::from(0x1234_5678_90ab_cdef_fedc_ba09_8765_4321u128),
        "1234567890ABCDEFFEDCBA0987654321"
    );
}

#[test]
fn display() {
    check_fmt!("{}", nfuint::U256::from(9u128), "9");
    check_fmt!("{}", nfuint::U256::from(10u128), "10");
    check_fmt!("{}", nfuint::U256::from(11u128), "11");
    check_fmt!(
        "{}",
        nfuint::U256::from(0x1234_5678_90ab_cdef_fedc_ba09_8765_4321u128),
        "24197857200151252746022454892744229665"
    );
}

#[test]
fn zero() {
    let zero = nfuint::U256::zero();

    check_fmt!("{:b}", zero, "0");
    check_fmt!("{:o}", zero, "0");
    check_fmt!("{:x}", zero, "0");
    check_fmt!("{:X}", zero, "0");
    check_fmt!("{}", zero, "0");

    check_fmt!("{:#b}", zero, "0b0");
    check_fmt!("{:#o}", zero, "0o0");
    check_fmt!("{:#x}", zero, "0x0");
    check_fmt!("{:#X}", zero, "0x0");
}

#[test]
fn one() {
    let one = nfuint::U256::one();

    check_fmt!("{:b}", one, "1");
    check_fmt!("{:o}", one, "1");
    check_fmt!("{:x}", one, "1");
    check_fmt!("{:X}", one, "1");
    check_fmt!("{}", one, "1");

    check_fmt!("{:#b}", one, "0b1");
    check_fmt!("{:#o}", one, "0o1");
    check_fmt!("{:#x}", one, "0x1");
    check_fmt!("{:#X}", one, "0x1");
}

#[test]
fn alternate() {
    let val = nfuint::U256::from(0x00fe_dcbau128);
    check_fmt!("{:#b}", val, "0b111111101101110010111010");
    check_fmt!("{:#o}", val, "0o77556272");
    check_fmt!("{:#x}", val, "0xfedcba");
    check_fmt!("{:#X}", val, "0xFEDCBA");
}
