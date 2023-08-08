let filter = fn(arr, f)    {
    let iter = fn(arr, accumulated) {
if (len(arr)==0) {
            accumulated
    } else {
            let head = first(arr);
let tail =rest(arr);
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
