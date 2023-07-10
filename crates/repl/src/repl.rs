use anyhow::Result;
use clap_derive::{Parser, ValueEnum};
use compiler::compiler::Compiler;
use interpreter::evaluator::Evaluator;
use lexer::lexer::Lexer;
use lexer::token::Token;
use parser::parser::Parser;
use std::io::{self, Write};
use std::{error::Error, fs};
use vm::vm::VM;

enum InputType {
    File(String),
    Repl,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    Lexer,
    Parser,
    Interpreter,
    Compiler,
}

#[derive(Parser)]
pub struct Cli {
    /// Sets the input file to use, if not specified, the REPL will be launched
    filename: Option<String>,

    // Sets the mode to use, if not specified, interpreter is used
    #[arg(short, long, value_name = "MODE")]
    mode: Option<Mode>,

    // Shows the logo
    #[clap(long)]
    logo: bool,
}

impl Cli {
    fn get_input_type(&self) -> InputType {
        match &self.filename {
            Some(filename) => InputType::File(filename.to_string()),
            None => InputType::Repl,
        }
    }
    fn get_mode(&self) -> Mode {
        match &self.mode {
            Some(mode) => *mode,
            None => Mode::Interpreter, // TODO: Change to compiler when ready
        }
    }

    pub fn run(&self) {
        match &self.get_input_type() {
            InputType::Repl => match self.get_mode() {
                Mode::Lexer => self.rlpl(),
                Mode::Parser => self.rppl(),
                Mode::Interpreter => self.interpreter(),
                Mode::Compiler => self.compiler(),
            },
            InputType::File(filename) => {
                self.run_file(filename);
            }
        }
    }

    fn rlpl(&self) {
        self.greeting_message();
        Cli::print_entry_header();
        std::io::stdin().lines().for_each(|line| {
            if let Ok(line) = line {
                lex(&line);
            }
            Cli::print_entry_header();
        });
    }

    pub fn rppl(&self) {
        self.greeting_message();
        Cli::print_entry_header();
        std::io::stdin().lines().for_each(|line| {
            if let Ok(line) = line {
                parse(&line);
            }
            Cli::print_entry_header();
        });
    }

    pub fn interpreter(&self) {
        self.greeting_message();
        Cli::print_entry_header();
        let mut evaluator = Evaluator::new();
        std::io::stdin().lines().for_each(|line| {
            if let Ok(line) = line {
                interpret(&mut evaluator, &line);
                Cli::print_entry_header();
            }
        });
    }

    pub fn compiler(&self) {
        self.greeting_message();
        Cli::print_entry_header();
        let mut compiler = Compiler::new();
        std::io::stdin().lines().for_each(|line| {
            if let Ok(line) = line {
                compile(&mut compiler, &line);
                Cli::print_entry_header();
            }
        });
    }

    fn greeting_message(&self) {
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
        if self.logo {
            println!("{greeting}");
        }
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

    fn run_file(&self, file_path: &str) {
        let contents = match Cli::read_file_contents(file_path) {
            Ok(contents) => contents,
            Err(error) => {
                eprintln!("{error}");
                return;
            }
        };

        match self.get_mode() {
            Mode::Lexer => lex(&contents),
            Mode::Parser => parse(&contents),
            Mode::Interpreter => {
                let mut evaluator = Evaluator::new();
                interpret(&mut evaluator, &contents)
            }
            Mode::Compiler => {
                let mut compiler = Compiler::new();
                compile(&mut compiler, &contents)
            }
        }
    }

    fn read_file_contents(file_path: &str) -> Result<String, Box<dyn Error>> {
        if file_path.ends_with(".monkey") {
            Ok(fs::read_to_string(file_path)?)
        } else {
            Err(String::from("Error: File must end with .monkey").into())
        }
    }
}

fn lex(line: &str) {
    let mut lexer = Lexer::new(line);
    let mut token = Token::Illegal;
    while token != Token::Eof {
        token = lexer.next_token();
        println!("{token}");
    }
}

fn parse(line: &str) {
    let lexer = Lexer::new(&line);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    if parser.errors.is_empty() {
        println!("{program}");
    } else {
        Cli::print_parse_errors(parser.errors);
    }
}

fn interpret(interpreter: &mut Evaluator, line: &str) {
    let lexer = Lexer::new(&line);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    if !parser.errors.is_empty() {
        Cli::print_parse_errors(parser.errors);
    }
    let evaluated = interpreter.eval(&program);
    println!("{evaluated}");
}

fn compile(compiler: &mut Compiler, line: &str) {
    let lexer = Lexer::new(&line);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    if !parser.errors.is_empty() {
        Cli::print_parse_errors(parser.errors);
    }
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
}
