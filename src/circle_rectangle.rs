use crate::{
    circle::Circle, point::Point, point_circle::point_circle_collision, rectangle::Rectangle,
};

pub fn circle_rectangle_collision(c: &Circle, r: &Rectangle) -> bool {
    let rx = c.center.x.clamp(r.left, r.right());
    let ry = c.center.y.clamp(r.bottom(), r.top);
    point_circle_collision(&Point { x: rx, y: ry }, c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_rectangle_collision_center() {
        let c = Circle {
            center: Point { x: 0., y: 0. },
            radius: 10.,
        };
        let r = Rectangle {
            top: 2.5,
            left: -2.5,
            width: 5.,
            height: 5.,
        };

        let result = circle_rectangle_collision(&c, &r);

        assert!(result);
    }

    #[test]
    fn test_point_rectangle_collision_collide() {
        let c = Circle {
            center: Point { x: 5., y: 0. },
            radius: 10.,
        };
        let r = Rectangle {
            top: 2.5,
            left: -2.5,
            width: 5.,
            height: 5.,
        };

        let result = circle_rectangle_collision(&c, &r);

        assert!(result);
    }

    #[test]
    fn test_point_rectangle_collision_not_collide() {
        let c = Circle {
            center: Point { x: 8., y: 0. },
            radius: 5.,
        };
        let r = Rectangle {
            top: 2.5,
            left: -2.5,
            width: 5.,
            height: 5.,
        };

        let result = circle_rectangle_collision(&c, &r);

        assert!(!result);
    }

    #[test]
    fn test_point_rectangle_collision_tangent_left() {
        let c = Circle {
            center: Point { x: -7.5, y: 0. },
            radius: 5.,
        };
        let r = Rectangle {
            top: 2.5,
            left: -2.5,
            width: 5.,
            height: 5.,
        };

        let result = circle_rectangle_collision(&c, &r);

        assert!(result);
    }

    #[test]
    fn test_point_rectangle_collision_tangent_top_left() {
        let c = Circle {
            center: Point { x: -3.5, y: 3.5 },
            radius: 2.0_f32.sqrt(),
        };
        let r = Rectangle {
            top: 2.5,
            left: -2.5,
            width: 5.,
            height: 5.,
        };

        let result = circle_rectangle_collision(&c, &r);

        assert!(result);
    }

    #[test]
    fn test_point_rectangle_collision_tangent_right() {
        let c = Circle {
            center: Point { x: 7.5, y: 0. },
            radius: 5.,
        };
        let r = Rectangle {
            top: 2.5,
            left: -2.5,
            width: 5.,
            height: 5.,
        };

        let result = circle_rectangle_collision(&c, &r);

        assert!(result);
    }

    #[test]
    fn test_point_rectangle_collision_tangent_top_right() {
        let c = Circle {
            center: Point { x: 3.5, y: 3.5 },
            radius: 2.0_f32.sqrt(),
        };
        let r = Rectangle {
            top: 2.5,
            left: -2.5,
            width: 5.,
            height: 5.,
        };

        let result = circle_rectangle_collision(&c, &r);

        assert!(result);
    }

    #[test]
    fn test_point_rectangle_collision_tangent_bottom() {
        let c = Circle {
            center: Point { x: 0., y: -7.5 },
            radius: 5.,
        };
        let r = Rectangle {
            top: 2.5,
            left: -2.5,
            width: 5.,
            height: 5.,
        };

        let result = circle_rectangle_collision(&c, &r);

        assert!(result);
    }

    #[test]
    fn test_point_rectangle_collision_tangent_bottom_right() {
        let c = Circle {
            center: Point { x: 3.5, y: -3.5 },
            radius: 2.0_f32.sqrt(),
        };
        let r = Rectangle {
            top: 2.5,
            left: -2.5,
            width: 5.,
            height: 5.,
        };

        let result = circle_rectangle_collision(&c, &r);

        assert!(result);
    }

    #[test]
    fn test_point_rectangle_collision_tangent_top() {
        let c = Circle {
            center: Point { x: 0., y: 7.5 },
            radius: 5.,
        };
        let r = Rectangle {
            top: 2.5,
            left: -2.5,
            width: 5.,
            height: 5.,
        };

        let result = circle_rectangle_collision(&c, &r);

        assert!(result);
    }

    #[test]
    fn test_point_rectangle_collision_tangent_bottom_left() {
        let c = Circle {
            center: Point { x: -3.5, y: -3.5 },
            radius: 2.0_f32.sqrt(),
        };
        let r = Rectangle {
            top: 2.5,
            left: -2.5,
            width: 5.,
            height: 5.,
        };

        let result = circle_rectangle_collision(&c, &r);

        assert!(result);
    }
}
