// Copyright 2018-2019 Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

macro_rules! check_rand {
    ($name:ident, $uint:ident) => {
        #[test]
        fn $name() {
            let x = nfhash::$uint::thread_random();
            let y = nfhash::$uint::thread_random();
            // If this test is failed, please check if there is a bug or you are too luckly.
            assert!(!x.is_zero());
            assert!(!y.is_zero());
            assert!(x != y);
        }
    };
}

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
