// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use criterion::{black_box, criterion_group, criterion_main, Criterion, ParameterizedBenchmark};

macro_rules! std_ops_binary {
    (direct: $opr:tt, $bencher:ident, $pair:ident, $type:ty) => {{
        let (ref lhs, ref rhs): ($type, $type) = $pair.into();
        $bencher.iter(move || {
            black_box(lhs) $opr black_box(rhs)
        })
    }};
    (clone: $opr:tt, $bencher:ident, $pair:ident, $type:ty) => {{
        let (ref lhs, ref rhs): ($type, $type) = $pair.into();
        $bencher.iter(move || {
            black_box(lhs.clone()) $opr black_box(rhs.clone())
        })
    }};
}

macro_rules! bench_std_ops_binary {
    ($opr:tt, $name:ident, $param:ident, $tag:ident) => {
        fn $name(c: &mut Criterion) {
            c.bench(
                stringify!($name),
                ParameterizedBenchmark::new(
                    "nfuint",
                    |b, p| std_ops_binary!($tag: $opr, b, p, nfuint::U256),
                    vec![nfuint_tests::tools::pair(
                        nfuint_tests::props::U256PairParameters::$param,
                    )],
                )
                .with_function("etypes", |b, p| {
                    std_ops_binary!($tag: $opr, b, p, etypes::U256)
                }),
            );
        }
    };
}

macro_rules! std_ops_unary {
    ($opr:tt, $bencher:ident, $bytes:ident, $type:ty) => {{
        let val: $type = $bytes.into();
        $bencher.iter(move || {
            $opr black_box(val.clone())
        })
    }}
}

macro_rules! bench_std_ops_unary {
    ($opr:tt, $name:ident) => {
        fn $name(c: &mut Criterion) {
            c.bench(
                stringify!($name),
                ParameterizedBenchmark::new(
                    "nfuint",
                    |b, v| std_ops_unary!($opr, b, v, nfuint::U256),
                    vec![nfuint_tests::tools::lebytes()],
                )
                .with_function("etypes", |b, v| std_ops_unary!($opr, b, v, etypes::U256)),
            );
        }
    };
}

bench_std_ops_binary!(+, add, CanAdd, direct);
bench_std_ops_binary!(-, sub, CanSub, direct);
bench_std_ops_binary!(*, mul, CanMul, direct);
bench_std_ops_binary!(/, div, CanDiv, direct);
bench_std_ops_binary!(%, rem, CanRem, direct);
bench_std_ops_binary!(&, bitand, Random, clone);
bench_std_ops_binary!(|, bitor, Random, clone);
bench_std_ops_binary!(^, bitxor, Random, clone);

bench_std_ops_unary!(!, not);

criterion_group!(arithmetic, add, sub, mul, div, rem, bitand, bitor, bitxor, not);
criterion_main!(arithmetic);
