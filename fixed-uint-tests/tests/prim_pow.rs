// Copyright 2018-2019 Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate proptest;

use nfuint_tests::props;
use proptest::prelude::any;

proptest! {

    #[test]
    fn pow(ref le in any::<props::U256LeBytes>(), n in any::<u32>()) {
        let (result_val, result_of) = {
            let val: nfuint::U256 = le.into();
            let (x, of) = val.overflowing_pow(n);
            let y: props::U256LeBytes = x.into();
            (y, of)
        };
        let (expected_val, expected_of) = {
            let val: etypes::U256 = le.into();
            let (x, of) = val.overflowing_pow(etypes::U256::from(n));
            let y: props::U256LeBytes = x.into();
            (y, of)
        };
        assert_eq!(result_val, expected_val);
        assert_eq!(result_of, expected_of);
    }

    #[test]
    fn pow_by_constant(ref le in any::<props::U256LeBytes>()) {
        let one = nfuint::U256::one();
        let val: nfuint::U256 = le.into();
        assert_eq!(val.pow(0), one);
        assert_eq!(val.pow(1), val);
    }

    #[test]
    fn is_power_of_two(ref le in any::<props::U256LeBytes>(), n in any::<u32>()) {
        let x: nfuint::U256 = le.into();
        assert_eq!(x.is_power_of_two(), x.highest_one()==x.lowest_one());
        assert_eq!(x.is_power_of_two(), x.rotate_left(n).is_power_of_two());
        assert_eq!(x.is_power_of_two(), x.rotate_right(n).is_power_of_two());
    }

    #[test]
    fn next_power_of_two(ref le in any::<props::U256LeBytes>()) {
        let mut x: nfuint::U256 = le.into();
        x >>= 1;
        let y = x.next_power_of_two();
        if !y.is_zero() {
            assert!(y.is_power_of_two());
        }
        assert_eq!(x.is_power_of_two(), x == y);
    }
}

#[test]
fn power_of_one() {
    let one = &nfuint::U256::one();
    for i in 0u8..255 {
        let x = one << i;
        assert!(x.is_power_of_two());
        assert_eq!(x, x.next_power_of_two());
    }
}

#[test]
fn power_of_two() {
    let one = &nfuint::U512::one();
    let two = &nfuint::U512::from(2u128);
    let mut sum = nfuint::U512::from(3u128);
    for i in 2u32..512 {
        let result = two.pow(i);

        assert_eq!(result, one << i);
        assert!(result.is_power_of_two());
        assert_eq!(result, result.next_power_of_two());

        assert!(!sum.is_power_of_two());
        assert_eq!(result, (&sum + one));
        assert_eq!(result, sum.next_power_of_two());

        sum += &result;

        if i < 128 {
            let expected = nfuint::U512::from(2u128.pow(i));
            assert_eq!(result, expected);
        }
    }
    let result = two.pow(511) - one + two.pow(511);
    let expected = nfuint::U512::max_value();
    assert_eq!(result, expected);
}

#[test]
#[should_panic]
fn power_of_two_panic_1() {
    let two = &nfuint::U256::from(2u128);
    let _ = two.pow(256);
}

#[test]
fn power_of_three() {
    let three = &nfuint::U256::from(3u128);
    for i in 2u32..21 {
        let expected = nfuint::U256::from(3u128.pow(i));
        let result = three.pow(i);
        assert!(!result.is_power_of_two());
        assert_eq!(result, expected);
    }
    // should not panic: (2^256-1) > 3^161
    let _ = three.pow(161);
}

#[test]
#[should_panic]
fn power_of_three_panic() {
    let three = &nfuint::U256::from(3u128);
    // should panic: (2^256-1) < 3^162
    let _ = three.pow(162);
}

#[test]
fn is_power_of_two_for_specific_cases() {
    let testcases = vec![
        (0u128, false),
        (1, true),
        (2, true),
        (3, false),
        (4, true),
        (5, false),
        (6, false),
        (7, false),
        (8, true),
    ];
    for (input, expected) in testcases.into_iter() {
        let result = nfuint::U256::from(input).is_power_of_two();
        assert_eq!(result, expected);
    }
}

#[test]
fn next_power_of_two_not_panic() {
    let two = &nfuint::U256::from(2u128);
    let val = two.pow(255);
    let _ = val.next_power_of_two();
}

#[test]
#[should_panic]
fn next_power_of_two_panic_1() {
    let one = &nfuint::U256::one();
    let two = &nfuint::U256::from(2u128);
    let val = two.pow(255) + one;
    let _ = val.next_power_of_two();
}

#[test]
#[should_panic]
fn next_power_of_two_panic_2() {
    let max = &nfuint::U256::max_value();
    let _ = max.next_power_of_two();
}
