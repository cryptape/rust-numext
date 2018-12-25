// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use nfuint_tests::props;
use proptest::{prelude::any, proptest, proptest_helper};

proptest! {
    #[test]
    fn with_serde_defun_public(ref le in any::<props::U256LeBytes>()) {
        let val: nfuint::U256 = le.into();
        let json = serde_json::to_string(&val);
        assert!(json.is_ok());
        let json = json.unwrap();
        let y = serde_json::from_str(&json);
        assert!(y.is_ok());
        assert_eq!(val, y.unwrap());
    }

    #[test]
    fn deserialize(ref json in "\"0x([[:xdigit:]&&[^0]][[:xdigit:]]{0,63}|0)\"") {
        let result = serde_json::from_str::<nfuint::U256>(&json);
        assert!(result.is_ok());
    }
}

macro_rules! check_de_error {
    ($name:ident, $uint:ident, $regex:expr, $msg_start:expr) => {
        proptest! {
            #[test]
            fn $name(ref json in $regex) {
                let result = serde_json::from_str::<nfuint::$uint>(&json);
                assert!(result.is_err());
                let errmsg = result.unwrap_err().to_string();
                assert!(errmsg.contains("with at most 64 digits"));
                assert!(errmsg.starts_with($msg_start));
            }
        }
    };
}

check_de_error!(
    deserialize_without_or_only_0x_prefix,
    U256,
    "\"([[:xdigit]:&&[^0]][[:xdigit:]]{0,63}|0x)\"",
    "invalid format"
);

check_de_error!(
    deserialize_with_redundant_zeros,
    U256,
    "\"0x0[[:xdigit:]]{1,63}\"",
    "invalid format"
);

check_de_error!(
    deserialize_with_invalid_hex_bytes,
    U256,
    "\"0x[[:alnum:]&&[:^xdigit:]]\"",
    "invalid hex bytes"
);

check_de_error!(
    deserialize_more_hex_bytes,
    U256,
    "\"0x[[:xdigit:]&&[^0]][[:xdigit:]]{64,}\"",
    "invalid length"
);
