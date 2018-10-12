// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate rustc_version;

use rustc_version::{version, version_meta, Channel, Version};

fn main() {
    let is_nightly = version_meta()
        .expect("Could not figure it out whether the rustc is nightly or not")
        .channel
        == Channel::Nightly;
    let ge_minver = version().expect("Could not figure it out which version of rustc was used.")
        >= Version::parse("1.30.0-alpha").unwrap();

    // some features about proc-macro was not supported in stable or beta before v1.30
    if !(is_nightly || ge_minver) {
        println!("cargo:rustc-cfg=feature=\"use-derive\"");
    }
}
