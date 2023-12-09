use std::collections::HashMap;

use crate::vector_math::euclidean_distance;

pub struct Point {
    pub features: Vec<f64>,
    pub label: String,
}

pub fn knn(k: usize, features: &[f64], points: &[Point]) -> String {
    let mut distance_to_points = points
        .iter()
        .map(|point| (point, euclidean_distance(&point.features, features)))
        .collect::<Vec<(&Point, f64)>>();

    distance_to_points.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let k_nearest_neighbors = &distance_to_points[0..k].into_iter();
    let mut label_counts = HashMap::new();
    for &(point, _distance) in k_nearest_neighbors.clone() {
        *label_counts.entry(point.label.clone()).or_insert(0) += 1;
    }

    label_counts
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(label, _)| label)
        .unwrap_or_default()
}
