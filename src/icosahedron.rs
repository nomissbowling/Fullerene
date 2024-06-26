//! icosahedron.rs
//!
//! Icosahedron
//! vertices (3 x 4) = 12
//! 3 x 20 faces (vertices 60 / 5 = 12)
//! edges 20 x 3 / 2 = 30
//!

// use num::{Float, zero, one};
use num::Float;

use ph_faces::{Polyhedron, calc_cg_with_volume};

/// Icosahedron
#[derive(Debug)]
pub struct Icosahedron<F: Float> {
  /// polyhedron tri: Vec 20 of Vec 1 indexed triangles
  pub ph: Polyhedron<F>,
  /// edges (duplex)
  pub edges: Vec<(u16, [u16; 5])>
}

/// Icosahedron
impl<F: Float + std::fmt::Debug> Icosahedron<F> where F: std::iter::Sum {
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
    let mut ph = Polyhedron{
    vtx: vec![
      [o, -i, -p], [o, i, -p], [o, i, p], [o, -i, p],
      [-p, o, -i], [-p, o, i], [p, o, i], [p, o, -i],
      [-i, -p, o], [i, -p, o], [i, p, o], [-i, p, o]
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
      vec![[0, 8, 4]],
      vec![[0, 9, 8]],
      vec![[0, 7, 9]],
      vec![[0, 1, 7]],
      vec![[4, 1, 0]],
      vec![[4, 11, 1]],
      vec![[4, 5, 11]],
      vec![[8, 5, 4]],
      vec![[8, 3, 5]],
      vec![[8, 9, 3]],
      vec![[6, 3, 9]],
      vec![[6, 9, 7]],
      vec![[6, 7, 10]],
      vec![[10, 7, 1]],
      vec![[10, 1, 11]],
      vec![[10, 11, 2]],
      vec![[2, 11, 5]],
      vec![[2, 5, 3]],
      vec![[2, 3, 6]],
      vec![[2, 6, 10]]
    ],
    uv: vec![],
    vol: o,
    center: false
    };
    let q = <F>::from(1e-6).unwrap();
    let (_cg, vol) = calc_cg_with_volume(&ph.tri, &ph.vtx, q);
    ph.vol = vol;
    let edges = vec![ // (duplex)
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
    ];
    Icosahedron{ph, edges}
  }
}
