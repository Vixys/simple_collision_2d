use crate::circle::Circle;
use crate::circle_circle::circle_circle_collision;
use crate::circle_rectangle::circle_rectangle_collision;
use crate::point::Point;
use crate::point_circle::point_circle_collision;
use crate::point_point::point_point_collision;
use crate::point_rectangle::point_rectangle_collision;
use crate::rectangle::Rectangle;
use crate::rectangle_rectangle::rectangle_rectangle_collision;
use crate::sat::polygon::Polygon;
use crate::sat::sat_collision;

#[derive(Debug)]
pub enum Collidable {
    Point(Point),
    Circle(Circle),
    Rectangle(Rectangle),
    Polygon(Polygon),
}

impl Collidable {
    pub fn collides_with(&self, other: &Collidable) -> bool {
        match (self, other) {
            (Collidable::Point(p1), Collidable::Point(p2)) => point_point_collision(p1, p2),
            (Collidable::Circle(c1), Collidable::Circle(c2)) => circle_circle_collision(c1, c2),
            (Collidable::Rectangle(r1), Collidable::Rectangle(r2)) => {
                rectangle_rectangle_collision(r1, r2)
            }
            (Collidable::Point(p), Collidable::Circle(c))
            | (Collidable::Circle(c), Collidable::Point(p)) => point_circle_collision(p, c),
            (Collidable::Point(p), Collidable::Rectangle(r))
            | (Collidable::Rectangle(r), Collidable::Point(p)) => point_rectangle_collision(p, r),
            (Collidable::Circle(c), Collidable::Rectangle(r))
            | (Collidable::Rectangle(r), Collidable::Circle(c)) => circle_rectangle_collision(c, r),
            (Collidable::Point(p), Collidable::Polygon(poly))
            | (Collidable::Polygon(poly), Collidable::Point(p)) => sat_collision(poly, p),
            (Collidable::Circle(_c), Collidable::Polygon(_p))
            | (Collidable::Polygon(_p), Collidable::Circle(_c)) => {
                panic!("polygon and circle collision not supported yet!")
            }
            (Collidable::Rectangle(r), Collidable::Polygon(p))
            | (Collidable::Polygon(p), Collidable::Rectangle(r)) => sat_collision(p, r),
            (Collidable::Polygon(p1), Collidable::Polygon(p2)) => sat_collision(p1, p2),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::point::Vector;

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

    #[test]
    fn test_polygon_point() {
        let poly = Collidable::Polygon(Polygon::new(vec![
            Vector::new(0., 1.),
            Vector::new(1., -1.),
            Vector::new(-1., -1.),
        ]));
        let point = Collidable::Point(Point::default());

        assert!(poly.collides_with(&point));
        assert!(point.collides_with(&poly));
    }

    #[test]
    #[should_panic]
    fn test_polygon_circle() {
        let poly = Collidable::Polygon(Polygon::new(vec![
            Vector::new(0., 1.),
            Vector::new(1., -1.),
            Vector::new(-1., -1.),
        ]));
        let c = Collidable::Circle(Circle::default());

        poly.collides_with(&c);
    }

    #[test]
    fn test_polygon_rectangle() {
        let poly = Collidable::Polygon(Polygon::new(vec![
            Vector::new(0., 1.),
            Vector::new(1., -1.),
            Vector::new(-1., -1.),
        ]));
        let r = Collidable::Rectangle(Rectangle::default());

        assert!(poly.collides_with(&r));
        assert!(r.collides_with(&poly));
    }

    #[test]
    fn test_polygon_polygon() {
        let p1 = Collidable::Polygon(Polygon::new(vec![
            Vector::ZERO,
            Vector::new(1., 0.),
            Vector::new(0.5, 1.),
        ]));
        let p2 = Collidable::Polygon(Polygon::new(vec![
            Vector::new(0.5, 0.5),
            Vector::new(0., 1.5),
            Vector::new(1., 1.5),
        ]));

        assert!(p1.collides_with(&p2));
        assert!(p2.collides_with(&p1));
    }
}
