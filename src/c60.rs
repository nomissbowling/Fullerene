//! c60.rs
//!
//! Fullerene as Truncated Icosahedron (3 x 4)
//! vertices (3 x 4) x 5 = 60
//! 5 x 12 faces (vertices 60 / 3 = 20)
//! 6 x 20 faces (vertices 120 / 3 = 40)
//!

use num::Float;

use ph_faces::{Polyhedron, center_indexed, divide_int};

use crate::icosahedron::Icosahedron;

/// C60
#[derive(Debug)]
pub struct C60<F: Float> {
  /// polyhedron tri: (Vec 12 of Vec 3) or (Vec 20 of Vec 4) indexed triangles
  pub ph: Polyhedron<F>,
  /// edges (duplex)
  pub edges: Vec<(u8, [u8; 3])>
}

/// C60
impl<F: Float> C60<F> {
  /// construct
  pub fn new(r: F) -> Self {
    let icosa_e = Icosahedron::<F>::new(r); // to keep lifetime
    let icosa = icosa_e.ph;
    let iv = &icosa.vtx;
    let vtx: Vec<_> = icosa_e.edges.iter().flat_map(|&(e, is)|
      is.iter().map(move |&i|
        divide_int(&iv[e as usize], &iv[i as usize], 1, 2)
      ).collect::<Vec<_>>()
    ).collect(); // 0-59
    let edges = vec![];
    let hx = vec![
      [3, 14, 13, 9, 8, 4], // 0
      [2, 16, 15, 10, 14, 3], // 1
      [1, 22, 21, 17, 16, 2], // 2
      [0, 25, 29, 23, 22, 1], // 3
      [4, 8, 7, 26, 25, 0], // 4
      [7, 6, 32, 31, 27, 26], // 5
      [5, 35, 39, 33, 32, 6], // 6
      [13, 12, 36, 35, 5, 9], // 7
      [11, 42, 41, 37, 36, 12], // 8
      [15, 19, 43, 42, 11, 10], // 9
      [18, 49, 48, 44, 43, 19], // 10
      [21, 20, 45, 49, 18, 17], // 11
      [24, 53, 52, 46, 45, 20], // 12
      [29, 28, 54, 53, 24, 23], // 13
      [27, 31, 30, 50, 54, 28], // 14
      [30, 34, 58, 57, 51, 50], // 15
      [39, 38, 59, 58, 34, 33], // 16
      [41, 40, 55, 59, 38, 37], // 17
      [48, 47, 56, 55, 40, 44], // 18
      [52, 51, 57, 56, 47, 46] // 19
    ];
    let mut tri: Vec<_> = (0..12).into_iter().map(|q: u8| {
      let o = q * 5;
      vec![[o, o + 1, o + 2], [o, o + 2, o + 3], [o, o + 3, o + 4]]
    }).collect();
    tri.extend(hx.iter().map(|&h| vec![
      [h[0], h[1], h[2]], [h[0], h[2], h[3]],
      [h[0], h[3], h[4]], [h[0], h[4], h[5]]
    ]));
/*
    let mut tri: Vec<_> = (0..12).into_iter().flat_map(|q: u8| {
      let o = q * 5;
      vec![[o, o + 1, o + 2], [o, o + 2, o + 3], [o, o + 3, o + 4]]
    }).collect();
    tri.extend(hx.iter().flat_map(|&h| vec![
      [h[0], h[1], h[2]], [h[0], h[2], h[3]],
      [h[0], h[3], h[4]], [h[0], h[4], h[5]]
    ]));
*/
/*
    tri.extend((12..32).into_iter().flat_map(|q: u8| vec![
      [0, 1, 2], [0, 2, 3], [0, 3, 4], [0, 4, 5]
    ]));
*/
    C60{ph: Polyhedron{vtx, tri}, edges}
  }
}

/// C60 with center
#[derive(Debug)]
pub struct C60Center<F: Float> {
  /// polyhedron tri: (Vec 12 of Vec 5) or (Vec 20 of Vec 6) indexed triangles
  pub ph: Polyhedron<F>,
  /// edges (duplex)
  pub edges: Vec<(u8, [u8; 3])>
}

/// C60 with center
impl<F: Float> C60Center<F> {
  /// construct
  pub fn new(r: F) -> Self {
    let icosa_e = Icosahedron::<F>::new(r); // to keep lifetime
    let icosa = icosa_e.ph;
    let iv = &icosa.vtx;
    let c60_e = C60::<F>::new(r); // to keep lifetime
    let c60 = c60_e.ph;
    let cv = &c60.vtx;
    let mut vtx: Vec<_> = cv.iter().map(|&p| p).collect(); // 0-59
    vtx.extend(icosa_e.edges.iter().map(|&(e, is)|
      divide_int(&iv[e as usize], &center_indexed(&is, &iv), 1, 2))); // 60-71
    vtx.extend(icosa.tri.iter().map(|t| center_indexed(&t[0], iv))); // 72-91
    let edges = vec![];
    let mut tri: Vec<_> = (60..72).into_iter().map(|q: u8| {
      let mut t = Vec::with_capacity(5); // vec![[0u8; 3]; 5];
      let o = q as usize - 60;
      t.push([q, c60.tri[o][0][0], c60.tri[o][0][1]]);
      let _: Vec<_> = (0..3).into_iter().map(|p| {
        t.push([q, c60.tri[o][p][1], c60.tri[o][p][2]]);
      }).collect();
      t.push([q, c60.tri[o][2][2], c60.tri[o][2][0]]);
      t
    }).collect();
    tri.extend((72..92).into_iter().map(|q: u8| {
      let mut t = Vec::with_capacity(6); // vec![[0u8; 3]; 6];
      let o = q as usize - 72 + 12;
      t.push([q, c60.tri[o][0][0], c60.tri[o][0][1]]);
      let _: Vec<_> = (0..4).into_iter().map(|p| {
        t.push([q, c60.tri[o][p][1], c60.tri[o][p][2]]);
      }).collect();
      t.push([q, c60.tri[o][3][2], c60.tri[o][3][0]]);
      t
    }));
/*
    let mut tri: Vec<_> = (60..72).into_iter().flat_map(|q: u8| {
      let mut t = Vec::with_capacity(5); // vec![[0u8; 3]; 5];
      let o = (q as usize - 60) * 3;
      t.push([q, c60.tri[o][0], c60.tri[o][1]]);
      let _: Vec<_> = (0..3).into_iter().map(|p| {
        t.push([q, c60.tri[o + p][1], c60.tri[o + p][2]]);
      }).collect();
      t.push([q, c60.tri[o + 2][2], c60.tri[o + 2][0]]);
      t
    }).collect();
    tri.extend((72..92).into_iter().flat_map(|q: u8| {
      let mut t = Vec::with_capacity(6); // vec![[0u8; 3]; 6];
      let o = (q as usize - 72) * 4 + 12 * 3;
      t.push([q, c60.tri[o][0], c60.tri[o][1]]);
      let _: Vec<_> = (0..4).into_iter().map(|p| {
        t.push([q, c60.tri[o + p][1], c60.tri[o + p][2]]);
      }).collect();
      t.push([q, c60.tri[o + 3][2], c60.tri[o + 3][0]]);
      t
    }));
*/
    // println!("{:?}", tri);
    C60Center{ph: Polyhedron{vtx, tri}, edges}
  }
}
