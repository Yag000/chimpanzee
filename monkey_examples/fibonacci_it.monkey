let fibonacci_it= fn(x) {
	if (x < 2){
		return x;
	}
	let iter = fn (i, table) {
		if (i > x) {
			return last(table);
		} else {
			let new_table = push(table, table[i-1] + table[i - 2]);
			return iter(i + 1, new_table);
		}
	};
	return iter(2, [0,1]);
};

let fib = fibonacci_it(20);

puts(fib);
