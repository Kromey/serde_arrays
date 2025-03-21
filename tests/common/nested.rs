// Copyright 2021 Travis Veazey
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// https://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
#![cfg(any(feature = "std", feature = "alloc"))]

use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct NestedArray<const N: usize> {
    #[serde(with = "serde_arrays")]
    pub arr: [[u32; N]; 2],
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct GenericNestedArray<const N: usize, const M: usize> {
    #[serde(with = "serde_arrays")]
    pub arr: [[u32; N]; M],
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct VecArray<const N: usize> {
    #[serde(with = "serde_arrays")]
    pub arr: Vec<[u32; N]>,
}
