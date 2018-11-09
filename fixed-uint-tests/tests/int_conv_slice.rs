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

proptest! {
    #[test]
    fn into_le_slice(ref le in any::<props::U256LeBytes>()) {
        let expected = {
            let mut ret = [0u8; 32];
            let val: uint::U256 = le.into();
            val.to_little_endian(&mut ret);
            ret
        };
        let result = {
            let mut ret = [0u8; 32];
            let val: nfuint::U256 = le.into();
            val.into_little_endian(&mut ret).unwrap();
            ret
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn into_be_slice(ref le in any::<props::U256LeBytes>()) {
        let expected = {
            let mut ret = [0u8; 32];
            let val: uint::U256 = le.into();
            val.to_big_endian(&mut ret);
            ret
        };
        let result = {
            let mut ret = [0u8; 32];
            let val: nfuint::U256 = le.into();
            val.into_big_endian(&mut ret).unwrap();
            ret
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn from_le_slice(ref le in any::<props::U256LeBytes>()) {
        let slice = {
            let mut ret = [0u8; 32];
            let val: uint::U256 = le.into();
            val.to_little_endian(&mut ret);
            ret
        };
        let le_uint: props::U256LeBytes = {
            let ret = uint::U256::from_little_endian(&slice[..]);
            ret.into()
        };
        let le_nfuint: props::U256LeBytes = {
            let ret = nfuint::U256::from_little_endian(&slice[..]);
            ret.unwrap().into()
        };
        assert_eq!(le_uint, le_nfuint);
    }

    #[test]
    fn from_be_slice(ref le in any::<props::U256LeBytes>()) {
        let slice = {
            let mut ret = [0u8; 32];
            let val: uint::U256 = le.into();
            val.to_big_endian(&mut ret);
            ret
        };
        let le_uint: props::U256LeBytes = {
            let ret = uint::U256::from_little_endian(&slice[..]);
            ret.into()
        };
        let le_nfuint: props::U256LeBytes = {
            let ret = nfuint::U256::from_little_endian(&slice[..]);
            ret.unwrap().into()
        };
        assert_eq!(le_uint, le_nfuint);
    }
}
