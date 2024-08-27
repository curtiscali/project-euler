pub struct Vector2D {
    pub x: f64,
    pub y: f64
}

pub struct Line2D {
    source: Vector2D,
    destination: Vector2D
}

pub fn v2_sub(v1: &Vector2D, v2: &Vector2D) -> Vector2D {
    let Vector2D { x: x1, y: y1 } = v1;
    let Vector2D { x: x2, y: y2 } = v2;

    return Vector2D { x: x1 - x2, y: y1 - y2};
}

pub fn v2_dotprod(v1: &Vector2D, v2: &Vector2D) -> f64 {
    let Vector2D { x: x1, y: y1 } = v1;
    let Vector2D { x: x2, y: y2 } = v2;

    return (*x1 * *x2) + (*y1 * *y2);
}