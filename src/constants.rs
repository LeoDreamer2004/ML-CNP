use nalgebra::Point3;
use std::f64::consts::SQRT_2;

pub const EPS: f64 = 1e-8;

pub const DIST: f64 = SQRT_2;

pub const INIT_POINTS: [Point3<f64>; 4] = [
    Point3::new(1.0, 0.0, 0.0),
    Point3::new(0.0, 1.0, 0.0),
    Point3::new(0.0, 0.0, 1.0),
    Point3::new(1.0, 1.0, 1.0),
];

pub const WL_TEST_ROUNDS: usize = 5;
