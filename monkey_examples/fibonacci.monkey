let fibonacci = fn (x) {
    if (x < 2) {
        x
    } else {
        fibonacci(x - 1) + fibonacci(x - 2)
    }
};
puts(fibonacci(30));
