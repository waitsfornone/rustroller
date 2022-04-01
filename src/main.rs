use clap::Parser;
use rand::Rng;
use std::fs;
use serde_json;

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
    let raw_data = fs::read_to_string("/home/tenders/Documents/code/rustroller/lyque.json").expect("Unable to read file");
    let res: serde_json::Value = serde_json::from_str(&raw_data).expect("Unable to parse");
    println!("{}", res);
    println!("{}", res["level"]);
    let args = Args::parse();

    println!("Hello {}!", args.dice);
    let result = dx(args.dice);
    for i in &result {
        println!("{}", i);
    }
}