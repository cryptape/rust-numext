// Copyright 2018-2019 Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use nfuint::{u128, u256, u4096, U128, U256, U4096};

const U128_ZERO: U128 = u128!("0");
const U128_100: U128 = u128!("100");

#[test]
fn constructor() {
    assert_eq!(U128_ZERO, U128::zero());
    {
        let x1 = u128!("0b110_0100");
        let x2 = u128!("0o144");
        let x3 = u128!("0x64");
        let y = U128::from(100u8);
        assert_eq!(x1, y);
        assert_eq!(x2, y);
        assert_eq!(x3, y);
        assert_eq!(U128_100, y);
    }
    {
        let x = u4096!("0x_ab_cdef");
        let y = U4096([
            0x00ab_cdef,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ]);
        assert_eq!(x, y);
    }
    {
        let x = u256!("100_000_000_000_000_000_000");
        let y = U256([0x6bc7_5e2d_6310_0000, 0x5, 0, 0]);
        assert_eq!(x, y);
    }
}
