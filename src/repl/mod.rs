mod errors;

use crate::{
    compiler::{
        symbol_table::SymbolTable,
        {Bytecode, Compiler},
    },
    interpreter::evaluator::Evaluator,
    lexer::{token::Token, Lexer},
    object::{
        builtins::BuiltinFunction,
        {Object, NULL},
    },
    parser::{Parser, ParserErrors},
    repl::errors::{CompilerError, LexerErrors, RuntimeError},
    vm::{GLOBALS_SIZE, VM},
};

use clap_derive::{Parser, ValueEnum};
use std::io::{self, Write};
use std::rc::Rc;
use std::{error::Error, fs};

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
pub struct ReplCli {
    /// Input file, if not specified, the REPL will be launched
    filename: Option<String>,

    /// Set the mode to use, if not specified, compiler is used
    #[arg(short, long, value_name = "MODE")]
    mode: Option<Mode>,

    /// Show the logo
    #[clap(long)]
    logo: bool,
}

impl ReplCli {
    fn get_input_type(&self) -> InputType {
        match &self.filename {
            Some(filename) => InputType::File(filename.to_string()),
            None => InputType::Repl,
        }
    }
    fn get_mode(&self) -> Mode {
        match &self.mode {
            Some(mode) => *mode,
            None => Mode::Compiler,
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        match &self.get_input_type() {
            InputType::Repl => match self.get_mode() {
                Mode::Lexer => Ok(self.rlpl()?),
                Mode::Parser => Ok(self.rppl()?),
                Mode::Interpreter => self.interpreter(),
                Mode::Compiler => self.compiler(),
            },
            InputType::File(filename) => self.run_file(filename),
        }
    }

    fn rlpl(&self) -> Result<(), LexerErrors> {
        self.greeting_message();
        ReplCli::print_entry_header();
        let mut errors = LexerErrors::new();
        std::io::stdin().lines().for_each(|line| {
            if let Ok(line) = line {
                let new_error = lex(&line);
                if let Err(err) = new_error {
                    errors.add_errors(err);
                }
            }
            ReplCli::print_entry_header();
        });
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn rppl(&self) -> Result<(), ParserErrors> {
        self.greeting_message();
        ReplCli::print_entry_header();
        let mut errors = ParserErrors::new();
        std::io::stdin().lines().for_each(|line| {
            if let Ok(line) = line {
                let new_error = parse(&line);
                if let Err(err) = new_error {
                    errors.add_errors(err.errors);
                }
            }
            ReplCli::print_entry_header();
        });

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn interpreter(&self) -> Result<(), Box<dyn Error>> {
        self.greeting_message();
        ReplCli::print_entry_header();
        let mut evaluator = Evaluator::new();
        for line in std::io::stdin().lines().flatten() {
            match interpret(&mut evaluator, &line) {
                Ok(str) => {
                    if str != Object::NULL.to_string() {
                        println!("{str}");
                    }
                }
                Err(err) => eprintln!("{err}",),
            }
            ReplCli::print_entry_header();
        }
        Ok(())
    }

    pub fn compiler(&self) -> Result<(), Box<dyn Error>> {
        self.greeting_message();
        ReplCli::print_entry_header();
        let mut symbol_table = SymbolTable::new();
        for (i, builtin) in BuiltinFunction::get_builtins_names().iter().enumerate() {
            symbol_table.define_builtin(i, builtin.clone());
        }
        let mut constants = Vec::new();
        let mut globals = {
            let mut v = Vec::with_capacity(GLOBALS_SIZE);
            (0..GLOBALS_SIZE).for_each(|_| v.push(Rc::new(NULL)));
            v
        };
        for line in std::io::stdin().lines().flatten() {
            let lexer = Lexer::new(&line);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();
            if !parser.errors.is_empty() {
                eprintln!("{}", parser.errors);
            }
            let mut compiler = Compiler::new_with_state(symbol_table.clone(), constants.clone());
            if let Err(err) = compiler.compile(program) {
                let err = CompilerError::new(err);
                eprintln!("{err}",);
            }

            let mut vm = VM::new_with_global_store(compiler.bytecode(), globals.clone());
            if let Err(err) = vm.run() {
                eprintln!("{err}",);
            }
            constants = compiler.constants;
            symbol_table = compiler.symbol_table;

            let vm_result: Result<String, Box<dyn Error>> = match vm.last_popped_stack_element() {
                Ok(obj) => match obj.as_ref() {
                    Object::ERROR(error) => Err(Box::new(RuntimeError::new(error.clone()))),
                    x => Ok(x.to_string()),
                },
                Err(_) => Err(Box::new(RuntimeError::new(String::from(
                    "No object returned from VM",
                )))),
            };

            globals = vm.globals;
            match vm_result {
                Ok(str) => {
                    if str != Object::NULL.to_string() {
                        println!("{str}");
                    }
                }
                Err(err) => eprintln!("{err}",),
            }

            ReplCli::print_entry_header();
        }
        Ok(())
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
        println!("Welcome to the Monkey programming language! Compiler and Interpreter by @Yag000, in Rust");
        println!("Feel free to type in commands\n");
    }

    fn print_entry_header() {
        print!(">> ");
        io::stdout().flush().unwrap();
    }

    fn run_file(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let contents = ReplCli::read_file_contents(file_path)?;

        match self.get_mode() {
            Mode::Lexer => lex(&contents)?,
            Mode::Parser => parse(&contents)?,
            Mode::Interpreter => {
                let mut evaluator = Evaluator::new();
                interpret(&mut evaluator, &contents)?;
            }
            Mode::Compiler => {
                let bytecode = compile(&contents)?;
                run_vm(bytecode)?;
            }
        }
        Ok(())
    }

    fn read_file_contents(file_path: &str) -> Result<String, Box<dyn Error>> {
        if file_path.ends_with(".monkey") {
            Ok(fs::read_to_string(file_path)?)
        } else {
            Err(String::from("Error: File must end with .monkey").into())
        }
    }
}

fn lex(line: &str) -> Result<(), LexerErrors> {
    let mut lexer = Lexer::new(line);
    let mut token = Token::Illegal(String::new());
    let mut errors = LexerErrors::new();
    while token != Token::Eof {
        token = lexer.next_token();
        if let Token::Illegal(ref s) = token {
            errors.add_error(s.clone());
        }
        println!("{token}");
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn parse(line: &str) -> Result<(), ParserErrors> {
    let lexer = Lexer::new(line);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    if parser.errors.is_empty() {
        println!("{program}");
        Ok(())
    } else {
        Err(parser.errors)
    }
}

fn interpret(interpreter: &mut Evaluator, line: &str) -> Result<String, Box<dyn Error>> {
    let lexer = Lexer::new(line);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    if !parser.errors.is_empty() {
        return Err(Box::new(parser.errors));
    }
    let evaluated = interpreter.eval(program);

    if let Object::ERROR(error) = evaluated {
        Err(Box::new(RuntimeError::new(error)))
    } else {
        Ok(evaluated.to_string())
    }
}

fn compile(line: &str) -> Result<Bytecode, Box<dyn Error>> {
    let lexer = Lexer::new(line);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    if !parser.errors.is_empty() {
        return Err(Box::new(parser.errors));
    }
    let mut compiler = Compiler::new();
    match compiler.compile(program) {
        Ok(()) => Ok(compiler.bytecode()),
        Err(e) => Err(Box::new(CompilerError::new(e))),
    }
}

fn run_vm(bytecode: Bytecode) -> Result<String, Box<dyn Error>> {
    let mut vm = VM::new(bytecode);
    match vm.run() {
        Ok(()) => match vm.last_popped_stack_element() {
            Ok(obj) => match obj.as_ref() {
                Object::ERROR(error) => Err(Box::new(RuntimeError::new(error.clone()))),
                x => Ok(x.to_string()),
            },
            Err(_) => Err(Box::new(RuntimeError::new(String::from(
                "No object returned from VM",
            )))),
        },
        Err(e) => Err(Box::new(RuntimeError::new(e))),
    }
}
