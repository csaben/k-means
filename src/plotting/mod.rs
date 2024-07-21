use plotters::prelude::*;
use crate::{Point, Centroid};

pub fn plot_clusters(points: &[Point], clusters: &[Vec<Point>], centroids: &[Centroid], filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(filename, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("K-means Clustering", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f64..100f64, 0f64..100f64)?;

    chart.configure_mesh().draw()?;

    let colors = [RED, GREEN, BLUE, CYAN, MAGENTA, YELLOW];

    for (i, cluster) in clusters.iter().enumerate() {
        chart.draw_series(cluster.iter().map(|point| Circle::new((point.x, point.y), 3, colors[i % colors.len()].filled())))?;
    }

    chart.draw_series(centroids.iter().map(|centroid| {
        Circle::new((centroid.coordinate.x, centroid.coordinate.y), 5, BLACK.filled())
    }))?;

    root.present()?;
    Ok(())
}