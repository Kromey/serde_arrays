// Copyright 2021 Travis Veazey
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// https://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

mod common;
use common::nested::*;

#[test]
fn serialize_nested_array() {
    let nested = NestedArray { arr: [[1; 3]; 2] };
    let generic = GenericNestedArray { arr: [[1; 3]; 2] };
    let vecced = VecArray {
        arr: vec![[1; 3]; 2],
    };

    let j_nested = serde_json::to_string(&nested).unwrap();
    let j_generic = serde_json::to_string(&generic).unwrap();
    let j_vecced = serde_json::to_string(&vecced).unwrap();

    let json = "{\"arr\":[[1,1,1],[1,1,1]]}";
    assert_eq!(json, &j_nested);
    assert_eq!(json, &j_generic);
    assert_eq!(json, &j_vecced);
}

#[test]
fn serialize_flat_as_nested() {
    let flat = FlatArray { arr: [1; 3] };
    let j_flat = serde_json::to_string(&flat).unwrap();
    assert_eq!("{\"arr\":[1,1,1]}", &j_flat);
}
