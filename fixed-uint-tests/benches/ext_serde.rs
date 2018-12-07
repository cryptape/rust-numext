// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use criterion::{criterion_group, criterion_main, Criterion};

fn serde(c: &mut Criterion) {
    let x = nfuint::U256::max_value();
    let y = etypes::U256::max_value();

    let x_json = serde_json::to_string(&x).unwrap();
    let y_json = serde_json::to_string(&y).unwrap();

    c.bench_function("bench_ser_numext_u256", move |b| {
        b.iter(|| {
            let _ = serde_json::to_string(&x);
        })
    });

    c.bench_function("bench_ser_ethereum_types_u256", move |b| {
        b.iter(|| {
            let _ = serde_json::to_string(&y);
        })
    });

    c.bench_function("bench_de_numext_u256", move |b| {
        b.iter(|| {
            let _: nfuint::U256 = serde_json::from_str(&x_json).unwrap();
        })
    });

    c.bench_function("bench_de_ethereum_types_u256", move |b| {
        b.iter(|| {
            let _: etypes::U256 = serde_json::from_str(&y_json).unwrap();
        })
    });
}

criterion_group!(ext, serde);
criterion_main!(ext);
