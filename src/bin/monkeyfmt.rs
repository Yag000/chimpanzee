use clap::Parser;
use formatter::cli::FormatterCli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = FormatterCli::parse();
    args.run()
}
