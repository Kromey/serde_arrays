// Copyright 2021 Travis Veazey
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// https://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use serde::{Deserialize, Serialize};

/// A simple struct containing a const generic array
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct GenericArray<const N: usize> {
    #[serde(with = "serde_arrays")]
    pub arr: [u32; N],
}

/// A simple struct containing a fixed array
///
/// This struct specifically uses a length of 36 for its array because that's beyond the size Serde
/// can normally handle ([T; 0] through [T; 32]), thereby proving not just that this module works,
/// but that it's not accidentally deferring to any built-in implementations within Serde. At the
/// same time, 36 is small enough to still be manageable for the strings appearing in the tests.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct FixedArray {
    #[serde(with = "serde_arrays")]
    pub arr: [u32; 36],
}

/// A tuple struct containing a const generic array
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct GenericTupleStruct<const N: usize>(#[serde(with = "serde_arrays")] pub [u32; N]);
