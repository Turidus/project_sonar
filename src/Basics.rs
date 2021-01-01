pub mod pole_vec {
    use std::ptr::eq;
    use std::cmp::Ordering;
    use std::fmt::{Display, Formatter};
    use std::fmt;


    #[derive(Debug)]
    struct PoleVec {
        r: f64, //radius in m and range 0..
        phi: f64, //azimut angle in degree and range 0..360
        theta: f64, //polar angle in degree and range 0..180
    }

    impl PartialEq for PoleVec {
        fn eq(&self, other: &Self) -> bool {
            &self.r == &other.r && &self.phi == &other.phi && &self.theta == &other.theta
        }

        fn ne(&self, other: &Self) -> bool {
            !eq(&self, &other)
        }
    }
    impl Eq for PoleVec {}

    impl PartialOrd for PoleVec {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(&other))
        }
    }
    impl Ord for PoleVec {

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

    impl Display for PoleVec {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "[Radius: {:?} m, azimut angle Phi: {:?}°, polar angle Theta: {:?}°]", self.r, self.phi, self.theta)
        }
    }

    impl PoleVec {
        pub fn new(r: f64, phi: f64, theta: f64) -> PoleVec {
            let (r,phi,theta) = PoleVec::get_uni_coords(r,phi,theta);
            PoleVec {r,phi,theta}
        }

        //Projects the coordinates into unique coordinates
        fn get_uni_coords(mut r: f64, mut phi: f64, mut theta: f64) -> (f64,f64,f64) {

            if r == 0.0 {
                phi = 0.0;
                theta = 0.0;
            }
            else if theta == 0.0 || theta == 180.0 {
                phi = 0.0;
            }
            else {
                if phi < 0.0 || phi >= 360.0 {
                    phi = phi.rem_euclid(360.0);
                }
                if theta < 0.0 || theta >= 180.0 {
                    theta = theta.rem_euclid(180.0);
                }
            }

            if r < 0.0 {
                r = r.abs();
                let pre_phi = 180.0 + phi;
                phi = pre_phi.rem_euclid(360.0);
                theta = 180.0 - theta;
            }

            (r,phi,theta)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_new_vec(){
            let a = PoleVec::new(0.0, 1.0, 1.0);
            let b = PoleVec::new(0.0, 0.0, 0.0);
            assert_eq!(a,b);

            let a = PoleVec::new(5.0, 1.0, 0.0);
            let b = PoleVec::new(5.0, 0.0, 0.0);
            assert_eq!(a,b);

            let a = PoleVec::new(5.0, 1.0, 1.0);
            let b = PoleVec::new(5.0, 361.0, 1.0);
            assert_eq!(a,b);

            let a = PoleVec::new(5.0, -1.5, 1.0);
            let b = PoleVec::new(5.0, 358.5, 1.0);
            assert_eq!(a,b);

            let a = PoleVec::new(5.0, 1.0, -1.0);
            let b = PoleVec::new(5.0, 1.0, 179.0);
            assert_eq!(a,b);

            let a = PoleVec::new(5.0, 1.0, 181.0);
            let b = PoleVec::new(5.0, 1.0, 1.0);
            assert_eq!(a,b);

            let a = PoleVec::new(-5.0, 180.0, 90.0);
            let b = PoleVec::new(5.0, 0.0, 90.0);
            assert_eq!(a,b);

            let a = PoleVec::new(-5.0, 200.0, 45.0);
            let b = PoleVec::new(5.0, 20.0, 135.0);
            assert_eq!(a,b);
        }

        #[test]
        fn test_partial_eq() {
            let a = PoleVec::new(1.0, 1.0, 1.0);
            let b = PoleVec::new(1.0, 1.0, 1.0);
            let c = PoleVec::new(2.0, 1.0, 1.0);
            assert_eq!(a,b);
            assert_ne!(a,c);
        }
        #[test]
        fn test_total_ordering(){
            let a = PoleVec::new(1.0, 1.0, 1.0);
            let b = PoleVec::new(1.0, 1.0, 1.0);
            let c = PoleVec::new(1.0, 2.0, 1.0);
            let d = PoleVec::new(1.0, 2.0, 2.0);
            let e = PoleVec::new(2.0, 2.0, 2.0);

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
            let a = PoleVec::new(1.0, 1.0, 1.0);
            let b = PoleVec::new(1.0, 1.0, 1.0);
            let c = PoleVec::new(1.0, 2.0, 1.0);
            let d = PoleVec::new(1.0, 2.0, 2.0);
            let e = PoleVec::new(2.0, 2.0, 2.0);

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
            let a = PoleVec::new(1.0, 1.0, 1.0);
            println!("{:?}", a)
        }

        #[test]
        fn test_display(){
            let a = PoleVec::new(1.0, 1.0, 1.0);
            println!("{}", a)
        }

    }
}