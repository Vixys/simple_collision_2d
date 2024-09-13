mod circle_circle;
mod circle_rectangle;
mod point_circle;
mod point_point;
mod point_rectangle;
pub mod rectangle;
mod rectangle_rectangle;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn euclidean_distance(&self, other: &Point) -> f32 {
        let Point { x: x1, y: y1 } = self;
        let Point { x: x2, y: y2 } = other;
        ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Circle {
    pub center: Point,
    pub radius: f32,
}
