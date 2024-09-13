use crate::Point;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rectangle {
    pub top: f32,
    pub left: f32,
    pub width: f32,
    pub height: f32,
}

impl Rectangle {
    pub fn center(&self) -> Point {
        Point {
            x: self.left + self.width / 2.,
            y: self.top - self.height / 2.,
        }
    }

    pub fn right(&self) -> f32 {
        self.left + self.width
    }

    pub fn bottom(&self) -> f32 {
        self.top - self.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_center_zero() {
        let r = Rectangle {
            top: 0.,
            left: 0.,
            width: 0.,
            height: 0.,
        };

        assert_eq!(r.center(), Point { x: 0., y: 0. })
    }

    #[test]
    fn test_center() {
        let r = Rectangle {
            top: 0.,
            left: 0.,
            width: 10.,
            height: 10.,
        };

        assert_eq!(r.center(), Point { x: 5., y: -5. })
    }

    #[test]
    fn test_right() {
        let r = Rectangle {
            top: 0.,
            left: 0.,
            width: 10.,
            height: 10.,
        };

        assert_eq!(r.right(), 10.)
    }

    #[test]
    fn test_bottom() {
        let r = Rectangle {
            top: 0.,
            left: 0.,
            width: 10.,
            height: 10.,
        };

        assert_eq!(r.bottom(), -10.)
    }
}
