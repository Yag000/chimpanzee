use clap::Parser;
use formatter::formatter::cli::Cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    args.run()
}
