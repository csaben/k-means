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
    pub fn plus_plus_initialize(k: usize, points: &[Point]) -> Vec<Centroid> {
        let mut rng = rand::thread_rng();
        // set initial centroid as first
        let centroids: Vec[Point] = Vec::with_capacity(k);
        let c0: Point = points[0];
        centroids.push(c0);
        for _ k.iter() {
            // calculate dist b/w point and it's nearest centroid
            // get list of those
            dist_sq: Vec<f64> = Vec::with_capacity(points.len());
            for point in points {
                let closest_centroid_index = centroids
                .iter()
                .enumerate()
                .min_by_key(|(_, centroid)| {
                    let dist = distance(*point, centroid.coordinate);
                    (dist * 1000.0) as i64 // Convert to integer for comparison
                } )
                .map(|(index, _)| index)
                .unwrap();
            }
            dist_sq[closest_centroid_index].push(*point);

            // calculate prob / sum for each element, ps
            let probs: Vec<f64> = dist_sq.iter()
            .map(|&d| d / dist_sq.iter().sum()::<f64>())
            .collect();

            // calculate cumsum (.1,.2,.4..3)->(.1,.3,.7,1.0)
            pub fn cumsum(probs: Vec<f64>)-> Vec<f64> {
                // O(n)
                let mut cumulative_probs = Vec::with_capacity(probs.len());
                let mut sum = 0.0;
                for &prob in probs.iter() {
                    sum += prob;
                    cumulative_probs.push(sum);
                }
            }
            let cumsum_vec: Vec<64> = cumsum(probs);
            // vs O(n^2)
            // let cumprobs: Vec<f64> = probs.iter()
            // .enumerate()
            // .map(|index &p| probs.iter().take(index+1).sum()::<f64>())
            // .collect();

            // get a random num, r
            let r: f64 = rand.gen(); // float bw 0 and 1 

            // for i, p in enumerate(ps) if r < p then centroids.append(points[i])
            for (i, p) in cumsum_vec.iter()
            .enumerate() {
                if r < p {
                    let j = i;
                }
                break;
            }
            // -> these two steps are a computationally efficient, hacky way
            // of 'indexing' a prob close to those generated

        centroids.push(&points[j]);
        }
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
    
    for iter in 0..max_iterations {
        clusters = assign_clusters(&points, &centroids);
        
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