use super::polar_vector::PolarVec;
use crate::physics::polar_vector::VectorPoint;

pub trait CoordinateSystem: {
    type CoSys;

    fn get_id(&self) -> &String;

    fn get_origin(&self) -> &PolarVec;

    fn get_parent_coord_system(&self) -> Option<&Self::CoSys>;
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct WorldCoordSystem {
    id: String,
    origin: PolarVec
}

impl CoordinateSystem for WorldCoordSystem {
    type CoSys = WorldCoordSystem;

    fn get_id(&self) -> &String {
        &self.id
    }

    fn get_origin(&self) -> &PolarVec {
        &self.origin
    }

    fn get_parent_coord_system(&self) -> Option<&Self::CoSys> {
        None
    }
}

impl WorldCoordSystem {
    pub fn new() -> WorldCoordSystem {
        WorldCoordSystem {
            id: "world".to_string(),
            origin: PolarVec::get_world_origin()
        }
    }
}
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct GeneralCoordSystem<T>
    where T: CoordinateSystem {
    id: String,
    parent_coord_system: T,
    origin: PolarVec
}

impl<T: CoordinateSystem> CoordinateSystem for GeneralCoordSystem<T>{
    type CoSys = T;

    fn get_id(&self) -> &String {
        &self.id
    }

    fn get_origin(&self) -> &PolarVec {
        &self.origin
    }

    fn get_parent_coord_system(&self) -> Option<&Self::CoSys> {
        Some(&self.parent_coord_system)
    }
}

impl<T: CoordinateSystem> GeneralCoordSystem<T>{
    pub fn new(id: String, parent_coord_system: T, origin: PolarVec) -> GeneralCoordSystem<T>{
        GeneralCoordSystem{
            id,
            parent_coord_system,
            origin
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::physics::coordinate_system::{WorldCoordSystem, CoordinateSystem, GeneralCoordSystem};
    use crate::physics::polar_vector::PolarVec;

    #[test]
    fn default_creation(){
        let cs = WorldCoordSystem::new();
        assert_eq!(&"world".to_string(), cs.get_id());
        assert_eq!(&PolarVec::get_world_origin(), cs.get_origin());
        assert_eq!(None, cs.get_parent_coord_system());
    }

    #[test]
    fn eq(){
        let a = WorldCoordSystem::new();
        let b = WorldCoordSystem::new();

        assert_eq!(a,b);
    }

    #[test]
    fn creation(){
        let wcs = WorldCoordSystem::new();
        let origin = PolarVec::new(10.0,90.0,90.0);
        let gcs = GeneralCoordSystem::new("gcs".to_string(), wcs, origin);

        println!("{:?}", wcs);
        println!("{:?}", origin);
        println!("{:?}", gcs);
    }
}

