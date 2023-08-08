use clap::Parser;
use repl::repl::ReplCli;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = ReplCli::parse();
    args.run()
}
