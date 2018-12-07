// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
mod tools;

use failure::Fail;

numext_constructor::construct_fixed_hashes!(
    H128 {
        size = 128,
    },
    H160 {
        size = 160,
    },
    H224 {
        size = 224,
    },
    H256 {
        size = 256,
    },
    H384 {
        size = 384,
    },
    H512 {
        size = 512,
    },
    H520 {
        size = 520,
    },
    H1024 {
        size = 1024,
    },
    H2048 {
        size = 2048,
    },
    H4096 {
        size = 4096,
    },
);

convert_between!(U128, H128, 16);
convert_between!(U160, H160, 20);
convert_between!(U224, H224, 28);
convert_between!(U256, H256, 32);
convert_between!(U384, H384, 48);
convert_between!(U512, H512, 64);
convert_between!(U1024, H1024, 128);
convert_between!(U2048, H2048, 256);
convert_between!(U4096, H4096, 512);
