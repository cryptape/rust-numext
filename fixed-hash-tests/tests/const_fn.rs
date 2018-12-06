#![allow(dead_code)]

extern crate numext_fixed_hash as nhash;
use nhash::H256;

const ZERO: H256 = H256::zero();
const MAX: H256 = H256::repeat_byte(1);
