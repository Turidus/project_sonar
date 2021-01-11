use crate::physics::vectors::{PolarVec, Vector};
use crate::physics::coordinate_system::CoordinateSystem;

/// This struct describes a point in space in a given coordinate system by
/// using a cartesian vector.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VectorPoint<'a, T>
    where T: CoordinateSystem {
    cord_sys: &'a T,
    vector: Vector
}

impl<T: CoordinateSystem> VectorPoint<'_, T> {
    // Creates a new VectorPoint out of a coordinate system and a Vector.
    pub fn new(cord_sys: &T, vector: Vector) -> VectorPoint<T>{
        VectorPoint {cord_sys, vector}
    }
    // Returns a reference to the coordinate system in which this VectorPoint is valid.
    pub fn get_cord_sys(&self) -> &T {
        &self.cord_sys
    }
    // Returns the vector that defines the point
    pub fn get_vector(&self) -> &Vector {
        &self.vector
    }
}
/// This struct describes a point in space in a given coordinate system by
/// using a polar vector.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PolarVectorPoint<'a, T>
    where T: CoordinateSystem {
    cord_sys: &'a T,
    vector: PolarVec
}

impl<T: CoordinateSystem> PolarVectorPoint<'_, T> {
    // Creates a new VectorPoint out of a coordinate system and a PolarVector.
    pub fn new(cord_sys: &T, vector: PolarVec) -> PolarVectorPoint<T>{
        PolarVectorPoint {cord_sys, vector}
    }
    // Returns a reference to the coordinate system in which this PolarVectorPoint is valid.
    pub fn get_cord_sys(&self) -> &T {
        &self.cord_sys
    }
    // Returns the vector that defines the point
    pub fn get_vector(&self) -> &PolarVec {
        &self.vector
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod vector_point {
        use super::*;
        use crate::physics::coordinate_system::{WorldCoordSystem, GeneralCoordSystem};

        #[test]
        fn creation(){
            let wcs = WorldCoordSystem::new();
            let vec = Vector::new(10.0,90.0,90.0);
            let vp = VectorPoint::new(&wcs, vec);

            assert_eq!(&vec, vp.get_vector());
            assert_eq!(&wcs, vp.get_cord_sys());

            let gcs = GeneralCoordSystem::new("gcs".to_string(), &wcs, vec);
            let vp = VectorPoint::new(&gcs, vec);
            assert_eq!(&vec, vp.get_vector());
            assert_eq!(&gcs, vp.get_cord_sys());
        }
    }

    mod polar_vector_point {
        use super::*;
        use crate::physics::coordinate_system::WorldCoordSystem;
        use std::f64::consts::{PI, FRAC_PI_2};

        #[test]
        fn creation(){
            let wcs = WorldCoordSystem::new();
            let pv = PolarVec::new(10.0,PI,FRAC_PI_2);
            let vp = PolarVectorPoint::new(&wcs, pv);

            assert_eq!(&pv, vp.get_vector());
            assert_eq!(&wcs, vp.get_cord_sys());
        }
    }
}