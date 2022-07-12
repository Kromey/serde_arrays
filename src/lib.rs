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
//! Even nested arrays are supported:
//!
//! ```
//! # use serde::{Serialize, Deserialize};
//! # use serde_json;
//! #[derive(Serialize, Debug, PartialEq, Eq)]
//! struct NestedArray {
//!     #[serde(with = "serde_arrays")]
//!     arr: [[u32; 64]; 64],
//!     #[serde(with = "serde_arrays")]
//!     vec: Vec<[u32; 96]>,
//! }
//! # let data = NestedArray{ arr: [[1; 64]; 64], vec: vec![[2; 96]; 37], };
//! # let json = serde_json::to_string(&data)?;
//! # //let de_data = serde_json::from_str(&json)?;
//! # //assert_eq!(data, de_data);
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
//!  * [serde-big-array](https://crates.io/crates/serde-big-array) is a similar crate for large
//!    arrays and const generic arrays
//!  * [serde_with](https://crates.io/crates/serde_with/) is a much more flexible and powerful
//!    crate, but with arguably more complex ergonomics
//!
//! [Serde]: https://serde.rs/

use serde::{
    de::{self, Deserialize, Deserializer, SeqAccess, Visitor},
    ser::{Serialize, Serializer},
};
use std::{fmt, marker::PhantomData, mem::MaybeUninit};

#[doc(hidden)]
pub mod serializable;
mod wrapper;
pub use serializable::Serializable;

/// Serialize const generic or arbitrarily-large arrays
///
/// Types must implement the [`Serializable`] trait; while this requirement sharply limits how
/// composable the final result is, the simple ergonomics make up for it.
///
/// For greater flexibility see [`serde_with`][serde_with].
///
/// [serde_with]: https://crates.io/crates/serde_with/
pub fn serialize<A, S, T, const N: usize>(data: &A, ser: S) -> Result<S::Ok, S::Error>
where
    A: Serializable<T, N>,
    S: Serializer,
    T: Serialize,
{
    data.serialize(ser)
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
        // Safety: `assume_init` is sound because the type we are claiming to have
        // initialized here is a bunch of `MaybeUninit`s, which do not require
        // initialization.
        let mut arr: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };

        // Iterate over the array and fill the elemenets with the ones obtained from
        // `seq`.
        let mut place_iter = arr.iter_mut();
        let mut cnt_filled = 0;
        let err = loop {
            match (seq.next_element(), place_iter.next()) {
                (Ok(Some(val)), Some(place)) => *place = MaybeUninit::new(val),
                // no error, we're done
                (Ok(None), None) => break None,
                // error from serde, propagate it
                (Err(e), _) => break Some(e),
                // lengths do not match, report invalid_length
                (Ok(None), Some(_)) | (Ok(Some(_)), None) => {
                    break Some(de::Error::invalid_length(cnt_filled, &self))
                }
            }
            cnt_filled += 1;
        };
        if let Some(err) = err {
            if std::mem::needs_drop::<T>() {
                for elem in std::array::IntoIter::new(arr).take(cnt_filled) {
                    // Safety: `assume_init()` is sound because we did initialize CNT_FILLED
                    // elements. We call it to drop the deserialized values.
                    unsafe {
                        elem.assume_init();
                    }
                }
            }
            return Err(err);
        }

        // Safety: everything is initialized and we are ready to transmute to the
        // initialized array type.

        // See https://github.com/rust-lang/rust/issues/62875#issuecomment-513834029
        //let ret = unsafe { std::mem::transmute::<_, [T; N]>(arr) };

        let ret = unsafe { std::mem::transmute_copy(&arr) };
        std::mem::forget(arr);

        Ok(ret)
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
