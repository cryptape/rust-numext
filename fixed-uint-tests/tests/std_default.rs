// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate numext_fixed_uint;

macro_rules! check_default {
    ($uint:ident) => {
        assert_eq!(numext_fixed_uint::$uint::zero(), 0u8.into());
    };
}

#[test]
fn default() {
    check_default!(U128);
    check_default!(U160);
    check_default!(U224);
    check_default!(U256);
    check_default!(U384);
    check_default!(U512);
    check_default!(U1024);
    check_default!(U2048);
    check_default!(U4096);
}
