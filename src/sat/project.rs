use crate::vector::Vector;

use super::projection::Projection;

pub trait Sat {
    fn project(&self, axis: &Vector) -> Projection;
    fn axes(&self) -> Vec<Vector>;
    fn is_curved(&self) -> bool;
    fn get_center(&self) -> Vector;
}
