use std::{env,process};

use opinionated_rust_to_typescript::transpile::config::Config;
use opinionated_rust_to_typescript::transpile::rs_to_ts::rs_to_ts;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("ERROR: Expected 2 args, got {}. Try:", args.len());
        eprint!("    cargo run --example transpile-arg -- ");
        eprintln!(r#""const ROUGHLY_PI: f32 = 3.14;""#);
        process::exit(1);
    }
    let result = rs_to_ts(&args[1], Config::new());
    println!("{}", result.main_lines[0]);
}
