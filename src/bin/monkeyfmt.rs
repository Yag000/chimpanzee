use clap::Parser;
use chimpanzee::formatter::cli::FormatterCli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = FormatterCli::parse();
    args.run()
}
