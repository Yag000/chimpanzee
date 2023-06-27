use interpreter_monkey::{evaluator::Evaluator, Lexer, Parser, Token};
use std::io::{self, Write};

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

fn main() {
    repl();
}
