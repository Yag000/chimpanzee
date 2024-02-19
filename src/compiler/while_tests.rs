#[allow(clippy::too_many_lines)]
#[cfg(test)]
pub mod tests {

    use crate::{
        compiler::{
            code::Opcode,
            test_utils::{flatten_instructions, run_compiler, CompilerTestCase},
        },
        object::Object,
    };

    #[test]
    fn test_while_statements() {
        let tests = vec![CompilerTestCase {
            input: r#"
                    while (true){
                        puts("yes");
                    }
                    "#
            .to_string(),
            expected_constants: vec![Object::STRING("yes".to_string())],
            expected_instructions: flatten_instructions(vec![
                Opcode::True.make(vec![]),            // 000
                Opcode::JumpNotTruthy.make(vec![15]), // 001
                Opcode::GetBuiltin.make(vec![5]),     // 004
                Opcode::Constant.make(vec![0]),       // 006
                Opcode::Call.make(vec![1]),           // 009
                Opcode::Pop.make(vec![]),             // 011
                Opcode::Jump.make(vec![0]),           // 012
                                                      // 015
            ]),
        }];

        run_compiler(tests);
    }

    #[test]
    fn test_break_in_while() {
        let tests = vec![CompilerTestCase {
            input: r"
                    while (true){
                        break;
                    }
                    "
            .to_string(),
            expected_constants: vec![],
            expected_instructions: flatten_instructions(vec![
                Opcode::True.make(vec![]),            // 000
                Opcode::JumpNotTruthy.make(vec![10]), // 001
                Opcode::Jump.make(vec![10]),          // 004
                Opcode::Jump.make(vec![0]),           // 007
                                                      // 010
            ]),
        }];

        run_compiler(tests);
    }

    #[test]
    fn test_nested_breaks_in_while() {
        let tests = vec![CompilerTestCase {
            input: r"
                    while (true){
                        while (true){
                            break;
                        }
                        break;
                    }
                    "
            .to_string(),
            expected_constants: vec![],
            expected_instructions: flatten_instructions(vec![
                Opcode::True.make(vec![]),            // 000
                Opcode::JumpNotTruthy.make(vec![20]), // 001
                Opcode::True.make(vec![]),            // 004
                Opcode::JumpNotTruthy.make(vec![14]), // 005
                Opcode::Jump.make(vec![14]),          // 008
                Opcode::Jump.make(vec![4]),           // 011
                Opcode::Jump.make(vec![20]),          // 014
                Opcode::Jump.make(vec![0]),           // 017
                                                      // 020
            ]),
        }];

        run_compiler(tests);
    }
    #[test]
    fn test_continue_in_while() {
        let tests = vec![CompilerTestCase {
            input: r"
                        while (true){
                            continue;
                        }
                        "
            .to_string(),
            expected_constants: vec![],
            expected_instructions: flatten_instructions(vec![
                Opcode::True.make(vec![]),            // 000
                Opcode::JumpNotTruthy.make(vec![10]), // 001
                Opcode::Jump.make(vec![0]),           // 004
                Opcode::Jump.make(vec![0]),           // 007
                                                      // 010
            ]),
        }];

        run_compiler(tests);
    }

    #[test]
    fn test_nested_continue_in_while() {
        let tests = vec![CompilerTestCase {
            input: r"
                    while (true){
                        while (true){
                            continue;
                        }
                        continue;
                    }
                    "
            .to_string(),
            expected_constants: vec![],
            expected_instructions: flatten_instructions(vec![
                Opcode::True.make(vec![]),            // 000
                Opcode::JumpNotTruthy.make(vec![20]), // 001
                Opcode::True.make(vec![]),            // 004
                Opcode::JumpNotTruthy.make(vec![14]), // 005
                Opcode::Jump.make(vec![4]),           // 008
                Opcode::Jump.make(vec![4]),           // 011
                Opcode::Jump.make(vec![0]),           // 014
                Opcode::Jump.make(vec![0]),           // 017
                                                      // 020
            ]),
        }];

        run_compiler(tests);
    }

    #[test]
    fn test_continue_and_break_in_while() {
        let tests = vec![CompilerTestCase {
            input: r"
                    while (true){
                        while (true){
                            continue;
                        }
                        break;
                    }
                    "
            .to_string(),
            expected_constants: vec![],
            expected_instructions: flatten_instructions(vec![
                Opcode::True.make(vec![]),            // 000
                Opcode::JumpNotTruthy.make(vec![20]), // 001
                Opcode::True.make(vec![]),            // 004
                Opcode::JumpNotTruthy.make(vec![14]), // 005
                Opcode::Jump.make(vec![4]),           // 008
                Opcode::Jump.make(vec![4]),           // 011
                Opcode::Jump.make(vec![20]),          // 014
                Opcode::Jump.make(vec![0]),           // 017
                                                      // 020
            ]),
        }];

        run_compiler(tests);
    }
}
