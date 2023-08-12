use clap_derive::Parser;

use crate::formatter::formatter::Formatter;

trait Logger {
    fn log(&mut self, msg: &str) -> Result<(), Box<dyn std::error::Error>>;
}

struct StdoutLogger;

impl Logger for StdoutLogger {
    fn log(&mut self, msg: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("{msg}");
        Ok(())
    }
}

struct FileLogger {
    filename: String,
}

impl Logger for FileLogger {
    fn log(&mut self, msg: &str) -> Result<(), Box<dyn std::error::Error>> {
        std::fs::write(&self.filename, msg)?;
        Ok(())
    }
}

#[derive(Parser)]
pub struct FormatterCli {
    /// Input file
    filename: String,

    /// Indicates if you want to replace the input file
    /// with the formatted output
    #[clap(short, long, value_name = "replace")]
    replace: bool,
}

impl FormatterCli {
    fn get_logger(&self) -> Box<dyn Logger> {
        if self.replace {
            Box::new(FileLogger {
                filename: self.filename.clone(),
            })
        } else {
            Box::new(StdoutLogger)
        }
    }
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut logger = self.get_logger();
        self.run_with_logger(logger.as_mut())
    }
    fn run_with_logger(&self, logger: &mut dyn Logger) -> Result<(), Box<dyn std::error::Error>> {
        let input = std::fs::read_to_string(&self.filename)?;
        let output = Formatter::format(&input);
        logger.log(&output)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestLogger {
        pub msg: String,
    }

    impl Logger for TestLogger {
        fn log(&mut self, msg: &str) -> Result<(), Box<dyn std::error::Error>> {
            self.msg = msg.to_string();
            Ok(())
        }
    }

    #[test]
    fn test_cli() {
        let filename = "src/formatter/ressources/test_formatting.monkey".to_string();
        let input = std::fs::read_to_string(&filename).unwrap();

        let cli = FormatterCli {
            filename,
            replace: false,
        };

        let mut logger = TestLogger {
            msg: String::new(),
        };

        cli.run_with_logger(&mut logger).unwrap();

        assert_eq!(logger.msg, Formatter::format(&input));
    }
}
