use criterion::{black_box, criterion_group, criterion_main, Criterion};

use chimpanzee::utils::{compile_program, execute_interpreter, execute_vm, parse_program};

const ARRAY_APPEND: &str = r#"
let push_n = fn (arr, n) {
    if (n < 0) {
        arr
    } else {
        let new_arr = push(arr, n);
        push_n(new_arr, n - 1)
    }
};
let a = [];
push_n(a, 500);
"#;

pub fn array_append_compiler_benchmark(c: &mut Criterion) {
    let program = parse_program(ARRAY_APPEND);
    let compiler = compile_program(program);
    c.bench_function("Array append 100000 compiler", |b| {
        b.iter(|| execute_vm(black_box(&compiler)))
    });
}

pub fn array_append_interpreter_benchmark(c: &mut Criterion) {
    let program = parse_program(ARRAY_APPEND);
    c.bench_function("Array append 100000 interpreter", |b| {
        b.iter(|| execute_interpreter(black_box(&program)))
    });
}

criterion_group!(
    benches,
    array_append_compiler_benchmark,
    array_append_interpreter_benchmark
);
criterion_main!(benches);
