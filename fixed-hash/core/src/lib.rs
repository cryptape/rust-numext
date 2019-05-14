// Copyright 2018-2019 Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This is a internal crate used by [numext-fixed-hash].
//!
//! **Notice:
//! You should NOT use this crate directly.
//! Please use [numext-fixed-hash] instead of this crate.**
//!
//! [numext-fixed-hash]: https://docs.rs/numext-fixed-hash

use failure::Fail;

#[macro_use]
mod tools;

constructor::construct_fixed_hashes!(
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

#[cfg(feature = "bits_128")]
convert_between!(U128, H128, 16);
#[cfg(feature = "bits_160")]
convert_between!(U160, H160, 20);
#[cfg(feature = "bits_224")]
convert_between!(U224, H224, 28);
#[cfg(feature = "bits_256")]
convert_between!(U256, H256, 32);
#[cfg(feature = "bits_384")]
convert_between!(U384, H384, 48);
#[cfg(feature = "bits_512")]
convert_between!(U512, H512, 64);
#[cfg(feature = "bits_520")]
convert_between!(U520, H520, 65);
#[cfg(feature = "bits_1024")]
convert_between!(U1024, H1024, 128);
#[cfg(feature = "bits_2048")]
convert_between!(U2048, H2048, 256);
#[cfg(feature = "bits_4096")]
convert_between!(U4096, H4096, 512);
