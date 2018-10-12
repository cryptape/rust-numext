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

use nfuint_tests::props;
use proptest::prelude::any;

macro_rules! std_ops_shift {
    ($opr:tt, $bits:expr, $name:ident) => {
        proptest! {
            #[test]
            fn $name(ref le in any::<props::U256LeBytes>()) {
                let le_uint: props::U256LeBytes = {
                    let val: uint::U256 = le.into();
                    let ret: uint::U256 = val $opr $bits;
                    (&ret).into()
                };
                let le_nfuint: props::U256LeBytes = {
                    let val: nfuint::U256 = le.into();
                    let ret: nfuint::U256 = val $opr $bits;
                    (&ret).into()
                };
                assert_eq!(le_uint, le_nfuint);
            }
        }
    };
}

macro_rules! std_ops_shift_neg {
    (<<, $bits:expr, $name:ident) => {
        std_ops_shift_neg!(<<, >>, $bits, $name);
    };
    (>>, $bits:expr, $name:ident) => {
        std_ops_shift_neg!(<<, >>, $bits, $name);
    };
    ($opr:tt, $opr_neg:tt, $bits:expr, $name:ident) => {
        proptest! {
            #[test]
            fn $name(ref le in any::<props::U256LeBytes>()) {
                let result: nfuint::U256 = {
                    let val: nfuint::U256 = le.into();
                    val $opr $bits
                };
                let expected: nfuint::U256 = {
                    let val: nfuint::U256 = le.into();
                    val $opr_neg $bits.abs()
                };
                assert_eq!(result, expected);
            }
        }
    };
}

std_ops_shift!(<<, 0u8, ushl0);
std_ops_shift!(<<, 1u8, ushl1);
std_ops_shift!(<<, 7u16, ushl17);
std_ops_shift!(<<, 8u16, ushl18);
std_ops_shift!(<<, 31u32, ushl31);
std_ops_shift!(<<, 32u32, ushl32);
std_ops_shift!(<<, 129u64, ushl129);
std_ops_shift!(<<, 511u64, ushl511);
std_ops_shift!(<<, 513usize, ushl513);

std_ops_shift!(>>, 0u8, ushr0);
std_ops_shift!(>>, 1u8, ushr1);
std_ops_shift!(>>, 7u16, ushr7);
std_ops_shift!(>>, 8u16, ushr8);
std_ops_shift!(>>, 31u32, ushr31);
std_ops_shift!(>>, 32u32, ushr32);
std_ops_shift!(>>, 129u64, ushr129);
std_ops_shift!(>>, 511u64, ushr511);
std_ops_shift!(>>, 513usize, ushr513);

std_ops_shift!(<<, 0i8, ishl0);
std_ops_shift!(<<, 1i8, ishl1);
std_ops_shift!(<<, 7i16, ishl17);
std_ops_shift!(<<, 8i16, ishl18);
std_ops_shift!(<<, 31i32, ishl31);
std_ops_shift!(<<, 32i32, ishl32);
std_ops_shift!(<<, 129i64, ishl129);
std_ops_shift!(<<, 511i64, ishl511);
std_ops_shift!(<<, 513isize, ishl513);

std_ops_shift!(>>, 0i8, ishr0);
std_ops_shift!(>>, 1i8, ishr1);
std_ops_shift!(>>, 7i16, ishr7);
std_ops_shift!(>>, 8i16, ishr8);
std_ops_shift!(>>, 31i32, ishr31);
std_ops_shift!(>>, 32i32, ishr32);
std_ops_shift!(>>, 129i64, ishr129);
std_ops_shift!(>>, 511i64, ishr511);
std_ops_shift!(>>, 513isize, ishr513);

std_ops_shift_neg!(<<, -1i8, nishl1);
std_ops_shift_neg!(<<, -7i16, nishl17);
std_ops_shift_neg!(<<, -8i16, nishl18);
std_ops_shift_neg!(<<, -31i32, nishl31);
std_ops_shift_neg!(<<, -32i32, nishl32);
std_ops_shift_neg!(<<, -129i64, nishl129);
std_ops_shift_neg!(<<, -511i64, nishl511);
std_ops_shift_neg!(<<, -513isize, nishl513);

std_ops_shift_neg!(>>, -1i8, nishr1);
std_ops_shift_neg!(>>, -7i16, nishr7);
std_ops_shift_neg!(>>, -8i16, nishr8);
std_ops_shift_neg!(>>, -31i32, nishr31);
std_ops_shift_neg!(>>, -32i32, nishr32);
std_ops_shift_neg!(>>, -129i64, nishr129);
std_ops_shift_neg!(>>, -511i64, nishr511);
std_ops_shift_neg!(>>, -513isize, nishr513);
