// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate numext_fixed_uint as nfuint;

#[test]
fn div_throw_add_overflow() {
    let one = nfuint::U256::one();
    for i in 0..255 {
        let x = nfuint::U256::one() << i;
        let y = &x / &one;
        assert_eq!(x, y);
    }
}
