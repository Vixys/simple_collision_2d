use crate::point::Point;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Circle {
    pub center: Point,
    pub radius: f32,
}

impl Default for Circle {
    fn default() -> Self {
        Self {
            center: Default::default(),
            radius: 1.0,
        }
    }
}
