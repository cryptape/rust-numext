// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Implement built-in traits in [`::std::ops`].
//!
//! Not implement `Deref` and `DerefMut` traits to reduce confusion.
//!
//! [`::std::ops`]: https://doc.rust-lang.org/std/ops/index.html#traits

use super::super::constructor::UintConstructor;
use super::super::utils;

impl UintConstructor {
    pub fn impl_traits_std_ops(&self) {
        self.impl_traits_std_ops_arith("Add", "add", "_add");
        self.impl_traits_std_ops_arith("Sub", "sub", "_sub");
        self.impl_traits_std_ops_arith("Mul", "mul", "_mul");
        self.impl_traits_std_ops_arith("Div", "div", "_div");
        self.impl_traits_std_ops_arith("Rem", "rem", "_rem");
        self.impl_traits_std_ops_bitwise("BitAnd", "bitand", "_bitand");
        self.impl_traits_std_ops_bitwise("BitOr", "bitor", "_bitor");
        self.impl_traits_std_ops_bitwise("BitXor", "bitxor", "_bitxor");
        self.impl_traits_std_ops_not();
        self.impl_traits_std_ops_shift('l');
        self.impl_traits_std_ops_shift('r');
    }

    // Apply a template to implement some arithmetic traits.
    fn impl_traits_std_ops_arith(&self, trait_name_str: &str, func_name: &str, realfunc: &str) {
        let name = &self.ts.name;
        let trait_name = utils::ident_to_ts(trait_name_str);
        let func_name = utils::ident_to_ts(func_name);
        let real_func = utils::ident_to_ts(realfunc);
        let trait_assign_name = utils::ident_to_ts(format!("{}Assign", trait_name).as_ref());
        let func_assign_name = utils::ident_to_ts(format!("{}_assign", func_name).as_ref());
        let panic_stmt = match trait_name_str {
            "Add" => quote!(panic!("{}: attempt to add with overflow", stringify!(#name));),
            "Sub" => quote!(panic!("{}: attempt to subtract with overflow", stringify!(#name));),
            "Mul" => quote!(panic!("{}: attempt to multiply with overflow", stringify!(#name));),
            "Div" => quote!(panic!("{}: attempt to divide by zero", stringify!(#name));),
            "Rem" => {
                quote!(panic!("{}: attempt to calculate the remainder with a divisor of zero", stringify!(#name));)
            }
            _ => unreachable!(),
        };
        let part = quote!(
            impl<'a, Rhs> ::std::ops::#trait_name<Rhs> for &'a #name
            where
                Rhs: ::std::convert::Into<#name>
            {
                type Output = #name;
                #[inline]
                fn #func_name(self, other: Rhs) -> Self::Output {
                    let (ret, of) = self.#real_func(&other.into());
                    if of { #panic_stmt }
                    ret
                }
            }
            impl<Rhs> ::std::ops::#trait_name<Rhs> for #name
            where
                Rhs: ::std::convert::Into<#name>
            {
                type Output = #name;
                #[inline]
                fn #func_name(self, other: Rhs) -> Self::Output {
                    let (ret, of) = self.#real_func(&other.into());
                    if of { #panic_stmt }
                    ret
                }
            }
            impl<Rhs> ::std::ops::#trait_assign_name<Rhs> for #name
            where
                Rhs: ::std::convert::Into<#name>
            {
                #[inline]
                fn #func_assign_name(&mut self, other: Rhs) {
                    let (ret, of) = self.#real_func(&other.into());
                    if of { #panic_stmt }
                    *self = ret;
                }
            }
            impl<'a, 'b> ::std::ops::#trait_name<&'b #name> for &'a #name
            {
                type Output = #name;
                #[inline]
                fn #func_name(self, other: &#name) -> Self::Output {
                    let (ret, of) = self.#real_func(other);
                    if of { #panic_stmt }
                    ret
                }
            }
            impl<'a> ::std::ops::#trait_name<&'a #name> for #name
            {
                type Output = #name;
                #[inline]
                fn #func_name(self, other: &'a #name) -> Self::Output {
                    let (ret, of) = self.#real_func(other);
                    if of { #panic_stmt }
                    ret
                }
            }
            impl<'a> ::std::ops::#trait_assign_name<&'a #name> for #name
            {
                #[inline]
                fn #func_assign_name(&mut self, other: &#name) {
                    let (ret, of) = self.#real_func(other);
                    if of { #panic_stmt }
                    *self = ret;
                }
            }
        );
        self.implt(part);
    }

    // Apply a template to implement some bits operations traits.
    fn impl_traits_std_ops_bitwise(&self, trait_name: &str, func_name: &str, realfunc: &str) {
        let name = &self.ts.name;
        let trait_name = utils::ident_to_ts(trait_name);
        let func_name = utils::ident_to_ts(func_name);
        let real_func = utils::ident_to_ts(realfunc);
        let trait_assign_name = utils::ident_to_ts(format!("{}Assign", trait_name).as_ref());
        let func_assign_name = utils::ident_to_ts(format!("{}_assign", func_name).as_ref());
        let part = quote!(
            impl<'a, Rhs> ::std::ops::#trait_name<Rhs> for &'a #name
            where
                Rhs: ::std::convert::Into<#name>
            {
                type Output = #name;
                #[inline]
                fn #func_name(self, other: Rhs) -> Self::Output {
                    self.#real_func(&other.into())
                }
            }
            impl<Rhs> ::std::ops::#trait_name<Rhs> for #name
            where
                Rhs: ::std::convert::Into<#name>
            {
                type Output = #name;
                #[inline]
                fn #func_name(self, other: Rhs) -> Self::Output {
                    self.#real_func(&other.into())
                }
            }
            impl<Rhs> ::std::ops::#trait_assign_name<Rhs> for #name
            where
                Rhs: ::std::convert::Into<#name>
            {
                #[inline]
                fn #func_assign_name(&mut self, other: Rhs) {
                    *self = self.#real_func(&other.into());
                }
            }
            impl<'a, 'b> ::std::ops::#trait_name<&'b #name> for &'a #name
            {
                type Output = #name;
                #[inline]
                fn #func_name(self, other: &#name) -> Self::Output {
                    self.#real_func(other)
                }
            }
            impl<'a> ::std::ops::#trait_name<&'a #name> for #name
            {
                type Output = #name;
                #[inline]
                fn #func_name(self, other: &#name) -> Self::Output {
                    self.#real_func(other)
                }
            }
            impl<'a> ::std::ops::#trait_assign_name<&'a #name> for #name
            {
                #[inline]
                fn #func_assign_name(&mut self, other: &#name) {
                    *self = self.#real_func(other);
                }
            }
        );
        self.implt(part);
    }

    // Implement `Not` traits.
    fn impl_traits_std_ops_not(&self) {
        let name = &self.ts.name;
        let part = quote!(
            impl<'a> ::std::ops::Not for &'a #name {
                type Output = #name;
                #[inline]
                fn not(self) -> Self::Output {
                    self._not()
                }
            }
            impl ::std::ops::Not for #name {
                type Output = #name;
                #[inline]
                fn not(self) -> Self::Output {
                    self._not()
                }
            }
        );
        self.implt(part);
    }

    fn impl_traits_std_ops_shift(&self, direction: char) {
        let name = &self.ts.name;
        let trait_name = utils::ident_to_ts(format!("Sh{}", direction).as_ref());
        let func_name = utils::ident_to_ts(format!("sh{}", direction).as_ref());
        let trait_assign_name = utils::ident_to_ts(format!("Sh{}Assign", direction).as_ref());
        let func_assign_name = utils::ident_to_ts(format!("sh{}_assign", direction).as_ref());
        for uint_name in &["u8", "u16", "u32", "u64", "u128", "usize"] {
            let uint_name = utils::ident_to_ts(uint_name);
            let real_func = utils::ident_to_ts(format!("_ush{}", direction).as_ref());
            let part = quote!(
                impl<'a,'b> ::std::ops::#trait_name<&'a #uint_name> for &'b #name {
                    type Output = #name;
                    #[inline]
                    fn #func_name(self, other: &#uint_name) -> Self::Output {
                        self.#real_func(*other as u128)
                    }
                }
                impl<'a> ::std::ops::#trait_name<#uint_name> for &'a #name {
                    type Output = #name;
                    #[inline]
                    fn #func_name(self, other: #uint_name) -> Self::Output {
                        self.#real_func(other as u128)
                    }
                }
                impl<'a> ::std::ops::#trait_name<&'a #uint_name> for #name {
                    type Output = #name;
                    #[inline]
                    fn #func_name(self, other: &#uint_name) -> Self::Output {
                        self.#real_func(*other as u128)
                    }
                }
                impl ::std::ops::#trait_name<#uint_name> for #name {
                    type Output = #name;
                    #[inline]
                    fn #func_name(self, other: #uint_name) -> Self::Output {
                        self.#real_func(other as u128)
                    }
                }
                impl<'a> ::std::ops::#trait_assign_name<&'a #uint_name> for #name {
                    #[inline]
                    fn #func_assign_name(&mut self, other: &#uint_name) {
                        let ret = self.#real_func(*other as u128);
                        *self = ret;
                    }
                }
                impl ::std::ops::#trait_assign_name<#uint_name> for #name {
                    #[inline]
                    fn #func_assign_name(&mut self, other: #uint_name) {
                        let ret = self.#real_func(other as u128);
                        *self = ret;
                    }
                }
            );
            self.implt(part);
        }
        for int_name in &["i8", "i16", "i32", "i64", "i128", "isize"] {
            let int_name = utils::ident_to_ts(int_name);
            let real_func = utils::ident_to_ts(format!("_ish{}", direction).as_ref());
            let part = quote!(
                impl<'a,'b> ::std::ops::#trait_name<&'a #int_name> for &'b #name {
                    type Output = #name;
                    #[inline]
                    fn #func_name(self, other: &#int_name) -> Self::Output {
                        self.#real_func(*other as i128)
                    }
                }
                impl<'a> ::std::ops::#trait_name<#int_name> for &'a #name {
                    type Output = #name;
                    #[inline]
                    fn #func_name(self, other: #int_name) -> Self::Output {
                        self.#real_func(other as i128)
                    }
                }
                impl<'a> ::std::ops::#trait_name<&'a #int_name> for #name {
                    type Output = #name;
                    #[inline]
                    fn #func_name(self, other: &#int_name) -> Self::Output {
                        self.#real_func(*other as i128)
                    }
                }
                impl ::std::ops::#trait_name<#int_name> for #name {
                    type Output = #name;
                    #[inline]
                    fn #func_name(self, other: #int_name) -> Self::Output {
                        self.#real_func(other as i128)
                    }
                }
                impl<'a> ::std::ops::#trait_assign_name<&'a #int_name> for #name {
                    #[inline]
                    fn #func_assign_name(&mut self, other: &#int_name) {
                        let ret = self.#real_func(*other as i128);
                        *self = ret;
                    }
                }
                impl ::std::ops::#trait_assign_name<#int_name> for #name {
                    #[inline]
                    fn #func_assign_name(&mut self, other: #int_name) {
                        let ret = self.#real_func(other as i128);
                        *self = ret;
                    }
                }
            );
            self.implt(part);
        }
    }
}
