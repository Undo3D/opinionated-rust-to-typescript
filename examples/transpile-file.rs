use std::{env,fs,process};

use opinionated_rust_to_typescript::transpile::config::Config;
use opinionated_rust_to_typescript::transpile::rs_to_ts::rs_to_ts;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("ERROR: Expected 2 args, got {}. Try:", args.len());
        eprintln!(r#"    echo "const FOUR: u8 = 4;" > four.rs"#);
        eprintln!("    cargo run --example transpile-file -- four.rs");
        process::exit(1);
    }
    let contents = fs::read_to_string(&args[1]).unwrap_or_else(|err| {
        eprintln!("ERROR: Problem reading the file:\n    {}", err);
        process::exit(2);
    });
    let result = rs_to_ts(&contents, Config::new());
    println!("{}", result.main_lines[0]);
}
