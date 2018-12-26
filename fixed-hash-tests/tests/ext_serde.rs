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
    fn deserialize(ref json in "\"0x[[:xdigit:]]{64}\"") {
        let result = serde_json::from_str::<nfhash::H256>(&json);
        assert!(result.is_ok());
    }
}

macro_rules! check_de_error {
    ($name:ident, $hash:ident, $regex:expr, $msg_start:expr) => {
        proptest! {
            #[test]
            fn $name(ref json in $regex) {
                let result = serde_json::from_str::<nfhash::$hash>(&json);
                assert!(result.is_err());
                let errmsg = result.unwrap_err().to_string();
                assert!(errmsg.contains("with 64 digits"));
                assert!(errmsg.starts_with($msg_start));
            }
        }
    };
}

check_de_error!(
    deserialize_without_or_only_0x_prefix,
    H256,
    "\"([[:xdigit:]]{64}|0x)\"",
    "invalid format"
);

check_de_error!(
    deserialize_with_invalid_hex_bytes,
    H256,
    "\"0x[[:alnum:]&&[:^xdigit:]][[:xdigit:]]{63}\"",
    "invalid hex bytes"
);

check_de_error!(
    deserialize_less_hex_bytes,
    H256,
    "\"0x[[:xdigit:]]{1,63}\"",
    "invalid length"
);

check_de_error!(
    deserialize_more_hex_bytes,
    H256,
    "\"0x[[:xdigit:]]{65,}\"",
    "invalid length"
);
