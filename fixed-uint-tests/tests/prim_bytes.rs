// Copyright 2018-2019 Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use nfuint_tests::props;
use proptest::{prelude::any, proptest};

proptest! {
    #[test]
    fn swap_bytes(ref mut input in any::<props::U256LeBytes>()) {
        let tmp: nfuint::U256 = (input as &props::U256LeBytes).into();
        input.reverse();
        let expected: nfuint::U256 = (input as &props::U256LeBytes).into();
        let result = tmp.swap_bytes();
        assert_eq!(result, expected);
    }

    #[test]
    fn conv_le_bytes(ref input in any::<props::U256LeBytes>()) {
        let expected: nfuint::U256 = input.into();
        let bytes = expected.to_le_bytes();
        let result = nfuint::U256::from_le_bytes(&bytes);
        assert_eq!(result, expected);
    }

    #[test]
    fn conv_be_bytes(ref input in any::<props::U256LeBytes>()) {
        let expected: nfuint::U256 = input.into();
        let bytes = expected.to_be_bytes();
        let result = nfuint::U256::from_be_bytes(&bytes);
        assert_eq!(result, expected);
    }

    #[test]
    fn conv_ne_bytes(ref input in any::<props::U256LeBytes>()) {
        let expected: nfuint::U256 = input.into();
        let bytes = expected.to_ne_bytes();
        let bytes_same = if cfg!(target_endian = "little") {
            expected.to_le_bytes()
        } else {
            expected.to_be_bytes()
        };
        assert_eq!(bytes, bytes_same);
        let result = nfuint::U256::from_ne_bytes(&bytes);
        let result_same = if cfg!(target_endian = "little") {
            nfuint::U256::from_le_bytes(&bytes_same)
        } else {
            nfuint::U256::from_be_bytes(&bytes_same)
        };
        assert_eq!(result, result_same);
        assert_eq!(result, expected);
    }
}

#[test]
fn conv_bytes_for_specific_cases() {
    {
        let x_str = "111122223333444455556666777788889999aaaabbbbccccddddeeeeffff";
        let y_str = "ffffeeeeddddccccbbbbaaaa9999888877776666555544443333222211110000";
        let x = nfuint::U256::from_hex_str(x_str).unwrap();
        let y = nfuint::U256::from_hex_str(y_str).unwrap();
        assert_eq!(x, y.clone().swap_bytes());
        assert_eq!(x.clone().swap_bytes(), y);
    }
    {
        let bytes = {
            const ____: u8 = 0x00;
            [
                ____, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, ____, ____, ____, ____, ____,
                ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                ____, ____, ____, ____,
            ]
        };
        let str_le = "efcdab896745230100";
        let str_be = "000123456789abcdef0000000000000000000000000000000000000000000000";
        let expected_le = &nfuint::U256::from_hex_str(str_le).unwrap();
        let expected_be = &nfuint::U256::from_hex_str(&str_be[3..]).unwrap();
        let expected_ne = if cfg!(target_endian = "little") {
            expected_le
        } else {
            expected_be
        };
        let result_le = &nfuint::U256::from_le_bytes(&bytes);
        let result_be = &nfuint::U256::from_be_bytes(&bytes);
        let result_ne = &nfuint::U256::from_ne_bytes(&bytes);
        assert_eq!(result_le, expected_le);
        assert_eq!(result_be, expected_be);
        assert_eq!(result_ne, expected_ne);
        assert_eq!(bytes, result_le.to_le_bytes());
        assert_eq!(bytes, result_be.to_be_bytes());
        assert_eq!(bytes, result_ne.to_ne_bytes());
    }
}
