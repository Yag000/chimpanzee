use clap::Parser;
use chimpanzee::repl::ReplCli;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = ReplCli::parse();
    args.run()
}
