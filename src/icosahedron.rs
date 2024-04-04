//! icosahedron.rs
//!
//! Icosahedron
//! vertices (3 x 4) = 12
//! 3 x 20 faces (vertices 60 / 5 = 12)
//! edges 20 x 3 / 2 = 30
//!

// use num::{Float, zero, one};
use num::Float;

/// Icosahedron
#[derive(Debug)]
pub struct Icosahedron<F: Float> {
  /// vtx
  pub vtx: Vec<[F; 3]>,
  /// edges (duplex)
  pub edges: Vec<(u8, [u8; 5])>,
  /// tri: Vec 20 indexed triangles
  pub tri: Vec<[u8; 3]>
}

/// Icosahedron
impl<F: Float> Icosahedron<F> {
  /// construct
  pub fn new(r: F) -> Self {
    let o = <F>::from(0).unwrap(); // = zero(); // = zero::<F>();
    let i = <F>::from(1).unwrap() * r; // = one(); // = one::<F>();
/*
    let d = <F>::from(2).unwrap();
    let r5 = <F>::from(5).unwrap().sqrt() * r;
    let p = (r5 + i) / d; // golden ratio
*/
    let p = <F>::from((5.0.sqrt() + 1.0) / 2.0).unwrap() * r; // golden ratio
    Icosahedron{
    vtx: vec![
      [o, -i, -p], [o, i, -p], [o, i, p], [o, -i, p],
      [-p, o, -i], [-p, o, i], [p, o, i], [p, o, -i],
      [-i, -p, o], [i, -p, o], [i, p, o], [-i, p, o]
    ],
    edges: vec![ // (duplex)
      (0, [1, 7, 9, 8, 4]), // 6 2 11 8 9 (x = 0) 6 + 8 9 9 1 9
      (4, [5, 11, 1, 0, 8]), // (y = 0)
      (8, [9, 3, 5, 4, 0]), // (z = 0)
      (9, [8, 0, 7, 6, 3]), // (z = 0)
      (7, [6, 9, 0, 1, 10]), // (y = 0)
      (1, [0, 4, 11, 10, 7]), // 4 7 11 9 5 (x = 0) 4 + 3 4 10 8 11
      (11, [10, 1, 4, 5, 2]), // (z = 0)
      (5, [4, 8, 3, 2, 11]), // (y = 0)
      (3, [2, 5, 8, 9, 6]), // 3 3 1 9 8 (x = 0) 3 + 0 10 8 11 7
      (6, [7, 10, 2, 3, 9]), // (y = 0)
      (10, [11, 2, 6, 7, 1]), // (z = 0)
      (2, [3, 6, 10, 11, 5]) // 3 4 1 6 10 (x = 0) 3 + 1 9 5 4 5
    ],
    tri: vec![
/*
      [0, 8, 4], // triple x 1
      [0, 9, 8], [4, 1, 0], [8, 5, 4], // double x 3
      [0, 1, 7], [0, 7, 9], // single x 2 (x = 0)
      [4, 5, 11], [4, 11, 1], // single x 2 (y = 0)
      [8, 9, 3], [8, 3, 5], // single x 2 (z = 0)
      [6, 9, 7], [6, 3, 9], // single x 2 (y = 0)
      [10, 1, 11], [10, 7, 1], // single x 2 (z = 0)
      [2, 5, 3], [2, 11, 5], // single x 2 (x = 0)
      [2, 3, 6], [6, 7, 10], [10, 11, 2], // double x 3
      [2, 6, 10] // triple x 1
*/
      [0, 8, 4],
      [0, 9, 8],
      [0, 7, 9],
      [0, 1, 7],
      [4, 1, 0],
      [4, 11, 1],
      [4, 5, 11],
      [8, 5, 4],
      [8, 3, 5],
      [8, 9, 3],
      [6, 3, 9],
      [6, 9, 7],
      [6, 7, 10],
      [10, 7, 1],
      [10, 1, 11],
      [10, 11, 2],
      [2, 11, 5],
      [2, 5, 3],
      [2, 3, 6],
      [2, 6, 10]
    ]
    }
  }
}
