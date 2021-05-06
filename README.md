# serde_arrays

A simple module to support serializing and deserializing const generic or arbitrarily-large arrays.

```
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct GenericArray<const N: usize> {
    #[serde(with = "serde_arrays")]
    arr: [u32; N],
}

let data = GenericArray{ arr: [1; 16] };
let json = serde_json::to_string(&data)?;
let de_data = serde_json::from_str(&json)?;

assert_eq!(data, de_data);
# Ok::<(), serde_json::Error>(())
```

## MSRV

The minimum supported Rust version (MSRV) for `serde_arrays` is 1.51.0.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as
above, without any additional terms or conditions.
