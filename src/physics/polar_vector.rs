use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::fmt;
use crate::constants::WORLD_ORIGIN;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64
}
impl Eq for Vector{}

impl Ord for Vector{
    fn cmp(&self, other: &Self) -> Ordering {
        return if self.eq(&other) {
            Ordering::Equal
        } else if self.x.ne(&other.x) {
            if self.x > other.x {
                Ordering::Greater
            } else { Ordering::Less }
        } else if self.y.ne(&other.y) {
            if self.y > other.y {
                Ordering::Greater
            } else { Ordering::Less }
        } else {
            if self.z > other.z {
                Ordering::Greater
            } else { Ordering::Less }
        }
    }
}

impl Display for Vector {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[X: {:?} m, Y: {:?} m, Z: {:?} m]", self.x, self.y, self.z)
    }
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector{x,y,z}
    }

    pub fn get_world_origin() -> Vector {
        let (x,y,z) = WORLD_ORIGIN;
        Vector{x,y,z}
    }

    pub fn get_x(&self) -> f64 {
        self.x
    }
    pub fn get_y(&self) -> f64 {
        self.y
    }
    pub fn get_z(&self) -> f64 {
        self.z
    }

    pub fn add(&self, other: &Self) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn subtract(&self, other: &Self) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn to_polar_vector(&self) -> PolarVec {
        let r = (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt();
        PolarVec {
            r,
            phi: self.y.atan2(self.x),
            theta: (self.z / r).acos()
        }
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PolarVec {
    r: f64, //radius in m and range 0..
    phi: f64, //azimut angle in rad and range 0..2*pi
    theta: f64, //polar angle in rad and range 0..pi
}

impl Eq for PolarVec {}

impl PartialOrd for PolarVec {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}
impl Ord for PolarVec {

    fn cmp(&self, other: &Self) -> Ordering {
        return if self.eq(&other) {
            Ordering::Equal
        } else if self.phi.ne(&other.phi) {
            if self.phi > other.phi {
                Ordering::Greater
            } else { Ordering::Less }
        } else if self.theta.ne(&other.theta) {
            if self.theta > other.theta {
                Ordering::Greater
            } else { Ordering::Less }
        } else {
            if self.r > other.r {
                Ordering::Greater
            } else { Ordering::Less }
        }
    }
}

impl Display for PolarVec {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[Radius: {:?} m, Phi (azimut): {:?}°, Theta (polar): {:?}°]", self.r, self.phi, self.theta)
    }
}

impl PolarVec {
    pub fn new(r: f64, phi: f64, theta: f64) -> PolarVec {
        let (r,phi,theta) = PolarVec::get_uni_coords(r, phi, theta);
        PolarVec {r,phi,theta}
    }

    pub fn get_world_origin() -> PolarVec {
        let (r,phi,theta) = WORLD_ORIGIN;
        let (r,phi,theta) = PolarVec::get_uni_coords(r, phi, theta);
        PolarVec {r,phi,theta}
    }

    pub fn get_radius_in_m(&self) -> f64 {
        self.r
    }

    pub fn get_phi_in_deg(&self) -> f64 {
        self.phi
    }

    pub fn get_theta_in_deg(&self) -> f64 {
        self.theta
    }

    pub fn get_angle_difference_phi(&self, other: &Self) -> f64 {
        other.phi - self.phi
    }

    pub fn get_angle_difference_theta(&self, other: &Self) -> f64 {
        other.theta - self.theta
    }

    pub fn to_vector(&self) -> Vector {
        Vector {
            x: self.r * self.phi.cos() * self.theta.sin(),
            y: self.r * self.phi.sin() * self.theta.sin(),
            z: self.r * self.theta.cos()
        }
    }

    //Transforms the given coordinates into unique coordinate system, to assure that two vectors
    //that have the same physical properties have also the values.
    //For example: (1,10,90), (1,370,90) and (1,370,270) are three different ways to describe the same vector.
    //After transformation, these inputs would all have the values (1,10,90).
    fn get_uni_coords(mut r: f64, mut phi: f64, mut theta: f64) -> (f64,f64,f64) {

        if phi < 0.0 || phi >= 360.0 {
            phi = phi.rem_euclid(360.0);
        }
        if theta < 0.0 || theta >= 180.0 {
            theta = theta.rem_euclid(180.0);
        }

        if r == 0.0 {
            phi = 0.0;
            theta = 0.0;
        }
        else if theta == 0.0 || theta == 180.0 {
            phi = 0.0;
        }

        if r < 0.0 {
            r = r.abs();
            phi = (phi + 180.0) % 360.0;
            theta = 180.0 - theta;
        }

        (r,phi,theta)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod vector {
        use crate::physics::polar_vector::Vector;

        #[test]
        fn creation() {
            let a = Vector::new(0.0, 0.0, 0.0);
            let b = Vector::get_world_origin();

            assert_eq!(a,b);
        }

        #[test]
        fn getter() {
            let a = Vector::new(10.0, 5.05, 6.0);

            assert_eq!(10.0, a.get_x());
            assert_eq!(5.05, a.get_y());
            assert_eq!(6.0, a.get_z());
        }

        #[test]
        fn add() {
            let a = Vector::new(10.0, 5.05, 6.0);
            let b = Vector::new(10.0, 5.05, 6.0);
            let c = a.add(&b);
            let d = Vector::new(20.0, 10.1, 12.0);
            assert_eq!(d,c);
        }

        #[test]
        fn substract() {
            let a = Vector::new(10.0, 5.05, 6.0);
            let b = Vector::new(5.0, 5.05, 8.0);
            let c = a.subtract(&b);
            let d = Vector::new(5.0, 0.0, -2.0);
            assert_eq!(d,c);
        }

        #[test]
        fn to_polar_vector() {
            let a = Vector::new(10.0, 0.0, 0.0);
            let b = a.to_polar_vector();

            println!("{}", b)
        }
    }

    mod polar_vector {
        use super::*;

        #[test]
        fn test_new_vec(){
            let a = PolarVec::new(0.0, 1.0, 1.0);
            let b = PolarVec::new(0.0, 0.0, 0.0);
            assert_eq!(a,b);

            let a = PolarVec::new(5.0, 1.0, 0.0);
            let b = PolarVec::new(5.0, 0.0, 0.0);
            assert_eq!(a,b);

            let a = PolarVec::new(5.0, 1.0, 1.0);
            let b = PolarVec::new(5.0, 361.0, 1.0);
            assert_eq!(a,b);

            let a = PolarVec::new(5.0, 360.0, 1.0);
            let b = PolarVec::new(5.0, 0.0, 1.0);
            assert_eq!(a,b);

            let a = PolarVec::new(5.0, -1.5, 1.0);
            let b = PolarVec::new(5.0, 358.5, 1.0);
            assert_eq!(a,b);

            let a = PolarVec::new(5.0, 1.0, 180.0);
            let b = PolarVec::new(5.0, 1.0, 0.0);
            assert_eq!(a,b);

            let a = PolarVec::new(5.0, 1.0, -1.0);
            let b = PolarVec::new(5.0, 1.0, 179.0);
            assert_eq!(a,b);

            let a = PolarVec::new(5.0, 1.0, 181.0);
            let b = PolarVec::new(5.0, 1.0, 1.0);
            assert_eq!(a,b);

            let a = PolarVec::new(-5.0, 180.0, 90.0);
            let b = PolarVec::new(5.0, 0.0, 90.0);
            assert_eq!(a,b);

            let a = PolarVec::new(-5.0, 200.0, 45.0);
            let b = PolarVec::new(5.0, 20.0, 135.0);
            assert_eq!(a,b);
        }

        #[test]
        fn get_world_origin(){
            let a = PolarVec::get_world_origin();
            assert_eq!(WORLD_ORIGIN.0, a.get_radius_in_m());
            assert_eq!(WORLD_ORIGIN.1, a.get_phi_in_deg());
            assert_eq!(WORLD_ORIGIN.2, a.get_theta_in_deg());
        }

        #[test]
        fn get_field(){
            let a = PolarVec::new(1.0, 1.0, 1.0);
            assert_eq!(1.0, a.get_radius_in_m());
            assert_eq!(1.0, a.get_phi_in_deg());
            assert_eq!(1.0, a.get_theta_in_deg());

            assert_eq!(1.0, a.get_radius_in_m());
            assert_eq!(1.0, a.get_phi_in_deg());
            assert_eq!(1.0, a.get_theta_in_deg());
        }

        #[test]
        fn get_angle_difference(){
            let a = PolarVec::new(1.0, 20.0, 10.0);
            let b = PolarVec::new(1.0, 40.0, 160.0);

            assert_eq!(-20.0, b.get_angle_difference_phi(&a));
            assert_eq!(20.0, a.get_angle_difference_phi(&b));
            assert_eq!(150.0, a.get_angle_difference_theta(&b));
            assert_eq!(- 150.0, b.get_angle_difference_theta(&a));

        }

        #[test]
        fn to_vector(){
            let polar_vec_a = PolarVec::new(10.0, 90.0, 90.0);
            let vec_a = polar_vec_a.to_vector();
            println!("{}", vec_a)
        }

        #[test]
        fn test_partial_eq() {
            let a = PolarVec::new(1.0, 1.0, 1.0);
            let b = PolarVec::new(1.0, 1.0, 1.0);
            let c = PolarVec::new(2.0, 1.0, 1.0);
            assert_eq!(a,b);
            assert_ne!(a,c);
        }
        #[test]
        fn test_total_ordering(){
            let a = PolarVec::new(1.0, 1.0, 1.0);
            let b = PolarVec::new(1.0, 1.0, 1.0);
            let c = PolarVec::new(1.0, 2.0, 1.0);
            let d = PolarVec::new(1.0, 2.0, 2.0);
            let e = PolarVec::new(2.0, 2.0, 2.0);

            assert_eq!(Ordering::Equal, a.cmp(&b));
            assert_eq!(Ordering::Less, b.cmp(&c));
            assert_eq!(Ordering::Less, c.cmp(&d));
            assert_eq!(Ordering::Less, d.cmp(&e));
            assert_eq!(Ordering::Greater, e.cmp(&a));
            assert_eq!(Ordering::Greater, d.cmp(&a));
            assert_eq!(Ordering::Greater, c.cmp(&a));
        }

        #[test]
        fn test_partial_ordering(){
            let a = PolarVec::new(1.0, 1.0, 1.0);
            let b = PolarVec::new(1.0, 1.0, 1.0);
            let c = PolarVec::new(1.0, 2.0, 1.0);
            let d = PolarVec::new(1.0, 2.0, 2.0);
            let e = PolarVec::new(2.0, 2.0, 2.0);

            assert_eq!(Ordering::Equal, a.partial_cmp(&b).unwrap());
            assert_eq!(Ordering::Less, b.partial_cmp(&c).unwrap());
            assert_eq!(Ordering::Less, c.partial_cmp(&d).unwrap());
            assert_eq!(Ordering::Less, d.partial_cmp(&e).unwrap());
            assert_eq!(Ordering::Greater, e.partial_cmp(&a).unwrap());
            assert_eq!(Ordering::Greater, d.partial_cmp(&a).unwrap());
            assert_eq!(Ordering::Greater, c.partial_cmp(&a).unwrap());
        }

        #[test]
        fn test_debug(){
            let a = PolarVec::new(1.0, 1.0, 1.0);
            println!("{:?}", a)
        }

        #[test]
        fn test_display(){
            let a = PolarVec::new(1.0, 1.0, 1.0);
            println!("{}", a)
        }
    }
}

