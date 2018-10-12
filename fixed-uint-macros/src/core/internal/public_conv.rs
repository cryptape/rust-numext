// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Define public methods about convert.

use core::constructor::UintConstructor;
use core::utils;

impl UintConstructor {
    pub fn defun_pub_conv(&self) {
        self.defun_pub_conv_from_bytes();
        self.defun_pub_conv_into_bytes();
        self.attach_error_for_conv_from_str();
        self.defun_pub_conv_from_bin_str();
        self.defun_pub_conv_from_oct_str();
        self.defun_pub_conv_from_hex_str();
        self.defun_pub_conv_from_dec_str();
    }

    fn attach_error_for_conv_bytes(&self, conv_type: &str, type_explain: &str) {
        let error_name = &self.ts.error_name;
        let error_item = utils::ident_to_ts(format!("{}Bytes", conv_type).as_ref());
        let inner_error_name = utils::ident_to_ts(format!("{}BytesError", conv_type).as_ref());
        let error_explain = format!("failed to convert {} bytes since {{}}", type_explain);
        let part = quote!(
            /// Error for parse from bytes.
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

    fn defun_pub_conv_from_bytes(&self) {
        self.attach_error_for_conv_bytes("From", "from");
        let error_name = &self.ts.error_name;
        let inner_type = &self.ts.inner_type;
        let bytes_size = &self.ts.bytes_size;
        let unit_amount = &self.ts.unit_amount;
        let part = quote!(
            #[inline]
            fn _from_bytes(input: &[u8]) -> Self {
                let mut ret: #inner_type = [0; #unit_amount];
                unsafe {
                    let bytes = &mut *(&mut ret as *mut #inner_type as *mut [u8; #bytes_size]);
                    bytes[0..input.len()].copy_from_slice(input);
                }
                Self::new(ret)
            }
            #[inline]
            fn _from_bytes_opposite_endian(input: &[u8]) -> Self {
                let mut ret: #inner_type = [0; #unit_amount];
                unsafe {
                    let bytes = &mut *(&mut ret as *mut #inner_type as *mut [u8; #bytes_size]);
                    let mut bytes_ptr = bytes.as_mut_ptr();
                    let mut input_ptr = input.as_ptr().offset(input.len() as isize - 1);
                    for _ in 0..input.len() {
                        *bytes_ptr = *input_ptr;
                        bytes_ptr = bytes_ptr.offset(1);
                        input_ptr = input_ptr.offset(-1);
                    }
                }
                Self::new(ret)
            }
            /// Convert from little-endian bytes.
            #[inline]
            pub fn from_little_endian(input: &[u8]) -> Result<Self, #error_name> {
                if input.len() > #bytes_size {
                    Err(FromBytesError::InvalidLength(input.len()))?
                } else if cfg!(target_endian = "little") {
                    Ok(Self::_from_bytes(input))
                } else {
                    Ok(Self::_from_bytes_opposite_endian(input))
                }
            }
            /// Convert from big-endian bytes.
            #[inline]
            pub fn from_big_endian(input: &[u8]) -> Result<Self, #error_name> {
                if input.len() > #bytes_size {
                    Err(FromBytesError::InvalidLength(input.len()))?
                } else if cfg!(target_endian = "big") {
                    Ok(Self::_from_bytes(input))
                } else {
                    Ok(Self::_from_bytes_opposite_endian(input))
                }
            }
        );
        self.defun(part);
    }

    fn defun_pub_conv_into_bytes(&self) {
        self.attach_error_for_conv_bytes("Into", "into");
        let error_name = &self.ts.error_name;
        let inner_type = &self.ts.inner_type;
        let bytes_size = &self.ts.bytes_size;
        let loop_bytes_size = &utils::pure_uint_list_to_ts(0..self.info.bytes_size);
        let part = quote!(
            #[inline]
            fn _into_bytes(&self, output: &mut [u8]) {
                let inner = self.inner();
                unsafe {
                    let bytes = &*(inner as *const #inner_type as *const  [u8; #bytes_size]);
                    output[0..#bytes_size].copy_from_slice(bytes);
                }
            }
            #[inline]
            fn _into_bytes_opposite_endian(&self, output: &mut [u8]) {
                let inner = self.inner();
                unsafe {
                    let bytes = &*(inner as *const #inner_type as *const  [u8; #bytes_size]);
                    let mut bytes_ptr = bytes.as_ptr().offset(#bytes_size - 1);
                    let mut output_ptr = output.as_mut_ptr();
                    #({
                        let _ = #loop_bytes_size;
                        *output_ptr = *bytes_ptr;
                        bytes_ptr = bytes_ptr.offset(-1);
                        output_ptr = output_ptr.offset(1);
                    })*
                }
            }
            /// Convert into little-endian bytes.
            #[inline]
            pub fn into_little_endian(&self, output: &mut [u8]) -> Result<(), #error_name> {
                if output.len() != #bytes_size {
                    Err(IntoBytesError::InvalidLength(output.len()))?
                } else if cfg!(target_endian = "little") {
                    self._into_bytes(output);
                    Ok(())
                } else {
                    self._into_bytes_opposite_endian(output);
                    Ok(())
                }
            }
            /// Convert into big-endian bytes.
            #[inline]
            pub fn into_big_endian(&self, output: &mut [u8]) -> Result<(), #error_name> {
                if output.len() != #bytes_size {
                    Err(IntoBytesError::InvalidLength(output.len()))?
                } else if cfg!(target_endian = "big") {
                    self._into_bytes(output);
                    Ok(())
                } else {
                    self._into_bytes_opposite_endian(output);
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
                #[fail(display = "number is too big (length is {})", _0)]
                Overflow(usize),
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

    fn defun_pub_conv_from_bin_str(&self) {
        let error_name = &self.ts.error_name;
        let bits_size = &self.ts.bits_size;
        let unit_bits_size = &self.ts.unit_bits_size;
        let unit_amount = &self.ts.unit_amount;
        let inner_type = &self.ts.inner_type;
        let loop_unit_bits_size = &utils::pure_uint_list_to_ts(0..self.info.unit_bits_size);
        let part = quote!(
            /// Convert from a binary string.
            #[inline]
            pub fn from_bin_str(input: &str) -> Result<Self, #error_name> {
                let len = input.len();
                if len == 0 || len > #bits_size {
                    Err(FromStrError::InvalidLength(len))?;
                } else if len != 1 && input.as_bytes()[0] == b'0' {
                    Err(FromStrError::InvalidCharacter { chr: b'0', idx: 0 })?;
                }
                let mut src = input.bytes().enumerate();
                let mut ret: #inner_type = [0; #unit_amount];
                let unit_cnt = len / #unit_bits_size;
                let chars_more = len % #unit_bits_size;
                for i in 0..chars_more {
                    let (idx, chr) = src.next().unwrap();
                    ret[unit_cnt] <<= 1;
                    match chr {
                        b'0' => {},
                        b'1' => ret[unit_cnt] |= 1,
                        _ => Err(FromStrError::InvalidCharacter { chr, idx })?,
                    }
                }
                for i in (0..unit_cnt).rev() {
                    #({
                        let _ = #loop_unit_bits_size;
                        let (idx, chr) = src.next().unwrap();
                        ret[i] <<= 1;
                        match chr {
                            b'0' => {},
                            b'1' => ret[i] |= 1,
                            _ => Err(FromStrError::InvalidCharacter { chr, idx })?,
                        }
                    })*
                }
                Ok(Self::new(ret))
            }
        );
        self.defun(part);
    }

    fn defun_pub_conv_from_oct_str(&self) {
        let name = &self.ts.name;
        let error_name = &self.ts.error_name;
        let char_amount_max = utils::pure_uint_to_ts(if self.info.bits_size % 3 == 0 {
            self.info.bits_size / 3
        } else {
            (f64::from(self.info.bits_size as u32) / 3f64).ceil() as u64
        });
        let part = quote!(
            /// Convert from a octal string.
            #[inline]
            pub fn from_oct_str(input: &str) -> Result<Self, #error_name> {
                let len = input.len();
                if len == 0 || len > #char_amount_max {
                    Err(FromStrError::InvalidLength(len))?;
                } else if len != 1 && input.as_bytes()[0] == b'0' {
                    Err(FromStrError::InvalidCharacter { chr: b'0', idx: 0 })?;
                }
                let mut ret = Self::zero();
                for (idx, chr) in input.bytes().enumerate() {
                    if chr < b'0' && chr > b'7' {
                        Err(FromStrError::InvalidCharacter { chr, idx })?;
                    }
                    let (ret_new, of) = ret._mul_unit(8);
                    if of {
                        Err(FromStrError::Overflow(len))?;
                    }
                    let u = #name::from(chr - b'0');
                    let (ret_new, of) = ret_new._add(&u);
                    if of {
                        Err(FromStrError::Overflow(len))?;
                    }
                    ret = ret_new;
                }
                Ok(ret)
            }
        );
        self.defun(part);
    }

    fn defun_pub_conv_from_hex_str(&self) {
        let name = &self.ts.name;
        let error_name = &self.ts.error_name;
        let char_amount_max = utils::pure_uint_to_ts(if self.info.bits_size % 4 == 0 {
            self.info.bits_size / 4
        } else {
            (f64::from(self.info.bits_size as u32) / 4f64).ceil() as u64
        });
        let part = quote!(
            /// Convert from a hexadecimal string.
            #[inline]
            pub fn from_hex_str(input: &str) -> Result<Self, #error_name> {
                let len = input.len();
                if len == 0 || len > #char_amount_max {
                    Err(FromStrError::InvalidLength(len))?;
                } else if len != 1 && input.as_bytes()[0] == b'0' {
                    Err(FromStrError::InvalidCharacter { chr: b'0', idx: 0 })?;
                }
                let mut ret = Self::zero();
                for (idx, chr) in input.bytes().enumerate() {
                    let (ret_new, of) = ret._mul_unit(16);
                    if of {
                        Err(FromStrError::Overflow(len))?;
                    }
                    let v = match chr {
                        b'a'...b'f' => chr - b'a' + 10,
                        b'A'...b'F' => chr - b'A' + 10,
                        b'0'...b'9' => chr - b'0',
                        _ => Err(FromStrError::InvalidCharacter { chr, idx })?,
                    };
                    let u = #name::from(v);
                    let (ret_new, of) = ret_new._add(&u);
                    if of {
                        Err(FromStrError::Overflow(len))?;
                    }
                    ret = ret_new;
                }
                Ok(ret)
            }
        );
        self.defun(part);
    }

    fn defun_pub_conv_from_dec_str(&self) {
        let name = &self.ts.name;
        let error_name = &self.ts.error_name;
        let char_amount_max = utils::pure_uint_to_ts(if self.info.bits_size % 10 == 0 {
            self.info.bits_size / 10
        } else {
            (f64::from(self.info.bits_size as u32) / 10f64.log2()).ceil() as u64
        });
        let part = quote!(
            /// Convert from a decimal string.
            #[inline]
            pub fn from_dec_str(input: &str) -> Result<Self, #error_name> {
                let len = input.len();
                if len == 0 || len > #char_amount_max {
                    Err(FromStrError::InvalidLength(len))?;
                } else if len != 1 && input.as_bytes()[0] == b'0' {
                    Err(FromStrError::InvalidCharacter { chr: b'0', idx: 0 })?;
                }
                let mut ret = Self::zero();
                for (idx, chr) in input.bytes().enumerate() {
                    if chr < b'0' && chr > b'9' {
                        Err(FromStrError::InvalidCharacter { chr, idx })?;
                    }
                    let (ret_new, of) = ret._mul_unit(10);
                    if of {
                        Err(FromStrError::Overflow(len))?;
                    }
                    let u = #name::from(chr - b'0');
                    let (ret_new, of) = ret_new._add(&u);
                    if of {
                        Err(FromStrError::Overflow(len))?;
                    }
                    ret = ret_new;
                }
                Ok(ret)
            }
        );
        self.defun(part);
    }
}
