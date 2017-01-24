# linalg

Library for low-dimensional linear algebra written in Rust

## Structs
+ Vectors with sizes 2, 3 and 4 -- `vecN` where `N` is 2, 3 or 4;
+ Matrices with sizes 2, 3 and 4 for each dimension -- `matNxM` (or `matN` for square matrices) where `N` and `M` are 2, 3 or 4

All of them are templated by type `T` that should implement `num::Num` trait or should be `bool`

## Unittesting
1. `git checkout unittest`
2. `cargo test`

## TODO:
- [x] determinant, matrix inversion
- [x] short names for vectors and matrices of bool, i32, f32 and etc.
- [ ] `sqr`, `length` of vector
- [ ] component-wise multiplication of 2 matrices
- [ ] map with lambda
- [ ] logic ops for boolean vector
- [ ] component-wise `abs`, `sign` and etc
- [ ] impl Default trait
