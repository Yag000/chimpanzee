use clap::Parser;
use repl::repl::Cli;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    args.run()
}
