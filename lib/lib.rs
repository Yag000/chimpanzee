use compiler::compiler::Compiler;
use interpreter::evaluator::Evaluator;
use lexer::lexer::Lexer;
use object::object::Object;
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

pub fn execute_vm(compiler: &Compiler) -> Object {
    let bytecode = compiler.bytecode();
    let mut vm = VM::new(bytecode);
    vm.run().unwrap();
    vm.last_popped_stack_element().unwrap().as_ref().clone()
}

pub fn execute_interpreter(program: &Program) {
    let mut interpreter = Evaluator::new();

    interpreter.eval(program.clone());
}

pub fn run_input(input: &str) -> Object {
    let program = parse_program(input);
    let compiler = compile_program(program.clone());
    execute_vm(&compiler)
}
