// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[test]
fn div_throw_add_overflow() {
    let one = nfuint::U256::one();
    for i in 0..255 {
        let x = nfuint::U256::one() << i;
        let y = &x / &one;
        assert_eq!(x, y);
    }
    let x = nfuint::U256::from(4096u32);
    let y = ((nfuint::U256::one() << 255) / &x) << 1;
    let z = ((nfuint::U256::one() << 255) / &y) << 1;
    assert_eq!(x, z);
}
