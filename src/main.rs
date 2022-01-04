use clap::Parser;
use rand::Rng;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(short, long)]
    dice: String,
    // #[clap(short = 'l', long)]
    // blargh: String,

    // #[clap(short, long, default_value_t = 1)]
    // count: u8,
}

fn dx(dice: String) -> Vec<u8> {
    // let split = dice.split("d");
    let mut rolls: Vec<u8> = Vec::new();
    let vec: Vec<&str> = dice.split("d").collect(); 
    let n: u8 = vec[0].parse().unwrap();
    let d: u8 = vec[1].parse().unwrap();
    let mut rng = rand::thread_rng();
    for _ in 1..n+1 {
        rolls.push(rng.gen_range(1..d+1));
    }
    return rolls;
}

fn main() {
    let args = Args::parse();

    println!("Hello {}!", args.dice);
    let result = dx(args.dice);
    for i in &result {
        println!("{}", i);
    }
}