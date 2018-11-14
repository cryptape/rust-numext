// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate numext_fixed_hash as nfhash;

macro_rules! check_default {
    ([$( $hash:ident: $size:expr ),+ ,]) => {{
        check_default![[ $( $hash: $size ),+ ]]
    }};
    ([$( $hash:ident: $size:expr ),+]) => {{
        $( check_default!($hash: $size); )+
    }};
    ($hash:ident: $size:expr) => {
        assert_eq!(nfhash::$hash::default(), [0u8; $size].into());
    };
}

#[test]
fn default() {
    check_default!([
        H128: 16,
        H160: 20,
        H224: 28,
        H256: 32,
        H384: 48,
        H512: 64,
        H520: 65,
        H1024: 128,
        H2048: 256,
        H4096: 512
    ]);
}
