mod k_nearest_neigbhors;
mod vector_math;

#[cfg(test)]
mod tests {
    use crate::k_nearest_neigbhors::{knn, Point};
    use crate::vector_math::euclidean_distance;

    #[test]
    fn test_euclidean_distance() {
        let v1: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let v2: Vec<f64> = vec![5.0, 4.0, 3.0, 2.0, 1.0];
        assert!((euclidean_distance(&v1, &v2) - 6.32).abs() < 0.005);
    }

    #[test]
    fn test_basic_functionality() {
        let points = vec![
            Point {
                features: vec![1.0, 2.0],
                label: "A".to_string(),
            },
            Point {
                features: vec![5.0, 5.0],
                label: "B".to_string(),
            },
        ];
        let features = vec![2.0, 3.0];
        let k = 1;

        assert_eq!(knn(k, &features, &points), "A");
    }
}
