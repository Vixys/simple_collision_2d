use crate::{rectangle::Rectangle, Point};

pub fn point_rectangle_collision(p: &Point, r: &Rectangle) -> bool {
    p.x <= r.right() && p.x >= r.left && p.y <= r.top && p.y >= r.bottom()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_rectangle_collision_center() {
        let p = Point { x: 0., y: 0. };
        let r = Rectangle {
            top: 2.5,
            left: -2.5,
            width: 5.,
            height: 5.,
        };

        let result = point_rectangle_collision(&p, &r);

        assert!(result);
    }

    #[test]
    fn test_point_rectangle_collision_collide() {
        let p = Point { x: 2., y: 2. };
        let r = Rectangle {
            top: 2.5,
            left: -2.5,
            width: 5.,
            height: 5.,
        };

        let result = point_rectangle_collision(&p, &r);

        assert!(result);
    }

    #[test]
    fn test_point_rectangle_collision_not_collide() {
        let p = Point { x: 5., y: 5. };
        let r = Rectangle {
            top: 2.5,
            left: -2.5,
            width: 5.,
            height: 5.,
        };

        let result = point_rectangle_collision(&p, &r);

        assert!(!result);
    }

    #[test]
    fn test_point_rectangle_collision_tangent_left() {
        let p = Point { x: -2.5, y: 0. };
        let r = Rectangle {
            top: 2.5,
            left: -2.5,
            width: 5.,
            height: 5.,
        };

        let result = point_rectangle_collision(&p, &r);

        assert!(result);
    }

    #[test]
    fn test_point_rectangle_collision_tangent_right() {
        let p = Point { x: 2.5, y: 0. };
        let r = Rectangle {
            top: 2.5,
            left: -2.5,
            width: 5.,
            height: 5.,
        };

        let result = point_rectangle_collision(&p, &r);

        assert!(result);
    }

    #[test]
    fn test_point_rectangle_collision_tangent_bottom() {
        let p = Point { x: 0., y: -2.5 };
        let r = Rectangle {
            top: 2.5,
            left: -2.5,
            width: 5.,
            height: 5.,
        };

        let result = point_rectangle_collision(&p, &r);

        assert!(result);
    }

    #[test]
    fn test_point_rectangle_collision_tangent_top() {
        let p = Point { x: 0., y: 2.5 };
        let r = Rectangle {
            top: 2.5,
            left: -2.5,
            width: 5.,
            height: 5.,
        };

        let result = point_rectangle_collision(&p, &r);

        assert!(result);
    }
}
