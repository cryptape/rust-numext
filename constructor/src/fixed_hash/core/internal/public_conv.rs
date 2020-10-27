// Copyright 2018-2019 Cryptape Technologies LLC.
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
        self.defun_pub_conv_from_hex_str_dict();
        self.defun_pub_conv_from_hex_str();
    }

    fn attach_error_for_conv_slice(&self, conv_type: &str, type_explain: &str) {
        let error_item = utils::ident_to_ts(format!("{}Slice", conv_type).as_ref());
        let inner_error_name = utils::ident_to_ts(format!("{}SliceError", conv_type).as_ref());
        let error_explain = format!("failed to convert {} slice since {{0}}", type_explain);
        let part = quote!(
            /// Error for parse from slice.
            #[derive(Debug, Error)]
            pub enum #inner_error_name {
                #[error("invalid length: {0}")]
                InvalidLength(usize),
            }
        );
        self.attach_common(part);
        let part = quote!(
            #[error(#error_explain)]
            #error_item(#[from] #inner_error_name),
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
                    Err(FromSliceError::InvalidLength(input.len()).into())
                } else {
                    let mut ret = Self::empty();
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
                    Err(IntoSliceError::InvalidLength(output.len()).into())
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
        let part = quote!(
            /// Error for parse from string.
            #[derive(Debug, Error)]
            pub enum FromStrError {
                #[error("invalid character code `{chr}` at {idx}")]
                InvalidCharacter { chr: u8, idx: usize },
                #[error("invalid length: {0}")]
                InvalidLength(usize),
            }
        );
        self.attach_common(part);
        #[rustfmt::skip]
        let part = quote!(
            #[error("failed to parse from string since {0}")]
            FromStr(#[from] FromStrError),
        );
        self.error(part);
    }

    fn defun_pub_conv_from_hex_str_dict(&self) {
        let part = quote!(
            pub(crate) const DICT_HEX_ERROR: u8 = u8::max_value();
            pub(crate) static DICT_HEX_LO: [u8; 256] = {
                const ____: u8 = DICT_HEX_ERROR;
                [
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, 0x00, 0x01, 0x02, 0x03,
                    0x04, 0x05, 0x06, 0x07, 0x08, 0x09, ____, ____, ____, ____, ____, ____, ____,
                    0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____,
                ]
            };
            pub(crate) static DICT_HEX_HI: [u8; 256] = {
                const ____: u8 = DICT_HEX_ERROR;
                [
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, 0x00, 0x10, 0x20, 0x30,
                    0x40, 0x50, 0x60, 0x70, 0x80, 0x90, ____, ____, ____, ____, ____, ____, ____,
                    0xa0, 0xb0, 0xc0, 0xd0, 0xe0, 0xf0, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, 0xa0, 0xb0, 0xc0, 0xd0, 0xe0, 0xf0, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____,
                ]
            };
        );
        self.util(part);
    }

    fn defun_pub_conv_from_hex_str(&self) {
        let error_name = &self.ts.error_name;
        let utils_name = &self.ts.utils_name;
        let char_amount_max = utils::pure_uint_to_ts(self.info.unit_amount * 2);
        let part_core = if self.info.expand {
            let loop_unit_amount = &utils::pure_uint_list_to_ts(0..self.info.unit_amount);
            let loop_utils_name_copy1 = &vec![utils_name; self.info.unit_amount as usize];
            let loop_utils_name_copy2 = &vec![utils_name; self.info.unit_amount as usize];
            let loop_utils_name_copy3 = &vec![utils_name; self.info.unit_amount as usize];
            let loop_utils_name_copy4 = &vec![utils_name; self.info.unit_amount as usize];
            quote!(
                let mut input_bytes = input.bytes();
                #({
                    let idx = #loop_unit_amount;
                    let hi = {
                        let chr = input_bytes.next().unwrap_or_else(|| unreachable!());
                        let hi = #loop_utils_name_copy1::DICT_HEX_HI[usize::from(chr)];
                        if hi == #loop_utils_name_copy2::DICT_HEX_ERROR {
                            return Err(FromStrError::InvalidCharacter { chr, idx: idx*2 }.into());
                        };
                        hi
                    };
                    let lo = {
                        let chr = input_bytes.next().unwrap_or_else(|| unreachable!());
                        let lo = #loop_utils_name_copy3::DICT_HEX_LO[usize::from(chr)];
                        if lo == #loop_utils_name_copy4::DICT_HEX_ERROR {
                            return Err(FromStrError::InvalidCharacter { chr, idx: idx*2+1 }.into());
                        };
                        lo
                    };
                    inner[idx] = hi | lo;
                })*
            )
        } else {
            quote!(for (idx, chr) in input.bytes().enumerate() {
                let val = if idx % 2 == 0 {
                    #utils_name::DICT_HEX_HI[usize::from(chr)]
                } else {
                    #utils_name::DICT_HEX_LO[usize::from(chr)]
                };
                if val == #utils_name::DICT_HEX_ERROR {
                    return Err(FromStrError::InvalidCharacter { chr, idx }.into());
                }
                inner[idx / 2] |= val;
            })
        };
        let part = quote!(
            /// Convert from a fixed length hexadecimal string.
            #[inline]
            pub fn from_hex_str(input: &str) -> Result<Self, #error_name> {
                let len = input.len();
                if len != #char_amount_max {
                    return Err(FromStrError::InvalidLength(len).into());
                }
                let mut ret = Self::empty();
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
                    return Err(FromStrError::InvalidLength(len).into());
                } else if input.as_bytes()[0] == b'0' {
                    if len == 1 {
                        return Ok(Self::empty());
                    } else {
                        return Err(FromStrError::InvalidCharacter { chr: b'0', idx: 0 }.into());
                    }
                }
                let mut ret = Self::empty();
                let mut input_bytes = input.bytes();
                let mut idx = 0;
                let mut unit_idx = (#char_amount_max - len) / 2;
                let mut high = len % 2 == 0;
                {
                    let inner = ret.mut_inner();
                    for chr in input_bytes {
                        let val = if high {
                            #utils_name::DICT_HEX_HI[usize::from(chr)]
                        } else {
                            #utils_name::DICT_HEX_LO[usize::from(chr)]
                        };
                        if val == #utils_name::DICT_HEX_ERROR {
                            return Err(FromStrError::InvalidCharacter { chr, idx }.into());
                        }
                        idx += 1;
                        inner[unit_idx] |= val;
                        if high {
                            high = false;
                        } else {
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
