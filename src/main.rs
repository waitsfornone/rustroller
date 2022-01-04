use clap::Parser;
use rand::Rng;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    name: String,
    // #[clap(short = 'l', long)]
    // blargh: String,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}

fn d20() -> u8 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(1..21);
}






fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
    let result = d20();
    print!("The roll is {}", result);
}