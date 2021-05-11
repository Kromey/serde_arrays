// Copyright 2021 Travis Veazey
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// https://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Serialize const generic or large arrays nested within arrays or `Vec`s
//! 
//! This module extends the functionality of [`serde_arrays`][crate] to additionally support const generic
//! and large arrays that are nested within const generic or large arrays, or `Vec`s.
//! 
//! ```
//! use serde::{Serialize};
//! use serde_json;
//!
//! #[derive(Serialize, Debug, PartialEq, Eq)]
//! struct NestedArray<const N: usize, const M: usize> {
//!     #[serde(with = "serde_arrays::nested")]
//!     arr: [[u32; N]; M],
//! }
//!
//! let data = NestedArray{ arr: [[1; 16]; 64] };
//! let json = serde_json::to_string(&data)?;
//! # //let de_data = serde_json::from_str(&json)?;
//!
//! # //assert_eq!(data, de_data);
//! # Ok::<(), serde_json::Error>(())
//! ```
//! 

use serde::ser::{Serialize, Serializer, SerializeTuple, SerializeSeq};

struct ArrayWrap<'a, T: Serialize, const N: usize> {
    inner: &'a [T; N],
}

impl<'a, T: Serialize, const N: usize> ArrayWrap<'a, T, N> {
    pub fn new(array: &'a [T; N]) -> ArrayWrap<'a, T, N> {
        ArrayWrap {
            inner: array,
        }
    }
}

impl<'a, T: Serialize, const N: usize> Serialize for ArrayWrap<'a, T, N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        super::serialize(self.inner, serializer)
    }
}

pub trait NestedArray<T: Serialize, const N: usize> {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;
}
impl<T: Serialize, const N: usize, const M: usize> NestedArray<T, N> for [[T; N]; M] {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
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
impl<T: Serialize, const N: usize> NestedArray<T, N> for Vec<[T; N]> {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let mut s = ser.serialize_seq(Some(self.len()))?;
        for item in self {
            let wrapped = ArrayWrap::new(item);
            s.serialize_element(&wrapped)?;
        }
        s.end()
    }
}

pub fn serialize<A, S, T, const N: usize>(data: &A, ser: S) -> Result<S::Ok, S::Error>
where
    A: NestedArray<T, N>,
    S: Serializer,
    T: Serialize,
{
    data.serialize(ser)
}
