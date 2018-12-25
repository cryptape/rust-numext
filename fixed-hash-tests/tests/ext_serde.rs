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

    #[test]
    fn without_0x_prefix(ref json in "\"[0-9a-fA-F]{64}\"") {
        let y = serde_json::from_str::<nfhash::H256>(&json);
        assert!(y.is_err());
        assert!(format!("{:?}", y.err().unwrap()).contains("with 64 digits"));
    }
}

#[test]
fn deserialize_error_message() {
    let json = "\"0123456789ABCDEF0123456789ABCDEF\"";
    let y: Result<nfhash::H128, serde_json::Error> = serde_json::from_str(&json);
    assert!(y.unwrap_err().to_string().starts_with("invalid format"));

    let json = "\"0x0123456789ABCDEF\"";
    let y: Result<nfhash::H128, serde_json::Error> = serde_json::from_str(&json);
    assert!(y.unwrap_err().to_string().starts_with("invalid length"));

    let json = "\"0x0123456789ABCDEF0123456789ABCDEG\"";
    let y: Result<nfhash::H128, serde_json::Error> = serde_json::from_str(&json);
    assert!(y.unwrap_err().to_string().starts_with("invalid hex bytes"));
}
