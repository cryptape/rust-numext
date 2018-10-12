// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate numext_fixed_uint;

use numext_fixed_uint::U256;

// If this test is failed, please check if there is a bug or you are too luckly.
#[test]
fn with_rand_defun_public() {
    let x = U256::thread_random();
    let y = U256::thread_random();
    assert!(!x.is_zero());
    assert!(!y.is_zero());
    assert!(x != y);
}
