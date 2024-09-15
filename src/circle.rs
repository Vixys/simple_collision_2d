use crate::point::Point;
use crate::point::Vector;
use crate::sat::project::Sat;
use crate::sat::projection::Projection;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Circle {
    pub center: Point,
    pub radius: f32,
}

impl Sat for Circle {
    fn project(&self, axis: &Vector) -> Projection {
        let normalized_axis = axis.normalize();
        let p1 = (self.center + normalized_axis * self.radius).dot(&normalized_axis);
        let p2 = (self.center + normalized_axis * -self.radius).dot(&normalized_axis);
        Projection {
            min: p1.min(p2),
            max: p1.max(p2),
        }
    }

    fn axes(&self) -> Vec<Vector> {
        Vec::new()
    }

    fn is_curved(&self) -> bool {
        true
    }

    fn get_center(&self) -> Vector {
        self.center
    }
}

impl Default for Circle {
    fn default() -> Self {
        Self {
            center: Default::default(),
            radius: 1.0,
        }
    }
}

impl Circle {
    pub fn new(center: Point, radius: f32) -> Self {
        Self { center, radius }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project() {
        let c = Circle::new(Point::ZERO, 2.);
        let axis1 = Vector::new(0., 1.).normalize();
        let axis2 = Vector::new(0., -1.).normalize();
        let axis3 = Vector::new(1., 1.).normalize();

        assert_eq!(c.project(&axis1), Projection { min: -2., max: 2. });
        assert_eq!(c.project(&axis2), Projection { min: -2., max: 2. });
        assert_eq!(c.project(&axis3), Projection { min: -2., max: 2. });
    }

    #[test]
    fn test_project_advanced() {
        let c = Circle::new(Point::new(2., 3.5), 2.);
        let axis1 = Vector::new(0., 1.).normalize();
        let axis2 = Vector::new(0., -1.).normalize();

        assert_eq!(c.project(&axis1), Projection { min: 1.5, max: 5.5 });
        assert_eq!(
            c.project(&axis2),
            Projection {
                min: -5.5,
                max: -1.5,
            }
        );
    }
}
