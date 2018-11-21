// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate criterion;
extern crate ethereum_types as etypes;
extern crate numext_fixed_hash as nfhash;
extern crate numext_fixed_hash_tests as nfhash_tests;
extern crate serde_json;

use criterion::Criterion;

fn serde(c: &mut Criterion) {
    let x = nfhash::H256::repeat_byte(2);
    let y = etypes::H256([2; 32]);

    let x_json = serde_json::to_string(&x).unwrap();
    let y_json = serde_json::to_string(&y).unwrap();

    c.bench_function("bench_ser_numext_h256", move |b| {
        b.iter(|| {
            let _ = serde_json::to_string(&x);
        })
    });

    c.bench_function("bench_ser_ethereum_types_h256", move |b| {
        b.iter(|| {
            let _ = serde_json::to_string(&y);
        })
    });

    c.bench_function("bench_de_numext_h256", move |b| {
        b.iter(|| {
            let _: nfhash::H256 = serde_json::from_str(&x_json).unwrap();
        })
    });

    c.bench_function("bench_de_ethereum_types_h256", move |b| {
        b.iter(|| {
            let _: etypes::H256 = serde_json::from_str(&y_json).unwrap();
        })
    });
}

criterion_group!(ext, serde);
criterion_main!(ext);
