# vecmat

[![Build Status](https://travis-ci.org/nthend/vecmat-rs.png?branch=master)](https://travis-ci.org/nthend/vecmat-rs)
[![Version](https://img.shields.io/crates/v/vecmat.svg)](https://crates.io/crates/vecmat)

Low-dimensional vector and matrix structures and common operations for them

## Documentation

+ [Master branch documentation](https://nthend.github.io/vecmat-rs/target/doc/vecmat/)

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
    println!("{}", (va - vb).abs());

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

+ Vectors: `VecN<T>` where `N` is 2, 3 or 4
+ Matrices: `MatNxM<T>` (or `MatN<T>` for square matrices) where `N` and `M` are 2, 3 or 4
+ Transformations: `map::AffineN<T>` - affine transformation for vectors

## Features:

- [x] vector and matrix arithmetcs (component-wise `+`, `-`, `*`, `/`)
- [x] integer vector arithmetics (`div_floor` and `mod_floor` for integer vectors)
- [x] boolean vectors (vector comparison, `all`, `any`)
- [x] `dot`, `cross` and `outer` products for vectors
- [x] matrix-matrix and matrix-vector multiplication
- [x] square matrix `det` and `inverse`
- [x] affine transformations for vector (with `inverse`)

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
