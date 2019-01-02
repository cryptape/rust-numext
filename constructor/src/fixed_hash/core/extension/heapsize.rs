// Copyright 2018-2019 Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Use [`heapsize`] for measuring heap allocations in Rust programs.

//! [`heapsize`]: https://crates.io/crates/heapsize

use crate::fixed_hash::HashConstructor;
use quote::quote;

impl HashConstructor {
    pub fn with_heapsize(&self) {
        self.with_heapsize_defun_pub();
    }

    fn with_heapsize_defun_pub(&self) {
        let name = &self.ts.name;
        let part = quote!(
            #[cfg(feature = "support_heapsize")]
            impl heapsize::HeapSizeOf for #name {
                fn heap_size_of_children(&self) -> usize {
                    0
                }
            }
        );
        self.implt(part);
    }
}
