// Copyright 2018-2019 Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate proptest;

use nfuint::{U128, U256};
use nfuint_tests::{props, tools};
use proptest::prelude::any;

#[test]
fn bits_basic() {
    let hi = tools::gen_nonzero::<u64>();
    let lo = tools::gen_nonzero::<u64>();

    let val = (u128::from(hi) << 64) + u128::from(lo);
    let x = U128::from(val);
    assert!(x.count_ones() == lo.count_ones() + hi.count_ones());
    assert!(x.count_zeros() == lo.count_zeros() + hi.count_zeros());

    let val = u128::from(lo);
    let x = U128::from(val);
    assert!(x.leading_zeros() == lo.leading_zeros() + 64);
    assert!(x.trailing_zeros() == lo.trailing_zeros());

    let val = u128::from(hi) << 64;
    let x = U128::from(val);
    assert!(x.leading_zeros() == hi.leading_zeros());
    assert!(x.trailing_zeros() == hi.trailing_zeros() + 64);

    let x = U128::from(0u128);
    assert!(x.leading_zeros() == 128);
    assert!(x.trailing_zeros() == 128);
}

proptest! {
    #[test]
    fn rotate_bits_1(v in any::<u128>(), n in any::<u32>()) {
        assert_eq!(U128::from(v).rotate_left(n), U128::from(v.rotate_left(n)));
        assert_eq!(U128::from(v).rotate_right(n), U128::from(v.rotate_right(n)));
    }

    #[test]
    fn rotate_bits_2(ref le in any::<props::U256LeBytes>(), n in any::<u32>()) {
        let v: U256 = le.into();

        assert_eq!(v, v.rotate_left(n).rotate_right(n));
        assert_eq!(v, v.rotate_right(n).rotate_left(n));

        if n == 0 {
            assert_eq!(v, v.rotate_left(n));
            assert_eq!(v, v.rotate_right(n));
        } else {
            let c = U256::count_bits() as u32;
            let m = c - (n % c);
            assert_eq!(v, v.rotate_left(n).rotate_left(m));
            assert_eq!(v, v.rotate_right(n).rotate_right(m));
            assert_eq!(v, v.rotate_left(m).rotate_left(n));
            assert_eq!(v, v.rotate_right(m).rotate_right(n));
        }
    }
}
