use std::env;

use interpreter_monkey::repl::{repl_compiler, run_file};

fn main() {
    let args: Vec<String> = env::args().collect();
    let is_repl = args.len() == 1;
    if is_repl {
        repl_compiler();
    } else {
        let filename = &args[1];
        run_file(filename);
    }
}
