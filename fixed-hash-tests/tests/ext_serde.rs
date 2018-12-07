// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use nfhash_tests::props;
use proptest::{prelude::any, proptest, proptest_helper};

proptest! {
    #[test]
    fn with_serde_defun_public(ref le in any::<props::H256Bytes>()) {
        let val: nfhash::H256 = le.into();
        let json = serde_json::to_string(&val);
        assert!(json.is_ok());
        let json = json.unwrap();
        let y = serde_json::from_str(&json);
        assert!(y.is_ok());
        assert_eq!(val, y.unwrap());
    }
}
