use crate::{circle::Circle, point::Point};

pub fn point_circle_collision(p: &Point, c: &Circle) -> bool {
    p.euclidean_distance(&c.center) <= c.radius
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_circle_collision_center() {
        let p = Point { x: 0., y: 0. };
        let c = Circle {
            center: Point { x: 0., y: 0. },
            radius: 5.,
        };

        let result = point_circle_collision(&p, &c);

        assert!(result);
    }

    #[test]
    fn test_point_circle_collision_collide() {
        let p = Point { x: 2., y: 2. };
        let c = Circle {
            center: Point { x: 0., y: 0. },
            radius: 5.,
        };

        let result = point_circle_collision(&p, &c);

        assert!(result);
    }

    #[test]
    fn test_point_circle_collision_not_collide() {
        let p = Point { x: 5., y: 5. };
        let c = Circle {
            center: Point { x: 0., y: 0. },
            radius: 5.,
        };

        let result = point_circle_collision(&p, &c);

        assert!(!result);
    }

    #[test]
    fn test_point_circle_collision_tangent() {
        let p = Point { x: 5., y: 0. };
        let c = Circle {
            center: Point { x: 0., y: 0. },
            radius: 5.,
        };

        let result = point_circle_collision(&p, &c);

        assert!(result);
    }
}
