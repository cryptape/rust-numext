// Copyright 2018-2019 Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A series of fixed non-negative integer types.
//!
//! # Constructors
//!
//! This crate provides a series of macros that used to construct fixed uints in compile time.
//!
//! The input is a string literal, and the macros support several formats of the input:
//! - A decimal string.
//! - A binary string with `0b` prefix.
//! - A octal string with `0o` prefix.
//! - A hexadecimal string with `0x` prefix.
//!
//! And you can use any number of `_` in the string literal to separate it for more readable.
//!
//! ## Examples
//!
//! ```rust
//! use numext_fixed_uint::{u128, U128};
//!
//! const U128_100: U128 = u128!("100");
//!
//! fn main () -> ::std::io::Result<()> {
//!     let x1 = u128!("0b110_0100");
//!     let x2 = u128!("0o144");
//!     let x3 = u128!("0x64");
//!     let y = U128::from(100u8);
//!     assert_eq!(x1, y);
//!     assert_eq!(x2, y);
//!     assert_eq!(x3, y);
//!     assert_eq!(U128_100, y);
//!     Ok(())
//! }
//! ```

use proc_macro_hack::proc_macro_hack;

pub use numext_fixed_uint_core::prelude;

macro_rules! reexport {
    ([$(($name:ident, $macro_name:ident),)+]) => {
        $(reexport!($name, $macro_name);)+
    };
    ([$(($name:ident, $macro_name:ident)),+]) => {
        $(reexport!($name, $macro_name);)+
    };
    ($name:ident, $macro_name:ident) =>    {
        pub use numext_fixed_uint_core::$name;
        /// A macro used to construct a fixed uint in compile time.
        #[proc_macro_hack]
        pub use numext_fixed_uint_hack::$macro_name;
    };
}

reexport!([
    (U128, u128),
    (U160, u160),
    (U224, u224),
    (U256, u256),
    (U384, u384),
    (U512, u512),
    (U520, u520),
    (U1024, u1024),
    (U2048, u2048),
    (U4096, u4096),
]);
