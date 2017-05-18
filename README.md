# linalg
[![Build Status](https://travis-ci.org/nthend/linalg-rs.png?branch=refactor)](https://travis-ci.org/nthend/linalg-rs)

Library for low-dimensional linear algebra written in Rust

## Structs
+ Vectors with sizes 2, 3 and 4 -- `vecN` where `N` is 2, 3 or 4;
+ Matrices with sizes 2, 3 and 4 for each dimension -- `matNxM` (or `matN` for square matrices) where `N` and `M` are 2, 3 or 4

## TODO:
- [x] determinant, matrix inversion
- [x] short names for vectors and matrices of bool, i32, f32 and etc.
- [x] `sqr`, `length`, `normalize` vector
- [x] logic ops for boolean vectors
- [x] `div_floor` and `mod_floor` for integer vectors
- [x] component-wise multiplication of two matrices
- [x] map vector or matrix with lambda
- [x] impl Default trait
