use chimpanzee::formatter::cli::FormatterCli;
use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = FormatterCli::parse();
    args.run()
}
