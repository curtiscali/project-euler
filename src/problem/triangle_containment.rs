use super::Problem;

type Vector = (i64, i64);
type Triangle = (Vector, Vector, Vector);

fn vsub(v1: Vector, v2: Vector) -> Vector {
    let (x1, y1) = v1;
    let (x2, y2) = v2;

    return (x1 - x2, y1 - y2);
}

fn dotprod(v1: Vector, v2: Vector) -> i64 {
    let (x1, y1) = v1;
    let (x2, y2) = v2;

    return (x1 * x2) + (y1 * y2);
}

fn contains(triangle: Triangle, point: Vector) -> bool {
    let (a, b, c) = triangle;

    let v1 = vsub(c, a);
    let v2 = vsub(b, a);
    let v3 = vsub(point, a);

    let dot11 = dotprod(v1, v1) as f64;
    let dot12 = dotprod(v1, v2) as f64;
    let dot13 = dotprod(v1, v3) as f64;
    let dot22 = dotprod(v2, v2) as f64;
    let dot23 = dotprod(v2, v3) as f64;

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
        let origin: Vector = (0, 0);
        let lines = file_data.lines();
        for line in lines {
            let coordinates = Vec::from_iter(line.trim().split(","));
            let triangle: Triangle = (
                (coordinates[0].parse::<i64>().unwrap(), coordinates[1].parse::<i64>().unwrap()), 
                (coordinates[2].parse::<i64>().unwrap(), coordinates[3].parse::<i64>().unwrap()), 
                (coordinates[4].parse::<i64>().unwrap(), coordinates[5].parse::<i64>().unwrap())
            );

            if contains(triangle, origin) {
                contains_origin_count += 1;
            }
        }

        return format!("{}", contains_origin_count);
    }
}