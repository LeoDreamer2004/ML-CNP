use nalgebra::{Matrix3, Point3};

/// Rotates a 3D point around a specified axis by a given angle.
///
/// # Arguments
///
/// * `point` - A reference to the 3D point to be rotated, represented as a `Point3<f64>`.
/// * `axis_start` - A reference to the starting point of the rotation axis, also a `Point3<f64>`.
/// * `axis_end` - A reference to the ending point of the rotation axis, used along with `axis_start` to define the direction of the axis, `Point3<f64>`.
/// * `angle_rad` - The angle of rotation in radians, `f64`.
///
/// # Returns
/// * A `Point3<f64>` representing the new position of the point after the rotation.
///
/// # Notes
///
/// - The function first calculates the normalized axis vector from `axis_start` to `axis_end`.
/// - It then constructs the rotation matrix based on the axis and the angle.
/// - The input point is translated to the origin, rotated, and then translated back.
/// - This method assumes the use of the `nalgebra` crate for handling points and matrices.
pub(crate) fn rotate_point(
    point: &Point3<f64>,
    axis_start: &Point3<f64>,
    axis_end: &Point3<f64>,
    angle_rad: f64,
) -> Point3<f64> {
    let axis_vector = axis_end - axis_start;
    let a = axis_vector.normalize();
    let (ax, ay, az) = (a.x, a.y, a.z);

    let cos_theta = angle_rad.cos();
    let sin_theta = angle_rad.sin();
    let t = 1.0 - cos_theta;

    let r = Matrix3::new(
        cos_theta + ax * ax * t,
        ax * ay * t - az * sin_theta,
        ax * az * t + ay * sin_theta,
        ay * ax * t + az * sin_theta,
        cos_theta + ay * ay * t,
        ay * az * t - ax * sin_theta,
        az * ax * t - ay * sin_theta,
        az * ay * t + ax * sin_theta,
        cos_theta + az * az * t,
    );

    let translated_point = point - axis_start;
    let rotated_translated = r * translated_point;
    axis_start + rotated_translated
}

/// Projects a given 3D point onto a line defined by two points.
///
/// # Arguments
/// * `point` - A reference to the 3D point to be projected, of type `Point3<f64>`.
/// * `axis_a` - A reference to one of the two 3D points that define the line, of type `Point3<f64>`.
/// * `axis_b` - A reference to the other 3D point defining the line, of type `Point3<f64>`.
///
/// # Returns
/// * A `Point3<f64>` representing the projection of the input point onto the line.
pub(crate) fn point_to_line_proj(
    point: &Point3<f64>,
    axis_a: &Point3<f64>,
    axis_b: &Point3<f64>,
) -> Point3<f64> {
    let ab = axis_b - axis_a;

    let ap = point - axis_a;

    if ab.norm_squared() < 1e-10 {
        return *axis_a;
    }

    let t = ap.dot(&ab) / ab.norm_squared();

    axis_a + ab * t
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;
    use std::f64::consts::PI;

    #[test]
    fn test_rotate() {
        let original = Point3::new(0.0, 0.0, 0.0);
        let x = Point3::new(1.0, 1.0, 0.0);
        let y = Point3::new(1.0, 1.0, 1.0);

        let rotated = rotate_point(&original, &x, &y, PI / 2.0);
        assert_abs_diff_eq!(rotated, Point3::new(2.0, 0.0, 0.0));
    }
}
