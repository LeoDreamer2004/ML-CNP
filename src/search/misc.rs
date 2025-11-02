use crate::linalg::point_to_line_proj;
use nalgebra::Point3;

/// Calculates the rotation angle required to rotate a point around an axis
/// defined by two points, such that the point reaches a specified distance from its original position.
///
/// # Arguments
///
/// * `original` - A reference to the `Point3<f64>` representing the original position of the point.
/// * `axis_start` - A reference to the `Point3<f64>` marking the start of the rotation axis.
/// * `axis_end` - A reference to the `Point3<f64>` marking the end of the rotation axis.
/// * `target_dist` - The target distance from the original position after rotation.
///
/// # Returns
///
/// * The rotation angle in radians as `f64`.
pub fn get_rotate_angle(
    original: &Point3<f64>,
    axis_start: &Point3<f64>,
    axis_end: &Point3<f64>,
    target_dist: f64,
) -> f64 {
    let proj = point_to_line_proj(original, axis_start, axis_end);
    let dist = (proj - original).norm();
    (target_dist / 2.0 / dist).asin() * 2.0
}
