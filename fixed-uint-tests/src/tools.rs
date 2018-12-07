// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Testing tools.

use crate::props::{U256LeBytes, U256Pair, U256PairParameters};
use proptest::test_runner::TestRunner;
use rand::{self, Rng};

pub fn gen_nonzero<T>() -> T
where
    T: std::cmp::PartialEq,
    T: std::convert::From<bool>,
    rand::distributions::Standard: rand::distributions::Distribution<T>,
{
    let mut rng = rand::thread_rng();
    let mut x: T = rng.gen();
    while x == false.into() {
        x = rng.gen();
    }
    x
}

pub fn pair(param: U256PairParameters) -> U256Pair {
    let mut runner = TestRunner::default();
    let rng = runner.rng();
    U256Pair::new(param, rng)
}

pub fn lebytes() -> U256LeBytes {
    let mut runner = TestRunner::default();
    let rng = runner.rng();
    U256LeBytes::any(rng)
}
