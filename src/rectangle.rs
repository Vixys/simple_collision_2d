use crate::point::Point;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rectangle {
    pub top: f32,
    pub left: f32,
    pub width: f32,
    pub height: f32,
}

impl Default for Rectangle {
    fn default() -> Self {
        Self {
            top: 0.5,
            left: -0.5,
            width: 1.,
            height: 1.,
        }
    }
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

    pub fn top_left(&self) -> Point {
        Point {
            x: self.left,
            y: self.top,
        }
    }

    pub fn top_right(&self) -> Point {
        Point {
            x: self.right(),
            y: self.top,
        }
    }

    pub fn bottom_right(&self) -> Point {
        Point {
            x: self.right(),
            y: self.bottom(),
        }
    }

    pub fn bottom_left(&self) -> Point {
        Point {
            x: self.left,
            y: self.bottom(),
        }
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

    #[test]
    fn test_bottom_left() {
        let r = Rectangle {
            top: 0.,
            left: 0.,
            width: 10.,
            height: 10.,
        };

        assert_eq!(r.bottom_left(), Point { x: 0., y: -10. })
    }

    #[test]
    fn test_bottom_right() {
        let r = Rectangle {
            top: 0.,
            left: 0.,
            width: 10.,
            height: 10.,
        };

        assert_eq!(r.bottom_right(), Point { x: 10., y: -10. })
    }

    #[test]
    fn test_top_left() {
        let r = Rectangle {
            top: 0.,
            left: 0.,
            width: 10.,
            height: 10.,
        };

        assert_eq!(r.top_left(), Point { x: 0., y: 0. })
    }

    #[test]
    fn test_top_right() {
        let r = Rectangle {
            top: 0.,
            left: 0.,
            width: 10.,
            height: 10.,
        };

        assert_eq!(r.top_right(), Point { x: 10., y: 0. })
    }
}
