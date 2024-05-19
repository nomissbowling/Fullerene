#![doc(html_root_url = "https://docs.rs/Fullerene/0.4.1")]
//! Fullerene on the ODE (Open Dynamics Engine) for Rust
//!

pub mod c60;
pub mod dodecahedron;
pub mod icosahedron;

pub use c60::{C60, C60Center};
pub use dodecahedron::{Dodecahedron, DodecahedronCenter};
pub use icosahedron::Icosahedron;

pub use ph_faces::{self, f_to_f32, f_to_f64, polyhedron::{self, PHF, TUV}};

/// tests
#[cfg(test)]
mod tests {
  use super::*;

  /// [-- --nocapture] [-- --show-output]
  #[test]
  fn test_icosahedron() {
    let icosa32_e = Icosahedron::new(1.0f32);
    let icosa32 = icosa32_e.ph;
    assert_eq!(icosa32.tri.len(), 20);
    assert_eq!(icosa32.vtx.len(), 12);
    let s32 = format!("{:?}", icosa32.vtx[0]);
    println!("{}", s32);
    assert_eq!(s32, "[0.0, -1.0, -1.618034]");

    let icosa64_e = Icosahedron::new(1.0f64);
    let icosa64 = icosa64_e.ph;
    assert_eq!(icosa64.tri.len(), 20);
    assert_eq!(icosa64.vtx.len(), 12);
    let s64 = format!("{:?}", icosa64.vtx[0]);
    println!("{}", s64);
    assert_eq!(s64, "[0.0, -1.0, -1.618033988749895]");
  }

  #[test]
  fn test_dodecahedron() {
    let dodeca32_e = Dodecahedron::<f32>::new(1.0f32);
    let dodeca32 = dodeca32_e.ph;
    assert_eq!(dodeca32.tri.len(), 12); // 12(pentagon) x 3
    assert_eq!(dodeca32.vtx.len(), 20);
    let dodeca64_e = Dodecahedron::<f64>::new(1.0f64);
    let dodeca64 = dodeca64_e.ph;
    assert_eq!(dodeca64.tri.len(), 12); // 12(pentagon) x 3
    assert_eq!(dodeca64.vtx.len(), 20);
  }

  #[test]
  fn test_dodecahedron_center() {
    let dodeca32_center_e = DodecahedronCenter::new(1.0f32);
    let dodeca32_center = dodeca32_center_e.ph;
    assert_eq!(dodeca32_center.tri.len(), 12); // 12 x 5
    assert_eq!(dodeca32_center.vtx.len(), 32); // 20 + 12
    let dodeca64_center_e = DodecahedronCenter::new(1.0f64);
    let dodeca64_center = dodeca64_center_e.ph;
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
    let c60_32_e = C60::<f32>::new(1.0f32);
    let c60_32 = c60_32_e.ph;
    assert_eq!(c60_32.tri.len(), 32); // 12(pentagon) x 3 + 20(hexagon) x 4
    assert_eq!(c60_32.vtx.len(), 60);
    let c60_64_e = C60::<f64>::new(1.0f64);
    let c60_64 = c60_64_e.ph;
    assert_eq!(c60_64.tri.len(), 32); // 12(pentagon) x 3 + 20(hexagon) x 4
    assert_eq!(c60_64.vtx.len(), 60);
  }

  #[test]
  fn test_fullerene_center() {
    let c60_32_center_e = C60Center::new(1.0f32);
    let c60_32_center = c60_32_center_e.ph;
    assert_eq!(c60_32_center.tri.len(), 32); // 12 x 5 + 20 x 6
    assert_eq!(c60_32_center.vtx.len(), 92); // 60 + 32
    let c60_64_center_e = C60Center::new(1.0f64);
    let c60_64_center = c60_64_center_e.ph;
    assert_eq!(c60_64_center.tri.len(), 32); // 12 x 5 + 20 x 6
    assert_eq!(c60_64_center.vtx.len(), 92); // 60 + 32
  }
}
