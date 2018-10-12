// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[derive(FixedUint)]
pub struct U128([u64; 2]);

#[derive(FixedUint)]
pub struct U160([u32; 5]);

#[derive(FixedUint)]
pub struct U224([u32; 7]);

#[derive(FixedUint)]
pub struct U256([u64; 4]);

#[derive(FixedUint)]
pub struct U384([u64; 6]);

#[derive(FixedUint)]
pub struct U512([u64; 8]);

#[derive(FixedUint)]
pub struct U1024([u64; 16]);

#[derive(FixedUint)]
pub struct U2048([u64; 32]);

#[derive(FixedUint)]
pub struct U4096([u64; 64]);
