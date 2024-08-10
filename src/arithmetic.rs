pub fn linear_sum(n: usize) -> usize {
    return (n * (n + 1)) / 2;
}

pub fn quadratic_sum(n: usize) -> usize {
    return (n * (n + 1) * ((2 * n) + 1)) / 6;
}

pub fn is_perfect_square(n: usize) -> bool {
    let root = ((n as f64).sqrt() + 0.5) as usize;

    return root * root == n; 
}

pub fn is_triangular(n: usize) -> bool {
    return is_perfect_square((8 * n) + 1);
}