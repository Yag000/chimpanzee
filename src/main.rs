use std::env;

use repl::repl::{compiler, run_file};

fn main() {
    let args: Vec<String> = env::args().collect();
    let is_repl = args.len() == 1;
    if is_repl {
        compiler();
    } else {
        let filename = &args[1];
        run_file(filename);
    }
}
