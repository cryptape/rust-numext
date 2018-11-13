// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate failure;
#[cfg(feature = "support_serde")]
extern crate faster_hex;
#[cfg(feature = "support_heapsize")]
extern crate heapsize;
#[cfg(feature = "support_rand")]
extern crate rand;
#[cfg(feature = "support_serde")]
extern crate serde;

extern crate numext_constructor;

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
