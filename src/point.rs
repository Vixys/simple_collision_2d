use std::ops::{Add, Div, Mul, Sub};

use crate::sat::{project::Sat, projection::Projection};

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

pub type Point = Vector;

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<f32> for Vector {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<f32> for Vector {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Sat for Vector {
    fn project(&self, axis: &Vector) -> Projection {
        let p = axis.dot(self);
        Projection { min: p, max: p }
    }

    fn axes(&self) -> Vec<Vector> {
        Vec::new()
    }

    fn is_curved(&self) -> bool {
        false
    }

    fn get_center(&self) -> Vector {
        unreachable!("This is not supposed to be called")
    }
}

impl Vector {
    pub const ZERO: Self = Self { x: 0., y: 0. };
    pub const ONE: Self = Self { x: 1., y: 1. };

    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn euclidean_distance(&self, other: &Self) -> f32 {
        (*other - *self).magnitude()
    }

    pub fn perp(&self) -> Self {
        Self {
            x: self.y,
            y: -self.x,
        }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let m = self.magnitude();
        if m == 0. {
            Self::ZERO
        } else {
            Self {
                x: self.x / m,
                y: self.y / m,
            }
        }
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euclidean_distance() {
        let p1 = Vector { x: 0., y: 0. };
        let p2 = Vector { x: 5., y: 0. };

        assert_eq!(p1.euclidean_distance(&p2), 5.);
    }

    #[test]
    fn test_euclidean_distance_advanced() {
        let p1 = Vector { x: 0., y: 0. };
        let p2 = Vector { x: 5., y: 5. };

        assert_eq!(p1.euclidean_distance(&p2), 50.0_f32.sqrt());
    }

    #[test]
    fn test_perp() {
        let p1 = Vector { x: 1., y: 0. };

        assert_eq!(p1.perp(), Vector { x: 0., y: -1. });
    }

    #[test]
    fn test_add() {
        let p1 = Vector { x: 0., y: 0. };
        let p2 = Vector { x: 5., y: 5. };

        assert_eq!(p1 + p2, Vector { x: 5., y: 5. });
    }

    #[test]
    fn test_sub() {
        let p1 = Vector { x: 0., y: 0. };
        let p2 = Vector { x: 5., y: 5. };

        assert_eq!(p1 - p2, Vector { x: -5., y: -5. });
    }

    #[test]
    fn test_magnitude() {
        let p1 = Vector { x: 0., y: 0. };
        let p2 = Vector { x: 2., y: 2. };
        let p3 = Vector { x: 1., y: 2. };

        assert_eq!(p1.magnitude(), 0.);
        assert_eq!(p2.magnitude(), 8.0_f32.sqrt());
        assert_eq!(p3.magnitude(), 5.0_f32.sqrt());
    }

    #[test]
    fn test_normalize() {
        let p1 = Vector { x: 0., y: 0. };
        let p2 = Vector { x: 1., y: 0. };
        let p3 = Vector { x: 0., y: 1. };
        let p4 = Vector { x: 1., y: 1. };
        let p5 = Vector { x: 1., y: 2. };

        assert_eq!(p1.normalize(), Vector { x: 0., y: 0. });
        assert_eq!(p2.normalize(), Vector { x: 1., y: 0. });
        assert_eq!(p3.normalize(), Vector { x: 0., y: 1. });
        assert_eq!(
            p4.normalize(),
            Vector {
                x: 1. / 2.0_f32.sqrt(),
                y: 1. / 2.0_f32.sqrt()
            }
        );
        assert_eq!(
            p5.normalize(),
            Vector {
                x: 1. / 5.0_f32.sqrt(),
                y: 2. / 5.0_f32.sqrt(),
            }
        );
    }

    #[test]
    fn test_dot() {
        let p1 = Vector { x: 0., y: 0. };
        let p2 = Vector { x: 1., y: 0. };

        assert_eq!(p1.dot(&p2), 0.);
    }

    #[test]
    fn test_project() {
        let p1 = Vector { x: 0.5, y: 10. };
        let axis = Vector { x: 1., y: 0. };

        assert_eq!(p1.project(&axis), Projection { min: 0.5, max: 0.5 });
    }
}
