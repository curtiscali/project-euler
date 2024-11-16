use super::Problem;

fn eval(a: f64, b: f64) -> f64 {
    return b * a.ln();
}

pub struct LargestExponentialProblem { }

impl Problem for LargestExponentialProblem {
    fn name(&self) -> String {
        String::from("Largest Exponential")
    }

    fn number(&self) -> u16 {
        99
    }

    fn solve(&self) -> String {
        let bytes = include_bytes!("../data_files/base_exp.txt");
        let file_data = String::from_utf8_lossy(bytes);

        let mut i: u32 = 0;
        let mut max_index: u32 = 0;

        let mut max_exp: f64 = 0.0;
        let lines = file_data.lines();
        for line in lines {
            let exponents = Vec::from_iter(line.trim().split(","));
            let a = exponents[0].parse::<f64>().unwrap();
            let b = exponents[1].parse::<f64>().unwrap();

            if eval(a, b) > max_exp {
                max_exp = eval(a, b);
                max_index = i;
            }

            i += 1;
        }

        return format!("{}", max_index + 1);
    }
}
