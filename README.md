# Rust-NumExt

[![License]](#license)
[![Travis CI]](https://travis-ci.com/cryptape/rust-numext)
[![AppVeyor]](https://ci.appveyor.com/project/cryptape/rust-numext)

Extend the rust built-in numeric types.

[License]: https://img.shields.io/badge/License-Apache--2.0%20OR%20MIT-blue.svg
[Travis CI]: https://img.shields.io/travis/com/cryptape/rust-numext.svg
[AppVeyor]: https://ci.appveyor.com/api/projects/status/github/cryptape/rust-numext?branch=master&svg=true

## Crates

| Name                   | Crate                                                               | Documentation                                            | Description                 |
| ---------------------- | ------------------------------------------------------------------- | -------------------------------------------------------- | --------------------------- |
| [`numext-fixed-uint`]  | [![Uint Badge]](https://crates.io/crates/numext-fixed-uint)         | [![Uint Doc]](https://docs.rs/numext-fixed-uint)         | Fixed-size uint structures. |
| [`numext-fixed-hash`]  | [![Hash Badge]](https://crates.io/crates/numext-fixed-hash)         | [![Hash Doc]](https://docs.rs/numext-fixed-hash)         | Fixed-size hash structures. |

[`numext-fixed-uint`]: fixed-uint
[`numext-fixed-hash`]: fixed-hash

[Uint Badge]: https://img.shields.io/crates/v/numext-fixed-uint.svg
[Hash Badge]: https://img.shields.io/crates/v/numext-fixed-hash.svg

[Uint Doc]: https://docs.rs/numext-fixed-uint/badge.svg
[Hash Doc]: https://docs.rs/numext-fixed-hash/badge.svg

## Requirements

- `rustc 1.45.0+`.

  Require [Stabilizing function-like procedural macros in expressions, patterns, and statements](https://blog.rust-lang.org/2020/07/16/Rust-1.45.0.html#stabilizing-function-like-procedural-macros-in-expressions-patterns-and-statements).

## Benchmark

### Howto

```
cargo bench
```

### Results on Intel(R) Core(TM) i7-6700 CPU @ 3.40GHz

- `+` means better; `-` means worse.
- `nfuint` and `nfhash` is short for [`numext-fixed-uint`] and [`numext-fixed-hash`].
- `etypes` is short for [`ethereum-types`](https://crates.io/crates/ethereum-types).

Serialization / Deserialization Benchmark:
```diff
+serde/ser/nfhash/h256   time:   [56.812 ns 56.834 ns 56.855 ns]
-serde/ser/etypes/h256   time:   [101.41 ns 102.24 ns 103.30 ns]
+serde/de/nfhash/h256    time:   [96.613 ns 96.643 ns 96.672 ns]
-serde/de/etypes/h256    time:   [160.63 ns 160.73 ns 160.89 ns]

-serde/ser/nfuint/u256   time:   [114.36 ns 114.66 ns 115.14 ns]
+serde/ser/etypes/u256   time:   [102.49 ns 102.55 ns 102.59 ns]
+serde/de/nfuint/u256    time:   [117.83 ns 117.85 ns 117.88 ns]
-serde/de/etypes/u256    time:   [175.12 ns 175.20 ns 175.29 ns]
```

Calculation Benchmark:
```diff
+gcd/nfuint              time:   [2.7441 us 2.7444 us 2.7447 us]
-gcd/num_bigint          time:   [6.9668 us 6.9683 us 6.9697 us]

+add/nfuint              time:   [2.7960 ns 2.8020 ns 2.8067 ns]
-add/etypes              time:   [4.9120 ns 4.9152 ns 4.9183 ns]
+sub/nfuint              time:   [2.7734 ns 2.7812 ns 2.7884 ns]
-sub/etypes              time:   [4.9641 ns 4.9679 ns 4.9715 ns]
+mul/nfuint              time:   [13.786 ns 13.788 ns 13.790 ns]
-mul/etypes              time:   [14.946 ns 14.950 ns 14.956 ns]
+div/nfuint              time:   [25.820 ns 25.826 ns 25.832 ns]
-div/etypes              time:   [77.297 ns 77.300 ns 77.304 ns]
+rem/nfuint              time:   [26.454 ns 26.455 ns 26.458 ns]
-rem/etypes              time:   [72.860 ns 72.902 ns 72.958 ns]
+bitand/nfuint           time:   [3.6945 ns 3.6946 ns 3.6947 ns]
-bitand/etypes           time:   [5.3184 ns 5.3224 ns 5.3267 ns]
+bitor/nfuint            time:   [3.6945 ns 3.6947 ns 3.6948 ns]
-bitor/etypes            time:   [5.3298 ns 5.3323 ns 5.3347 ns]
+bitxor/nfuint           time:   [3.6944 ns 3.6946 ns 3.6947 ns]
-bitxor/etypes           time:   [5.3315 ns 5.3342 ns 5.3367 ns]
+not/nfuint              time:   [2.3861 ns 2.3867 ns 2.3873 ns]
-not/etypes              time:   [3.8522 ns 3.8526 ns 3.8530 ns]

+shift/left/0/nfuint     time:   [2.4974 ns 2.4978 ns 2.4981 ns]
-shift/left/0/etypes     time:   [9.7666 ns 9.7678 ns 9.7692 ns]
+shift/left/7/nfuint     time:   [2.5193 ns 2.5196 ns 2.5199 ns]
-shift/left/7/etypes     time:   [12.425 ns 12.426 ns 12.428 ns]
+shift/left/65/nfuint    time:   [2.2779 ns 2.2832 ns 2.2905 ns]
-shift/left/65/etypes    time:   [11.685 ns 11.725 ns 11.775 ns]
+shift/left/511/nfuint   time:   [1.9075 ns 1.9083 ns 1.9091 ns]
-shift/left/511/etypes   time:   [5.6710 ns 5.6788 ns 5.6893 ns]

+shift/right/0/nfuint    time:   [2.4972 ns 2.4977 ns 2.4981 ns]
-shift/right/0/etypes    time:   [9.7640 ns 9.7645 ns 9.7651 ns]
+shift/right/7/nfuint    time:   [2.5281 ns 2.5284 ns 2.5286 ns]
-shift/right/7/etypes    time:   [12.409 ns 12.410 ns 12.411 ns]
+shift/right/65/nfuint   time:   [2.3753 ns 2.3755 ns 2.3757 ns]
-shift/right/65/etypes   time:   [11.878 ns 11.879 ns 11.880 ns]
+shift/right/511/nfuint  time:   [1.9077 ns 1.9088 ns 1.9098 ns]
-shift/right/511/etypes  time:   [5.7064 ns 5.7336 ns 5.7679 ns]
```

## License

Licensed under either of [Apache License, Version 2.0] or [MIT License], at
your option.

[Apache License, Version 2.0]: LICENSE-APACHE
[MIT License]: LICENSE-MIT
