// Copyright 2018-2019 Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use nfuint::U128;

const U128_MIN: U128 = U128::min_value();
const U128_MAX: U128 = U128::max_value();

#[test]
fn boundary() {
    assert!(U128_MIN == U128::from(0u128));
    assert!(U128_MAX == U128::from(!0u128));
}
