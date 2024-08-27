use super::Problem;
use crate::linear_algebra::{Vector2D, v2_sub, v2_dotprod};

type Triangle = (Vector2D, Vector2D, Vector2D);

fn contains(triangle: Triangle, point: &Vector2D) -> bool {
    let (a, b, c) = triangle;

    let v1 = v2_sub(&c, &a);
    let v2 = v2_sub(&b, &a);
    let v3 = v2_sub(&point, &a);

    let dot11 = v2_dotprod(&v1, &v1);
    let dot12 = v2_dotprod(&v1, &v2);
    let dot13 = v2_dotprod(&v1, &v3);
    let dot22 = v2_dotprod(&v2, &v2);
    let dot23 = v2_dotprod(&v2, &v3);

    let inv_denom = 1.0 / ((dot11 * dot22) - (dot12 * dot12));
    let u = ((dot22 * dot13) - (dot12 * dot23)) * inv_denom;
    let v = ((dot11 * dot23) - (dot12 * dot13)) * inv_denom;

    return (u >= 0.0) && (v >= 0.0) && (u + v < 1.0);
}

pub struct TriangleContainmentProblem { }

impl Problem for TriangleContainmentProblem {
    fn solve(&self) -> String {
        let bytes = include_bytes!("../data_files/triangles.txt");
        let file_data = String::from_utf8_lossy(bytes);
        
        let mut contains_origin_count: u32 = 0;
        let origin = Vector2D { x: 0.0, y: 0.0 };
        let lines = file_data.lines();
        for line in lines {
            let coordinates = Vec::from_iter(line.trim().split(",").map(|s| s.parse::<f64>().unwrap()));
            let triangle: Triangle = (
                Vector2D { x: coordinates[0], y: coordinates[1] }, 
                Vector2D { x: coordinates[2], y: coordinates[3] }, 
                Vector2D { x: coordinates[4], y: coordinates[5] }
            );

            if contains(triangle, &origin) {
                contains_origin_count += 1;
            }
        }

        return format!("{}", contains_origin_count);
    }
}