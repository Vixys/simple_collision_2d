use crate::{point_rectangle::point_rectangle_collision, rectangle::Rectangle, Point};

pub fn rectangle_rectangle_collision(r1: &Rectangle, r2: &Rectangle) -> bool {
    point_rectangle_collision(&r1.top_left(), r2)
        || point_rectangle_collision(&r1.top_right(), r2)
        || point_rectangle_collision(&r1.bottom_left(), r2)
        || point_rectangle_collision(&r1.bottom_right(), r2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle_rectangle_collision_center() {
        let r = Rectangle {
            top: 2.5,
            left: -2.5,
            width: 5.,
            height: 5.,
        };

        let result = rectangle_rectangle_collision(&r, &r);

        assert!(result);
    }

    #[test]
    fn test_rectangle_rectangle_collision_collide() {
        let r1 = Rectangle {
            top: 2.5,
            left: -2.5,
            width: 5.,
            height: 5.,
        };
        let r2 = Rectangle {
            top: 0.,
            left: 0.,
            width: 5.,
            height: 5.,
        };

        let result = rectangle_rectangle_collision(&r1, &r2);

        assert!(result);
    }

    #[test]
    fn test_rectangle_rectangle_collision_not_collide() {
        let r1 = Rectangle {
            top: 2.5,
            left: -2.5,
            width: 5.,
            height: 5.,
        };
        let r2 = Rectangle {
            top: 0.,
            left: 3.,
            width: 5.,
            height: 5.,
        };

        let result = rectangle_rectangle_collision(&r1, &r2);

        assert!(!result);
    }

    #[test]
    fn test_rectangle_rectangle_collision_left() {
        let r1 = Rectangle {
            top: 2.5,
            left: -2.5,
            width: 5.,
            height: 5.,
        };
        let r2 = Rectangle {
            top: 2.5,
            left: -7.5,
            width: 5.,
            height: 5.,
        };

        let result = rectangle_rectangle_collision(&r1, &r2);

        assert!(result);
    }

    #[test]
    fn test_rectangle_rectangle_collision_top_left() {
        let r1 = Rectangle {
            top: 2.5,
            left: -2.5,
            width: 5.,
            height: 5.,
        };
        let r2 = Rectangle {
            top: 7.5,
            left: -7.5,
            width: 5.,
            height: 5.,
        };

        let result = rectangle_rectangle_collision(&r1, &r2);

        assert!(result);
    }

    #[test]
    fn test_rectangle_rectangle_collision_bottom_left() {
        let r1 = Rectangle {
            top: 2.5,
            left: -2.5,
            width: 5.,
            height: 5.,
        };
        let r2 = Rectangle {
            top: -2.5,
            left: -7.5,
            width: 5.,
            height: 5.,
        };

        let result = rectangle_rectangle_collision(&r1, &r2);

        assert!(result);
    }

    #[test]
    fn test_rectangle_rectangle_collision_right() {
        let r1 = Rectangle {
            top: 2.5,
            left: -2.5,
            width: 5.,
            height: 5.,
        };
        let r2 = Rectangle {
            top: 2.5,
            left: 2.5,
            width: 5.,
            height: 5.,
        };

        let result = rectangle_rectangle_collision(&r1, &r2);

        assert!(result);
    }

    #[test]
    fn test_rectangle_rectangle_collision_top_right() {
        let r1 = Rectangle {
            top: 2.5,
            left: -2.5,
            width: 5.,
            height: 5.,
        };
        let r2 = Rectangle {
            top: 7.5,
            left: 2.5,
            width: 5.,
            height: 5.,
        };

        let result = rectangle_rectangle_collision(&r1, &r2);

        assert!(result);
    }

    #[test]
    fn test_rectangle_rectangle_collision_bottom_right() {
        let r1 = Rectangle {
            top: 2.5,
            left: -2.5,
            width: 5.,
            height: 5.,
        };
        let r2 = Rectangle {
            top: -2.5,
            left: 2.5,
            width: 5.,
            height: 5.,
        };

        let result = rectangle_rectangle_collision(&r1, &r2);

        assert!(result);
    }

    #[test]
    fn test_rectangle_rectangle_collision_top() {
        let r1 = Rectangle {
            top: 2.5,
            left: -2.5,
            width: 5.,
            height: 5.,
        };
        let r2 = Rectangle {
            top: 7.5,
            left: -2.5,
            width: 5.,
            height: 5.,
        };

        let result = rectangle_rectangle_collision(&r1, &r2);

        assert!(result);
    }

    #[test]
    fn test_rectangle_rectangle_collision_bottom() {
        let r1 = Rectangle {
            top: 2.5,
            left: -2.5,
            width: 5.,
            height: 5.,
        };
        let r2 = Rectangle {
            top: -2.5,
            left: -2.5,
            width: 5.,
            height: 5.,
        };

        let result = rectangle_rectangle_collision(&r1, &r2);

        assert!(result);
    }
}
