// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Define public methods about convert.

use crate::fixed_hash::HashConstructor;
use crate::utils;
use quote::quote;

impl HashConstructor {
    pub fn defun_pub_conv(&self) {
        self.defun_pub_conv_from_slice();
        self.defun_pub_conv_into_slice();
        self.attach_error_for_conv_from_str();
        self.defun_pub_conv_from_hex_str();
    }

    fn attach_error_for_conv_slice(&self, conv_type: &str, type_explain: &str) {
        let error_name = &self.ts.error_name;
        let error_item = utils::ident_to_ts(format!("{}Slice", conv_type).as_ref());
        let inner_error_name = utils::ident_to_ts(format!("{}SliceError", conv_type).as_ref());
        let error_explain = format!("failed to convert {} slice since {{}}", type_explain);
        let part = quote!(
            /// Error for parse from slice.
            #[derive(Debug, Fail)]
            pub enum #inner_error_name {
                #[fail(display = "invalid length: {}", _0)]
                InvalidLength(usize),
            }

            impl From<#inner_error_name> for #error_name {
                fn from(err: #inner_error_name) -> #error_name {
                    #error_name::#error_item(err)
                }
            }
        );
        self.attach_common(part);
        let part = quote!(
            #[fail(display = #error_explain, _0)]
            #error_item(#[fail(cause)] #inner_error_name),
        );
        self.error(part);
    }

    fn defun_pub_conv_from_slice(&self) {
        self.attach_error_for_conv_slice("From", "from");
        let error_name = &self.ts.error_name;
        let bytes_size = &self.ts.unit_amount;
        let part = quote!(
            /// Convert from slice.
            #[inline]
            pub fn from_slice(input: &[u8]) -> Result<Self, #error_name> {
                if input.len() != #bytes_size {
                    Err(FromSliceError::InvalidLength(input.len()))?
                } else {
                    let mut ret = Self::zero();
                    ret.mut_inner()[..].copy_from_slice(input);
                    Ok(ret)
                }
            }
        );
        self.defun(part);
    }

    fn defun_pub_conv_into_slice(&self) {
        self.attach_error_for_conv_slice("Into", "into");
        let error_name = &self.ts.error_name;
        let bytes_size = &self.ts.unit_amount;
        let part = quote!(
            /// Convert into slice.
            #[inline]
            pub fn into_slice(&self, output: &mut [u8]) -> Result<(), #error_name> {
                if output.len() != #bytes_size {
                    Err(IntoSliceError::InvalidLength(output.len()))?
                } else {
                    let inner = self.inner();
                    output.copy_from_slice(&inner[..]);
                    Ok(())
                }
            }
        );
        self.defun(part);
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

    fn defun_pub_conv_from_hex_str(&self) {
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
            /// Convert from a fixed length hexadecimal string.
            #[inline]
            pub fn from_hex_str(input: &str) -> Result<Self, #error_name> {
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
            /// Convert from an arbitrary length zero-trimmed hexadecimal string.
            /// Fisrt char should not be zero if the input has more than one char.
            #[inline]
            pub fn from_trimmed_hex_str(input: &str) -> Result<Self, #error_name> {
                let len = input.len();
                if len == 0 || len > #char_amount_max {
                    Err(FromStrError::InvalidLength(len))?;
                } else if input.as_bytes()[0] == b'0' {
                    if len == 1 {
                        return Ok(Self::zero());
                    } else {
                        Err(FromStrError::InvalidCharacter { chr: b'0', idx: 0 })?;
                    }
                }
                let mut ret = Self::zero();
                let mut input_bytes = input.bytes();
                let mut idx = 0;
                let mut unit_idx = (#char_amount_max - len) / 2;
                let mut high = len % 2 == 0;
                {
                    let inner = ret.mut_inner();
                    for chr in input_bytes {
                        let v = match chr {
                            b'a'...b'f' => chr - b'a' + 10,
                            b'A'...b'F' => chr - b'A' + 10,
                            b'0'...b'9' => chr - b'0',
                            _ => Err(FromStrError::InvalidCharacter { chr, idx })?,
                        };
                        idx += 1;
                        if high {
                            inner[unit_idx] = v * 16;
                            high = false;
                        } else {
                            inner[unit_idx] += v;
                            high = true;
                            unit_idx += 1;
                        }
                    }
                }
                Ok(ret)
            }
        );
        self.defun(part);
    }
}
