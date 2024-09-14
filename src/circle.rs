use crate::point::Point;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Circle {
    pub center: Point,
    pub radius: f32,
}
