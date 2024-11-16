use num::integer::Roots;
use super::Problem;

fn get_factor_pairs(n: u32) -> Vec<(u32, u32)> {
    let mut factors: Vec<(u32, u32)> = vec![(1, n)];

    let sqrt = n.sqrt();
    let mut i = 2;
    while i < sqrt {
        if n % i == 0 {
            factors.push((i, n / i));
        }
        
        i += 1;
    }

    return factors;
}

fn get_pythagorean_triple(factor_pair: (u32, u32)) -> (u32, u32, u32) {
    let m = factor_pair.0;
    let n = factor_pair.1;

    let m2 = m * m; 
    let n2 = n * n;

    return (m2.abs_diff(n2), 2 * m * n, m2 + n2);
}

pub struct SpecialPythagoreanTripletProblem {}

impl Problem for SpecialPythagoreanTripletProblem {
    fn name(&self) -> String {
        String::from("Special Pythagorean Triplet") 
    }

    fn number(&self) -> u16 {
        9
    }

    fn solve(&self) -> String {
        const TARGET_SUM: u32 = 1000;

        let mut i = TARGET_SUM / 10;
        let mut special_triple: (u32, u32, u32) = (0, 0, 0);
        let mut found_triple = false;
        while i <= TARGET_SUM / 2 {
            for factor_pair in get_factor_pairs(i / 2) {
                let (a, b, c) = get_pythagorean_triple(factor_pair);

                if a + b + c == TARGET_SUM {
                    special_triple = (a, b, c);
                    found_triple = true;
                    break;
                }
            }

            if found_triple {
                break;
            }

            i += 2;
        }

        return format!("{}", special_triple.0 * special_triple.1 * special_triple.2);
    }
}
