// Copyright 2021 Travis Veazey
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// https://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

mod common;
use common::*;

#[test]
fn serialize_generic_array() {
    let obj = GenericArray::<16> { arr: [1; 16] };

    let j = serde_json::to_string(&obj).unwrap();

    assert_eq!("{\"arr\":[1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]}", &j);
}

#[test]
fn serialize_fixed_size_array() {
    let obj = FixedArray { arr: [1; 36] };

    let j = serde_json::to_string(&obj).unwrap();

    assert_eq!(
        "{\"arr\":[1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]}",
        &j
    );
}

#[test]
fn serialize_tuple_struct_with_generic_array() {
    let obj = GenericTupleStruct::<16>([1; 16]);

    let j = serde_json::to_string(&obj).unwrap();

    assert_eq!("[1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]", &j);
}
