use crate::compiler::Compiler;
use crate::evaluator::object::Object;
use crate::vm::VM;
use crate::{evaluator::Evaluator, Lexer, Parser, Token};
use std::io::{self, Write};
use std::{error::Error, fs};

#[allow(dead_code)]
pub fn rlpl() {
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
pub fn rppl() {
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

#[allow(dead_code)]
pub fn repl_interpreter() {
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

pub fn repl_compiler() {
    greeting_message();
    print_entry_header();
    std::io::stdin().lines().for_each(|line| {
        if let Ok(line) = line {
            let lexer = Lexer::new(&line);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();
            if !parser.errors.is_empty() {
                print_parse_errors(parser.errors);
            }
            let mut compiler = Compiler::new();
            match compiler.compile(program) {
                Ok(()) => {
                    let mut vm = VM::new(compiler.bytecode());
                    match vm.run() {
                        Ok(()) => {
                            let stack_top = match vm.last_popped_stack_element() {
                                Some(x) => x.to_string(),
                                None => "Error: No stack top".to_string(),
                            };
                            println!("{stack_top}");
                        }
                        Err(e) => println!("Bytecode evaluation error: {e}"),
                    };
                }
                Err(e) => {
                    println!("Compilation failed: {e}");
                }
            }

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

pub fn run_file(file_path: &str) {
    let contents = match read_file_contents(file_path) {
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

    if let Object::ERROR(error) = evaluator.eval(&program) {
        eprintln!("{error}");
    }
}

fn read_file_contents(file_path: &str) -> Result<String, Box<dyn Error>> {
    if file_path.ends_with(".monkey") {
        Ok(fs::read_to_string(file_path)?)
    } else {
        Err(String::from("Error: File must end with .monkey").into())
    }
}
