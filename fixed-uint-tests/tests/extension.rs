// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate numext_fixed_uint;
extern crate serde_json;

macro_rules! check_rand {
    ($name:ident, $uint:ident) => {
        #[test]
        fn $name() {
            let x = numext_fixed_uint::$uint::thread_random();
            let y = numext_fixed_uint::$uint::thread_random();
            // If this test is failed, please check if there is a bug or you are too luckly.
            assert!(!x.is_zero());
            assert!(!y.is_zero());
            assert!(x != y);
        }
    };
}

check_rand!(rand_u128, U128);
check_rand!(rand_u160, U160);
check_rand!(rand_u224, U224);
check_rand!(rand_u256, U256);
check_rand!(rand_u384, U384);
check_rand!(rand_u512, U512);
check_rand!(rand_u520, U520);
check_rand!(rand_u1024, U1024);
check_rand!(rand_u2048, U2048);
check_rand!(rand_u4096, U4096);

check_rand!(rand_h128, H128);
check_rand!(rand_h160, H160);
check_rand!(rand_h224, H224);
check_rand!(rand_h256, H256);
check_rand!(rand_h384, H384);
check_rand!(rand_h512, H512);
check_rand!(rand_h520, H520);
check_rand!(rand_h1024, H1024);
check_rand!(rand_h2048, H2048);
check_rand!(rand_h4096, H4096);

#[test]
fn with_serde_defun_public() {
    let x = U256::thread_random();
    let json = serde_json::to_string(&x);
    assert!(json.is_ok());
    let json = json.unwrap();
    let y = serde_json::from_str(&json);
    assert!(y.is_ok());
    assert_eq!(x, y.unwrap());
}
