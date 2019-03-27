// Copyright 2018-2019 Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use proc_macro_hack::proc_macro_hack;

pub use numext_fixed_uint_core::*;

macro_rules! reexport {
    ([$($name:ident,)+]) => {
        $(reexport!($name);)+
    };
    ([$($name:ident),+]) => {
        $(reexport!($name);)+
    };
    ($name:ident) =>    {
        #[proc_macro_hack]
        pub use numext_fixed_uint_hack::$name;
    };
}

reexport!([u128, u160, u224, u256, u384, u512, u520, u1024, u2048, u4096]);
