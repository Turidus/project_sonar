use super::polar_vector::Vector;

pub trait CoordinateSystem: {
    type CoSys: CoordinateSystem;

    fn get_id(&self) -> &String;

    fn get_origin(&self) -> &Vector;

    fn get_parent_coord_system(&self) -> Option<&Self::CoSys>;
}
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct WorldCoordSystem {
    id: String,
    origin: Vector
}

impl CoordinateSystem for WorldCoordSystem {
    type CoSys = WorldCoordSystem;

    fn get_id(&self) -> &String {
        &self.id
    }

    fn get_origin(&self) -> &Vector {
        &self.origin
    }

    fn get_parent_coord_system(&self) -> Option<&Self::CoSys> {
        None
    }
}

impl CoordinateSystem for &WorldCoordSystem {
    type CoSys = WorldCoordSystem;

    fn get_id(&self) -> &String {
        &self.id
    }

    fn get_origin(&self) -> &Vector {
        &self.origin
    }

    fn get_parent_coord_system(&self) -> Option<&Self::CoSys> {
        None
    }
}

impl WorldCoordSystem {
    pub fn new() -> WorldCoordSystem {
        WorldCoordSystem {
            id: "wcs".to_string(),
            origin: Vector::get_world_origin()
        }
    }
}
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct GeneralCoordSystem<'a, T>
    where T: CoordinateSystem {
    id: String,
    parent_coord_system: &'a T,
    origin: Vector
}

impl<T: CoordinateSystem> CoordinateSystem for GeneralCoordSystem<'_, T>{
    type CoSys = T;

    fn get_id(&self) -> &String {
        &self.id
    }

    fn get_origin(&self) -> &Vector {
        &self.origin
    }

    fn get_parent_coord_system(&self) -> Option<&Self::CoSys> {
        Some(&self.parent_coord_system)
    }
}

impl<T: CoordinateSystem> GeneralCoordSystem<'_, T>{
    pub fn new(id: String, parent_coord_system: &T, origin: Vector) -> GeneralCoordSystem<T> {
        GeneralCoordSystem{
            id,
            parent_coord_system,
            origin
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_creation(){
        let cs = WorldCoordSystem::new();
        assert_eq!(&"wcs".to_string(), cs.get_id());
        assert_eq!(&Vector::get_world_origin(), cs.get_origin());
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
        let origin = Vector::new(10.0,90.0,90.0);
        let gcs = GeneralCoordSystem::new("gcs".to_string(), &wcs, origin);

        println!("{:?}", wcs);
        println!("{:?}", origin);
        println!("{:?}", gcs);
    }
}

