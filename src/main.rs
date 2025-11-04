use crate::cli::Cli;
use clap::Parser;
use dr_script::run_source;
use std::fs::read_to_string;

mod cli;

fn main() {
    let cli = Cli::parse();
    let code = read_to_string(&cli.file).unwrap_or_else(|_| panic!("File {} not found", cli.file));
    let r = run_source(code.as_str());
    println!("{:?}", r);
    ()
}
