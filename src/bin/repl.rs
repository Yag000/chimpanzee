use interpreter_monkey::{Lexer, Token};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::io::stdin().lines().for_each(|line| {
        if let Ok(line) = line {
            let mut lexer = Lexer::new(line);

            let mut token = Token::Illegal;

            while token != Token::Eof {
                token = lexer.next_token();
                println!("{} ", token);
            }
        }
    });
    return Ok(());
}
