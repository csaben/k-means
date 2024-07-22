use rand::{distributions::Uniform, Rng}; // 0.8.0
// for seed in speed testing
use rand::SeedableRng;
use rand::rngs::StdRng;
// for fast_plus_plus_initialize
use rand::distributions::{Distribution, WeightedIndex};
pub mod plotting;
pub mod tests;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64
}

pub fn generate_points(k: usize, seed: u64) -> Vec<Point> {
    // let mut rng = rand::thread_rng();
    let mut rng = StdRng::seed_from_u64(seed);

    let range = Uniform::from(0..100);
    
    (0..k).map(|_| Point {
        x: rng.sample(&range) as f64,
        y: rng.sample(&range) as f64,
    }).collect()
}

pub fn distance(point_1: Point, point_2: Point) -> f64 {
    let dx: f64 = point_1.x as f64 - point_2.x as f64;
    let dy: f64 = point_1.y as f64 - point_2.y as f64;
    ((dx * dx) + (dy * dy)).sqrt()

}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Centroid {
    // A centroid is a data point that represents the center of the cluster (the mean)
    pub coordinate: Point
}

#[derive(Debug, Clone, PartialEq)]
pub struct Centroids {
    pub centroids: Vec<Centroid>
}

impl Centroids {
    pub fn randomly_initialize(k: usize, points: &[Point], seed: u64) -> Vec<Centroid> {
        // o(k * n)
        // let mut rng = rand::thread_rng();
        let mut rng = StdRng::seed_from_u64(seed);
        (0..k)
            .map(|_| {
                let index = rng.gen_range(0..points.len());
                Centroid {
                    coordinate: points[index],
                }
            })
            .collect()
    }

    pub fn plus_plus_initialize(k: usize, points: &[Point], seed: u64) -> Vec<Centroid> {
        // o(k^2 * n)
        // let mut rng = rand::thread_rng();
        let mut rng = StdRng::seed_from_u64(seed);

        let mut centroids: Vec<Centroid> = Vec::with_capacity(k);
        centroids.push(Centroid{ coordinate: points[0].clone()});

        for _ in 1..k {
            let mut dist_sq: Vec<f64> = Vec::with_capacity(points.len());
            for point in points {
                let min_dist = centroids.iter()
                    .map(|c| distance(*point, c.coordinate))
                    // what does this line do?
                    // map returned an iterable list of distances
                    // min_by goes through and compares idx 0 to 1, .. and returns smallest dist value
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap();
                dist_sq.push(min_dist * min_dist);
            }

            let total: f64 = dist_sq.iter().sum();
            let probs: Vec<f64> = dist_sq.iter().map(|&d| d / total).collect();
            let cumsum_vec: Vec<f64> = cumsum(&probs);

            let r: f64 = rng.gen();
            let j = cumsum_vec.iter().position(|&p| r < p).unwrap();
            centroids.push(Centroid {coordinate: points[j].clone() } );
        }

        centroids
    }
    pub fn fast_plus_plus_initialize(k: usize, points: &[Point], seed: u64) -> Vec<Centroid> {
        // let mut rng = rand::thread_rng();
        let mut rng = StdRng::seed_from_u64(seed);
        let mut centroids: Vec<Centroid> = Vec::with_capacity(k);
        let mut distances: Vec<f64> = vec![f64::INFINITY; points.len()];
        
        // Choose the first centroid randomly
        let first_index = rng.gen_range(0..points.len());
        centroids.push(Centroid { coordinate: points[first_index].clone() });
        
        for _ in 1..k {
            let mut total_distance = 0.0;
            
            // Update distances
            for (i, point) in points.iter().enumerate() {
                let dist = distance(*point, centroids.last().unwrap().coordinate);
                if dist < distances[i] {
                    distances[i] = dist;
                }
                total_distance += distances[i] * distances[i];
            }
            
            // Choose next centroid
            let dist = WeightedIndex::new(distances.iter().map(|d| d * d)).unwrap();
            let chosen_index = dist.sample(&mut rng);
            centroids.push(Centroid { coordinate: points[chosen_index].clone() });
        }
        
        centroids
    }
}

fn cumsum(probs: &[f64]) -> Vec<f64> {
    let mut cumulative_probs = Vec::with_capacity(probs.len());
    let mut sum = 0.0;
    for &prob in probs {
        sum += prob;
        cumulative_probs.push(sum);
    }
    cumulative_probs
}


pub fn calculate_centroid(cluster: Vec<Point>) -> Centroid {
    // cluster.iter().x.sum() / cluster.len()
    let size: f64 = cluster.len() as f64;
    let avg_x = cluster.iter().map(|p| p.x).sum::<f64>()/size ;
    let avg_y = cluster.iter().map(|p| p.y).sum::<f64>()/size;

    Centroid {
        coordinate: Point {
        x: avg_x,
        y: avg_y
        }
    }
}


pub fn assign_clusters(points: &[Point], centroids: &[Centroid]) -> Vec<Vec<Point>> {
    let mut clusters: Vec<Vec<Point>> = vec![Vec::new(); centroids.len()];
    
    for point in points {
        let closest_centroid_index = centroids
            .iter()
            .enumerate()
            .min_by_key(|(_, centroid)| {
                let dist = distance(*point, centroid.coordinate);
                (dist * 1000.0) as i64 // Convert to integer for comparison
            })
            .map(|(index, _)| index)
            .unwrap();
        
        clusters[closest_centroid_index].push(*point);
    }
    
    clusters
}

pub fn k_means_algo<F>(
    k: usize,
    points: Vec<Point>,
    max_iterations: usize,
    init_func: F,
    seed: u64
) -> Vec<Vec<Point>>
where
    F: Fn(usize, &[Point], u64) -> Vec<Centroid>
    {
        let mut centroids = init_func(k, &points, seed);
        
        for iter in 0..max_iterations {
            let clusters = assign_clusters(&points, &centroids);
            
            let new_centroids: Vec<Centroid> = clusters
                .iter()
                .map(|cluster| calculate_centroid(cluster.to_vec()))
                .collect();
            
            if new_centroids == centroids {
                println!{"local minimum found before max iterations reached! {}/{} iters needed:", iter, max_iterations};
                break;
            }
            
            centroids = new_centroids;
        }
        
        assign_clusters(&points, &centroids)
    }