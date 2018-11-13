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

extern crate numext_fixed_uint as nfuint;
extern crate numext_fixed_uint_tests as nfuint_tests;

use criterion::{black_box, Criterion, ParameterizedBenchmark};

macro_rules! std_ops_shift {
    ($opr:tt, $bencher:ident, $bytes:ident, $type:ty, $bits:expr) => {{
        let val: $type = $bytes.into();
        $bencher.iter(move || {
            black_box(val.clone()) $opr $bits
        })
    }}
}

macro_rules! bench_std_ops_shift {
    ($opr:tt, $name:ident, $bits:expr) => {
        fn $name(c: &mut Criterion) {
            c.bench(
                stringify!($name),
                ParameterizedBenchmark::new(
                    "nfuint",
                    |b, v| std_ops_shift!($opr, b, v, nfuint::U256, $bits),
                    vec![nfuint_tests::tools::lebytes()],
                ).with_function("etypes", |b, v| {
                    std_ops_shift!($opr, b, v, etypes::U256, $bits)
                }),
            );
        }
    };
}

bench_std_ops_shift!(<<, ushl0, 0u8);
bench_std_ops_shift!(<<, ushl7, 7u16);
bench_std_ops_shift!(<<, ishl65, 65i32);
bench_std_ops_shift!(<<, ushl511, 511u64);

bench_std_ops_shift!(>>, ushr0, 0u8);
bench_std_ops_shift!(>>, ushr7, 7u16);
bench_std_ops_shift!(>>, ishr65, 65i32);
bench_std_ops_shift!(>>, ushr511, 511u64);

criterion_group!(shift, ushl0, ushl7, ishl65, ushl511, ushr0, ushr7, ishr65, ushr511);
criterion_main!(shift);
