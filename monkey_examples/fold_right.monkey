
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
