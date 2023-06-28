let fibonacci_it= fn(x) {
	if (x < 2){
		return x;
	}
	let iter = fn (i, table) {
		if (i > x) {
			return table[x];
		} else {
			let table = push(table, table[i - 1] + table[i - 2]);
			return iter(i + 1, table);
		}
	};
	return iter(2, [0,1]);
};

let fib = fibonacci_it(20);

puts(fib);
