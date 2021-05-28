// Copyright 2021 Travis Veazey
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// https://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

mod common;
use common::nested::*;

#[cfg(not(feature = "std"))]
use heapless::Vec;

#[test]
#[cfg(feature = "std")]
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
#[cfg(not(feature = "std"))]
fn serialize_nested_array_no_std() {
    let nested = NestedArray { arr: [[1; 3]; 2] };
    let generic = GenericNestedArray { arr: [[1; 3]; 2] };

    let mut arr = Vec::<[u32; 3], 2>::new();
    arr.push([1; 3]).unwrap();
    arr.push([1; 3]).unwrap();
    let vecced = VecArray { arr };

    let j_nested = serde_json::to_string(&nested).unwrap();
    let j_generic = serde_json::to_string(&generic).unwrap();
    let j_vecced = serde_json::to_string(&vecced).unwrap();

    let json = "{\"arr\":[[1,1,1],[1,1,1]]}";
    assert_eq!(json, &j_nested);
    assert_eq!(json, &j_generic);
    assert_eq!(json, &j_vecced);
}
