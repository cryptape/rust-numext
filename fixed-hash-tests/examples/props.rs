// Copyright 2018-2019 Cryptape Technologies LLC.
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

fn props_bytes() {
    let mut runner = proptest::test_runner::TestRunner::default();
    let strategy = nfhash_tests::props::H256Bytes::arbitrary();
    let mut val = strategy.new_tree(&mut runner).unwrap();
    println!("bytes val    = {:?}", val.current());
    while val.simplify() {
        println!("        (-) = {:?}", val.current());
    }
    while val.complicate() {
        println!("        (+) = {:?}", val.current());
    }
}

fn main() {
    props_bytes();
}
