# vecmat

[![Crates.io][crates_badge]][crates]
[![Docs.rs][docs_badge]][docs]
[![Travis CI][travis_badge]][travis]
[![Appveyor][appveyor_badge]][appveyor]
[![Codecov.io][codecov_badge]][codecov]
[![License][license_badge]][license]

[crates_badge]: https://img.shields.io/crates/v/vecmat.svg
[docs_badge]: https://docs.rs/vecmat/badge.svg
[travis_badge]: https://api.travis-ci.org/agerasev/vecmat-rs.svg?branch=master
[appveyor_badge]: https://ci.appveyor.com/api/projects/status/e43qp5a1alb9ilcp/branch/master?svg=true
[codecov_badge]: https://codecov.io/gh/agerasev/vecmat-rs/graphs/badge.svg
[license_badge]: https://img.shields.io/crates/l/vecmat.svg

[crates]: https://crates.io/crates/vecmat
[docs]: https://docs.rs/vecmat
[travis]: https://travis-ci.org/agerasev/vecmat-rs
[appveyor]: https://ci.appveyor.com/project/agerasev/vecmat-rs
[codecov]: https://codecov.io/gh/agerasev/vecmat-rs
[license]: #license

Low-dimensional vector algebra with `min_const_generics` support.

*Rust 1.51 or higher requred because of `min_const_generics` feature.*

## Cargo features

+ `std` - use std. Crate could be used with `no_std`.
+ `rand` - distributions for generating random entities.
+ `approx` - approximate comparison.

All these features are enabled by default.

## [Documentation](https://docs.rs/vecmat)

## Content

### Primitives

+ `Vector`.
+ `Matrix`.
+ `Complex` and `Quaternion`.

### Transformations

+ `Shift`.
+ `Linear`.
+ `Affine`.
+ `Rotation2` and `Rotation3`.
+ `Moebius` (over `Complex` and `Quaternion`).

## Functionality

### Implemented

+ `min_const_generics` support.
+ Vector and matrix arithmetcs (`+`, `-`, `*`, `/`, `%`).
+ Integer vectors and matrices (including `div_floor`, `mod_floor` and bitwise).
+ Boolean vectors and matrices (comparison, `all`, `any`).
+ Support for non-`Copy` (and non-`Clone`) elements.
+ `into_iter()` for vectors (and `map`, `zip`, `unzip`, `fold`, `scan`, etc.).
+ `dot`, `cross` and `outer` products for vectors.
+ Matrix-matrix and matrix-vector multiplication.
+ Square matrix determinant and inversion.

### Planning

+ Eigen- and singular decomposition for matrices.
+ `Rotation4`.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
