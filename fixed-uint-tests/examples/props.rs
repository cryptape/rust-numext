// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use proptest::{
    arbitrary::Arbitrary,
    strategy::{Strategy, ValueTree},
};

macro_rules! props_pair_simplify {
    ($name:ident, $param:ident) => {{
        let mut runner = proptest::test_runner::TestRunner::default();
        let params = nfuint_tests::props::U256PairParameters::$param;
        let strategy = nfuint_tests::props::U256Pair::arbitrary_with(params);
        let mut val = strategy.new_tree(&mut runner).unwrap();
        println!("{}     val = {:?}", stringify!($name), val.current());
        while val.simplify() {
            println!("        (-) = {:?}", val.current());
        }
        while val.complicate() {
            println!("        (+) = {:?}", val.current());
        }
    }};
}

fn props_lebytes() {
    let mut runner = proptest::test_runner::TestRunner::default();
    let strategy = nfuint_tests::props::U256LeBytes::arbitrary();
    let mut val = strategy.new_tree(&mut runner).unwrap();
    println!("le bytes val = {:?}", val.current());
    while val.simplify() {
        println!("        (-) = {:?}", val.current());
    }
    while val.complicate() {
        println!("        (+) = {:?}", val.current());
    }
}

fn main() {
    props_pair_simplify!(rnd, Random);
    props_pair_simplify!(add, CanAdd);
    props_pair_simplify!(sub, CanSub);
    props_pair_simplify!(mul, CanMul);
    props_pair_simplify!(div, CanDiv);
    props_pair_simplify!(rem, CanRem);
    props_lebytes();
}
