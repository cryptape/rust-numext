// Copyright 2018-2019 Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(dead_code)]

use nfhash::H256;

const EMPTY: H256 = H256::empty();
const FULL: H256 = H256::full();
const ALL_BYTE_ARE_ONE: H256 = H256::repeat_byte(1);
