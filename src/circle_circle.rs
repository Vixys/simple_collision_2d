use crate::circle::Circle;

pub fn circle_circle_collision(c1: &Circle, c2: &Circle) -> bool {
    c1.center.euclidean_distance(&c2.center) <= c1.radius + c2.radius
}

#[cfg(test)]
mod tests {
    use crate::vector::Point;

    use super::*;

    #[test]
    fn test_circle_circle_collision_collide() {
        let c1 = Circle {
            center: Point { x: 0., y: 0. },
            radius: 10.,
        };
        let c2 = Circle {
            center: Point { x: 5., y: 5. },
            radius: 10.,
        };

        let result = circle_circle_collision(&c1, &c2);

        assert!(result);
    }

    #[test]
    fn test_circle_circle_collision_not_collide() {
        let c1 = Circle {
            center: Point { x: 0., y: 0. },
            radius: 5.,
        };
        let c2 = Circle {
            center: Point { x: 10., y: 10. },
            radius: 5.,
        };

        let result = circle_circle_collision(&c1, &c2);

        assert!(!result);
    }

    #[test]
    fn test_circle_circle_collision_tangent() {
        let c1 = Circle {
            center: Point { x: 0., y: 0. },
            radius: 5.,
        };
        let c2 = Circle {
            center: Point { x: 10., y: 0. },
            radius: 5.,
        };

        let result = circle_circle_collision(&c1, &c2);

        assert!(result);
    }

    #[test]
    fn test_circle_circle_collision_same() {
        let c1 = Circle {
            center: Point { x: 0., y: 0. },
            radius: 5.,
        };

        let result = circle_circle_collision(&c1, &c1);

        assert!(result);
    }
}
