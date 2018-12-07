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
    fn into_slice(ref bytes in any::<props::H256Bytes>()) {
        let expected = bytes.inner;
        let result = {
            let mut ret = [0u8; 32];
            let val: nfhash::H256 = bytes.into();
            val.into_slice(&mut ret).unwrap();
            ret
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn from_slice(ref bytes in any::<props::H256Bytes>()) {
        let slice = &bytes.inner[..];
        let expected: props::H256Bytes = {
            let ret: etypes::H256 = bytes.into();
            ret.into()
        };
        let result: props::H256Bytes = {
            let ret = nfhash::H256::from_slice(slice);
            ret.unwrap().into()
        };
        assert_eq!(expected, result);
    }
}
