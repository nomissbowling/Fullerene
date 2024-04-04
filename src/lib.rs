#![doc(html_root_url = "https://docs.rs/Fullerene/0.1.1")]
//! Fullerene on the Open Dynamics Engine ( ODE / OYK ) for Rust
//!

pub mod c60;
pub mod dodecahedron;
pub mod icosahedron;

pub use c60::{C60, C60Center};
pub use dodecahedron::{Dodecahedron, DodecahedronCenter};
pub use icosahedron::Icosahedron;

use num::Float;

/// sum of vec [F; 3] without trait Sum
/// when use += need trait Float + std::ops::AddAssign &lt; [F; 3] &gt;
pub fn sum_f3<F: Float>(vs: &Vec<[F; 3]>) -> Vec<F> {
  vs.iter().fold(vec![<F>::from(0).unwrap(); 3], |s, p|
    s.iter().zip(p.iter()).map(|(&q, &p)| q + p).collect())
}

/// avg of vec [F; 3]
pub fn avg_f3<F: Float>(vs: &Vec<[F; 3]>) -> Vec<F> {
  let n = <F>::from(vs.len()).unwrap();
  sum_f3(vs).iter().map(|&v| v / n).collect()
}

/// center indexed
pub fn center_indexed<F: Float>(idx: &[u8], vtx: &Vec<[F; 3]>) -> [F; 3] {
  let p = avg_f3(&idx.iter().map(|&i| vtx[i as usize]).collect());
  p.as_slice().try_into().unwrap()
}

/// divide internally
pub fn divide_int<F: Float>(p: &[F; 3], q: &[F; 3], m: i32, n: i32) -> [F; 3] {
  let mf = <F>::from(m).unwrap();
  let nf = <F>::from(n).unwrap();
  let sf = mf + nf;
  p.iter().zip(q.iter()).map(|(&a, &b)|
    (nf * a + mf * b) / sf).collect::<Vec<_>>().as_slice().try_into().unwrap()
}

/// divide externally
pub fn divide_ext<F: Float>(p: &[F; 3], q: &[F; 3], m: i32, n: i32) -> [F; 3] {
  divide_int(p, q, m, -n)
}

/// tests
#[cfg(test)]
mod tests {
  use super::*;

  /// [-- --nocapture] [-- --show-output]
  #[test]
  fn test_icosahedron() {
    let icosa32 = Icosahedron::new(1.0f32);
    assert_eq!(icosa32.tri.len(), 20);
    assert_eq!(icosa32.vtx.len(), 12);
    let s32 = format!("{:?}", icosa32.vtx[0]);
    println!("{}", s32);
    assert_eq!(s32, "[0.0, -1.0, -1.618034]");

    let icosa64 = Icosahedron::new(1.0f64);
    assert_eq!(icosa64.tri.len(), 20);
    assert_eq!(icosa64.vtx.len(), 12);
    let s64 = format!("{:?}", icosa64.vtx[0]);
    println!("{}", s64);
    assert_eq!(s64, "[0.0, -1.0, -1.618033988749895]");
  }

  #[test]
  fn test_dodecahedron() {
    let dodeca32 = Dodecahedron::<f32>::new(1.0f32);
    assert_eq!(dodeca32.tri.len(), 12); // 12(pentagon) x 3
    assert_eq!(dodeca32.vtx.len(), 20);
    let dodeca64 = Dodecahedron::<f64>::new(1.0f64);
    assert_eq!(dodeca64.tri.len(), 12); // 12(pentagon) x 3
    assert_eq!(dodeca64.vtx.len(), 20);
  }

  #[test]
  fn test_dodecahedron_center() {
    let dodeca32_center = DodecahedronCenter::new(1.0f32);
    assert_eq!(dodeca32_center.tri.len(), 12); // 12 x 5
    assert_eq!(dodeca32_center.vtx.len(), 32); // 20 + 12
    let dodeca64_center = DodecahedronCenter::new(1.0f64);
    assert_eq!(dodeca64_center.tri.len(), 12); // 12 x 5
    assert_eq!(dodeca64_center.vtx.len(), 32); // 20 + 12

    let mt = [
  [[20, 0, 4], [20, 4, 3], [20, 3, 2], [20, 2, 1], [20, 1, 0]], // 0
  [[21, 4, 0], [21, 0, 7], [21, 7, 6], [21, 6, 5], [21, 5, 4]], // 4
  [[22, 0, 1], [22, 1, 9], [22, 9, 8], [22, 8, 7], [22, 7, 0]], // 8
  [[23, 1, 2], [23, 2, 11], [23, 11, 10], [23, 10, 9], [23, 9, 1]], // 9
  [[24, 2, 3], [24, 3, 13], [24, 13, 12], [24, 12, 11], [24, 11, 2]], // 7
  [[25, 3, 4], [25, 4, 5], [25, 5, 14], [25, 14, 13], [25, 13, 3]], // 1
  [[26, 15, 14], [26, 14, 5], [26, 5, 6], [26, 6, 16], [26, 16, 15]], // 11
  [[27, 16, 6], [27, 6, 7], [27, 7, 8], [27, 8, 17], [27, 17, 16]], // 5
  [[28, 17, 8], [28, 8, 9], [28, 9, 10], [28, 10, 18], [28, 18, 17]], // 3
  [[29, 18, 10], [29, 10, 11], [29, 11, 12], [29, 12, 19], [29, 19, 18]], // 6
  [[30, 19, 12], [30, 12, 13], [30, 13, 14], [30, 14, 15], [30, 15, 19]], // 10
  [[31, 19, 15], [31, 15, 16], [31, 16, 17], [31, 17, 18], [31, 18, 19]] // 2
    ];
    assert_eq!(dodeca32_center.tri, mt);
    assert_eq!(dodeca64_center.tri, mt);
  }

  #[test]
  fn test_fullerene() {
    let c60_32 = C60::<f32>::new(1.0f32);
    assert_eq!(c60_32.tri.len(), 32); // 12(pentagon) x 3 + 20(hexagon) x 4
    assert_eq!(c60_32.vtx.len(), 60);
    let c60_64 = C60::<f64>::new(1.0f64);
    assert_eq!(c60_64.tri.len(), 32); // 12(pentagon) x 3 + 20(hexagon) x 4
    assert_eq!(c60_64.vtx.len(), 60);
  }

  #[test]
  fn test_fullerene_center() {
    let c60_32_center = C60Center::new(1.0f32);
    assert_eq!(c60_32_center.tri.len(), 32); // 12 x 5 + 20 x 6
    assert_eq!(c60_32_center.vtx.len(), 92); // 60 + 32
    let c60_64_center = C60Center::new(1.0f64);
    assert_eq!(c60_64_center.tri.len(), 32); // 12 x 5 + 20 x 6
    assert_eq!(c60_64_center.vtx.len(), 92); // 60 + 32
  }
}
