// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Implement built-in traits in [`::std::cmp`].
//!
//! [`::std::cmp`]: https://doc.rust-lang.org/std/cmp/index.html#traits

use fixed_uint::UintConstructor;
use utils;

impl UintConstructor {
    pub fn impl_traits_std_cmp(&self) {
        self.impl_traits_std_cmp_partialeq();
        self.impl_traits_std_cmp_eq();
        self.impl_traits_std_cmp_partialord();
        self.impl_traits_std_cmp_ord();
    }

    fn impl_traits_std_cmp_partialeq(&self) {
        let name = &self.ts.name;
        let lidx = utils::pure_uint_list_to_ts(0..self.info.unit_amount);
        let ridx = lidx.clone();
        let part = quote!(
            impl ::std::cmp::PartialEq for #name {
                #[inline]
                fn eq(&self, other: &Self) -> bool {
                    let lhs = self.inner();
                    let rhs = other.inner();
                    #(
                        if lhs[#lidx] != rhs[#ridx] {
                            return false;
                        }
                    )*
                    true
                }
            }
        );
        self.implt(part);
    }

    fn impl_traits_std_cmp_eq(&self) {
        let name = &self.ts.name;
        let part = quote!(
            impl ::std::cmp::Eq for #name {}
        );
        self.implt(part);
    }

    fn impl_traits_std_cmp_partialord(&self) {
        let name = &self.ts.name;
        let part = quote!(
            impl ::std::cmp::PartialOrd for #name {
                #[inline]
                fn partial_cmp(&self, other: &Self) -> Option<::std::cmp::Ordering> {
                    Some(self.cmp(other))
                }
            }
        );
        self.implt(part);
    }

    fn impl_traits_std_cmp_ord(&self) {
        let name = &self.ts.name;
        let idx = utils::pure_uint_list_to_ts((0..self.info.unit_amount).rev());
        let part = quote!(
            impl ::std::cmp::Ord for #name {
                #[inline]
                fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
                    let lhs = self.inner();
                    let rhs = other.inner();
                    #({
                        let idx = #idx;
                        if lhs[idx] != rhs[idx] {
                            return if lhs[idx] > rhs[idx] {
                                ::std::cmp::Ordering::Greater
                            } else {
                                ::std::cmp::Ordering::Less
                            };
                        }
                    })*
                    ::std::cmp::Ordering::Equal
                }
            }
        );
        self.implt(part);
    }
}
