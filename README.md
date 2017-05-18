# vecmat
[![Build Status](https://travis-ci.org/nthend/vecmat-rs.png?branch=master)](https://travis-ci.org/nthend/vecmat-rs)

Lightweight low-dimensional vector and matrix library written in Rust

## [Documentation](https://nthend.github.io/vecmat-rs/target/doc/vecmat/)

## Structs:
+ Vectors: `VecN<T>` where `N` is {2, 3, 4}
+ Matrices: `MatNxM<T>` (or `MatN<T>` for square matrices) where `N` and `M` are {2, 3, 4}

## Features:
- [x] vector and matrix arithmetcs
- [x] boolean vector logics
- [x] vector dot, cross and outer products 
- [x] matrix-matrix and matrix-vector products
- [x] square matrix determinant and inversion
- [x] `div_floor` and `mod_floor` for integer vectors

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
