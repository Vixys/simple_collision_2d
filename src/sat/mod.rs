use project::Sat;

pub mod polygon;
pub mod project;
pub mod projection;

pub fn sat_collision(p1: &dyn Sat, p2: &dyn Sat) -> bool {
    let mut axes = p1.axes();
    axes.append(&mut p2.axes());

    for axis in axes {
        if !p1.project(&axis).overlap(&p2.project(&axis)) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::vector::Vector;
    use crate::sat::polygon::Polygon;

    use super::*;

    #[test]
    fn test_collide() {
        let p1 = Polygon::new(vec![
            Vector::ZERO,
            Vector::new(1., 0.),
            Vector::new(0.5, 1.),
        ]);
        let p2 = Polygon::new(vec![
            Vector::new(0.5, 0.5),
            Vector::new(0., 1.5),
            Vector::new(1., 1.5),
        ]);

        assert!(sat_collision(&p1, &p2));
    }

    #[test]
    fn test_do_not_collide() {
        let p1 = Polygon::new(vec![
            Vector::ZERO,
            Vector::new(1., 0.),
            Vector::new(0.5, 1.),
        ]);
        let p2 = Polygon::new(vec![
            Vector::new(1.5, 1.5),
            Vector::new(0., 2.5),
            Vector::new(1., 2.5),
        ]);

        assert!(!sat_collision(&p1, &p2));
    }
}
