use std::env::{self};

use coverage::generate_coverage;

pub mod coverage;
pub mod entities;
pub mod helpers;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && args.contains(&"version".to_string()) {
        println!("x-cc - version 1.0.0");
    } else {
        let index = args
            .iter()
            .position(|x| x == "-path")
            .expect("-path (.xcresult file path) is required");
        let xcresult_path = args[index + 1].clone();

        generate_coverage(&xcresult_path)
    }
}
