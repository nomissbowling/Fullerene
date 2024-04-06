//! dodecahedron.rs
//!
//! Dodecahedron
//! vertices 20
//! 5 x 12 faces
//! edges 30
//!

use num::Float;

use crate::{TUV, center_indexed, divide_int};
use crate::icosahedron::Icosahedron;

/// Dodecahedron
#[derive(Debug)]
pub struct Dodecahedron<F: Float> {
  /// vtx
  pub vtx: Vec<[F; 3]>,
  /// edges (duplex)
  pub edges: Vec<(u8, [u8; 3])>,
  /// tri: Vec 12 of Vec 3 indexed triangles
  pub tri: Vec<Vec<[u8; 3]>>
}

/// impl trait TUV for Dodecahedron
impl<F: Float> TUV<F> for Dodecahedron<F> {
  /// get uv from the one texture (f v i: vertex id of expanded polyhedron)
  fn get_uv_t(&self, _f: usize, _v: usize, _i: usize) -> [F; 2] {
    [<F>::from(0.0).unwrap(), <F>::from(0.0).unwrap()]
  }
  /// ref vtx
  fn ref_vtx(&self) -> &Vec<[F; 3]> { &self.vtx }
  /// ref tri
  fn ref_tri(&self) -> &Vec<Vec<[u8; 3]>> { &self.tri }
}

/// Dodecahedron
impl<F: Float> Dodecahedron<F> {
  /// construct
  pub fn new(r: F) -> Self {
    let icosa = Icosahedron::<F>::new(r);
    let vtx: Vec<_> = icosa.tri.iter().map(|t|
      center_indexed(&t[0], &icosa.vtx)).collect(); // 0-19
    let edges = vec![];
    let tri = vec![
  vec![[0, 4, 3], [0, 3, 2], [0, 2, 1]], // 0 [0, [4, 3, 2, 1]]
  vec![[4, 0, 7], [4, 7, 6], [4, 6, 5]], // 4 [4, [0, 7, 6, 5]]
  vec![[0, 1, 9], [0, 9, 8], [0, 8, 7]], // 8 [0, [1, 9, 8, 7]]
  vec![[1, 2, 11], [1, 11, 10], [1, 10, 9]], // 9 [1, [2, 11, 10, 9]]
  vec![[2, 3, 13], [2, 13, 12], [2, 12, 11]], // 7 [2, [3, 13, 12, 11]]
  vec![[3, 4, 5], [3, 5, 14], [3, 14, 13]], // 1 [3, [4, 5, 14, 13]]
  vec![[15, 14, 5], [15, 5, 6], [15, 6, 16]], // 11 [15, [14, 5, 6, 16]]
  vec![[16, 6, 7], [16, 7, 8], [16, 8, 17]], // 5 [16, [6, 7, 8, 17]]
  vec![[17, 8, 9], [17, 9, 10], [17, 10, 18]], // 3 [17, [8, 9, 10, 18]]
  vec![[18, 10, 11], [18, 11, 12], [18, 12, 19]], // 6 [18, [10, 11, 12, 19]]
  vec![[19, 12, 13], [19, 13, 14], [19, 14, 15]], // 10 [19, [12, 13, 14, 15]]
  vec![[19, 15, 16], [19, 16, 17], [19, 17, 18]] // 2 [19, [15, 16, 17, 18]]
    ];
    Dodecahedron{vtx, edges, tri}
  }
}

/// Dodecahedron with center
#[derive(Debug)]
pub struct DodecahedronCenter<F: Float> {
  /// vtx
  pub vtx: Vec<[F; 3]>,
  /// edges (duplex)
  pub edges: Vec<(u8, [u8; 3])>,
  /// tri: Vec 12 of Vec 5 indexed triangles
  pub tri: Vec<Vec<[u8; 3]>>
}

/// impl trait TUV for DodecahedronCenter
impl<F: Float> TUV<F> for DodecahedronCenter<F> {
  /// get uv from the one texture (f v i: vertex id of expanded polyhedron)
  fn get_uv_t(&self, _f: usize, _v: usize, _i: usize) -> [F; 2] {
    [<F>::from(0.0).unwrap(), <F>::from(0.0).unwrap()]
  }
  /// ref vtx
  fn ref_vtx(&self) -> &Vec<[F; 3]> { &self.vtx }
  /// ref tri
  fn ref_tri(&self) -> &Vec<Vec<[u8; 3]>> { &self.tri }
}

/// Dodecahedron with center
impl<F: Float> DodecahedronCenter<F> {
  /// construct
  pub fn new(r: F) -> Self {
    let icosa = Icosahedron::<F>::new(r);
    let iv = &icosa.vtx;
/*
    let mut vtx: Vec<_> = icosa.edges.iter().map(|&(e, is)|
      divide_int(&iv[e as usize], &center_indexed(&is, &iv), 2, 1)).collect();
    let dodeca = Dodecahedron::<F>::new();
    vtx.extend(dodeca.vtx.iter().map(|&p| p));
*/
    let dodeca = Dodecahedron::<F>::new(r);
    let mut vtx: Vec<_> = dodeca.vtx.iter().map(|&p| p).collect(); // 0-19
    vtx.extend(icosa.edges.iter().map(|&(e, is)|
      divide_int(&iv[e as usize], &center_indexed(&is, &iv), 2, 1))); // 20-31
    let edges = vec![];
    let tri: Vec<_> = (20..32).into_iter().map(|q: u8| {
      let mut t = Vec::with_capacity(5); // vec![[0u8; 3]; 5];
      let o = q as usize - 20;
      t.push([q, dodeca.tri[o][0][0], dodeca.tri[o][0][1]]);
      let _: Vec<_> = (0..3).into_iter().map(|p| {
        t.push([q, dodeca.tri[o][p][1], dodeca.tri[o][p][2]]);
      }).collect();
      t.push([q, dodeca.tri[o][2][2], dodeca.tri[o][2][0]]);
      t
    }).collect();
/*
    let tri: Vec<_> = (20..32).into_iter().flat_map(|q: u8| {
      let mut t = Vec::with_capacity(5); // vec![[0u8; 3]; 5];
      let o = (q as usize - 20) * 3;
      t.push([q, dodeca.tri[o][0], dodeca.tri[o][1]]);
      let _: Vec<_> = (0..3).into_iter().map(|p| {
        t.push([q, dodeca.tri[o + p][1], dodeca.tri[o + p][2]]);
      }).collect();
      t.push([q, dodeca.tri[o + 2][2], dodeca.tri[o + 2][0]]);
      t
    }).collect();
*/
    // println!("{:?}", tri);
    DodecahedronCenter{vtx, edges, tri}
  }
}
