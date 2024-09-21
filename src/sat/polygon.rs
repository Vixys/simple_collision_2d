use crate::vector::{Point, Vector};

use super::{project::Sat, projection::Projection};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Polygon {
    pub vertices: Vec<Point>,
}

impl Polygon {
    pub fn new(vertices: Vec<Vector>) -> Self {
        Self { vertices }
    }

    fn ensure_valid(&self) {
        if self.vertices.len() < 3 {
            panic!("Polygon must have at least 3 edges.")
        }
    }
}

impl Sat for Polygon {
    fn project(&self, axis: &Vector) -> Projection {
        self.ensure_valid();
        let mut min = axis.dot(&self.vertices[0]);
        let mut max = min;
        for v in self.vertices.iter().skip(1) {
            let p = axis.dot(v);
            min = min.min(p);
            max = max.max(p);
        }
        Projection { min, max }
    }

    fn axes(&self) -> Vec<Vector> {
        self.ensure_valid();

        let mut axes = Vec::new();

        for i in 0..self.vertices.len() {
            let p1: Point = self.vertices[i];
            let p2: Point = self.vertices[if i + 1 < self.vertices.len() {
                i + 1
            } else {
                0
            }];
            axes.push((p1 - p2).perp().normalize())
        }
        axes
    }

    fn is_curved(&self) -> bool {
        false
    }

    fn get_center(&self) -> Vector {
        unreachable!("This is not supposed to be called")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_ensure_valid() {
        let polygon = Polygon::new(vec![Vector::ZERO, Vector::ONE]);

        polygon.ensure_valid();
    }

    #[test]
    fn test_axes() {
        let polygon = Polygon::new(vec![
            Vector::ZERO,
            Vector::new(1., 0.),
            Vector::new(1., 1.),
            Vector::new(0., 1.),
        ]);

        let axes = polygon.axes();

        assert_eq!(
            axes,
            vec![
                Vector { x: 0., y: 1. },
                Vector { x: -1., y: 0. },
                Vector { x: 0., y: -1. },
                Vector { x: 1., y: 0. },
            ]
        );
    }

    #[test]
    fn test_project() {
        let polygon = Polygon::new(vec![
            Vector::ZERO,
            Vector::new(1., 0.),
            Vector::new(0.5, 1.),
        ]);

        let projection = polygon.project(&Vector { x: 0., y: -1. });
        let projection2 = polygon.project(&Vector { x: 0., y: 1. });

        assert_eq!(projection, Projection { min: -1., max: 0. });
        assert_eq!(projection2, Projection { min: 0., max: 1. });
    }
}
