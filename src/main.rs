use rand::{distributions::Uniform, Rng}; // 0.8.0
// use k_means::{generate_points, Point, calculate_centroid};


// fn main() {
//     let samples: usize = 5;
//     let clusters: usize = 2;
//     let points = generate_points(samples);
//     let centroid = calculate_centroid(points);
//     println!("{:?}", centroid)
//     // k_means_naive(clusters, points);
    
// }

// pub fn k_means_naive(k: usize, points: Vec<Point>) {
//     // takes k clusters and Vec<Point> and returns Points organized by clusters
//     let clusters = (0..k).map(|_| Vec::new() as Vec<Point>).collect::<Vec<_>>();
//     println!("{:?}", clusters);
// }

use k_means::{generate_points, Point, k_means_naive};
use k_means::plotting::{plot_clusters};

// fn main() {
//     let samples: usize = 100;
//     let clusters: usize = 3;
//     let points = generate_points(samples);
//     let max_iterations = 100;
    
//     let result = k_means_naive(clusters, points, max_iterations);
    
//     for (i, cluster) in result.iter().enumerate() {
//         println!("Cluster {}: {} points", i, cluster.len());
//         println!("{:?}", cluster);
//     }
// }

use k_means::{Centroid, calculate_centroid};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let samples: usize = 100;
    let clusters: usize = 3;
    let points = generate_points(samples);
    let max_iterations = 100;
    
    let result = k_means_naive(clusters, points.clone(), max_iterations);
    
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