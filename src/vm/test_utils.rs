use crate::{
    compiler::{code::Instructions, Compiler},
    object::{test_utils::check_constants, Object},
    parser::parse,
    vm::VM,
};

#[allow(dead_code)]
pub(crate) struct VmTestCase {
    pub(crate) input: String,
    pub(crate) expected: Object,
}

#[allow(dead_code)]
pub(crate) fn run_vm_tests(tests: Vec<VmTestCase>) {
    for test in tests {
        println!("Running test: {}", test.input);
        let program = parse(&test.input);
        let mut compiler = Compiler::new();
        compiler.compile(program).unwrap();
        let bytecode = compiler.bytecode();

        for (i, constant) in bytecode.constants.iter().enumerate() {
            match constant {
                Object::COMPILEDFUNCTION(cf) => {
                    println!("Compiled function:");
                    let instructions = Instructions::new(cf.instructions.clone());
                    println!("{instructions}");
                }
                _ => println!("{i}: {constant}"),
            }
        }

        let mut vm = VM::new(bytecode);
        vm.run().unwrap();
        let got = vm.last_popped_stack_element().unwrap();
        check_constants(&[test.expected], &vec![got]);
    }
}

#[allow(dead_code)]
pub(crate) fn run_vm_with_error_output(input: &str) -> Result<(), String> {
    let program = parse(input);
    let mut compiler = Compiler::new();
    compiler.compile(program).unwrap();
    let bytecode = compiler.bytecode();

    let mut vm = VM::new(bytecode);
    vm.run()
}
