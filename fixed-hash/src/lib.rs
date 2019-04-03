// Copyright 2018-2019 Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A series of fixed hash types.
//!
//! # Constructors
//!
//! This crate provides a series of macros that used to construct fixed hashes in compile time.
//!
//! The input is a hexadecimal string literal with `0x` prefix.
//! Completed strings or trimmed strings both are allowed.
//!
//! And you can use any number of `_` in the string literal to separate it for more readable.
//!
//! ## Examples
//!
//! ```rust
//! use numext_fixed_hash::{h128, H128};
//!
//! const H128_VAL: H128 = h128!("0x123456789abcdef");
//!
//! fn main () -> ::std::io::Result<()> {
//!     let x1 = h128!("0x123456789abcdef");
//!     let x2 = h128!("0x00000000000000000123456789abcdef");
//!     let y = H128::from_trimmed_hex_str("123456789abcdef").unwrap();
//!     assert_eq!(x1, y);
//!     assert_eq!(x2, y);
//!     assert_eq!(H128_VAL, y);
//!     Ok(())
//! }
//! ```

use proc_macro_hack::proc_macro_hack;

pub use numext_fixed_hash_core::prelude;
pub use numext_fixed_hash_core::{FixedHashError, FromSliceError, FromStrError, IntoSliceError};

macro_rules! reexport {
    ([$(($name:ident, $macro_name:ident),)+]) => {
        $(reexport!($name, $macro_name);)+
    };
    ([$(($name:ident, $macro_name:ident)),+]) => {
        $(reexport!($name, $macro_name);)+
    };
    ($name:ident, $macro_name:ident) =>    {
        pub use numext_fixed_hash_core::$name;
        /// A macro used to construct a fixed hash in compile time.
        #[proc_macro_hack]
        pub use numext_fixed_hash_hack::$macro_name;
    };
}

reexport!([
    (H128, h128),
    (H160, h160),
    (H224, h224),
    (H256, h256),
    (H384, h384),
    (H512, h512),
    (H520, h520),
    (H1024, h1024),
    (H2048, h2048),
    (H4096, h4096),
]);
