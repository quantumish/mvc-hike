use las::{Read, Reader, point::Classification};
use kd_tree::{KdTree, KdPoint};
// use serde::{Serialize, Deserialize};
use log::*;
use kiss3d::resource::{AllocationType, BufferType, GPUVec};
use kiss3d::window::Window;

#[derive(Debug, Clone)]
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
            0 => self.0.x - 564400.0,
            1 => self.0.y - 4146800.0,
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
        let mapped = pts.clone().into_iter().map(|i| {
            // println!("{},{},{}", i.0.x, i.0.y, i.0.z);
            // println!("{},{},{}", i.0.x as f32, i.0.y as f32, i.0.z as f32);
            nalgebra::Point3::new(
            i.0.x as f32 - 564400.0,
            i.0.y as f32 - 4146800.0,
            i.0.z as f32
            )}).collect::<Vec<nalgebra::Point3<f32>>>();
        info!("{}", mapped.len());
        Trail {
            points: KdTree::par_build_by_ordered_float(pts),
            pcl: viewercloud::PointCloud {
                data: mapped
            }
        }
    }

    fn get_gradient(&self, coord: [f64; 2]) -> [f64; 2] {
        let all_pts = self.points.within_radius(&coord, 5.0);
        let (mut dzdx, mut dzdy) = (0.0, 0.0);
        // println!("{} {} {}", all_pts[0].0.x, all_pts[1].0.x, all_pts[2].0.x);
        for pt in all_pts.iter() {
            // println!("huh?");
            let pts = self.points.nearests(&[pt.0.x, pt.0.y], 2);
            // println!("{}", pts[1].squared_distance);
            let o = pts[1].item;
            let (dz, dx, dy) = (o.0.z-pt.0.z, o.0.x-pt.0.x, o.0.y-pt.0.y);
            // println!("{}, {}, {}", pt.0.x, pt.0.y, pt.0.z);
            // println!("{}, {}, {}", o.0.x, o.0.y, o.0.z);
            // println!("{dx}, {dy}, {dz}");
            // println!("");
            dzdx += dz/dx;
            dzdy += dz/dy;
        }
        // println!("dzdy = {}", dzdy);
        // println!("dz/dy = {}, dz/dx = {}", dzdy/(all_pts.len() as f64), dzdx/(all_pts.len() as f64));
        [0.0,0.0]
    }
}

pub fn main() {
    pretty_env_logger::init();    
    let t = Trail::new("/Users/davfrei/Downloads/points.las");
    info!("Constructed k-tree.");
    // t.get_gradient([-1000.0, -130.0]);


    let raw_pts = t.points.within_radius(&[-1050.0, -580.0], 125.0);
    let mut wtr = csv::Writer::from_path("pts.csv").unwrap();
    for pt in &raw_pts {
        wtr.write_record(&[
            (pt.0.x - 564400.0).to_string(),
            (pt.0.y - 4146800.0).to_string(),
            pt.0.z.to_string()
        ]);
    }
    wtr.flush().unwrap();
    
    let pts = raw_pts.into_iter().map(|i| nalgebra::Point3::new(
        i.0.x as f32 - 564400.0,
        i.0.y as f32 - 4146800.0,
        i.0.z as f32
    )).collect::<Vec<nalgebra::Point3<f32>>>();

    
    
    println!("{}", pts.len());
    
    let other_buf: Vec<nalgebra::Point3<f32>> = 
        t.pcl.data.clone();// .into_iter().filter(|i| !pts.contains(i)).collect();
    // println!("{}", other_buf.len());
    let point_cloud_data: viewercloud::PointCloudGPU =
        GPUVec::new(other_buf, BufferType::Array, AllocationType::StreamDraw);
    let point_cloud_data2: viewercloud::PointCloudGPU =
        GPUVec::new(vec![], BufferType::Array, AllocationType::StreamDraw);
    log::info!("gpu {}", point_cloud_data.len());
    let window = Window::new_with_size("Edgewood Park", 1500, 1000);
    let app = viewercloud::viewer::AppState {
        point_cloud_renderer: viewercloud::renderer::PointCloudRenderer::new(0.1, point_cloud_data, point_cloud_data2),
    };

    // // println!("{} {}", 564367.71- 564400.0,4146789.54- 4146800.0);
    window.render_loop(app);
    
    // let json = serde_json::to_string(&t).unwrap();
    
}
