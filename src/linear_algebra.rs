use std::fmt::Display;

use crate::arithmetic::f64_equals;

#[derive(Debug)]
pub struct Vector2D {
    pub x: f64,
    pub y: f64
}

impl Display for Vector2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug)]
pub struct Line2D {
    pub source: Vector2D,
    pub direction: Vector2D
}

pub fn v2_sub(v1: &Vector2D, v2: &Vector2D) -> Vector2D {
    let Vector2D { x: x1, y: y1 } = v1;
    let Vector2D { x: x2, y: y2 } = v2;

    return Vector2D { x: *x1 - *x2, y: *y1 - *y2};
}

pub fn v2_dotprod(v1: &Vector2D, v2: &Vector2D) -> f64 {
    let Vector2D { x: x1, y: y1 } = v1;
    let Vector2D { x: x2, y: y2 } = v2;

    return (*x1 * *x2) + (*y1 * *y2);
}

pub fn v2_scalar_mult(v: &Vector2D, scalar: f64) -> Vector2D {
    return Vector2D { x: scalar * v.x, y: scalar * v.y };
}

pub fn v2_to_unit_vector(v: &Vector2D) -> Vector2D {
    let vector_length = ((v.x * v.x) + (v.y * v.y)).sqrt();
    return Vector2D { x: v.x / vector_length, y: v.y / vector_length };
}

pub fn v2_get_normal_cw(v: &Vector2D) -> Vector2D {
    return Vector2D { x: v.y, y: -v.x };
}

pub fn v2_get_normal_ccw(v: &Vector2D) -> Vector2D {
    return Vector2D { x: -v.y, y: v.x };
}

pub fn v2_get_reflection_direction(incident_direction: &Vector2D, surface_normal_direction: &Vector2D) -> Vector2D {
    let incident_unit_vector = v2_to_unit_vector(incident_direction);
    let normal_unit_vector = v2_to_unit_vector(surface_normal_direction);

    let new_vector = v2_scalar_mult(&normal_unit_vector, 2.0 * v2_dotprod(&incident_unit_vector, &normal_unit_vector));
    return v2_sub(&incident_unit_vector, &new_vector);
}

pub fn y_intercept(line: &Line2D) -> f64 {
    let m = slope(line);
    return line.source.y - (m * line.source.x);
}

pub fn slope(line: &Line2D) -> f64 {
    return line.direction.y / line.direction.x;
}

pub fn line_contains_v2(line: &Line2D, point: &Vector2D) -> bool {
    let slope = slope(line);
    let y_intercept = y_intercept(line);

    return f64_equals(point.y, (slope * point.x) + y_intercept);
}
