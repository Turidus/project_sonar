#![allow(dead_code)]
//Acceleration through gravity on earth at 0 m elevation.
pub(crate) static G_ACC: f64 = 9.81;
//World origin, is always (0.0, 0.0, 0.0).
pub(crate) static WORLD_ORIGIN: (f64,f64,f64) = (0.0, 0.0, 0.0);
/// The maximal difference between two double values until which they are still considered equal.
pub(crate) static F64_DELTA: f64 = 0.000001;