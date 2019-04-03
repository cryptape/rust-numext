// Copyright 2018-2019 Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use nfhash::H128;
use nfhash::{FixedHashError, FromSliceError, FromStrError, IntoSliceError};
use std::str::FromStr;

#[test]
fn errors() {
    {
        let input = [0u8; 15];
        let err = H128::from_slice(&input);
        if let Err(FixedHashError::FromSlice(FromSliceError::InvalidLength(_))) = err {
        } else {
            panic!("this error should be `FromSliceError::InvalidLength`");
        }
    }
    {
        let mut input = [0u8; 17];
        let hash = H128::zero();
        let err = hash.into_slice(&mut input);
        if let Err(FixedHashError::IntoSlice(IntoSliceError::InvalidLength(_))) = err {
        } else {
            panic!("this error should be `IntoSliceError::InvalidLength`");
        }
    }
    {
        let err = H128::from_trimmed_hex_str("z");
        if let Err(FixedHashError::FromStr(FromStrError::InvalidCharacter { .. })) = err {
        } else {
            panic!("this error should be `FromStrError::InvalidCharacter`");
        }
    }
    {
        let err = H128::from_str("a");
        if let Err(FixedHashError::FromStr(FromStrError::InvalidLength(_))) = err {
        } else {
            panic!("this error should be `FromStrError::InvalidLength`");
        }
    }
}
