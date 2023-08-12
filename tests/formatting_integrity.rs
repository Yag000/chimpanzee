// Test suite to assert that the formatting of the codebase is consistent with
// the source code and that the evaluation of a formatted code is the same as
// the evaluation of the source code.

use chimpanzee::{formatter::formatter::Formatter, utils::run_input};

fn run_test(input: &str) {
    let input_evaluation = run_input(input);

    let formatted_input = Formatter::format(input);

    println!("{}", formatted_input);

    let formatted_evaluation = run_input(formatted_input.as_str());

    println!("{}", formatted_evaluation);

    assert_eq!(input_evaluation, formatted_evaluation);
}

#[test]
fn test_fibonacci_rec_integrity() {
    let input = r#"
    let fibonacci = fn(x) {
        if (x == 0) {
            0
        } else {
            if (x == 1) {
                return 1;
            } else {
                fibonacci(x - 1) + fibonacci(x - 2);
            }
        }
    };
    fibonacci(10);
    "#;

    run_test(input);
}

#[test]
fn test_fibonacci_iter_integrity() {
    let input = r#"
let fibonacci_it= fn(x) {
	if (x < 2){
		return x;
	}
	let iter = fn (i, table) {
		if (i > x) {
			return table[x];
		} else {
			let new_table = push(table, table[i - 1] + table[i - 2]);
			return iter(i + 1, new_table);
		}
	};
	return iter(2, [0,1]);
};

let fib = fibonacci_it(20);

puts(fib);
    "#;

    run_test(input);
}

#[test]
fn test_map_integrity() {
    let input = r#"
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
    "#;

    run_test(input);
}

#[test]
fn test_fold_left_integrity() {
    let input = r#"
        let foldl = fn(arr, initial, f) {
            let iter = fn(arr, result) {
                if (len(arr) == 0) {
                    result
                } else {
                    iter(rest(arr), f(result, first(arr)));
                }
            };
            iter(arr, initial);
        };
        let a = [1, 2, 3, 4];
        let sum = fn(x, y) { x + y };
        foldl(a, 0, sum);
    "#;

    run_test(input);
}

#[test]
fn test_fold_right_integrity() {
    let input = r#"
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
    "#;

    run_test(input);
}

#[test]
fn test_filter_integrity() {
    let input = r#"
        let filter = fn(arr, f) {
            let iter = fn(arr, accumulated) {
                if (len(arr) == 0) {
                    accumulated
                } else {
                    let head = first(arr);
                    let tail = rest(arr);
                    if (f(head)) {
                        iter(tail, push(accumulated, head));
                    } else {
                        iter(tail, accumulated);
                    }
                }
            };
            iter(arr, []);
        };
        let a = [1, 2, 3, 4,5,6,7,8,9,11,100];
        let is_even = fn(x) { (x/2)*2 == x };
        filter(a, is_even);
    "#;

    // We use an obscure is_even function because Monkey does not yet support
    // the modulo operator.

    run_test(input);
}

#[test]
fn test_closure_integrity() {
    let input = r#"
        let new_adder = fn(x) {
            fn(y) { x + y };
        };
        let add_two = new_adder(2);
        add_two(2);
    "#;

    run_test(input);
}

#[test]
fn test_complex_arithmetic_integrity() {
    let input = r#"
    let x = (1 + 2) * 3 - 4 / 5 * ((6 + 7) * 8 + 9) - 10;
    let y = 1 + 2 * 3 - 4 / 5 * 6 + 7 * 8 + 9 - 10;
    let z = 434 - ((((1 + 2) * 3 - 4 / 5 * ((6 + 7) * 8 + 9) - 10) + 1 + 2 * 3 - 4 / 5 * 6 + 7 * 8 + 9 - 10) * 2);
    x * (y - z);
    "#;

    run_test(input);
}
