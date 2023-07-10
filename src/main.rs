use clap::Parser;
use repl::repl::Cli;

fn main() {
    let args = Cli::parse();
    args.run();
}
