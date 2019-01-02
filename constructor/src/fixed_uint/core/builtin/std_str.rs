// Copyright 2018-2019 Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Implement built-in traits in [`::std::str`].
//!
//! Not implement `FromStr` traits to reduce confusion.
//! Use `from_bin_str`, `from_oct_str`, `from_hex_str` or `from_dec_str` to instead of.
//!
//! [`::std::str`]: https://doc.rust-lang.org/std/str/index.html#traits
