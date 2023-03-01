use las::{Read, Reader, point::Classification};
use std::time::Instant;

pub fn main() {    
    let mut reader = Reader::from_path("/Users/davfrei/Downloads/points.las").unwrap();
    let mut pts = Vec::new();    
    let now = Instant::now();
    for wrapped_point in reader.points() {
        let point = wrapped_point.unwrap();
        if point.classification == Classification::LowPoint {
            println!("{:?}", point.z);
        }
        pts.push(point);
    }
    println!("Read {} points in {}ms", pts.len(), now.elapsed().as_millis());
    println!("{:?} {:?}", pts[45678], pts[45679]);
}
