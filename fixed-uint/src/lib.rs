// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate failure;
#[cfg(feature = "impl_rand")]
extern crate rand;

extern crate numext_fixed_uint_macros;

numext_fixed_uint_macros::construct_fixed_uints!(
    U128 {
        size = 128,
    },
    U160 {
        size = 160,
    },
    U224 {
        size = 224,
    },
    U256 {
        size = 256,
    },
    U384 {
        size = 384,
    },
    U512 {
        size = 512,
    },
    U520 {
        size = 520,
    },
    U1024 {
        size = 1024,
    },
    U2048 {
        size = 2048,
    },
    U4096 {
        size = 4096,
    },
    H128 {
        size = 128,
        unit_size = 8,
    },
    H160 {
        size = 160,
        unit_size = 8,
    },
    H224 {
        size = 224,
        unit_size = 8,
    },
    H256 {
        size = 256,
        unit_size = 8,
    },
    H384 {
        size = 384,
        unit_size = 8,
    },
    H512 {
        size = 512,
        unit_size = 8,
    },
    H520 {
        size = 520,
        unit_size = 8,
    },
    H1024 {
        size = 1024,
        unit_size = 8,
    },
    H2048 {
        size = 2048,
        unit_size = 8,
    },
    H4096 {
        size = 4096,
        unit_size = 8,
    },
);
