use std::fmt::Display;

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

pub fn v2_get_normal_cw(v: &Vector2D) -> Vector2D {
    return Vector2D { x: v.y, y: -v.x };
}

pub fn v2_get_normal_ccw(v: &Vector2D) -> Vector2D {
    return Vector2D { x: -v.y, y: v.x };
}


pub fn v2_get_reflection_direction(incident_direction: &Vector2D, surface_normal_direction: &Vector2D) -> Vector2D {
    let new_vector = v2_scalar_mult(&v2_scalar_mult(surface_normal_direction, 2.0), v2_dotprod(surface_normal_direction, incident_direction));
    return v2_sub(incident_direction, &new_vector);
}