// Copyright 2018-2019 Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This is a internal crate used by [numext-fixed-uint].
//!
//! **Notice:
//! You should NOT use this crate directly.
//! Please use [numext-fixed-uint] instead of this crate.**
//!
//! [numext-fixed-uint]: https://docs.rs/numext-fixed-uint

extern crate constructor;

use thiserror::Error;

constructor::construct_fixed_uints!(
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
);
