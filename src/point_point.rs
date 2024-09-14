use crate::point::Point;

fn point_point_collision(p1: &Point, p2: &Point) -> bool {
    p1 == p2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_rectangle_collision_collide() {
        let p1 = Point { x: 0., y: 0. };

        let result = point_point_collision(&p1, &p1);

        assert!(result);
    }

    #[test]
    fn test_point_rectangle_collision_not_collide() {
        let p1 = Point { x: 0., y: 0. };
        let p2 = Point { x: 5., y: 5. };

        let result = point_point_collision(&p1, &p2);

        assert!(!result);
    }
}
