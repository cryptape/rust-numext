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

macro_rules! std_convert_from {
    ($name:ident, $from_type:ty) => {
        proptest! {
            #[test]
            fn $name(val in any::<$from_type>()) {
                let le_uint: props::U256LeBytes = {
                    let ret: uint::U256 = val.into();
                    ret.into()
                };
                let le_nfuint: props::U256LeBytes = {
                    let ret: nfuint::U256 = val.into();
                    ret.into()
                };
                assert_eq!(le_uint, le_nfuint);
            }
        }
    };
}

proptest! {
    #[test]
    fn from_bool(val in any::<bool>()) {
        let expected = if val {
            nfuint::U256::one()
        } else {
            nfuint::U256::zero()
        };
        assert_eq!(expected, val.into());
    }
}

std_convert_from!(from_u8, u8);
std_convert_from!(from_u16, u16);
std_convert_from!(from_u32, u32);
std_convert_from!(from_u64, u64);

proptest! {
    #[test]
    fn from_u128(val in any::<u128>()) {
        let le_uint: props::U256LeBytes = {
            let hi: uint::U256 = ((val >> 64) as u64).into();
            let lo: uint::U256 = (val as u64).into();
            let ret = (hi << 64) + lo;
            ret.into()
        };
        let le_nfuint: props::U256LeBytes = {
            let ret: nfuint::U256 = val.into();
            ret.into()
        };
        assert_eq!(le_uint, le_nfuint);
    }
}
