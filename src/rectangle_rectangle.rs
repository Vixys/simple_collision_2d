use crate::{rectangle::Rectangle, Point};

pub fn rectangle_rectangle_collision(r1: &Rectangle, r2: &Rectangle) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_rectangle_collision_center() {
        let r = Rectangle {
            top: 0.,
            left: 0.,
            width: 5.,
            height: 5.,
        };

        let result = rectangle_rectangle_collision(&r, &r);

        assert!(result);
    }
}
