#[derive(Default, Debug, Copy, Clone)]
pub struct Projection {
    pub min: f32,
    pub max: f32,
}

impl PartialEq for Projection {
    fn eq(&self, other: &Self) -> bool {
        (self.min - other.min).abs() < 0.00001 && (self.max - other.max).abs() < 0.00001
    }
}

impl Projection {
    pub fn overlap(&self, other: &Self) -> bool {
        (self.min < other.min && self.max > other.min)
            || (other.min < self.min && other.max > self.min)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlap_true() {
        let p1 = Projection { min: 1., max: 2. };
        let p2 = Projection { min: 1.5, max: 2.5 };

        assert!(p1.overlap(&p2));
        assert!(p2.overlap(&p1));
    }

    #[test]
    fn test_overlap_within() {
        let p1 = Projection { min: 1., max: 2. };
        let p2 = Projection {
            min: 1.25,
            max: 1.75,
        };

        assert!(p1.overlap(&p2));
        assert!(p2.overlap(&p1));
    }

    #[test]
    fn test_overlap_false() {
        let p1 = Projection { min: 1., max: 2. };
        let p2 = Projection { min: 2., max: 3. };

        assert!(!p1.overlap(&p2));
        assert!(!p2.overlap(&p1));
    }

    #[test]
    fn test_partial_eq() {
        let p1 = Projection {
            min: -2.0,
            max: 2.0,
        };

        assert_eq!(
            p1,
            Projection {
                min: -2.0000002,
                max: 2.0000002
            }
        );
    }
}
