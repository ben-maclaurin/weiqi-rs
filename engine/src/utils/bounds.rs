use crate::game::BoardSize;
use crate::mov::Intersection;

pub fn is_within_bounds(intersection: &Intersection, size: &BoardSize) -> bool {
    if (intersection.0 < 1 || intersection.1 < 1)
        || (intersection.0 > *size || intersection.1 > *size)
    {
        return false;
    }
    true
}
