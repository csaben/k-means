use rand::{distributions::Uniform, Rng}; // 0.8.0
pub mod plotting;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64
}

pub fn generate_points(k: usize) -> Vec<Point> {
    let mut rng = rand::thread_rng();
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
    pub fn randomly_initialize(k: usize, points: &[Point]) -> Vec<Centroid> {
        let mut rng = rand::thread_rng();
        (0..k)
            .map(|_| {
                let index = rng.gen_range(0..points.len());
                Centroid {
                    coordinate: points[index],
                }
            })
            .collect()
    }
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

// pub fn assign_clusters(points: Vec<Point>, centroids: Vec<Centroid>) {
//     // (0..points.len()).map()
// }
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

pub fn k_means_naive(k: usize, points: Vec<Point>, max_iterations: usize) -> Vec<Vec<Point>> {
    let mut centroids = Centroids::randomly_initialize(k, &points);
    let mut clusters: Vec<Vec<Point>>;
    
    for _ in 0..max_iterations {
        clusters = assign_clusters(&points, &centroids);
        
        let new_centroids: Vec<Centroid> = clusters
            .iter()
            .map(|cluster| calculate_centroid(cluster.to_vec()))
            .collect();
        
        if new_centroids == centroids {
            break;
        }
        
        centroids = new_centroids;
    }
    
    assign_clusters(&points, &centroids)
}