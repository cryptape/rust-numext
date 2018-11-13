// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Implement built-in traits in [`::std::fmt`].
//!
//! [`::std::fmt`]: https://doc.rust-lang.org/std/fmt/index.html#traits

use fixed_hash::HashConstructor;
use utils;

impl HashConstructor {
    pub fn impl_traits_std_fmt(&self) {
        self.impl_traits_std_fmt_debug();
        self.impl_traits_std_fmt_lowerhex();
        self.impl_traits_std_fmt_upperhex();
        self.impl_traits_std_fmt_display();
    }

    pub fn impl_traits_std_fmt_debug(&self) {
        let name = &self.ts.name;
        let loop_unit_amount = &utils::pure_uint_list_to_ts(0..self.info.unit_amount);
        let loop_unit_amount_skip_first = &utils::pure_uint_list_to_ts(1..self.info.unit_amount);
        let part = quote!(
            impl ::std::fmt::Debug for #name {
                #[inline]
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    let data = self.inner();
                    let alternate = f.alternate();
                    write!(f, stringify!(#name))?;
                    write!(f, " ( [")?;
                    if alternate {
                        writeln!(f)?;
                        #(
                            writeln!(f, "    {:#x},", data[#loop_unit_amount])?;
                        )*
                        writeln!(f, "]")
                    } else {
                        write!(f, " {:#x}", data[0])?;
                        #(
                            write!(f, ", {:#x}", data[#loop_unit_amount_skip_first])?;
                        )*
                        write!(f, " ] )")
                    }
                }
            }
        );
        self.implt(part);
    }

    fn impl_traits_std_fmt_base_16(&self, trait_name: &str, prefix_char: char, format_char: char) {
        let name = &self.ts.name;
        let trait_name = utils::ident_to_ts(trait_name);
        let prefix = format!("0{}", prefix_char);
        let write_tpl_padded = format!("{{:02{}}}", format_char);
        let loop_unit_amount = &utils::pure_uint_list_to_ts(0..self.info.unit_amount);
        let loop_write_tpl_padded =
            &vec![write_tpl_padded.as_str(); self.info.unit_amount as usize - 1];
        let part_core = if self.info.expand {
            quote!(#(write!(f, #loop_write_tpl_padded, data[#loop_unit_amount])?;)*)
        } else {
            quote!(for x in data.iter() {
                write!(f, #write_tpl_padded, x)?;
            })
        };
        let part = quote!(
            impl ::std::fmt::#trait_name for #name {
                #[inline]
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    let data = self.inner();
                    if f.alternate() {
                        write!(f, #prefix)?;
                    }
                    #part_core
                    Ok(())
                }
            }
        );
        self.implt(part);
    }

    pub fn impl_traits_std_fmt_lowerhex(&self) {
        self.impl_traits_std_fmt_base_16("LowerHex", 'x', 'x');
    }

    pub fn impl_traits_std_fmt_upperhex(&self) {
        self.impl_traits_std_fmt_base_16("UpperHex", 'x', 'X');
    }

    pub fn impl_traits_std_fmt_display(&self) {
        self.impl_traits_std_fmt_base_16("Display", 'x', 'x');
    }
}
