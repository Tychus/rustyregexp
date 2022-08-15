mod parser;
mod structs;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    regex: String,
}


fn main() {
    let args = Cli::parse();
    println!("{}", args.regex);
}
