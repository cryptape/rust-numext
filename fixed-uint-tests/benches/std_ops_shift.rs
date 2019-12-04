// Copyright 2018-2019 Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use criterion::{black_box, criterion_group, criterion_main, Criterion, ParameterizedBenchmark};

macro_rules! std_ops_shift {
    ($opr:tt, $bencher:ident, $bytes:ident, $type:ty, $bits:expr) => {{
        let val: $type = $bytes.into();
        $bencher.iter(move || {
            black_box(val.clone()) $opr $bits
        })
    }}
}

macro_rules! bench_std_ops_shift {
    ($opr:tt, $tag:literal, $func:ident, $bits:expr) => {
        fn $func(c: &mut Criterion) {
            c.bench(
                format!("{}/{}", $tag, $bits).as_str(),
                ParameterizedBenchmark::new(
                    "nfuint",
                    |b, v| std_ops_shift!($opr, b, v, nfuint::U256, $bits),
                    vec![nfuint_tests::tools::lebytes()],
                )
                .with_function("etypes", |b, v| {
                    std_ops_shift!($opr, b, v, etypes::U256, $bits)
                }),
            );
        }
    };
}

bench_std_ops_shift!(<<, "shift/left", ushl0, 0u8);
bench_std_ops_shift!(<<, "shift/left", ushl7, 7u16);
bench_std_ops_shift!(<<, "shift/left", ishl65, 65i32);
bench_std_ops_shift!(<<, "shift/left", ushl511, 511u64);

bench_std_ops_shift!(>>, "shift/right", ushr0, 0u8);
bench_std_ops_shift!(>>, "shift/right", ushr7, 7u16);
bench_std_ops_shift!(>>, "shift/right", ishr65, 65i32);
bench_std_ops_shift!(>>, "shift/right", ushr511, 511u64);

criterion_group!(shift, ushl0, ushl7, ishl65, ushl511, ushr0, ushr7, ishr65, ushr511);
criterion_main!(shift);
