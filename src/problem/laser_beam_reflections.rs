use crate::linear_algebra::{
    slope, v2_get_normal_ccw, v2_get_normal_cw, v2_get_reflection_direction, y_intercept, Line2D, Vector2D
};

use super::Problem;

fn ellipse_intersection_points(line: &Line2D) {
    let slope = slope(line);
    let y_intercept = y_intercept(line);

    let a = 100.0 - (25.0 * slope * slope);
    let b = 50.0 * slope * y_intercept;
    let c = (25.0 * y_intercept * y_intercept) - 2500.0;
    let root = ((b * b) - (4.0 * a * c)).sqrt();

    let x1 = (-b + root) / (2.0 * a);
    let x2 = (-b - root) / (2.0 * a);

    println!("{} {}", x1, x2);
}

fn point_on_ellipse(x: f64) -> Vector2D {
    let numerator = (2500.0 - (25.0 * x * x)).sqrt();
    return Vector2D { x, y: numerator / 5.0 };
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
        //while !has_exited_the_ellipse(&laser_beam) {
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
            let mut slope_normal = Vector2D {x: 0.0, y: 0.0 };

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
            println!("{} {} {}", slope_of_ellipse_at_impact, slope_normal, reflected_direction);
            laser_beam.direction = reflected_direction;

            ellipse_intersection_points(&laser_beam);

            // TODO: calculate next impact point of beam with new direction

            num_reflections += 1;
        //}

        return format!("{}", num_reflections);
    }
}