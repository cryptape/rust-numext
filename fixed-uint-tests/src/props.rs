// Copyright 2018-2019 Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Support proptest.

use etypes;
use nfuint;
use num_bigint;

use proptest::arbitrary::Arbitrary;
use proptest::prelude::RngCore;
use proptest::strategy::{NewTree, Strategy, ValueTree};
use proptest::test_runner::{TestRng, TestRunner};

#[derive(Clone, Copy)]
pub enum U256PairParameters {
    Random,
    CanAdd,
    CanSub,
    CanMul,
    CanDiv,
    CanRem,
}

impl ::std::fmt::Debug for U256PairParameters {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            U256PairParameters::Random => write!(f, "."),
            U256PairParameters::CanAdd => write!(f, "+"),
            U256PairParameters::CanSub => write!(f, "-"),
            U256PairParameters::CanMul => write!(f, "*"),
            U256PairParameters::CanDiv => write!(f, "/"),
            U256PairParameters::CanRem => write!(f, "%"),
        }
    }
}

impl ::std::default::Default for U256PairParameters {
    fn default() -> Self {
        U256PairParameters::Random
    }
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct U256LeBytes {
    pub inner: [u8; 32],
}

impl ::std::fmt::Debug for U256LeBytes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "0x")?;
        for i in self.inner.iter().rev() {
            write!(f, "{:02x}", i)?;
        }
        write!(f, "")
    }
}

impl<'a> ::std::convert::From<&'a U256LeBytes> for etypes::U256 {
    fn from(bytes: &U256LeBytes) -> Self {
        Self::from_little_endian(&bytes.inner)
    }
}

impl<'a> ::std::convert::From<&'a U256LeBytes> for nfuint::U256 {
    fn from(bytes: &U256LeBytes) -> Self {
        Self::from_little_endian(&bytes.inner).unwrap()
    }
}

impl<'a> ::std::convert::From<&'a U256LeBytes> for num_bigint::BigUint {
    fn from(bytes: &U256LeBytes) -> Self {
        Self::from_bytes_le(&bytes.inner)
    }
}

impl ::std::convert::From<U256LeBytes> for etypes::U256 {
    fn from(bytes: U256LeBytes) -> Self {
        Self::from_little_endian(&bytes.inner)
    }
}

impl ::std::convert::From<U256LeBytes> for nfuint::U256 {
    fn from(bytes: U256LeBytes) -> Self {
        Self::from_little_endian(&bytes.inner).unwrap()
    }
}

impl ::std::convert::From<U256LeBytes> for num_bigint::BigUint {
    fn from(bytes: U256LeBytes) -> Self {
        Self::from_bytes_le(&bytes.inner)
    }
}

impl<'a> ::std::convert::From<&'a etypes::U256> for U256LeBytes {
    fn from(u: &etypes::U256) -> Self {
        let mut inner = [0u8; 32];
        u.to_little_endian(&mut inner);
        Self { inner }
    }
}

impl<'a> ::std::convert::From<&'a nfuint::U256> for U256LeBytes {
    fn from(u: &nfuint::U256) -> Self {
        let mut inner = [0u8; 32];
        u.into_little_endian(&mut inner).unwrap();
        Self { inner }
    }
}

impl ::std::convert::From<etypes::U256> for U256LeBytes {
    fn from(u: etypes::U256) -> Self {
        let mut inner = [0u8; 32];
        u.to_little_endian(&mut inner);
        Self { inner }
    }
}

impl ::std::convert::From<nfuint::U256> for U256LeBytes {
    fn from(u: nfuint::U256) -> Self {
        let mut inner = [0u8; 32];
        u.into_little_endian(&mut inner).unwrap();
        Self { inner }
    }
}

impl U256LeBytes {
    pub fn any(rng: &mut TestRng) -> Self {
        let mut inner = [0u8; 32];
        rng.fill_bytes(&mut inner);
        Self { inner }
    }

    pub fn nonzero(rng: &mut TestRng) -> Self {
        let mut ret = Self::any(rng);
        'outer: loop {
            for unit in &ret.inner[..] {
                if *unit != 0 {
                    break 'outer;
                }
            }
            rng.fill_bytes(&mut ret.inner);
        }
        ret
    }

    pub fn bigger(&self, rng: &mut TestRng) -> Self {
        let mut ret = Self::any(rng);
        'outer: loop {
            for i in (0..32).rev() {
                if ret.inner[i] > self.inner[i] {
                    break 'outer;
                }
            }
            rng.fill_bytes(&mut ret.inner);
        }
        ret
    }

    pub fn reverse(&mut self) {
        self.inner.reverse();
    }

    fn highest_nonzero_bytes(&self) -> Option<usize> {
        let mut ret: Option<usize> = None;
        for i in 0..32 {
            if self.inner[31 - i] != 0 {
                ret = Some(31 - i);
                break;
            }
        }
        ret
    }

    fn shrink(&mut self) -> bool {
        if let Some(hi) = self.highest_nonzero_bytes() {
            self.inner[hi] >>= 1;
            true
        } else {
            false
        }
    }

    pub fn nonzero_pair(rng: &mut TestRng) -> (Self, Self) {
        (Self::nonzero(rng), Self::nonzero(rng))
    }

    pub fn nonzero_pair_asc(rng: &mut TestRng) -> (Self, Self) {
        let mut smaller = Self::nonzero(rng);
        let mut bigger = Self::nonzero(rng);
        'outer: loop {
            for i in (0..32).rev() {
                if smaller.inner[i] > bigger.inner[i] {
                    ::std::mem::swap(&mut smaller.inner[i], &mut bigger.inner[i]);
                    break 'outer;
                } else if smaller.inner[i] < bigger.inner[i] {
                    break 'outer;
                }
            }
        }
        (smaller, bigger)
    }

    pub fn nonzero_pair_desc(rng: &mut TestRng) -> (Self, Self) {
        let (smaller, bigger) = Self::nonzero_pair_asc(rng);
        (bigger, smaller)
    }

    pub fn can_add_without_overflow(rng: &mut TestRng) -> (Self, Self) {
        let (smaller, mut bigger) = Self::nonzero_pair_asc(rng);
        let mut inner = [0u8; 32];
        for (idx, value) in inner.iter_mut().enumerate() {
            *value = if smaller.inner[idx] <= bigger.inner[idx] {
                bigger.inner[idx] - smaller.inner[idx]
            } else {
                for j in (idx + 1)..32 {
                    if bigger.inner[j] == 0 {
                        bigger.inner[j] = !0;
                    } else {
                        bigger.inner[j] -= 1;
                        break;
                    }
                }
                (!0) - (smaller.inner[idx] - bigger.inner[idx]) + 1
            };
        }
        let diff = U256LeBytes { inner };
        (smaller, diff)
    }

    pub fn can_mul_without_overflow(rng: &mut TestRng) -> (Self, Self) {
        let (first, mut second) = Self::nonzero_pair_desc(rng);
        let mut zeros = 0;
        for i in 0..32 {
            if first.inner[31 - i] == 0 {
                zeros += 1;
            } else {
                break;
            }
        }
        if zeros == 0 {
            second.inner[0] = 1;
            zeros += 1;
        }
        for i in zeros..32 {
            second.inner[i] = 0;
        }
        (first, second)
    }
}

pub struct U256LeBytesValueTree {
    orig: U256LeBytes,
    curr: U256LeBytes,
    shrink_times: usize,
}

impl U256LeBytesValueTree {
    pub fn new(runner: &mut TestRunner) -> Self {
        let rng = runner.rng();
        let orig = U256LeBytes::any(rng);
        let curr = orig.clone();
        let shrink_times = 0;
        Self {
            orig,
            curr,
            shrink_times,
        }
    }
}

impl ValueTree for U256LeBytesValueTree {
    type Value = U256LeBytes;

    fn current(&self) -> Self::Value {
        self.curr.clone()
    }

    fn simplify(&mut self) -> bool {
        if self.curr.shrink() {
            self.shrink_times += 1;
            true
        } else {
            false
        }
    }

    fn complicate(&mut self) -> bool {
        if self.shrink_times > 0 {
            self.shrink_times -= 1;
            let mut prev = self.orig.clone();
            let mut times = self.shrink_times;
            while times > 0 {
                prev.shrink();
                times -= 1;
            }
            self.curr = prev;
            true
        } else {
            false
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct U256LeBytesParameters;

#[derive(Debug)]
pub struct U256LeBytesStrategy {
    params: U256LeBytesParameters,
}

impl U256LeBytesStrategy {
    pub fn new(params: U256LeBytesParameters) -> Self {
        Self { params }
    }
}

impl Strategy for U256LeBytesStrategy {
    type Tree = U256LeBytesValueTree;
    type Value = U256LeBytes;

    fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
        let tree = Self::Tree::new(runner);
        Ok(tree)
    }
}

impl Arbitrary for U256LeBytes {
    type Parameters = U256LeBytesParameters;
    type Strategy = U256LeBytesStrategy;
    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
        Self::Strategy::new(args)
    }
}

#[derive(Clone)]
pub struct U256Pair {
    pub lhs: U256LeBytes,
    pub rhs: U256LeBytes,
    pub opr: U256PairParameters,
}

impl ::std::fmt::Debug for U256Pair {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(
            f,
            "U256Pair {{ {:?} {:?} {:?} }}",
            self.lhs, self.opr, self.rhs
        )
    }
}

impl<'a> ::std::convert::From<&'a U256Pair> for (etypes::U256, etypes::U256) {
    fn from(pair: &U256Pair) -> Self {
        let U256Pair {
            ref lhs, ref rhs, ..
        } = pair;
        (lhs.into(), rhs.into())
    }
}

impl<'a> ::std::convert::From<&'a U256Pair> for (nfuint::U256, nfuint::U256) {
    fn from(pair: &U256Pair) -> Self {
        let U256Pair {
            ref lhs, ref rhs, ..
        } = pair;
        (lhs.into(), rhs.into())
    }
}

impl<'a> ::std::convert::From<&'a U256Pair> for (num_bigint::BigUint, num_bigint::BigUint) {
    fn from(pair: &U256Pair) -> Self {
        let U256Pair {
            ref lhs, ref rhs, ..
        } = pair;
        (lhs.into(), rhs.into())
    }
}

impl ::std::convert::From<U256Pair> for (etypes::U256, etypes::U256) {
    fn from(pair: U256Pair) -> Self {
        let U256Pair { lhs, rhs, .. } = pair;
        (lhs.into(), rhs.into())
    }
}

impl ::std::convert::From<U256Pair> for (nfuint::U256, nfuint::U256) {
    fn from(pair: U256Pair) -> Self {
        let U256Pair { lhs, rhs, .. } = pair;
        (lhs.into(), rhs.into())
    }
}

impl ::std::convert::From<U256Pair> for (num_bigint::BigUint, num_bigint::BigUint) {
    fn from(pair: U256Pair) -> Self {
        let U256Pair { lhs, rhs, .. } = pair;
        (lhs.into(), rhs.into())
    }
}

impl U256Pair {
    pub fn new(opr: U256PairParameters, rng: &mut TestRng) -> Self {
        let (lhs, rhs) = match opr {
            U256PairParameters::Random => U256LeBytes::nonzero_pair(rng),
            U256PairParameters::CanAdd => U256LeBytes::can_add_without_overflow(rng),
            U256PairParameters::CanSub => U256LeBytes::nonzero_pair_desc(rng),
            U256PairParameters::CanMul => U256LeBytes::can_mul_without_overflow(rng),
            U256PairParameters::CanDiv => U256LeBytes::nonzero_pair_desc(rng),
            U256PairParameters::CanRem => U256LeBytes::nonzero_pair_desc(rng),
        };
        Self { lhs, rhs, opr }
    }

    fn highest_nonzero_bytes(&self) -> (Option<usize>, Option<usize>) {
        (
            self.lhs.highest_nonzero_bytes(),
            self.rhs.highest_nonzero_bytes(),
        )
    }

    fn shrink_keep_lhs_bigger(&mut self) -> bool {
        match self.highest_nonzero_bytes() {
            (Some(lhi), Some(rhi)) => {
                if lhi == 0 {
                    false
                } else {
                    if (lhi - 1 < rhi)
                        || (lhi - 1 == rhi && self.lhs.inner[lhi - 1] <= self.rhs.inner[rhi])
                    {
                        self.rhs.inner[rhi] >>= 1;
                    } else {
                        self.lhs.inner[lhi] >>= 1;
                    }
                    true
                }
            }
            _ => false,
        }
    }

    fn shrink_alternately(&mut self) -> bool {
        match self.highest_nonzero_bytes() {
            (Some(lhi), Some(rhi)) => {
                if lhi > rhi {
                    self.lhs.inner[lhi] >>= 1;
                } else if lhi < rhi {
                    self.rhs.inner[rhi] >>= 1;
                } else if self.lhs.inner[lhi] > self.rhs.inner[rhi] {
                    self.lhs.inner[lhi] >>= 1;
                } else {
                    self.rhs.inner[rhi] >>= 1;
                }
                true
            }
            _ => false,
        }
    }

    fn shrink_lhs_to_rhs(&mut self) -> bool {
        match self.highest_nonzero_bytes() {
            (Some(lhi), Some(rhi)) => {
                let last = self.lhs.inner[0] & 0b1;

                for i in 0..lhi {
                    let carry = (self.lhs.inner[i + 1] << 7) & 0b10_000_000;
                    self.lhs.inner[i] >>= 1;
                    self.lhs.inner[i] |= carry;
                }
                self.lhs.inner[lhi] >>= 1;

                if rhi < 31 {
                    let carry = (self.rhs.inner[rhi] >> 7) & 0b1;
                    self.rhs.inner[rhi + 1] |= carry;
                }
                for i in (1..=rhi).rev() {
                    let carry = (self.rhs.inner[i - 1] >> 7) & 0b1;
                    self.rhs.inner[i] <<= 1;
                    self.rhs.inner[i] |= carry;
                }
                self.rhs.inner[0] <<= 1;
                self.rhs.inner[0] |= last;

                true
            }
            _ => false,
        }
    }

    fn shrink(&mut self) -> bool {
        match self.opr {
            U256PairParameters::Random => self.shrink_alternately(),
            U256PairParameters::CanAdd => self.shrink_alternately(),
            U256PairParameters::CanSub => self.shrink_keep_lhs_bigger(),
            U256PairParameters::CanMul => self.shrink_lhs_to_rhs(),
            U256PairParameters::CanDiv => self.shrink_alternately(),
            U256PairParameters::CanRem => self.shrink_keep_lhs_bigger(),
        }
    }
}

pub struct U256PairValueTree {
    pair_orig: U256Pair,
    pair_curr: U256Pair,
    shrink_times: usize,
}

impl U256PairValueTree {
    pub fn new(params: U256PairParameters, runner: &mut TestRunner) -> Self {
        let rng = runner.rng();
        let pair_orig = U256Pair::new(params, rng);
        let pair_curr = pair_orig.clone();
        let shrink_times = 0;
        Self {
            pair_orig,
            pair_curr,
            shrink_times,
        }
    }
}

impl ValueTree for U256PairValueTree {
    type Value = U256Pair;

    fn current(&self) -> Self::Value {
        self.pair_curr.clone()
    }

    fn simplify(&mut self) -> bool {
        if self.pair_curr.shrink() {
            self.shrink_times += 1;
            true
        } else {
            false
        }
    }

    fn complicate(&mut self) -> bool {
        if self.shrink_times > 0 {
            self.shrink_times -= 1;
            let mut pair_prev = self.pair_orig.clone();
            let mut times = self.shrink_times;
            while times > 0 {
                pair_prev.shrink();
                times -= 1;
            }
            self.pair_curr = pair_prev;
            true
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct U256PairStrategy {
    params: U256PairParameters,
}

impl U256PairStrategy {
    pub fn new(params: U256PairParameters) -> Self {
        Self { params }
    }
}

impl Strategy for U256PairStrategy {
    type Tree = U256PairValueTree;
    type Value = U256Pair;

    fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
        let tree = Self::Tree::new(self.params, runner);
        Ok(tree)
    }
}

impl Arbitrary for U256Pair {
    type Parameters = U256PairParameters;
    type Strategy = U256PairStrategy;
    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
        Self::Strategy::new(args)
    }
}
