// Copyright 2018-2019 Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[test]
fn div_throw_add_overflow() {
    let one = nfuint::U256::one();
    for i in 0..255 {
        let x = nfuint::U256::one() << i;
        let y = &x / &one;
        assert_eq!(x, y);
    }
    let x = nfuint::U256::from(4096u32);
    let y = ((nfuint::U256::one() << 255) / &x) << 1;
    let z = ((nfuint::U256::one() << 255) / &y) << 1;
    assert_eq!(x, z);
}

#[test]
fn div_too_slow() {
    let x =
        nfuint::U4096::from_hex_str("272184cdaf3736f0fa54c1d8529a9294bcc2ac0b180838228ab").unwrap();
    let y = nfuint::U4096::from_hex_str(
        "8000000000000000000000000000000000000000000000000000000000000000",
    )
    .unwrap();
    let expected = nfuint::U4096::from_hex_str(
        "6b851a863a5e38d58e175cb90a7b4dd5b7bcab518f09f17ade7398cc5621e239",
    )
    .unwrap();
    let result = &x * &x % y;
    assert_eq!(expected, result);
}
