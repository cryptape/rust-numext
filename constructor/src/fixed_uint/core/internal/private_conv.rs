// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Define private methods about convert.

use super::super::constructor::UintConstructor;
use utils;

impl UintConstructor {
    pub fn defun_priv_conv(&self) {
        self.defun_priv_from_primitive_uints();
    }

    fn defun_priv_from_primitive_uints(&self) {
        let unit_suffix = &self.ts.unit_suffix;
        for prim_bits_size in &[8u64, 16, 32, 64, 128] {
            let prim_type = utils::uint_suffix_to_ts(*prim_bits_size);
            let func_name = utils::ident_to_ts(format!("_from_u{}", prim_bits_size).as_ref());
            let part = match *prim_bits_size {
                x if x <= self.info.unit_bits_size => quote!(
                    #[inline]
                    fn #func_name(input: #prim_type) -> Self {
                        let mut ret = Self::zero();
                        ret.mut_inner()[0] = input as #unit_suffix;
                        ret
                    }
                ),
                _ => {
                    if prim_bits_size % self.info.unit_bits_size != 0 {
                        unreachable!();
                    }
                    let times = prim_bits_size / self.info.unit_bits_size;
                    let loop_times = &utils::pure_uint_list_to_ts(0..times);
                    let loop_shift = &utils::pure_uint_list_to_ts(
                        (0..times).map(|x| x * self.info.unit_bits_size),
                    );
                    let loop_unit_suffix = &vec![unit_suffix; times as usize];
                    quote!(
                        #[inline]
                        fn #func_name(input: #prim_type) -> Self {
                            let mut ret = Self::zero();
                            #({
                                ret.mut_inner()[#loop_times] = (input >> #loop_shift) as #loop_unit_suffix;
                            })*
                            ret
                        }
                    )
                }
            };
            self.defun(part);
        }
    }
}
