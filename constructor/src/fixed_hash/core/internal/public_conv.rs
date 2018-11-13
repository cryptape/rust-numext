// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Define public methods about convert.

use fixed_hash::HashConstructor;
use utils;

impl HashConstructor {
    pub fn defun_pub_conv(&self) {
        self.defun_pub_conv_from_slice();
        self.defun_pub_conv_into_slice();
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
                    ret.as_bytes_mut().copy_from_slice(input);
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
}
