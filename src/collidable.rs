use crate::{
    circle::Circle, circle_circle::circle_circle_collision,
    circle_rectangle::circle_rectangle_collision, point::Point,
    point_circle::point_circle_collision, point_point::point_point_collision,
    point_rectangle::point_rectangle_collision, rectangle::Rectangle,
    rectangle_rectangle::rectangle_rectangle_collision,
};

#[derive(Debug)]
pub enum Collidable {
    Point(Point),
    Circle(Circle),
    Rectangle(Rectangle),
}

impl Collidable {
    pub fn collides_with(&self, other: &Collidable) -> bool {
        match (self, other) {
            (Collidable::Point(p1), Collidable::Point(p2)) => point_point_collision(p1, p2),
            (Collidable::Point(p), Collidable::Circle(c))
            | (Collidable::Circle(c), Collidable::Point(p)) => point_circle_collision(p, c),
            (Collidable::Point(p), Collidable::Rectangle(r))
            | (Collidable::Rectangle(r), Collidable::Point(p)) => point_rectangle_collision(p, r),
            (Collidable::Circle(c1), Collidable::Circle(c2)) => circle_circle_collision(c1, c2),
            (Collidable::Circle(c), Collidable::Rectangle(r))
            | (Collidable::Rectangle(r), Collidable::Circle(c)) => circle_rectangle_collision(c, r),
            (Collidable::Rectangle(r1), Collidable::Rectangle(r2)) => {
                rectangle_rectangle_collision(r1, r2)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_point() {
        let p = Collidable::Point(Point::default());

        assert!(p.collides_with(&p));
    }

    #[test]
    fn test_point_circle() {
        let p = Collidable::Point(Point::default());
        let c = Collidable::Circle(Circle::default());

        assert!(p.collides_with(&c));
        assert!(c.collides_with(&p));
    }

    #[test]
    fn test_point_rectangle() {
        let p = Collidable::Point(Point::default());
        let r = Collidable::Rectangle(Rectangle::default());

        assert!(p.collides_with(&r));
        assert!(r.collides_with(&p));
    }

    #[test]
    fn test_circle_circle() {
        let c = Collidable::Circle(Circle::default());

        assert!(c.collides_with(&c));
    }

    #[test]
    fn test_circle_rectangle() {
        let c = Collidable::Circle(Circle::default());
        let r = Collidable::Rectangle(Rectangle::default());

        assert!(c.collides_with(&r));
        assert!(r.collides_with(&c));
    }

    #[test]
    fn test_rectangle_rectangle() {
        let r = Collidable::Rectangle(Rectangle::default());

        assert!(r.collides_with(&r));
    }
}
