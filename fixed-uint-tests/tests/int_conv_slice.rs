// Copyright 2018-2019 Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use nfuint_tests::props;
use proptest::{prelude::any, proptest, proptest_helper};

proptest! {
    #[test]
    fn into_le_slice(ref le in any::<props::U256LeBytes>()) {
        let expected = {
            let mut ret = [0u8; 32];
            let val: etypes::U256 = le.into();
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
            let val: etypes::U256 = le.into();
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
            let val: etypes::U256 = le.into();
            val.to_little_endian(&mut ret);
            ret
        };
        let expected: props::U256LeBytes = {
            let ret = etypes::U256::from_little_endian(&slice[..]);
            ret.into()
        };
        let result: props::U256LeBytes = {
            let ret = nfuint::U256::from_little_endian(&slice[..]);
            ret.unwrap().into()
        };
        assert_eq!(expected, result);
    }

    #[test]
    fn from_be_slice(ref le in any::<props::U256LeBytes>()) {
        let slice = {
            let mut ret = [0u8; 32];
            let val: etypes::U256 = le.into();
            val.to_big_endian(&mut ret);
            ret
        };
        let expected: props::U256LeBytes = {
            let ret = etypes::U256::from_big_endian(&slice[..]);
            ret.into()
        };
        let result: props::U256LeBytes = {
            let ret = nfuint::U256::from_big_endian(&slice[..]);
            ret.unwrap().into()
        };
        assert_eq!(expected, result);
    }

    #[test]
    fn conv_le_slice(ref le in any::<props::U256LeBytes>()) {
        let expected: nfuint::U256 = le.into();
        let mut slice = [0u8; 32];
        expected.into_little_endian(&mut slice).unwrap();
        let result = nfuint::U256::from_little_endian(&slice[..]).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn conv_be_slice(ref le in any::<props::U256LeBytes>()) {
        let expected: nfuint::U256 = le.into();
        let mut slice = [0u8; 32];
        expected.into_big_endian(&mut slice).unwrap();
        let result = nfuint::U256::from_big_endian(&slice[..]).unwrap();
        assert_eq!(result, expected);
    }
}
