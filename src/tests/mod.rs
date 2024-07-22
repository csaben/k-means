#[cfg(test)]
mod tests {
    use crate::*;
    use rand::SeedableRng;
    use rand::rngs::StdRng;
    use std::time::Instant;

    #[test]
    fn speed_test() {
        let clusters = 5;
        let samples = 1000;
        let max_iterations = 100;
        let seed = 420; // Use a fixed seed for reproducibility

        let mut rng = StdRng::seed_from_u64(seed);
        let points = generate_points(samples, seed);

        // Time randomly_initialize
        let start = Instant::now();
        let _result_random = k_means_algo(clusters, points.clone(), max_iterations, Centroids::randomly_initialize, seed);
        let duration_random = start.elapsed();
        println!("Time taken for randomly_initialize: {:?}", duration_random);

        // Time plus_plus_initialize
        let start = Instant::now();
        let _result_plus_plus = k_means_algo(clusters, points.clone(), max_iterations, Centroids::plus_plus_initialize, seed);
        let duration_plus_plus = start.elapsed();
        println!("Time taken for plus_plus_initialize: {:?}", duration_plus_plus);

        // Time fast_plus_plus_initialize
        let start = Instant::now();
        let _result_fast_plus_plus = k_means_algo(clusters, points.clone(), max_iterations, Centroids::fast_plus_plus_initialize, seed);
        let duration_fast_plus_plus = start.elapsed();
        println!("Time taken for fast_plus_plus_initialize: {:?}", duration_fast_plus_plus);

        // You might want to add some assertions here to ensure the functions complete successfully
        assert!(duration_random.as_secs() < 60, "randomly_initialize took too long");
        assert!(duration_plus_plus.as_secs() < 60, "plus_plus_initialize took too long");
        assert!(duration_fast_plus_plus.as_secs() < 60, "fast_plus_plus_initialize took too long");
    }
}