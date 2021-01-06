use crate::physics::vectors::{PolarVec, Vector};
use crate::physics::coordinate_system::CoordinateSystem;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VectorPoint<'a, T>
    where T: CoordinateSystem {
    cord_sys: &'a T,
    vector: Vector
}

impl<T: CoordinateSystem> VectorPoint<'_, T> {
    pub fn new(cord_sys: &T, vector: Vector) -> VectorPoint<T>{
        VectorPoint {cord_sys, vector}
    }

    pub fn get_cord_sys(&self) -> &T {
        &self.cord_sys
    }

    pub fn get_vector(&self) -> &Vector {
        &self.vector
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PolarVectorPoint<'a, T>
    where T: CoordinateSystem {
    cord_sys: &'a T,
    vector: PolarVec
}

impl<T: CoordinateSystem> PolarVectorPoint<'_, T> {
    pub fn new(cord_sys: &T, vector: PolarVec) -> PolarVectorPoint<T>{
        PolarVectorPoint {cord_sys, vector}
    }

    pub fn get_cord_sys(&self) -> &T {
        &self.cord_sys
    }

    pub fn get_vector(&self) -> &PolarVec {
        &self.vector
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod vector {
        use super::*;
        use crate::physics::coordinate_system::WorldCoordSystem;

        #[test]
        fn creation(){
            let wcs = WorldCoordSystem::new();
            let pv = Vector::new(10.0,90.0,90.0);
            let vp = VectorPoint::new(&wcs, pv);

            assert_eq!(&pv, vp.get_vector());
            assert_eq!(&wcs, vp.get_cord_sys());
        }
    }

    mod vector_point {
        use super::*;
        use crate::physics::coordinate_system::WorldCoordSystem;

        #[test]
        fn creation(){
            let wcs = WorldCoordSystem::new();
            let pv = PolarVec::new(10.0,90.0,90.0);
            let vp = PolarVectorPoint::new(&wcs, pv);

            assert_eq!(&pv, vp.get_vector());
            assert_eq!(&wcs, vp.get_cord_sys());
        }
    }
}