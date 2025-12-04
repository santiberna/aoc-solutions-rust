use nalgebra::Vector2;

use crate::utility::matrix::MatrixVec;
pub type IVec2 = Vector2<i64>;

pub const ALL: [IVec2; 8] = [
    IVec2::new(0, 1),
    IVec2::new(0, -1),
    IVec2::new(-1, 0),
    IVec2::new(1, 0),
    IVec2::new(1, 1),
    IVec2::new(-1, -1),
    IVec2::new(-1, 1),
    IVec2::new(1, -1),
];

pub const ORTHOGONAL: [IVec2; 4] = [
    IVec2::new(0, 1),
    IVec2::new(0, -1),
    IVec2::new(-1, 0),
    IVec2::new(1, 0),
];

impl<T> MatrixVec<T> {
    pub fn get_many(&self, x: usize, y: usize, offsets: &[IVec2]) -> Vec<Option<&T>> {
        offsets
            .iter()
            .map(|v| {
                let xi = x as i64 + v.x;
                let yi = y as i64 + v.y;
                self.get(xi.try_into().ok()?, yi.try_into().ok()?)
            })
            .collect()
    }
}
