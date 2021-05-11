// Copyright 2021 Travis Veazey
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// https://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use serde::ser::{Serialize, Serializer};

pub struct ArrayWrap<'a, T: Serialize, const N: usize> {
    inner: &'a [T; N],
}

impl<'a, T: Serialize, const N: usize> ArrayWrap<'a, T, N> {
    pub fn new(array: &'a [T; N]) -> ArrayWrap<'a, T, N> {
        ArrayWrap { inner: array }
    }
}

impl<'a, T: Serialize, const N: usize> Serialize for ArrayWrap<'a, T, N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        super::serialize(self.inner, serializer)
    }
}
