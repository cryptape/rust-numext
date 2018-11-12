// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate numext_fixed_uint;

macro_rules! check_default {
    ([$( $uint:ident ),+ ,]) => {{
        check_default![[ $( $uint ),+ ]]
    }};
    ([$( $uint:ident ),+]) => {{
        $( check_default!($uint); )+
    }};
    ($uint:ident) => {
        assert_eq!(numext_fixed_uint::$uint::zero(), 0u8.into());
    };
}

#[test]
fn default() {
    check_default!([U128, U160, U224, U256, U384, U512, U520, U1024, U2048, U4096]);
}
