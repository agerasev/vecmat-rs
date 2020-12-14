# vecmat

[![Crates.io][crates_badge]][crates]
[![Docs.rs][docs_badge]][docs]
[![Travis CI][travis_badge]][travis]
[![Appveyor][appveyor_badge]][appveyor]
[![Codecov.io][codecov_badge]][codecov]
[![License][license_badge]][license]

[crates_badge]: https://img.shields.io/crates/v/vecmat.svg
[docs_badge]: https://docs.rs/vecmat/badge.svg
[travis_badge]: https://api.travis-ci.org/agerasev/vecmat-rs.svg
[appveyor_badge]: https://ci.appveyor.com/api/projects/status/e43qp5a1alb9ilcp/branch/master?svg=true
[codecov_badge]: https://codecov.io/gh/agerasev/vecmat-rs/graphs/badge.svg
[license_badge]: https://img.shields.io/crates/l/vecmat.svg

[crates]: https://crates.io/crates/vecmat
[docs]: https://docs.rs/vecmat
[travis]: https://travis-ci.org/agerasev/vecmat-rs
[appveyor]: https://ci.appveyor.com/project/agerasev/vecmat-rs
[codecov]: https://codecov.io/gh/agerasev/vecmat-rs
[license]: #license

Low-dimensional vector and matrix structures and common operations for them

# TODO

+ Add reverse multiplication for vectors and matrices

## Documentation

+ [`crates.io` version documentation](https://docs.rs/vecmat)
+ [`master` branch documentation](https://agerasev.github.io/vecmat-rs/target/doc/vecmat/index.html)

## Usage

```rust
extern crate vecmat;

use vecmat::vec::*;
use vecmat::mat::*;

fn main() {
    // Create vectors
    let mut va = Vec3::<f64>::new(); // filled with zeros
    let vb = Vec3::<f64>::from(1.0, 2.0, 3.0); // from values
    println!("{}, {}", va, vb);

    // Vector access
    va[1] = vb[0]; // read and write 
    va[0] += 3.0; // add-assign
    println!("{}", va);

    // Vector products
    println!("{}", 2.0*vb); // scalar-by-vector
    println!("{}", va*vb); // component-wise
    println!("{}", va.dot(vb)); // dot
    println!("{}", va.cross(vb)); // cross

    // Distance between two vectors
    println!("{}", (va - vb).length());

    // Create matrices
    let mut ma = Mat3::<f64>::one(); // identity 3x3 matrix
    let mb = Mat3x2::<f64>::from( // 3x2 matrix from values
        1.0, 2.0, 0.0,
        0.0,-1.0, 1.0,
    );
    println!("{},\n{}", ma, mb);

    // Access matrix components
    ma[(1,1)] = 2.0; // access by (i,j) indices
    ma[(0,2)] = -3.0;
    ma[(2,0)] = -1.0;
    println!("{}", ma);

    // Transpose matrix
    println!("{}", mb.transpose());

    // Matrix-vector product
    println!("{}", mb.dot(vb));
    println!("{}", va.dot(ma));

    // Matrix-matrix product
    println!("{}", mb.dot(ma));

    // Outer product of vectors
    println!("{}", va.outer(vb));

    // Determinant and inverse matrix
    println!("{}", ma.det());
    println!("{}", ma.inverse());
}
```

## Structs:

+ Vectors: `vec::VecN<T>` where `N` is 2, 3 or 4
+ Matrices: `mat::MatNxM<T>` (or `mat::MatN<T>` for square matrices) where `N` and `M` are 2, 3 or 4
+ Transformations: `map::AffineN<T>` - affine transformation for vectors

## Features:

### Linear algebra
- [x] Vector and matrix arithmetcs (component-wise `+`, `-`, `*`, `/`)
- [x] Integer vector arithmetics (`div_floor` and `mod_floor` for integer vectors)
- [x] Boolean vectors (vector comparison, `all`, `any`)
- [x] `dot`, `cross` and `outer` products for vectors
- [x] Matrix-matrix and matrix-vector multiplication
- [x] Square matrix `det` and `inverse`

### Transformations
- [x] Affine transformations for vector (with `inverse`)

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
