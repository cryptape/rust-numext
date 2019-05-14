// Copyright 2018-2019 Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use nfhash::{h128, h4096, H128, H4096};
use std::str::FromStr;

const H128_ZERO: H128 = h128!("0x0");
const H64MAX: H4096 = h4096!("0x_ffff_ffff_ffff_ffff");

#[test]
fn constructor() {
    assert_eq!(H128_ZERO, H128::zero());
    {
        let x1 = h128!("0x123456789abcdef");
        let x2 = h128!("0x00000000000000000123456789abcdef");
        let y = H128::from_str("00000000000000000123456789abcdef").unwrap();
        assert_eq!(x1, y);
        assert_eq!(x2, y);
    }
    {
        let x = h4096!("0x_ffff_ffff_ffff_ffff");
        let y = H4096::from_trimmed_hex_str("ffffffffffffffff").unwrap();
        assert_eq!(x, y);
        assert_eq!(H64MAX, y);
    }
}
