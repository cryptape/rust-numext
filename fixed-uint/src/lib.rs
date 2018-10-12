// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate failure;
extern crate rand;

#[cfg(not(feature = "use-derive"))]
extern crate numext_fixed_uint_macros;

#[cfg(not(feature = "use-derive"))]
include!("include/construct-by-procmacro.rs");

#[cfg(feature = "use-derive")]
#[macro_use]
extern crate numext_fixed_uint_macros;

#[cfg(feature = "use-derive")]
include!("include/construct-by-derive.rs");
