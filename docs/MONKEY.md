# Monkey syntax

## Types

The monkey language supports the following types:

- Integers
- Booleans
- Strings
- Arrays
- Hashes
- Functions (yes, functions are a type in Monkey)

### Integers

Integers are 64-bit signed integers. They are written as follows:

```monkey
let a = 1;
let b = 2;
```

#### Operators

Integers support the following operators:

- `+`: addition
- `-`: subtraction
- `*`: multiplication
- `/`: division (integer division)
- `%`: modulo
- `==`: equality
- `!=`: inequality
- `<`: less than
- `>`: greater than
- `<=`: less than or equal to
- `>=`: greater than or equal to

### Booleans

Booleans are either `true` or `false`. They are written as follows:

```monkey
let a = true;
let b = false;
```

#### Operators

Booleans support the following operators:

- `==`: equality
- `!=`: inequality
- `!`: negation
- `&&`: and
- `||`: or

### Strings

Strings are sequences of characters. They are written as follows:

```monkey
let a = "Hello, world!";
```

#### String interpolation

Strings can be interpolated using the `+` operator. The following example shows how to interpolate a string:

```monkey
let a = "Hello " + "world!";
```

##### Built-in functions

Strings have the following built-in functions:

- `len()`: returns the length of the string

### Arrays

Arrays are sequences of values. They are written as follows:

```monkey
let a = [1, "two", [1,2,3]];
```

They can contain any type of value, including other arrays and functions.

#### Indexing

Arrays can be indexed using the `[]` operator. The index must be an integer. The index starts at 0. The following example shows how to index an array:

```monkey
let a = [1,2,3];
let b = a[0]; // b = 1
```

#### Built-in functions

Arrays have the following built-in functions:

- `len(array)`: returns the length of the array
- `first(array)`: returns the first element of the array
- `last(array)`: returns the last element of the array
- `rest(array)`: returns a new array containing all elements except the first
- `push(array,  value)`: returns a new array containing all elements of the original array and the new value (at the end)

### Hashes

Hashes are key-value pairs. They are written as follows:

```monkey
let a = {"one": 1, "two": 2};
```

The keys can be: `Integer` , `Boolean` or `String`. The values can be any type of value, including other hashes and functions.

#### Indexing

Hashes can be indexed using the `[]` operator. The index must be a key. The following example shows how to index a hash:

```monkey
let a = {"one": 1, "two": 2};
let b = a["one"]; // b = 1
```

#### Built-in functions

For now hashes have no built-in functions. In the future the following built-in functions will be supported:

- `keys(hash)`: returns an array containing all keys of the hash
- `values(hash)`: returns an array containing all values of the hash
- `add(hash, key, value)`: returns a new hash containing all key-value pairs of the original hash and the new key-value pair

### Functions

The function syntax is as follows:

```monkey
let add = fn(a, b) {
    return a + b;
};
```

Functions are first-class citizens in Monkey. This means that they can be assigned to variables, passed as arguments to other functions, and returned from other functions.
One example is the map function:

```monkey
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
```

### Return

Functions can return a value using the `return` keyword. The following example shows how to return a value from a function:

```monkey
let add = fn(a, b) {
    return a + b;
};
```

Note that the `return` keyword is optional, Monkey allows implicit returns. The following example shows how to use an implicit return:

```monkey
let add = fn(a, b) {
    a + b;
};
```

## Variables

Variables are declared using the `let` keyword. The following example shows how to declare a variable:

```monkey
let a = 1;
```

Shadowing is supported. The following example shows how to shadow a variable:

```monkey
let a = 1;
let a = 2;
```

## Control flow

### If-else

The if-else syntax is as follows:

```monkey

if (condition) {
    // code
} else {
    // code
}
```

The following example shows how to use if-else:

```monkey
let a = 1;
if (a == 1) {
    return "a is 1";
} else {
    return "a is not 1";
}
```

### Loops

While loops have been implemented.

```monkey
let a = 1;
while (a < 4) {
    puts(a);
    let a = a + 1;
}
```

You can also use `break` and `continue` inside a loop.

```monkey
let a = 1;
while (a < 4) {
    if (a == 2) {
        break;
    }
    puts(a);
    let a = a + 1;
}
```

## Comments

For now comments are not supported ( not a huge loss :) )

## Built-in functions

Monkey has the following built-in functions:

- `puts(value)`: prints the value to the console
- `len(value)`
- `first(array)`
- `last(array)`
- `rest(array)`
- `push(array, value)`
