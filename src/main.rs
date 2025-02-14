use std::env;
use rand::{distributions::Uniform, Rng}; // 0.8.0
use k_means::plotting::{plot_clusters};
use k_means::{Centroids, Centroid, calculate_centroid, generate_points, Point, k_means_algo};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // expects `cargo run NUM_CLUSTERS`
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("number of clusters required!");
        return Ok(());
    }
    // turbofish dis bish
    let clusters: usize = args[1].parse::<usize>().unwrap();
    let seed: u64 = 42;
    let samples: usize = 100;
    let max_iterations = 100;

    let points = generate_points(samples, seed);
    
    // let result = k_means_algo(clusters, points.clone(), max_iterations, Centroids::randomly_initialize, seed);
    // let result = k_means_algo(clusters, points.clone(), max_iterations, Centroids::plus_plus_initialize, seed);
    let result = k_means_algo(clusters, points.clone(), max_iterations, Centroids::fast_plus_plus_initialize, seed);
    
    let centroids: Vec<Centroid> = result
        .iter()
        .map(|cluster| calculate_centroid(cluster.to_vec()))
        .collect();

    // Use the plotting function
    plot_clusters(&points, &result, &centroids, "kmeans_plot.png")?;

    for (i, cluster) in result.iter().enumerate() {
        println!("Cluster {}: {} points", i, cluster.len());
    }

    Ok(())
}