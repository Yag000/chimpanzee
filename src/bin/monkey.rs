use chimpanzee::repl::ReplCli;
use clap::Parser;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = ReplCli::parse();
    args.run()
}
