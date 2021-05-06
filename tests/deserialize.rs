// Copyright 2021 Travis Veazey
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// https://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

mod common;
use common::*;

#[test]
fn deserialize_generic_array() {
    let obj: GenericArray<16> =
        serde_json::from_str("{\"arr\":[1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]}").unwrap();

    assert_eq!(GenericArray::<16> { arr: [1; 16] }, obj);
}

#[test]
#[should_panic(expected = "expected an array of size 16")]
fn deserialize_generic_array_with_invalid_input() {
    // JSON data with an insufficient length to the array
    let _: GenericArray<16> =
        serde_json::from_str("{\"arr\":[1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]}").unwrap();
}

#[test]
fn deserialize_fixed_size_array() {
    let obj: FixedArray = serde_json::from_str(
        "{\"arr\":[1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]}",
    )
    .unwrap();

    assert_eq!(FixedArray { arr: [1; 36] }, obj);
}

#[test]
#[should_panic(expected = "expected an array of size 36")]
fn deserialize_fixed_size_array_with_invalid_input() {
    // JSON data with an insufficient length to the array
    let obj: FixedArray = serde_json::from_str(
        "{\"arr\":[1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]}",
    )
    .unwrap();

    assert_eq!(FixedArray { arr: [1; 36] }, obj);
}

#[test]
fn deserialize_tuple_struct_with_generic_array() {
    let obj: GenericTupleStruct<16> =
        serde_json::from_str("[1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]").unwrap();

    assert_eq!(GenericTupleStruct::<16>([1; 16]), obj);
}
