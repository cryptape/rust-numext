// Copyright 2018-2019 Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use proc_macro_hack::proc_macro_hack;

pub use numext_fixed_hash_core::*;

macro_rules! reexport {
    ([$($name:ident,)+]) => {
        $(reexport!($name);)+
    };
    ([$($name:ident),+]) => {
        $(reexport!($name);)+
    };
    ($name:ident) =>    {
        #[proc_macro_hack]
        pub use numext_fixed_hash_hack::$name;
    };
}

reexport!([h128, h160, h224, h256, h384, h512, h520, h1024, h2048, h4096]);
