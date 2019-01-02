// Copyright 2018-2019 Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Support proptest.

use etypes;
use nfhash;

use proptest::arbitrary::Arbitrary;
use proptest::prelude::RngCore;
use proptest::strategy::{NewTree, Strategy, ValueTree};
use proptest::test_runner::{TestRng, TestRunner};

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct H256Bytes {
    pub inner: [u8; 32],
}

impl ::std::fmt::Debug for H256Bytes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "0x")?;
        for i in self.inner.iter() {
            write!(f, "{:02x}", i)?;
        }
        write!(f, "")
    }
}

impl<'a> ::std::convert::From<&'a H256Bytes> for etypes::H256 {
    fn from(bytes: &H256Bytes) -> Self {
        bytes.inner.into()
    }
}

impl<'a> ::std::convert::From<&'a H256Bytes> for nfhash::H256 {
    fn from(bytes: &H256Bytes) -> Self {
        bytes.inner.into()
    }
}

impl ::std::convert::From<H256Bytes> for etypes::H256 {
    fn from(bytes: H256Bytes) -> Self {
        bytes.inner.into()
    }
}

impl ::std::convert::From<H256Bytes> for nfhash::H256 {
    fn from(bytes: H256Bytes) -> Self {
        bytes.inner.into()
    }
}

impl<'a> ::std::convert::From<&'a etypes::H256> for H256Bytes {
    fn from(u: &etypes::H256) -> Self {
        let inner = u.0;
        Self { inner }
    }
}

impl<'a> ::std::convert::From<&'a nfhash::H256> for H256Bytes {
    fn from(u: &nfhash::H256) -> Self {
        let mut inner = [0u8; 32];
        u.into_slice(&mut inner).unwrap();
        Self { inner }
    }
}

impl ::std::convert::From<etypes::H256> for H256Bytes {
    fn from(u: etypes::H256) -> Self {
        let inner = u.0;
        Self { inner }
    }
}

impl ::std::convert::From<nfhash::H256> for H256Bytes {
    fn from(u: nfhash::H256) -> Self {
        let mut inner = [0u8; 32];
        u.into_slice(&mut inner).unwrap();
        Self { inner }
    }
}

impl H256Bytes {
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
}

pub struct H256BytesValueTree {
    orig: H256Bytes,
    curr: H256Bytes,
    shrink_times: usize,
}

impl H256BytesValueTree {
    pub fn new(runner: &mut TestRunner) -> Self {
        let rng = runner.rng();
        let orig = H256Bytes::any(rng);
        let curr = orig.clone();
        let shrink_times = 0;
        Self {
            orig,
            curr,
            shrink_times,
        }
    }
}

impl ValueTree for H256BytesValueTree {
    type Value = H256Bytes;

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
pub struct H256BytesParameters;

#[derive(Debug)]
pub struct H256BytesStrategy {
    params: H256BytesParameters,
}

impl H256BytesStrategy {
    pub fn new(params: H256BytesParameters) -> Self {
        Self { params }
    }
}

impl Strategy for H256BytesStrategy {
    type Tree = H256BytesValueTree;
    type Value = H256Bytes;

    fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
        let tree = Self::Tree::new(runner);
        Ok(tree)
    }
}

impl Arbitrary for H256Bytes {
    type Parameters = H256BytesParameters;
    type Strategy = H256BytesStrategy;
    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
        Self::Strategy::new(args)
    }
}
