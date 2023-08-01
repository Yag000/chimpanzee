use criterion::{black_box, criterion_group, criterion_main, Criterion};

use monkey::{compile_program, execute_interpreter, execute_vm, parse_program};

const FIBONACCI_20: &str = r#"
        let fibonacci = fn(x) {
            if (x == 0) {
                0
            } else {
                if (x == 1) {
                    1
                } else {
                    fibonacci(x - 1) + fibonacci(x - 2)
                }
            }
        };
        fibonacci(20);
        "#;

pub fn compiler_benchmark(c: &mut Criterion) {
    let program = parse_program(FIBONACCI_20);
    let compiler = compile_program(program);
    c.bench_function("fibonacci 20 compiler", |b| {
        b.iter(|| execute_vm(black_box(&compiler)))
    });
}

pub fn interpreter_benchmark(c: &mut Criterion) {
    let program = parse_program(FIBONACCI_20);
    c.bench_function("fibonacci 20 interpreter", |b| {
        b.iter(|| execute_interpreter(black_box(&program)))
    });
}

criterion_group!(benches, compiler_benchmark, interpreter_benchmark);
criterion_main!(benches);
