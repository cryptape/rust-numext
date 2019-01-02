// Copyright 2018-2019 Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

macro_rules! convert_between {
    ($uint:ident, $hash:ident, $bytes_size:expr) => {
        impl<'a> From<&'a nfuint::$uint> for $hash {
            #[inline]
            fn from(u: &nfuint::$uint) -> Self {
                let mut ret = [0u8; $bytes_size];
                u.into_big_endian(&mut ret).unwrap_or_else(|e| {
                    panic!(
                        "failed to convert from {} to {}: {}",
                        stringify!($uint),
                        stringify!($hash),
                        e
                    )
                });
                ret.into()
            }
        }
        impl From<nfuint::$uint> for $hash {
            #[inline]
            fn from(u: nfuint::$uint) -> Self {
                (&u).into()
            }
        }
        impl<'a> From<&'a $hash> for nfuint::$uint {
            #[inline]
            fn from(h: &$hash) -> Self {
                nfuint::$uint::from_big_endian(h.as_bytes()).unwrap_or_else(|e| {
                    panic!(
                        "failed to convert from {} to {}: {}",
                        stringify!($hash),
                        stringify!($uint),
                        e
                    )
                })
            }
        }
        impl From<$hash> for nfuint::$uint {
            #[inline]
            fn from(h: $hash) -> Self {
                (&h).into()
            }
        }
    };
}
