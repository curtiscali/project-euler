use crate::linear_algebra::{
    line_contains_v2, slope, v2_get_normal_ccw, v2_get_normal_cw, v2_get_reflection_direction, v2_to_unit_vector, y_intercept, Line2D, Vector2D
};

use super::Problem;

fn points_on_ellipse(x: f64) -> (Vector2D, Vector2D) {
    const A: f64 = 5.0;
    const B: f64 = 10.0;

    let numerator = ((B * B) * ((A * A) - (x * x))).sqrt();
    return (
        Vector2D { x, y: numerator / A },
        Vector2D { x, y: (-1.0 * numerator) / A }
    );
}

fn ellipse_intersection_points(line: &Line2D) -> (Vector2D, Vector2D, Vector2D, Vector2D) {
    let slope = slope(line);
    let y_intercept = y_intercept(line);

    let a = 100.0 + (50.0 * slope * slope);
    let b = 50.0 * slope * y_intercept;
    let c = 25.0 * ((y_intercept * y_intercept) - 100.0);

    let discriminant = (b * b) - (4.0 * a * c);
    let x1 = ((-1.0 * b) + discriminant.sqrt()) / (2.0 * a);
    let x2 = ((-1.0 * b) - discriminant.sqrt()) / (2.0 * a);

    println!("Source: {}\nm = {}\nb = {}\n", line.source, slope, y_intercept);

    let (p1, p2) = points_on_ellipse(x1);
    let (p3, p4) = points_on_ellipse(x2);

    println!("Solution 1: {}\nSolution 2: {}\nSolution 3: {}\nSolution 4: {}\n", p1, p2, p3, p4);

    return (p1, p2, p3, p4);
}

fn slope_at_point_on_ellipse(x: f64, y: f64) -> Vector2D {
    Vector2D { x: 1.0, y: (-4.0 * x) / y }
}

fn has_exited_the_ellipse(laser: &Line2D) -> bool {
    return laser.source.y >= 10.0 && (laser.source.x >= -0.01 && laser.source.x <= 0.01);
}

pub struct LaserBeamReflectionsProblem {}

impl Problem for LaserBeamReflectionsProblem {
    fn solve(&self) -> String {
        /* ALGORITHM
         * Let init vector = ((0, 10.1), (1.4, -9.6))
         * let init scope = dx/dy
         *
         * while dest(vector) != (0 +/- 0.01, 10):
         * calc slope of reflected line using derivative m = -4x/y for the ellipse
         * use that slope and dest(vector) to calculate next intersection with the ellipse
         * update vector = (dest(prev), intersection)
         */

        // This line corresponds to the initial (0, 10.1) -> (1.4, -9.6) where it strikes the mirror
        let mut laser_beam = Line2D {
            source: Vector2D { x: 1.4, y: -9.6 },
            direction: Vector2D { x: 1.4, y: -19.7 }
        };

        let mut num_reflections: u32 = 0;
        while !has_exited_the_ellipse(&laser_beam) {
            /* ALGO:
             * calculate slope of elipse @ impact point
             * determine which quadrant the impact point is, and use that to determine the normal vector
             * use normal vector to calculate reflection direction vector
             * update laser beam with new direction vector
             * use reflection direction and laser source point to calculate next impact with ellipse
             * update laser beam with new impact point
             * add 1 to num_reflections
             */

            let slope_of_ellipse_at_impact = slope_at_point_on_ellipse(laser_beam.source.x, laser_beam.source.y);
            let slope_normal: Vector2D;

            if slope_of_ellipse_at_impact.x >= 0.0 && slope_of_ellipse_at_impact.y >= 0.0 {
                if laser_beam.source.y >= 0.0 {
                    slope_normal = v2_get_normal_cw(&slope_of_ellipse_at_impact);
                } else {
                    slope_normal = v2_get_normal_ccw(&slope_of_ellipse_at_impact);
                }
            } else {
                if laser_beam.source.y >= 0.0 {
                    slope_normal = v2_get_normal_ccw(&slope_of_ellipse_at_impact);
                } else {
                    slope_normal = v2_get_normal_cw(&slope_of_ellipse_at_impact);
                }
            }

            let reflected_direction = v2_get_reflection_direction(&laser_beam.direction, &slope_normal);
            laser_beam.direction = Vector2D { x: reflected_direction.x, y: reflected_direction.y };

            let (p1, p2, p3, p4) = ellipse_intersection_points(&laser_beam);

            let new_source: Vector2D;
            // TODO: init new_source to be the first point that falls on the new line
            if line_contains_v2(&laser_beam, &p1) {
                new_source = p1;
            } else if line_contains_v2(&laser_beam, &p2) {
                new_source = p2;
            } else if line_contains_v2(&laser_beam, &p3) {
                new_source = p3;
            } else {
                new_source = p4;
            }

            laser_beam.source = new_source;
            num_reflections += 1;
        }

        return format!("{}", num_reflections);
    }    
}
