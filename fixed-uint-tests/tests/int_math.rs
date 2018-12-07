// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use nfuint::U256;
use nfuint_tests::props;
use num_bigint::BigUint;
use num_integer::Integer;
use proptest::{prelude::any_with, proptest, proptest_helper};

proptest! {
    #[test]
    fn gcd(ref pair in any_with::<props::U256Pair>(props::U256PairParameters::Random)) {
        let (ref lhs, ref rhs): (U256, U256) = pair.into();
        let x = lhs.gcd(rhs);
        let (ref lhs, ref rhs): (BigUint, BigUint) = pair.into();
        let y = lhs.gcd(rhs);
        assert_eq!(y.to_str_radix(16), format!("{:x}", x));
        println!("x = {:x}", x);
    }
}
