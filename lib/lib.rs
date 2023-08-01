use compiler::compiler::Compiler;
use interpreter::evaluator::Evaluator;
use lexer::lexer::Lexer;
use parser::ast::Program;
use parser::parser::Parser;
use vm::vm::VM;

pub fn parse_program(input: &str) -> Program {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    parser.parse_program()
}

pub fn compile_program(program: Program) -> Compiler {
    let mut compiler = Compiler::new();
    compiler.compile(program).unwrap();
    compiler
}

pub fn execute_vm(compiler: &Compiler) {
    let bytecode = compiler.bytecode();
    let mut vm = VM::new(bytecode);
    vm.run().unwrap();
}

pub fn execute_interpreter(program: &Program) {
    let mut interpreter = Evaluator::new();

    interpreter.eval(program.clone());
}
