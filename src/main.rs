use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of the problem for which you would like a solution as referenced on https://projecteuler.net
    #[arg(short, long)]
    problem: u8
}


fn main() {
    let args = Args::parse();

    println!("Selected problem: {}", args.problem);
}
