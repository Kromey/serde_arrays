// Copyright 2021 Travis Veazey
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// https://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Serialize and deserialize const generic or arbitrarily-large arrays with [Serde].
//!
//! Out of the box, Serde supports [a lot of types](https://serde.rs/data-model.html#types), but
//! unfortunately lacks support for arrays that use const generics. This library provides a module
//! that, in combination with Serde's [`with`](https://serde.rs/field-attrs.html#with) attribute,
//! adds that support.
//!
//! # Example usage
//!
//! ```
//! use serde::{Serialize, Deserialize};
//! use serde_json;
//!
//! #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
//! struct GenericArray<const N: usize> {
//!     #[serde(with = "serde_arrays")]
//!     arr: [u32; N],
//! }
//!
//! let data = GenericArray{ arr: [1; 16] };
//! let json = serde_json::to_string(&data)?;
//! let de_data = serde_json::from_str(&json)?;
//!
//! assert_eq!(data, de_data);
//! # Ok::<(), serde_json::Error>(())
//! ```
//!
//! As an added bonus, this also adds support for arbitrarily large arrays beyond the 32 elements
//! that Serde supports:
//!
//! ```
//! # use serde::{Serialize, Deserialize};
//! # use serde_json;
//! #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
//! struct LargeArray {
//!     #[serde(with = "serde_arrays")]
//!     arr: [u32; 64],
//! }
//! # let data = LargeArray{ arr: [1; 64] };
//! # let json = serde_json::to_string(&data)?;
//! # let de_data = serde_json::from_str(&json)?;
//! # assert_eq!(data, de_data);
//! # Ok::<(), serde_json::Error>(())
//! ```
//!
//! Tuple structs are supported just as easily:
//!
//! ```
//! # use serde::{Serialize, Deserialize};
//! # use serde_json;
//! #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
//! struct TupleStruct<const N: usize>(
//!     #[serde(with = "serde_arrays")]
//!     [u32; N],
//! );
//! # let data = TupleStruct([1; 64]);
//! # let json = serde_json::to_string(&data)?;
//! # let de_data = serde_json::from_str(&json)?;
//! # assert_eq!(data, de_data);
//! # Ok::<(), serde_json::Error>(())
//! ```
//!
//! # MSRV
//!
//! This library relies on the const generics feature introduced in Rust 1.51.0.
//!
//! # Relevant links
//!
//!  * The [Serde issue](https://github.com/serde-rs/serde/issues/1937) for const generics support
//!  * [serde-big-array](https://crates.io/crates/serde-big-array) is a similar crate, but it
//!    depends on `unsafe` code (whether its use of such is safe or not is beyond this scope)
//!
//! [Serde]: https://serde.rs/

use serde::{
    de::{self, Deserialize, Deserializer, SeqAccess, Visitor},
    ser::{Serialize, SerializeTuple, Serializer},
};
use std::{convert::TryInto, fmt, marker::PhantomData};

/// Serialize const generic or arbitrarily-large arrays
///
/// For any array up to length `usize::MAX`, this function will allow Serde to properly serialize
/// it, provided of course that the type `T` is itself serializable.
///
/// This implementation is adapted from the [Serde documentataion][serialize_map]
///
/// [serialize_map]: https://serde.rs/impl-serialize.html#serializing-a-sequence-or-map
pub fn serialize<S, T, const N: usize>(data: &[T; N], ser: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    // Fixed-length structures, including arrays, are supported in Serde as tuples
    // See: https://serde.rs/impl-serialize.html#serializing-a-tuple
    let mut s = ser.serialize_tuple(N)?;
    for item in data {
        s.serialize_element(item)?;
    }
    s.end()
}

/// A Serde Deserializer `Visitor` for [T; N] arrays
struct ArrayVisitor<T, const N: usize> {
    // Literally nothing (a "phantom"), but stops Rust complaining about the "unused" T parameter
    _marker: PhantomData<T>,
}

impl<'de, T, const N: usize> Visitor<'de> for ArrayVisitor<T, N>
where
    T: Deserialize<'de>,
{
    type Value = [T; N];

    /// Format a message stating we expect an array of size `N`
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "an array of size {}", N)
    }

    /// Process a sequence into an array
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        // Build a temporary container to hold our data as we deserialize it
        // We can't rely on a Default<T> implementation, so we can't use an array here
        let mut arr = Vec::with_capacity(N);

        while let Some(val) = seq.next_element()? {
            arr.push(val);
        }

        // We can convert a Vec into an array via TryInto, which will fail if the length of the Vec
        // doesn't match that of the array.
        match arr.try_into() {
            Ok(arr) => Ok(arr),
            Err(arr) => Err(de::Error::invalid_length(arr.len(), &self)),
        }
    }
}

/// Deserialize const generic or arbitrarily-large arrays
///
/// For any array up to length `usize::MAX`, this function will allow Serde to properly deserialize
/// it, provided the type `T` itself is deserializable.
///
/// This implementation is adapted from the [Serde documentation][deserialize_map].
///
/// [deserialize_map]: https://serde.rs/deserialize-map.html
pub fn deserialize<'de, D, T, const N: usize>(deserialize: D) -> Result<[T; N], D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    deserialize.deserialize_tuple(
        N,
        ArrayVisitor {
            _marker: PhantomData,
        },
    )
}

/// Hacky way to include README in doc-tests, but works until #[doc(include...)] is stabilized
/// https://github.com/rust-lang/cargo/issues/383#issuecomment-720873790
#[cfg(doctest)]
mod test_readme {
    macro_rules! external_doc_test {
        ($x:expr) => {
            #[doc = $x]
            extern "C" {}
        };
    }

    external_doc_test!(include_str!("../README.md"));
}
