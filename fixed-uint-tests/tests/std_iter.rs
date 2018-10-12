// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate numext_fixed_uint as nfuint;
extern crate uint;

#[test]
fn sum() {
    let multiple = nfuint::U256::one() << 223u64;
    let max = 65536u128;
    let result: nfuint::U256 = (1..=max * 2)
        .filter(|x| x % 2 != 0)
        .map(|x| nfuint::U256::from(x) * &multiple)
        .sum();
    let max_u256 = nfuint::U256::from(max);
    let check = &max_u256 * &max_u256 * &multiple;
    assert_eq!(result, check);
}

#[test]
fn product() {
    // 99! > 2**512 > 98!
    let mut check_uint = uint::U512::from(1);
    for max in 1u8..98 {
        check_uint *= max;
        let result: nfuint::U512 = (1..=max).map(nfuint::U512::from).product();
        let check = nfuint::U512::from_dec_str(format!("{}", check_uint).as_ref()).unwrap();
        assert_eq!(result, check);
    }
    let max = 536u128;
    let result: nfuint::U4096 = (1..=max).map(nfuint::U4096::from).product();
    let check = nfuint::U4096::from_hex_str(
        "ffb1f702086548f51cf4de42574c8d73bb1b404ad975927b72884f1d80672bfe\
         61ad8adfe7721a4fc7d8854fd37c17e891a9d90e18974c3fb9a690950dd518d7\
         459377443610e2c61546d7c4a0b13945023ba8905d7081bda4677bbcbf8fd995\
         8235da23c44a031b40652abeddeeb99ffe11aacbd440086d7086e78f47f4c656\
         ba89f60cc1ba36943a9b5deccb74b40cfe9fcfc640876bd2598ab6113228c021\
         a71dca583485f7901947be9564fd40e51279ac7b25536421e4d39a860b2304f0\
         1a1cad593cc918c5b839f2676ba8741198a7ba43491ecebd1fae1ebbf750cf4f\
         5fe9df70dcf8174103407f2270438fde8057750ec238e60989c7096c452b0b82\
         64736f89755064f950c281e1a6fa6b78fd8e12ce85ad9c1dcbb9fff7ba3a385a\
         b4bf05a513f0b3685fc3a126fe696dad6236778b3a5c430d74ecf65d960cecf1\
         6a399cb4207e6d4da3b67df3e35a6e42d723cf28b804d4fbe1c8b780e51daafa\
         84c219a58b536391a90192dd09035f42f0cfa332d617e764b82c4df26f2f1f50\
         71579ccf2182526c05f07a29a02a71e8f7404ea7cb6acbd8a2ccf9c8fce6107a\
         529f70ec42d367d4061ab230802f52de71805271f743024c940ab4a95a000000\
         0000000000000000000000000000000000000000000000000000000000000000\
         000000000000000000000000000000000000000000000000000000000000000",
    ).unwrap();
    assert_eq!(result, check);
}
