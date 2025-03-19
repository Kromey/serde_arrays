// Copyright 2021 Travis Veazey
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// https://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::wrapper::ArrayWrap;
use serde::ser::{Serialize, SerializeSeq, SerializeTuple, Serializer};
extern crate alloc;
use alloc::vec::Vec;

/// Trait for types serializable using `serde_arrays`
///
/// In order to serialize data using this crate, the type needs to implement this trait. While this
/// approach has limitations in what can be supported (namely it limits support to only those types
/// this trait is explicitly implemented on), the trade off is a significant increase in ergonomics.
///
/// If the greater flexibility lost by this approach is needed, see [`serde_with`][serde_with].
///
/// [serde_with]: https://crates.io/crates/serde_with/
pub trait Serializable<T: Serialize, const N: usize> {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;
}

impl<T: Serialize, const N: usize, const M: usize> Serializable<T, N> for [[T; N]; M] {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Fixed-length structures, including arrays, are supported in Serde as tuples
        // See: https://serde.rs/impl-serialize.html#serializing-a-tuple
        let mut s = ser.serialize_tuple(N)?;
        for item in self {
            let wrapped = ArrayWrap::new(item);
            s.serialize_element(&wrapped)?;
        }
        s.end()
    }
}

impl<T: Serialize, const N: usize> Serializable<T, N> for Vec<[T; N]> {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = ser.serialize_seq(Some(self.len()))?;
        for item in self {
            let wrapped = ArrayWrap::new(item);
            s.serialize_element(&wrapped)?;
        }
        s.end()
    }
}

impl<T: Serialize, const N: usize> Serializable<T, N> for [T; N] {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serialize_as_tuple(self, ser)
    }
}

/// Serialize an array
///
/// In Serde arrays (and other fixed-length structures) are supported as tuples
fn serialize_as_tuple<S, T, const N: usize>(data: &[T; N], ser: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    // See: https://serde.rs/impl-serialize.html#serializing-a-tuple
    let mut s = ser.serialize_tuple(N)?;
    for item in data {
        s.serialize_element(item)?;
    }
    s.end()
}
