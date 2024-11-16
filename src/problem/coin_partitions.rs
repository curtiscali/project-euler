use super::Problem;

fn pentagonal_number(k: i64) -> i64 {
    (k * ((3 * k) - 1)) / 2
}

pub struct CoinPartitionsProblem {}

impl Problem for CoinPartitionsProblem {
    fn name(&self) -> String {
        String::from("Coin Partitions<F3>")
    }

    fn number(&self) -> u16 {
        78
    }

    fn solve(&self) -> String {
        const N: usize = 100_000;
        const MOD: i64 = 1_000_000;

        let mut partitions = vec![1i64];

        // This algorithm implements a modular version of the pentagonal number theorem:
        // https://en.wikipedia.org/wiki/Pentagonal_number_theorem
        for n in 1..=N {
            let mut sum = 0;

            let mut i = 0usize;
            loop {
                let mut alternate = (1 + (i / 2)) as i64;
                if i % 2 == 1 {
                    alternate *= -1;
                }

                let offset = pentagonal_number(alternate);
                if (n as i64) < offset {
                    break;
                }

                if i % 4 < 2 {
                    sum += partitions[((n as i64) - offset) as usize];
                } else {
                    sum -= partitions[((n as i64) - offset) as usize];
                }

                sum %= MOD;

                i += 1;
            }

            if sum < 0 {
                sum += MOD;
            }

            if sum == 0 {
                break;
            }

            partitions.push(sum);
        }

        format!("{}", partitions.len())
    }
}
