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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euclidean_distance() {
        let p1 = Point { x: 0., y: 0. };
        let p2 = Point { x: 5., y: 0. };

        assert_eq!(p1.euclidean_distance(&p2), 5.);
    }

    #[test]
    fn test_euclidean_distance_advanced() {
        let p1 = Point { x: 0., y: 0. };
        let p2 = Point { x: 5., y: 5. };

        assert_eq!(p1.euclidean_distance(&p2), 50.0_f32.sqrt());
    }
}
