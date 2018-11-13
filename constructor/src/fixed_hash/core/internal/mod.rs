// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Add internal methods for hash.
//!
//! These methods do not depend on any traits or crates, except: Default, Clone.

mod kernel;
mod private_conv;
mod private_ops;
mod public_basic;
mod public_conv;
