/*
 * “Commons Clause” License Condition v1.0
 *
 * The Software is provided to you by the Licensor under the License, as defined below, subject to the following condition.
 *
 * Without limiting other conditions in the License, the grant of rights under the License will not include, and the License does not grant to you, the right to Sell the Software.
 *
 * For purposes of the foregoing, “Sell” means practicing any or all of the rights granted to you under the License to provide to third parties, for a fee or other consideration (including without limitation fees for hosting or consulting/ support services related to the Software), a product or service whose value derives, entirely or substantially, from the functionality of the Software. Any license notice or attribution required by the License must also include this Commons Cause License Condition notice.
 *
 * Software: project_sonar
 *
 * License: MIT
 *
 * Licensor: Lars Schulze-Falck
 *
 *
 * MIT License
 *
 * Copyright (c) 2021 Lars Schulze-Falck
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */

//! This crate contains the implementations of cartesian and polar vectors.

use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::fmt;
use crate::constants::WORLD_ORIGIN;
use std::f64::consts::{PI, FRAC_PI_2, TAU};
use crate::utils::helper_functions::*;


/// A cartesian vector from three double (```f64```) values.
/// * **x** points east
/// * **y** points north
/// * **z** points up
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
    /// Generates a new instance of vector.
    /// # Examples
    /// ```rust
    /// let vec = Vector::new(3.0, 5.0, -4.0);
    /// assert_eq!(3.0, vec.x);
    /// assert_eq!(5.0, vec.y);
    /// assert_eq!(-4.0, vec.z);
    /// ```
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector{x,y,z}
    }
    /// Returns a vector containing the [world origin] (0,0,0).
    /// # Examples
    /// ```rust
    /// let vec = Vector::get_world_origin();
    /// assert_eq!(0.0, vec.x);
    /// assert_eq!(0.0, vec.y);
    /// assert_eq!(0.0, vec.z);
    /// ```
    ///
    /// [world origin]: GetLinkLocation
    pub fn get_world_origin() -> Vector {
        let (x,y,z) = WORLD_ORIGIN;
        Vector{x,y,z}
    }
    /// Returns the x value of a vector.
    /// # Examples
    /// ```rust
    /// let vec = Vector::new(3.0, 5.0, -4.0);
    /// assert_eq!(3.0, vec.get_x());
    /// ```
    pub fn get_x(&self) -> f64 {
        self.x
    }
    /// Returns the y value of a vector.
    /// # Examples
    /// ```rust
    /// let vec = Vector::new(3.0, 5.0, -4.0);
    /// assert_eq!(5.0, vec.get_y());
    /// ```
    pub fn get_y(&self) -> f64 {
        self.y
    }
    /// Returns the z value of a vector.
    /// # Examples
    /// ```rust
    /// let vec = Vector::new(3.0, 5.0, -4.0);
    /// assert_eq!(-4.0, vec.get_z());
    /// ```
    pub fn get_z(&self) -> f64 {
        self.z
    }
    /// Returns a new vector created from the added values from another vector.
    /// # Examples
    /// ```rust
    /// let vec = Vector::new(3.0, 5.0, -4.0);
    /// let other_vec = Vector::new(-1.0, 2.0, 3.0);
    /// let vec = vec.add(&other_vec)
    /// assert_eq!(2.0, vec.get_x());
    /// assert_eq!(7.0, vec.get_y());
    /// assert_eq!(-1.0, vec.get_z());
    /// ```
    pub fn add(&self, other: &Self) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
    /// Returns a new vector created by subtracting another vector.
    /// # Examples
    /// ```rust
    /// let vec = Vector::new(3.0, 5.0, -4.0);
    /// let other_vec = Vector::new(-1.0, 2.0, 3.0);
    /// let vec = vec.sub(&other_vec)
    /// assert_eq!(4.0, vec.get_x());
    /// assert_eq!(3.0, vec.get_y());
    /// assert_eq!(-7.0, vec.get_z>());
    /// ```
    pub fn sub(&self, other: &Self) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
    /// Return a [PolarVector] representation of the vector.
    /// # Examples
    /// ```rust
    /// let a = Vector::new(10.0, 0.0, 0.0);
    /// let b = a.to_polar_vector();
    /// let abs_difference_phi = b.get_phi_in_rad().abs();
    /// let abs_difference_theta = b.get_theta_in_rad().abs() - FRAC_PI_2;
    /// assert_eq!(10.0, b.get_radius());
    /// assert!(abs_difference_phi < 0.00001);
    /// assert!(abs_difference_theta < 0.00001);
    /// ```
    pub fn to_polar_vector(&self) -> PolarVec {
        let r = (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt();
        PolarVec {
            r,
            phi: self.y.atan2(self.x),
            theta: (self.z / r).acos()
        }
    }
}


#[derive(Debug, Copy, Clone)]
pub struct PolarVec {
    r: f64, //radius in m and range 0..
    phi: f64, //azimut angle in rad and range 0..2*pi
    theta: f64, //polar angle in rad and range 0..pi
}

impl PartialEq for PolarVec {
    fn eq(&self, other: &Self) -> bool {
        return {
            (self.r == other.r) &&
                equal_with_delta(self.phi, other.phi) &&
                equal_with_delta(self.theta, other.theta)
        }
    }
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

    pub fn get_radius(&self) -> f64 {
        self.r
    }

    pub fn get_phi_in_rad(&self) -> f64 {
        self.phi
    }

    pub fn get_theta_in_rad(&self) -> f64 {
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

    fn get_uni_coords(mut r: f64, mut phi: f64, mut theta: f64) -> (f64,f64,f64) {

        if phi < 0.0 || phi >= TAU {
            phi = phi.rem_euclid(TAU);
        }
        if theta < 0.0 || theta >= PI {
            theta = theta.rem_euclid(PI);
        }

        if r == 0.0 {
            phi = 0.0;
            theta = 0.0;
        }
        else if theta == 0.0 || theta == PI {
            phi = 0.0;
        }

        if r < 0.0 {
            r = r.abs();
            phi = (phi + PI) % TAU;
            theta = PI - theta;
        }

        (r,phi,theta)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod vector {
        use crate::physics::vectors::Vector;
        use std::f64::consts::FRAC_PI_2;

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
        fn sub() {
            let a = Vector::new(10.0, 5.05, 6.0);
            let b = Vector::new(5.0, 5.05, 8.0);
            let c = a.sub(&b);
            let d = Vector::new(5.0, 0.0, -2.0);
            assert_eq!(d,c);
        }

        #[test]
        fn to_polar_vector() {
            let a = Vector::new(10.0, 0.0, 0.0);
            let b = a.to_polar_vector();
            let abs_difference_phi = b.get_phi_in_rad().abs();
            let abs_difference_theta = b.get_theta_in_rad().abs() - FRAC_PI_2;
            assert_eq!(10.0, b.get_radius());
            assert!(abs_difference_phi < 0.00001);
            assert!(abs_difference_theta < 0.00001);
        }
    }

    mod polar_vector {
        use super::*;
        use std::f64::consts::{FRAC_PI_8, FRAC_PI_4};

        #[test]
        fn test_new_vec(){
            let a = PolarVec::new(0.0, 1.0, 1.0);
            let b = PolarVec::new(0.0, 0.0, 0.0);
            assert_eq!(a,b);

            let a = PolarVec::new(5.0, 1.0, 0.0);
            let b = PolarVec::new(5.0, 0.0, 0.0);
            assert_eq!(a,b);

            let a = PolarVec::new(5.0, FRAC_PI_8, FRAC_PI_8);
            let b = PolarVec::new(5.0, TAU + FRAC_PI_8, FRAC_PI_8);
            assert_eq!(a,b);

            let a = PolarVec::new(5.0, TAU, FRAC_PI_8);
            let b = PolarVec::new(5.0, 0.0, FRAC_PI_8);
            assert_eq!(a,b);

            let a = PolarVec::new(5.0, -PI, 1.0);
            let b = PolarVec::new(5.0, PI, 1.0);
            assert_eq!(a,b);

            let a = PolarVec::new(5.0, FRAC_PI_8, PI);
            let b = PolarVec::new(5.0, 0.0, 0.0);
            assert_eq!(a,b);

            let a = PolarVec::new(5.0, FRAC_PI_8, - FRAC_PI_4);
            let b = PolarVec::new(5.0, FRAC_PI_8, PI - FRAC_PI_4);
            assert_eq!(a,b);

            let a = PolarVec::new(5.0, FRAC_PI_8, PI + FRAC_PI_4);
            let b = PolarVec::new(5.0, FRAC_PI_8, FRAC_PI_4);
            assert_eq!(a,b);

            let a = PolarVec::new(-5.0, PI, FRAC_PI_2);
            let b = PolarVec::new(5.0, 0.0, FRAC_PI_2);
            assert_eq!(a,b);

            let a = PolarVec::new(-5.0, PI + FRAC_PI_8, FRAC_PI_4);
            let b = PolarVec::new(5.0, FRAC_PI_8, FRAC_PI_2 + FRAC_PI_4);
            assert_eq!(a,b);
        }

        #[test]
        fn get_world_origin(){
            let a = PolarVec::get_world_origin();
            assert_eq!(WORLD_ORIGIN.0, a.get_radius());
            assert_eq!(WORLD_ORIGIN.1, a.get_phi_in_rad());
            assert_eq!(WORLD_ORIGIN.2, a.get_theta_in_rad());
        }

        #[test]
        fn get_field(){
            let a = PolarVec::new(1.0, 1.0, 1.0);
            assert_eq!(1.0, a.get_radius());
            assert_eq!(1.0, a.get_phi_in_rad());
            assert_eq!(1.0, a.get_theta_in_rad());

            assert_eq!(1.0, a.get_radius());
            assert_eq!(1.0, a.get_phi_in_rad());
            assert_eq!(1.0, a.get_theta_in_rad());
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

