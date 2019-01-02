// Copyright 2018-2019 Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate proptest;

use nfhash_tests::props;
use nfhash_tests::tools;
use proptest::prelude::any;

macro_rules! check_fmt {
    ($fmt_str:expr, $val:ident, $expected:expr) => {
        assert_eq!(format!($fmt_str, $val), $expected);
    };
    ($fmt_str:expr, $val:expr, $expected:expr) => {
        assert_eq!(format!($fmt_str, $val), $expected);
    };
}

macro_rules! check_all_fmt {
    ($hash:ident, $width:expr, $short:expr) => {{
        let val_str = tools::padding_str($short, $width);
        let val = nfhash::$hash::from_hex_str(val_str.as_str()).unwrap();
        check_fmt!("{:x}", val, val_str.to_lowercase());
        check_fmt!("{:X}", val, val_str.to_uppercase());
        check_fmt!("{:#x}", val, format!("0x{}", val_str.to_lowercase()));
        check_fmt!("{:#X}", val, format!("0x{}", val_str.to_uppercase()));
    }};
}

macro_rules! std_fmt {
    ($name:ident, $hash:ident, $width:expr, $regex:expr) => {
        proptest! {
            #[test]
            fn $name(ref s in $regex) {
                check_all_fmt!($hash, $width, s);
            }
        }
    };
}

proptest! {
    #[test]
    fn h256_shrink(ref bytes in any::<props::H256Bytes>()) {
        let expected = {
            let val: etypes::H256 = bytes.into();
            format!("{:x}", val)
        };
        let result_lower = {
            let val: nfhash::H256 = bytes.into();
            format!("{:x}", val)
        };
        let result_upper = {
            let val: nfhash::H256 = bytes.into();
            format!("{:X}", val)
        };
        assert_eq!(expected, result_lower);
        assert_eq!(expected.to_uppercase(), result_upper);
    }
}

std_fmt!(h256_random, H256, 32, "[[:xdigit:]]{64}");
std_fmt!(h4096_random, H4096, 512, "[[:xdigit:]]{1024}");

#[test]
fn debug() {
    check_fmt!(
        "{:?}",
        nfhash::H128::min_value(),
        "H128 ( [ \
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, \
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 \
         ] )"
    );
    check_fmt!(
        "{:?}",
        nfhash::H128::max_value(),
        "H128 ( [ \
         0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, \
         0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff \
         ] )"
    );
}

#[test]
fn display() {
    check_fmt!(
        "{}",
        nfhash::H128::min_value(),
        "00000000000000000000000000000000"
    );
    check_fmt!(
        "{}",
        nfhash::H128::max_value(),
        "ffffffffffffffffffffffffffffffff"
    );
    check_fmt!(
        "{}",
        nfhash::H4096::min_value(),
        "000000000000..(omit 1000)..000000000000"
    );
    check_fmt!(
        "{}",
        nfhash::H4096::max_value(),
        "ffffffffffff..(omit 1000)..ffffffffffff"
    );
}
