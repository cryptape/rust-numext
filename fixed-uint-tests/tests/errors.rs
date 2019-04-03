// Copyright 2018-2019 Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use nfuint::{FixedUintError, FromSliceError, FromStrError, IntoSliceError};
use nfuint::{U128, U256};

#[test]
fn errors() {
    {
        let input = [0u8; 17];
        let err = U128::from_little_endian(&input);
        if let Err(FixedUintError::FromSlice(FromSliceError::InvalidLength(_))) = err {
        } else {
            panic!("this error should be `FromSliceError::InvalidLength`");
        }
    }
    {
        let mut input = [0u8; 17];
        let uint = U128::zero();
        let err = uint.into_little_endian(&mut input);
        if let Err(FixedUintError::IntoSlice(IntoSliceError::InvalidLength(_))) = err {
        } else {
            panic!("this error should be `IntoSliceError::InvalidLength`");
        }
    }
    {
        let err = U128::from_hex_str("z");
        if let Err(FixedUintError::FromStr(FromStrError::InvalidCharacter { .. })) = err {
        } else {
            panic!("this error should be `FromStrError::InvalidCharacter`");
        }
    }
    {
        let err = U128::from_hex_str("");
        if let Err(FixedUintError::FromStr(FromStrError::InvalidLength(_))) = err {
        } else {
            panic!("this error should be `FromStrError::InvalidLength`");
        }
    }
    {
        let uint = U256::one() << 128;
        let uint_str = format!("{}", uint);
        let err = U128::from_dec_str(&uint_str);
        if let Err(FixedUintError::FromStr(FromStrError::Overflow(_))) = err {
        } else {
            panic!("this error should be `FromStrError::Overflow`");
        }
    }
}
