// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use criterion::{black_box, criterion_group, criterion_main, Criterion, ParameterizedBenchmark};
use num_integer::Integer;

macro_rules! gcd {
    ($bencher:ident, $pair:ident, $type:ty) => {{
        let (ref lhs, ref rhs): ($type, $type) = $pair.into();
        $bencher.iter(move || black_box(lhs).gcd(black_box(rhs)))
    }};
}

fn gcd(c: &mut Criterion) {
    c.bench(
        "gcd",
        ParameterizedBenchmark::new(
            "nfuint",
            |b, p| gcd!(b, p, nfuint::U256),
            vec![nfuint_tests::tools::pair(
                nfuint_tests::props::U256PairParameters::Random,
            )],
        )
        .with_function("num_bigint", |b, p| gcd!(b, p, num_bigint::BigUint)),
    );
}

criterion_group!(mathematics, gcd);
criterion_main!(mathematics);
