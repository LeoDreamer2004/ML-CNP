mod misc;

use crate::constants::{DIST, EPS};
use crate::graph::hash::Hashable;
use crate::graph::{Graph, VecVecGraph};
use crate::linalg::rotate_point;
use approx::AbsDiffEq;
use nalgebra::Point3;
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::Mutex;

/// Rotates a set of 3D points around a specified axis defined by two points, `axis_a` and `axis_b`.
///
/// # Arguments
///
/// * `points` - A slice of 3D points to be rotated.
/// * `target` - The target point which, along with `axis_a` and `axis_b`, helps in calculating the
///   rotation angle.
/// * `axis_a` - The first point defining the axis of rotation.
/// * `axis_b` - The second point defining the axis of rotation.
/// * `dist` - The distance between `target` and it's rotated point.
///
/// # Returns
///
/// A tuple containing two vectors:
/// - The first vector contains the points after rotating them around the axis from `axis_a` to
///   `axis_b` by the calculated angle.
/// - The second vector contains the points after rotating them around the axis from `axis_b` to
///   `axis_a` by the negative of the calculated angle.
pub fn rotate(
    points: &[Point3<f64>],
    target: &Point3<f64>,
    axis_a: &Point3<f64>,
    axis_b: &Point3<f64>,
    dist: f64,
) -> (Vec<Point3<f64>>, Vec<Point3<f64>>) {
    let angle = misc::get_rotate_angle(target, axis_a, axis_b, dist);

    let res1 = points
        .iter()
        .map(|p| rotate_point(p, axis_a, axis_b, angle))
        .collect();

    let res2 = points
        .iter()
        .map(|p| rotate_point(p, axis_a, axis_b, -angle))
        .collect();

    (res1, res2)
}

/// Computes the number of critical pairs between two sets of 3D points.
///
/// A pair of points, one from each set, is considered a critical pair if the Euclidean
/// distance between them is equal to the specified `dist` value, within a tolerance defined by `EPS`.
///
/// # Arguments
///
/// * `points1` - A slice of the first set of 3D points.
/// * `points2` - A slice of the second set of 3D points.
/// * `dist` - The target distance for which to find critical pairs.
///
/// # Returns
///
/// * The count of critical pairs that meet the distance criteria.
///
/// # Examples
///
/// ```
/// # use nalgebra::Point3;
/// # use ml_cnp::search::critical_pair_cnt;
///
/// let points1 = vec![Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0)];
/// let points2 = vec![Point3::new(1.0, 0.0, 0.0), Point3::new(2.0, 0.0, 0.0)];
/// let dist = 1.0;
/// let cnt = critical_pair_cnt(&points1, &points2, dist);
/// assert_eq!(cnt, 2);
/// ```
pub fn critical_pair_cnt(points1: &[Point3<f64>], points2: &[Point3<f64>], dist: f64) -> usize {
    let mut cnt = 0;
    for p1 in points1 {
        for p2 in points2 {
            if (p1 - p2).norm().abs_diff_eq(&dist, EPS) {
                cnt += 1;
            }
        }
    }

    cnt
}

/// Merges two slices of 3D points, `points1` and `points2`.
///
/// # Arguments
///
/// * `points1` - A slice of 3D points.
/// * `points2` - Another slice of 3D points.
///
/// # Returns
///
/// A `Vec<Point3<f64>>` representing the merged list of points.
///
/// # Examples
/// ```
/// # use nalgebra::Point3;
/// # use ml_cnp::search::merge;
///
/// let points1 = vec![Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0)];
/// let points2 = vec![Point3::new(1.0, 0.0, 0.0), Point3::new(2.0, 0.0, 0.0)];
///
/// let merged = merge(&points1, &points2);
///
/// assert_eq!(merged, vec![Point3::new(1.0, 0.0, 0.0), Point3::new(0.0, 0.0, 0.0), Point3::new(2.0, 0.0, 0.0)]);
/// ```
pub fn merge(points1: &[Point3<f64>], points2: &[Point3<f64>]) -> Vec<Point3<f64>> {
    // first, find points in both points1 and points2
    let mut res: Vec<_> = points1
        .iter()
        .filter(|p| points2.iter().any(|x| x.abs_diff_eq(p, EPS)))
        .cloned()
        .collect();

    let points1: Vec<_> = points1
        .iter()
        .filter(|p| !res.iter().any(|x| x.abs_diff_eq(p, EPS)))
        .collect();

    let points2: Vec<_> = points2
        .iter()
        .filter(|p| !res.iter().any(|x| x.abs_diff_eq(p, EPS)))
        .collect();

    res.extend(points1);
    res.extend(points2);

    res
}

fn points_hash(points: &[Point3<f64>]) -> u64 {
    let graph = VecVecGraph::from_points(points, DIST);
    graph.hash()
}

/// Generates the next layer of graphs by rotating and merging points.
///
/// This function takes a slice of 3D point graphs and generates a new set of
/// graphs by performing rotations and merges on the input graphs. For each graph,
/// it iterates over all pairs of points, rotates the graph around these points,
/// and then merges the rotated graph back with the original. The resulting unique
/// graphs (based on a hash of their points) are collected and returned as a vector.
///
/// # Arguments
///
/// * `graphs` - A slice of 3D point graphs, where each graph is represented as a vector of `Point3<f64>`.
///
/// # Returns
///
/// A `Vec<Vec<Point3<f64>>>` containing the newly generated graphs after rotation and merging.
pub fn next_layer(graphs: &[Vec<Point3<f64>>]) -> Vec<Vec<Point3<f64>>> {
    let graph_set = Mutex::new(HashMap::new());

    graphs.par_iter().for_each(|graph| {
        let len = graph.len();
        for i in 0..len {
            for j in i + 1..len {
                for p in graph {
                    if p.abs_diff_eq(&graph[i], EPS) || p.abs_diff_eq(&graph[j], EPS) {
                        continue;
                    }

                    let (rotated1, rotated2) = rotate(graph, p, &graph[i], &graph[j], DIST);
                    let merged = merge(&rotated1, graph);
                    let hash = points_hash(&merged);
                    graph_set.lock().unwrap().entry(hash).or_insert(merged);

                    let merged = merge(&rotated2, graph);
                    let hash = points_hash(&merged);
                    graph_set.lock().unwrap().entry(hash).or_insert(merged);
                }
            }
        }
    });

    graph_set.lock().unwrap().values().cloned().collect()
}

pub fn prune_layer(mut graphs: Vec<Vec<Point3<f64>>>, target_size: usize) -> Vec<Vec<Point3<f64>>> {
    graphs.sort_by_key(|g| {
        let graph = VecVecGraph::from_points(g, DIST);
        graph.edges().len() / g.len()
    });
    graphs.into_iter().take(target_size).collect()
}
