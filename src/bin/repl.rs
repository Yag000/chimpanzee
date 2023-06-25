use interpreter_monkey::{
    evaluator::{enviroment::Environment, evaluator:: Evaluator},
    Lexer, Parser, Token,
};

#[allow(dead_code)]
fn rlpl() -> Result<(), Box<dyn std::error::Error>> {
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

#[allow(dead_code)]
fn rppl() -> Result<(), Box<dyn std::error::Error>> {
    std::io::stdin().lines().for_each(|line| {
        if let Ok(line) = line {
            let lexer = Lexer::new(line);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();
            if parser.errors.len() != 0 {
                print_parse_errors(parser.errors);
            } else {
                println!("{}", program);
            }
        }
    });
    return Ok(());
}

fn repl() -> Result<(), Box<dyn std::error::Error>> {
    let mut enviroment = Environment::new();
    std::io::stdin().lines().for_each(|line| {
        if let Ok(line) = line {
            let lexer = Lexer::new(line);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();
            if parser.errors.len() != 0 {
                print_parse_errors(parser.errors);
            }
            let mut evaluator = Evaluator::new();
            let evaluated = evaluator.eval_program(&program, &mut enviroment);
            println!("{}", evaluated);
        }
    });
    return Ok(());
}

fn print_parse_errors(errors: Vec<String>) {
    let monkey_face: String = r#"
            __,__
   .--.  .-"     "-.  .--.
  / .. \/  .-. .-.  \/ .. \
 | |  '|  /   Y   \  |'  | |
 | \   \  \ 0 | 0 /  /   / |
  \ '- ,\.-"""""""-./, -' /
   ''-' /_   ^ ^   _\ '-''
       |  \._   _./  |
       \   \ '~' /   /
        '._ '-=-' _.'
           '-----'
"#
    .to_string();
    println!("{}", monkey_face);
    println!("Woops! We ran into some monkey business here!");
    println!(" parser errors:");
    for error in errors {
        println!("\t{}\n", error);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    repl()
}
