use las::{Read, Reader, point::Classification};
use kd_tree::{KdTree, KdPoint};
// use serde::{Serialize, Deserialize};
use log::*;
use kiss3d::resource::{AllocationType, BufferType, GPUVec};
use kiss3d::window::Window;

#[derive(Clone)]
struct Point(las::Point);

struct Trail {
    points: KdTree<Point>,
    pcl: viewercloud::PointCloud,
}

impl KdPoint for Point {
    type Scalar = f64;
    type Dim = typenum::U2;
    fn at(&self, k: usize) -> f64 {
        match k {
            0 => self.0.x,
            1 => self.0.y,
            _ => panic!("Got request to access dim {k} of 2d data!")
        }
    }
}

impl Trail {
    fn new(las_path: &str) -> Self {
        let mut reader = Reader::from_path(las_path).unwrap();
        let mut pts = Vec::new();
        for wrapped_point in reader.points() {
            let pt = wrapped_point.unwrap();
            pts.push(Point(pt));
            
        }
        info!("Loaded {} points from {}", pts.len(), las_path);
        Trail {
            points: KdTree::par_build_by_ordered_float(pts.clone()),
            pcl: viewercloud::PointCloud {
                data: pts.into_iter().map(|i| nalgebra::Point3::new(i.0.z as f32, i.0.x as f32, i.0.y as f32))
                    .collect::<Vec<nalgebra::Point3<f32>>>()
            }
        }
    }

    fn get_gradient(&self, coord: [f64; 2]) -> [f64; 2] {
        println!(
            "{}",
            self.points.within_radius(
                &coord,
                1.0,                
            ).len()
        );
        [0.0,0.0]
    }
}

pub fn main() {
    pretty_env_logger::init();    
    let t = Trail::new("/Users/davfrei/Downloads/points.las");
    info!("Constructed k-tree.");
    let point_cloud_data: viewercloud::PointCloudGPU =
        GPUVec::new(t.pcl.data, BufferType::Array, AllocationType::StreamDraw);
    let window = Window::new_with_size("Edgewood Park", 1500, 1000);
    let app = viewercloud::viewer::AppState {
        point_cloud_renderer: viewercloud::renderer::PointCloudRenderer::new(2.0, point_cloud_data),
    };

    window.render_loop(app);
    
    // let json = serde_json::to_string(&t).unwrap();
    
    // t.get_gradient([564367.71,4146789.54]);
}
