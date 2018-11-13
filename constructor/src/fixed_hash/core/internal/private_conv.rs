// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Define private methods about convert.

use fixed_hash::HashConstructor;
use utils;

impl HashConstructor {
    pub fn defun_priv_conv(&self) {
        self.attach_error_for_conv_from_str();
        self.defun_priv_conv_from_hex_str();
    }

    fn attach_error_for_conv_from_str(&self) {
        let error_name = &self.ts.error_name;
        let part = quote!(
            /// Error for parse from string.
            #[derive(Debug, Fail)]
            pub enum FromStrError {
                #[fail(display = "invalid character code `{}` at {}", chr, idx)]
                InvalidCharacter { chr: u8, idx: usize },
                #[fail(display = "invalid length: {}", _0)]
                InvalidLength(usize),
            }

            impl From<FromStrError> for #error_name {
                fn from(err: FromStrError) -> #error_name {
                    #error_name::FromStr(err)
                }
            }
        );
        self.attach_common(part);
        let part = quote!(#[fail(display = "failed to parse from string {}", _0)]
        FromStr(
            #[fail(cause)]
            FromStrError
        ),);
        self.error(part);
    }

    fn defun_priv_conv_from_hex_str(&self) {
        let error_name = &self.ts.error_name;
        let char_amount_max = utils::pure_uint_to_ts(self.info.unit_amount * 2);
        let part_core = if self.info.expand {
            let loop_unit_amount = &utils::pure_uint_list_to_ts(0..self.info.unit_amount);
            quote!(
                let mut input_bytes = input.bytes();
                #({
                    let idx = #loop_unit_amount;
                    let chr = input_bytes.next().unwrap_or_else(|| unreachable!());
                    let hi = match chr {
                        b'a'...b'f' => chr - b'a' + 10,
                        b'A'...b'F' => chr - b'A' + 10,
                        b'0'...b'9' => chr - b'0',
                        _ => Err(FromStrError::InvalidCharacter { chr, idx: idx*2 })?,
                    };
                    let chr = input_bytes.next().unwrap_or_else(|| unreachable!());
                    let lo = match chr {
                        b'a'...b'f' => chr - b'a' + 10,
                        b'A'...b'F' => chr - b'A' + 10,
                        b'0'...b'9' => chr - b'0',
                        _ => Err(FromStrError::InvalidCharacter { chr, idx: idx*2+1 })?,
                    };
                    inner[idx] = (hi << 4) | lo;
                })*
            )
        } else {
            quote!(for (idx, chr) in input.bytes().enumerate() {
                let val = match chr {
                    b'a'...b'f' => chr - b'a' + 10,
                    b'A'...b'F' => chr - b'A' + 10,
                    b'0'...b'9' => chr - b'0',
                    _ => Err(FromStrError::InvalidCharacter { chr, idx })?,
                };
                if idx % 2 == 0 {
                    inner[idx / 2] |= val << 4;
                } else {
                    inner[idx / 2] |= val;
                }
            })
        };
        let part = quote!(
            /// Convert from a hexadecimal string.
            #[inline]
            fn _from_hex_str(input: &str) -> Result<Self, #error_name> {
                let len = input.len();
                if len != #char_amount_max {
                    Err(FromStrError::InvalidLength(len))?;
                }
                let mut ret = Self::zero();
                {
                    let inner = ret.mut_inner();
                    #part_core
                }
                Ok(ret)
            }
        );
        self.defun(part);
    }
}
