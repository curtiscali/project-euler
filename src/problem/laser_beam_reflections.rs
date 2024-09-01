use crate::linear_algebra::{
    line_contains_v2, 
    slope, 
    v2_get_normal_ccw, 
    v2_get_normal_cw, 
    v2_get_reflection_direction,
    y_intercept, 
    Line2D, 
    Vector2D
};
use crate::arithmetic::f64_equals;
use super::Problem;

enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight
}

fn v2_quadrant(p: &Vector2D) -> Quadrant {
    if p.x >= 0.0 && p.y >= 0.0 {
        return Quadrant::TopRight;
    } else if p.x >= 0.0 && p.y < 0.0 {
        return Quadrant::BottomRight;
    } else if p.x < 0.0 && p.y >= 0.0 {
        return Quadrant::TopLeft;
    }

    return Quadrant::BottomLeft;
}

fn points_on_ellipse(x: f64) -> (Vector2D, Vector2D) {
    const A: f64 = 5.0;
    const B: f64 = 10.0;

    let numerator = ((B * B) * ((A * A) - (x * x))).sqrt();
    return (
        Vector2D { x, y: numerator / A },
        Vector2D { x, y: (-1.0 * numerator) / A }
    );
}

fn next_ellipse_intersection_point(line: &Line2D) -> Vector2D {
    let slope = slope(line);
    let y_intercept = y_intercept(line);

    let a = 100.0 + (25.0 * slope * slope);
    let b = 50.0 * slope * y_intercept;
    let c = 25.0 * ((y_intercept * y_intercept) - 100.0);

    let discriminant = (b * b) - (4.0 * a * c);
    let x1 = ((-1.0 * b) + discriminant.sqrt()) / (2.0 * a);
    let x2 = ((-1.0 * b) - discriminant.sqrt()) / (2.0 * a);

    let (p1, p2) = points_on_ellipse(x1);
    let (p3, p4) = points_on_ellipse(x2);

    let q1 = v2_quadrant(&p1);
    let q2 = v2_quadrant(&p2);
    let q3 = v2_quadrant(&p3);
    let q4 = v2_quadrant(&p4);

    let next_intersection = if f64_equals(p1.x, line.source.x) {
        if line_contains_v2(line, &p3) {
            p3
        } else {
            p4
        }
    } else {
        if line_contains_v2(line, &p1) {
            p1
        } else {
            p2
        }
    };

    return next_intersection;
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
            laser_beam.source = next_ellipse_intersection_point(&laser_beam);    

            num_reflections += 1;
        }

        return format!("{}", num_reflections);
    }    
}
