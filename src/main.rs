use interpreter_monkey::{evaluator::Evaluator, Lexer, Parser, Token};
use std::{
    env,
    error::Error,
    fs,
    io::{self, Write},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let is_repl = args.len() == 1;
    if is_repl {
        repl();
    } else {
        let filename = &args[1];
        run_file(filename);
    }
}

#[allow(dead_code)]
fn rlpl() {
    print_entry_header();
    std::io::stdin().lines().for_each(|line| {
        if let Ok(line) = line {
            let mut lexer = Lexer::new(&line);
            let mut token = Token::Illegal;
            while token != Token::Eof {
                token = lexer.next_token();
                println!("{token}");
            }
        }
        print_entry_header();
    });
}

#[allow(dead_code)]
fn rppl() {
    greeting_message();
    print_entry_header();
    std::io::stdin().lines().for_each(|line| {
        if let Ok(line) = line {
            let lexer = Lexer::new(&line);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();
            if parser.errors.is_empty() {
                println!("{program}");
            } else {
                print_parse_errors(parser.errors);
            }
        }
        print_entry_header();
    });
}

fn repl() {
    greeting_message();
    print_entry_header();
    let mut evaluator = Evaluator::new();
    std::io::stdin().lines().for_each(|line| {
        if let Ok(line) = line {
            let lexer = Lexer::new(&line);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();
            if !parser.errors.is_empty() {
                print_parse_errors(parser.errors);
            }
            let evaluated = evaluator.eval(&program);
            println!("{evaluated}");

            print_entry_header();
        }
    });
}

fn greeting_message() {
    let greeting = r#"
                                  @@@@@@@@@@@@
                                @@%#+-:...:-=*@@@
                              @@*:    .:::.    -%@
                             @#.  :+%@@@@@@@#=   #@
                            @#   *@@@@@@@@@@@@#   *@
                            @.  +@@@@@@@*:  :*@%:  -@@
                           @%   ##%@@@@@ -*+. +@@+   +%@@
                    @@@%%@@@%    ..:%@@@%@@@* :@@@%=   -#@@@
                  @@*:  *@@@#= .:  =@@@@%..  +@@@@@@*:  .+%@@
                 @%=    .#@@@@@:   :%@@@@@*+*%@@@@@@@@@#=   -*@@
               @%=    -*@@@@@#.     .:+@@@@@@@@@@@@@@@@@@@+.  :*@@
              @*    -#@@@@@@@#         .@@#=:.  .-#@@@@@@@@@*.  .*@@
            @@-   .#@@@@@@@@@@%. .*+    *-  .=+*+-.#@@@@@@@@@@+   :%@
           @%.   =@@@@@@@@@@@@@@#+:        *@@@@@@@@@@@@@@@@@@@%.   *@
          @%.   +@@@@@@@%@@@@@@@@%*+=--.  +@@@@@@@@@@@@@@@@@@@@@@:   #@
          @:   +@@@@@@@+.@@@@@@@@@@@@@@=  #@@@@@@@@@@@@@@@@@@@@@@@:   %@
         @=   =@@@@@@@@- %@@@@@@@@@@@@@+  *@@@@@@@@@@@@@@@@@@@@@@@%.  :@
        @%   .@@@@@@@#=  *@@@@@@@@@@@@@#  +@@@@@@@@@%@@@@@@@@@@@@@@*   #@
        @=   +@@@@@@+    :@@@@@@@@@@@@@=  +@@@@@@#-  *@@@@@@@@@@@@@@.  -@
        @:   %@@@@@@+     +@@@@@@@@@@@=   =@@@@@@. :%@@@@@@@@@@@@@@@=  .@@
        @   .@@@@@@@@*     :=*%@@@@@@-    =@@@@@#  *@@@@@@@@@@@@@@@@*   %@
        @   :@@@@@@@@@#-::     .:-=-      =@@@@@#  +@@@@@@@@@@@@@@@@#   %@
        @.  .@@@@@@@@@@@@@#=.       .=*#. =@@@@@#    -#@@@@@@@@@@@@@*   %@
        @:   %@@@@@@@@@@@@@@@#=   .#@@@@. =@@@@@*  =:  .*@@@@@@@@@@@=  .@
        @*   +@@@@@@@@@@@@@@@@%   %@@@@*  *@@@@@= .@@%-  +@@@@@@@@@@.  =@
         @.  .@@@@@@@@@@@@@@@@%   #@@@#  -@@@@@*  *@@@@#.+@@@@@@@@@+   %@
         @*   -@@@@@@@@@@@@@@@@=   #@#  :@@@@%-  *@@@@@@@@@@@@@@@@%   =@
          @=   =@@@@@@@@@@@@@@@@-   -  =@@@@+  :%@@@@@@@@@@@@@@@@%.  :@
           @-   =@@@@@@@@@@@@@@@@*.   *@@@*. .*@@@@@@@@@@@@@@@@@%.  :@
            @=   :%@@@@@@@@@@@@@@%. -%@@#: .+@@@@@@@@@@@@@*+@@@*   :@
             @*    +@@@@@@@@@@@@= .*@@#:   -#@@@@@@@@@@@@@: -%-   =@
              @%-   :*@@*=-+*#+..*@@#: .+*-   :=*%@@@@@@%=      .#@
               @@#:   ..  .     :+-. :*@@@@@#+-    .:::.      .+@@
                 @@#-    :+#+-:.  .=%@@@@@@@@@@#            :*@@
                   @@%+:    :=+*#@@@@@@@@@@#+-.          .=#@@
                     @@%*-.     .::-=++++=          .:+#@@@
                       @@@@%*+-:              ..-=*#@@@@
                           @@@@@@@%##*****##%@@@@@@@
                                  @@@@@@@@@@@     
"#;
    println!("{greeting}");
    println!("Welcome to the Monkey programming language!");
    println!("Feel free to type in commands\n");
}

fn print_entry_header() {
    print!(">> ");
    io::stdout().flush().unwrap();
}

fn print_parse_errors(errors: Vec<String>) {
    for error in errors {
        println!("{error}\n");
    }
}

fn run_file(file_path: &str) {
    let contents = match interpret_file(file_path) {
        Ok(contents) => contents,
        Err(error) => {
            eprintln!("{error}");
            return;
        }
    };

    let lexer = Lexer::new(&contents);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    if !parser.errors.is_empty() {
        print_parse_errors(parser.errors);
        return;
    }
    let mut evaluator = Evaluator::new();

    if let interpreter_monkey::evaluator::object::Object::ERROR(error) = evaluator.eval(&program) {
        eprintln!("{error}");
    }
}

fn interpret_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    if !file_path.ends_with(".monkey") {
        Err(String::from("Error: File must end with .monkey").into())
    } else {
        Ok(fs::read_to_string(file_path)?)
    }
}
