// Copyright 2018-2019 Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use nfuint::{U128, U256};

#[test]
fn bits_ops() {
    let mut x = U128::zero();
    let y: U128 = U128::from(0b1010u8) << 64u8;

    assert!(U128::count_bits() == 128);

    assert!(x.bit(128) == None);
    assert!(!x.set_bit(128, true));

    assert!(x.bit(65) == Some(false));
    assert!(y.bit(65) == Some(true));
    assert!(y.bit(64) == Some(false));

    assert!(x.highest_one() == None);
    assert!(x.lowest_one() == None);
    assert!(y.highest_one() == Some(67));
    assert!(y.lowest_one() == Some(65));

    assert!(x.set_bit(65, true));
    assert!(x.bit(65) == Some(true));
    assert!(x.set_bit(67, true));
    assert!(x == y);
}

#[test]
fn bytes_ops() {
    let mut x = U128::zero();
    let y: U128 = U128::from(0xcd00_ab00u64) << 64u8;

    assert!(U128::count_bytes() == 16);

    assert!(x.byte(16) == None);
    assert!(!x.set_byte(16, 0xab));

    assert!(x.byte(9) == Some(0));
    assert!(y.byte(9) == Some(0xab));
    assert!(y.byte(8) == Some(0));

    assert!(x.highest_nonzero_byte() == None);
    assert!(x.lowest_nonzero_byte() == None);
    assert!(y.highest_nonzero_byte() == Some(11));
    assert!(y.lowest_nonzero_byte() == Some(9));

    assert!(x.set_byte(9, 0xab));
    assert!(x.byte(9) == Some(0xab));
    assert!(x.set_byte(11, 0xcd));
    assert!(x == y);
}

#[test]
fn arith_ops() {
    let max = U256::max_value();
    let one = U256::one();
    let val1 = &max - &one;
    let val2 = &val1 - &one;
    let (low, high) = val1.complete_mul(&val2);
    assert_eq!(low, U256::from(6u8));
    assert_eq!(high, &max - U256::from(4u8));
    let val = {
        let mut ret = U256::thread_random();
        while ret.is_zero() {
            ret = U256::thread_random();
        }
        ret
    };
    let (quotient, remainder) = max.complete_div(&val);
    assert_eq!(quotient, &max / &val);
    assert_eq!(remainder, &max % &val);
}
