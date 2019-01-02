// Copyright 2018-2019 Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use nfuint::U128;
use nfuint_tests::tools;

#[test]
fn boundary() {
    assert!(U128::min_value() == U128::from(0u128));
    assert!(U128::max_value() == U128::from(!0u128));
}

#[test]
fn bits() {
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
