use std::collections::HashMap;
use super::Problem;

struct CoprimeTreeNode {
    pub pair: Option<(u32, u32)>,
    pub children: Vec<CoprimeTreeNode>
}

impl CoprimeTreeNode {
    fn from_tuple(pair: (u32, u32)) -> CoprimeTreeNode {
        CoprimeTreeNode {
            pair: Some(pair),
            children: vec![]
        }
    }
}

fn init_to_depth_rec(root: &mut CoprimeTreeNode, depth: u32, current_depth: u32) {
    if current_depth >= depth {
        return;
    }

    match root.pair {
        Some((m, n)) => {
            root.children.push(CoprimeTreeNode::from_tuple(((2 * m) - n, m)));
            root.children.push(CoprimeTreeNode::from_tuple(((2 * m) + n, m)));
            root.children.push(CoprimeTreeNode::from_tuple(((2 * n) + m, n)));
        }
        None => {}
    }

    let mut i = 0;
    while i < root.children.len() {
        let mut child = &mut root.children[i];

        init_to_depth_rec(&mut child, depth, current_depth + 1);
        
        i += 1;
    }
}

fn init_to_depth(root: &mut CoprimeTreeNode, depth: u32) {
    init_to_depth_rec(root, depth, 0);
}

fn calculate_solutions(root: &CoprimeTreeNode, solutions: &mut HashMap<u32, u32>) {
    if root.children.len() == 0 {
        return;
    }

    match root.pair {
        Some((m, n)) => {
            let triangle = get_pythagorean_triple(m, n);
            let perimeter = perimeter(triangle);

            solutions.entry(perimeter).and_modify(|c| *c += 1).or_insert(1);
        }
        None => {}
    }

    let mut i = 0;
    while i < root.children.len() {
        let child = &root.children[i];
        calculate_solutions(child, solutions);

        i += 1;
    }
}

fn get_pythagorean_triple(m: u32, n: u32) -> (u32, u32, u32) {
    ((m*m) - (n*n), 2*m*n,(m*m) + (n*n))
}

fn perimeter(triangle: (u32, u32, u32)) -> u32 {
    triangle.0 + triangle.1 + triangle.2
}

pub struct IntegerRightTrianglesProblem {}

impl Problem for IntegerRightTrianglesProblem {
    fn solve(&self) -> String {
        const MAX_PERIMETER: u32 = 1000;
        let mut solutions: HashMap<u32, u32> = HashMap::new();

        let mut root = CoprimeTreeNode {
            pair: None,
            children: vec![
                CoprimeTreeNode::from_tuple((2, 1)),
                CoprimeTreeNode::from_tuple((3, 1))
            ]
        };

        init_to_depth(&mut root, 11u32);

        calculate_solutions(&root, &mut solutions);

        let mut perimeter_with_most_tris = 0;
        let mut max_solutions = 0;
        for perimeter in solutions.keys() {
            let solution_count = solutions[perimeter];

            if solution_count > max_solutions && *perimeter <= MAX_PERIMETER {
                max_solutions = solution_count;
                perimeter_with_most_tris = *perimeter;
            }
        }

        return format!("{}", perimeter_with_most_tris);
    }
}
