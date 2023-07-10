use clap::Parser;
use repl::repl::Cli;

fn main()  -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    args.run()
}
