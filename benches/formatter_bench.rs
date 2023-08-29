use chimpanzee::formatter::Formatter;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn long_formatter_benchmark(c: &mut Criterion) {
    let input = r#"
let fibonacci_it = fn (x) {
    if (x < 2) {
        return x;
 


    }
    let    iter = fn (i, table) {
     
        if (i > x) {
            return last(table);
        } else {
            let new_table = push(table, table[i - 1] + table[i - 2]);
            return iter(i + 1, new_table);
        }
    };
    return     iter(2, [0, 1]);
};
        let fib = fibonacci_it(20);
puts   (fib);
let fibonacci =      fn (x) {
    if (x < 2) {   
        x 
    } else  {
        fibonacci(x - 1) + fibonacci(x - 2)
    } 
}; 


            puts(fibonacci(3));
let filter = fn (arr, f) {
    let iter = fn (arr, accumulated) {
        if (len(arr) == 0) {
            accumulated
        } else {
            let head = first(arr);
            let tail = rest(arr);
            if (f(head)) {
                iter(tail, push(accumulated, head))
            } else {
                iter(tail, accumulated)
            }
        }
    };
    iter(arr, [])
};
let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 100];
let is_even = fn (x) {
    x / 2 * 2 == x
};
filter(a, is_even);
let foldl = fn (arr, initial    , f) {
    let iter          = fn (arr, result) {
        if (len(arr) == 0) {



            result;
        } else {
            iter(rest(arr), f(result, first(arr)))
        }
    };
    iter(arr, initial)
};
let a = [1, 2, 3, 4];
let sum = fn (x, y) {
    x + y
};
foldl(a, 0, sum);

        let foldr = fn(arr, initial, f) {
            let iter = fn(arr, result) {
                if (len(arr) == 0) {
                    result
                } else {
                    iter(rest(arr), f(first(arr), result));
                }
            };
            iter(arr, initial);
        };
        let a = [1, 2, 3, 4];
        let sum = fn(x, y) { x + y };
        foldr(a, 0, sum);
let input = r;
"
        let map = fn(arr, f) {
            let iter = fn(arr, accumulated) {
                if (len(arr) == 0) {
                    accumulated
                } else {
                    iter(rest(arr), push(accumulated, f(first(arr))));
                }
            };
            iter(arr, []);
        };
        let a = [1, 2, 3, 4];
        let double = fn(x) { x * 2 };
        map(a, double);
";
~/Projects/monkey-rs 3-Formatter *2 !1 ?1 ❯ cat monkey_examples/*                                                                                                                                                                  17:00:36
let fibonacci_it = fn (x) {
    if (x < 2) {
        return x;
    }
    let iter = fn (i, table) {
        if (i > x) {
            return last(table);
        } else {
            let new_table = push(table, table[i - 1] + table[i - 2]);
            return iter(i + 1, new_table);
        }
    };
    return iter(2, [0, 1]);
};
let fib = fibonacci_it(20);
puts(fib);
let fibonacci = fn (x) {
    if (x < 2) {
        x
    } else {
        fibonacci(x - 1) + fibonacci(x - 2)
    }
};
puts(fibonacci(30));
let filter = fn (arr, f) {
    let iter = fn (arr, accumulated) {
        if (len(arr) == 0) {
            accumulated
        } else {
            let head = first(arr);
            let tail = rest(arr);
            if (f(head)) {
                iter(tail, push(accumulated, head))
            } else {
                iter(tail, accumulated)
            }
        }
    };
    iter(arr, [])
};
let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 100];
let is_even = fn (x) {
    x / 2 * 2 == x
};
filter(a, is_even);
let foldl = fn (arr, initial, f) {
    let iter = fn (arr, result) {
        if (len(arr) == 0) {
            result
        } else {
            iter(rest(arr), f(result, first(arr)))
        }
    };
    iter(arr, initial)
};
let a = [1, 2, 3, 4];
let sum = fn (x, y) {
x + y
};
foldl(a, 0, sum);
let foldr = fn(arr, initial, f) {
let iter = fn(arr, result) {
if (len(arr) == 0) {
result
} else {
iter(rest(arr), f(first(arr), result));
}
};
iter(arr, initial);
};
let a = [1, 2, 3, 4];
let sum = fn(x, y) { x + y };
foldr(a, 0, sum);
let map = fn(arr, f) {let iter = fn(arr, accumulated) {
        if (len(arr) == 0) {
            accumulated
        } else {
            iter(rest(arr), push(accumulated, f(first(arr))));
                }
            };
            iter(arr, []);
        };
        let a = [1, 2, 3, 4];
        let double = fn(x) { x * 2 };
        map(a, double);

        "#;

    c.bench_function("Long format", |b| {
        b.iter(|| Formatter::format(black_box(input)))
    });
}

criterion_group!(benches, long_formatter_benchmark);
criterion_main!(benches);
