// Copyright 2018-2019 Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use nfuint::U256;

#[test]
#[should_panic]
fn add_overflow() {
    let max = U256::max_value();
    let one = U256::one();
    let _ = max + one;
}

#[test]
#[should_panic]
fn sub_overflow() {
    let zero = U256::zero();
    let one = U256::one();
    let _ = zero - one;
}

#[test]
#[should_panic]
fn mul_overflow_size() {
    let max = U256::max_value();
    let _ = &max * &max;
}

#[test]
#[should_panic]
fn mul_overflow_value() {
    let val = U256::from(u128::max_value()) + U256::one();
    let _ = &val * &val;
}

#[test]
#[should_panic]
fn div_zero() {
    let val = U256::thread_random();
    let zero = U256::zero();
    let _ = val / zero;
}

#[test]
#[should_panic]
fn rem_zero() {
    let val = U256::thread_random();
    let zero = U256::zero();
    let _ = val % zero;
}
