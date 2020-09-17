Space for notes and experiments while reading "The Rust Programming Language" book.

Read the book online
https://doc.rust-lang.org/book/

Source code for the book
https://github.com/rust-lang/book

## 1. Getting Started

## 1.2. Hello, World!

2 steps to running a program written in Rust:

1. compiling the source code.
2. running the resulting executable file.

`rustc` is the rust compiler.

```
rustc main.rs
./main
```

## 1.3. Hello, Cargo!

Build the executable file with `cargo build`.
That will compile (starting at `src/main.rs`) into `target/debug/<name>`,
where `<name>` is whatever is in the `name` field of `cargo.toml`.

Running the file takes 2 step

- Building an executable: `cargo build`
- Running the executable: `./target/debug/<name>`

That can be done in 1 step:
`cargo run`

In both cases, the executable file will be unoptimized and contain debugging info.

Unoptimized because **the compiler is slow**.
To build an executable with optimizations, add the `--release` flag.
`cargo build --release`.
Cargo will put the resulting executable in `target/release` instead of `target/debug`.

`cargo check` to run the checks the compiler would, while not producing an executable.
This is much faster.

## 3. Common Programming Concepts

## 3.1. Variables and Mutability

Declare a variable with the `let` keyword.
`let x = 5`

Variables are immutable by default.

**The compiler is fussy. [This is a good thing.](https://nickymeuleman.netlify.app/blog/love-errors)**

The compiler checks for a lot of issues that can cause bugs and unwanted behaviour.

> In Rust, the compiler guarantees that when you state that a value won’t change, it really won’t change.

Use the `mut` keyword in front of the variable name to make it mutable.
`let mut x = 5;`

### Differences Between Variables and Constants

Declare constants with the `const` keyword.

_If variables are immutable by default, what's the difference between variables and constants?_

You aren't allowed to use `mut` on constants. No mutating, ever.

The type of the value _must_ be declared.
In a variable declaration, you _may_, in a constant declaration, you _must_.

Constants can only be set to a constant expression, _not_ the result of a function call or anything that could only be determined at runtime.

The naming convention for `const` is `SCREAMING_SNAKE_CASE`.
The naming convention for `let` is `snake_case`.

You can add underscores in the middle of numbers to help with readability, they won't affect the type.

`const MAX_POINTS: u32 = 100_000;`
is the same thing as
`const MAX_POINTS: u32 = 100000;`

### Shadowing

You can _declare_ a new variable with the same name as a variable that already exists.

_Why would I not just make that variable mutable instead?_

By redeclaring the variable, you can change the type of the variable.
A mutable variable can not change type. A new variable can have any and all types.
That new variable can just so happen to have the same name.

```rust
let spaces = "   ";
let spaces = spaces.len();
```

The first declaration of `spaces` is a string type.
The second declaration, which is a brand new variable, has a number type.

## 3.2. Data Types

> Every value in Rust is of a certain data type

When Rust compiles, it needs to know the types of all variables.
It can infer a bunch of those types, if not, declare them explicitly.

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

If that `u32` is left off, the compiler will complain and suggest you add a type annotation.

### Sidebranch: understanding that code snippet

The code they used here confuses me. I want to know what it does, off to the docs!
Specifically, what do those 2 methods do?

`"42"` is a string type. The method `parse()` gets called on it.

https://doc.rust-lang.org/stable/std/primitive.str.html#method.parse
This parses the string into another type, in this case, a number. More specifically, a `u32`.

The docs say:

> Because parse is so general, it can cause problems with type inference. As such, parse is one of the few times you'll see the syntax affectionately known as the 'turbofish': ::<>. This helps the inference algorithm understand specifically which type you're trying to parse into.

I don't understand half of that at this point so this is my takeaway:
We want to parse the string 42 into the number 42.
We call `.parse()` on `"42"`.
But because `.parse()` is so general, the compiler doesn't know which type that string turned into.
We do, because huumans smort.
`.parse()` returns something called `Result`.
What that does, I don't know yet.
What I do know is I can find the right documentation for the `.expect()` function called on it.

https://doc.rust-lang.org/stable/std/result/enum.Result.html#method.expect
The docs say:

> Returns the contained Ok value, consuming the self value.

That "self value", while I don't know exactly.
I assume that in this case it refers to the thing the `.expect()` function is called on. So, `"42"`.
A bit like `this` in JavaScript.

Alright, I have no idea what this `Ok` is here.
Time to look at what that `Result` is first.

https://doc.rust-lang.org/stable/std/result/enum.Result.html
The docs say:

> Result is a type that represents either success (Ok) or failure (Err).

So if there are no errors, `Ok` contains the resulting value.
If there was an error, `Err` contains the resulting error.

The docs for `expect()` makes more sense now.

> Returns the contained Ok value, consuming the self value.

On that same page:

> Panics if the value is an Err, with a panic message including the passed message, and the content of the Err.

So the string that was entered in the code example will be shown in an error message when the code panics.

My next question: _what does panic mean?_
Rust panicks when an unexpected error happens, when nothing handles that error.
Rust immediately exits and shows an error message.

To recap:

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

The `parse` function is called on `"42"`.
It returns a `Result` that contains either the successful result of the operation, or an error.
The `except` function is called on that `Result`.
It returns either that successful result, or it panics.
If it panicks, it will display the error from the `Result` in the console and it will add whatever was passed to `expect`.
In this case `"Not a number!"` will be shown if that code panics.

---

Back to the regularly scheduled Rust book.

### Scalar types

Scalar types represent a single value.
Rust has four of those _integers, floating-point numbers, booleans, and characters_.

#### integers

Integers are whole numbers.
There are 2 types of integers, signed ones and unsigned ones.
Signed ones can be negative (they include a sign, hence, signed).
Unsigned ones don't include a sign and are assumed to be positive.

| Length  | Signed | Unsigned |
| ------- | ------ | -------- |
| 8-bit   | i8     | u8       |
| 16-bit  | i16    | u16      |
| 32-bit  | i32    | u32      |
| 64-bit  | i64    | u64      |
| 128-bit | i128   | u128     |
| arch    | isize  | usize    |

The numbers are how many bits are used to store the number and directly affect how large the stored numbers can be.
For example: a u8 can store numbers from 0 to 255 (in decimal notation).
Under the hood. The signed numbers are stored using [two's complements](https://en.wikipedia.org/wiki/Two%27s_complement).

`isize` and `usize` are whoever many bytes correspond to your type of computer.
64 for 64bit architecture or 32 for 32bit architecture.

| Number literals | Example     |
| --------------- | ----------- |
| Decimal         | 98_222      |
| Hex             | 0xff        |
| Octal           | 0o77        |
| Binary          | 0b1111_0000 |
| Byte(u8 only)   | b'A'        |

Repeating from earlier: numbers can have a `_` in them to improve readability.
Decimal numbers are not prefixed.
Hex numbers are prefixed with `0x`.
Octal numbers are prefixed with `0o`.
Binary numbers are prefixed with `0b`.

A note on overflow:
Integer overflow will cause a program to panic.
When building with `--release`, those checks won't happen and two's complement wrapping occurs instead.
If you rely on it, you should do so explicity. By using [`Wrapping`](https://doc.rust-lang.org/std/num/struct.Wrapping.html).

#### Floating-point

Floating point numbers are numbers with a decimal point in them.

2 types: `f32` and `f64`.
`f32` is single precision. `f64` has double precision.
The default is `f64`.

#### Numeric Operations

The usual suspects exist for math operators in Rust, and they behave as expected.

| Operator | Operation      |
| -------- | -------------- |
| `+`      | addition       |
| `-`      | subtraction    |
| `*`      | multiplication |
| `/`      | division       |
| `%`      | remainder      |

#### Boolean Type

The `bool` type is a datatype which can be either `true` or `false`.
The boolean type uses one byte of memory.

#### The Character Type

A value with the `char` type holds a [Unicode scalar value](http://www.unicode.org/glossary/#unicode_scalar_value).
The `char` type is four bytes in size.

A `char` is specified with single quotes `'`.
Strings are specified with double quotes `"`.

Not only ASCII can be represented, more complex characters too.
Emojis are unicode, they fit.

```rust
let c = 'z';
let z = 'ℤ';
let face = '\u{1F600}';
// is the same as
let face = '😀';
```

### Compound types

Compound types group multiple values into one type.
Rust has two primitive compound types: tuples and arrays.

#### Tuple

A tuple is a grouping of multiple values.
A tuple has a fixed length.
A tuple can be made up out of multiple different types. (a type made out of types? Whaaaa, yo dawg /xzibit)

The syntax is: a comma seperated list inside parentheses.

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
```

`tup` contains the entire tuple.
It has a length of 3.
That tuple consists of an `i32`, an `f64`, and a `u8`.

The values inside a tuple can be destructured using pattern matching.

```rust
fn main() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    println!("The value of tup is: {:?}", tup);
}
```

Gets the first, second, and third value out of the tuple.
Those values are assigned to the variable names `x`, `y`, and `z`.

Doing this does not destroy the tuple, you can still access it afterwards.
Nevermind that `:?` there, that's a way to get tuples to print to the console.
This book will cover that later, I'm sure.

Accessing things in a tuple doesn't have to be done that way.
You can access what's inside a tuple via the index it's located at.

Syntax: `tuple.index`

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

#### Array

An array is a grouping of multiple values.
An array has a fixed length.
An array can only contain values of a single type.

Those first two are the same as a tuple.

The syntax is: a comma seperated list inside square brackets.

```rust
let a = [1, 2, 3, 4, 5];
```

The data is allocated on the stack, not the heap.
(I wouldn't have understood this if I didn't watch [Frederico Terzi's course](https://www.youtube.com/watch?v=fdx6KsjYhO8&list=PLcVYa6NNTe2PaUV1eMH9Di8WpqdhOTTIw)).
TL;DW: It's stored in a different way, at a different location in memory.

If you're looking for the array you know from other programming languages, one that can change size, that's a vector.
When unsure about whether to choose array or vector. Go with vector.

Syntax for the type annotation: `[type;amount]` where the amount is the length of the array.

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

You can use a similar syntax to the _type_ annotation to initialize a _value_.

```rust
let a = [3; 5];
// same as
let a = [3, 3, 3, 3, 3];
```

Will initialize an array that's of length 5 and every item in it will be `3`.

The values inside an array can be destructured using pattern matching.

```rust
fn main() {
    let arr = [1, 2, 3, 4, 5];
    let [v, w, x, y, z] = arr;
    println!("The value of y is: {}", y);
    println!("The value of arr is: {:?}", arr);
}
```

Accessing things in an array doesn't have to be done that way.
You can access what's inside an array via the index it's located at.

Syntax: `array[index]`

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
}
```

Trying to access an invalid element

```rust
// This code will panic
fn main() {
    let a = [1, 2, 3, 4, 5];
    let index = 10;
    let element = a[index];
    println!("The value of element is: {}", element);
}
```

This code will compile, but will panic at runtime when it comes across the attempted access of the element at index 10.
The compiler knows `index` is an integer.
But it doesn't keep track of which integer.
Thus it doesn't know the value held inside (`10`) is out of bounds for the array.
If we tried to access the array at index `10` by passing the `10` directly.
Rust would know and the code would not compile.

```rust
// This code will not compile
fn main() {
    let a = [1, 2, 3, 4, 5];
    let element = a[10];
    println!("The value of element is: {}", element);
}
```

## 3.3. Functions

syntax: `fn name(parameter, list) {}`
Keyword: `fn`, followed by a name, in snake_case by convention.
A set of parenthesis after the function name `()`.
ithin them possible parameters in a comma seperated list.
Followed by an opening curly brace `{` that marks the start of the function body.
Ends with an ending curly brace `}` to mark the end of the function body.
No semicolon after a function declaration.

The `main` function is special and serves as entry-point of programs.

A function can be declared after it's used in sourcecode order.
As long as it's declared, it can be used.

You _must_ declare the type of each paramater.

### Function Bodies Contain Statements and Expressions

Statements are instructions that _do_ something, they don't return a value.
Expressions evaluate to a value, they _return_ that value.

Statements: return the empty tuple `()`. Rust's way of saying: "nothing".
Expressions: return a value.

Function bodies are a bunch of statements.
A body can end in an expression.

Expressions can be part of a statement.

A variable declaration is a statement.
A function definition is a statement.

Because statements do not return values, you can't use the result of a statement and assign it to a variable.

```rust
// does not compile
fn main() {
    let x = (let y = 6);
}
```

`let y = 6` does not return anything, so there is nothing for the `x` variable to contain.

While a function _declaration_ is a statement, thus, returns nothing.
A function _invocation_ is an expression, thus, returns something.

Calling a macro is an expression.
(macros are the things with exclamation points `!`, like `println!`.)

A block that creates a new scope (`{}`) is an expression.

```rust
fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}
```

the:

```rust
{
    let x = 3;
    x + 1
};
```

Is an expression, it evaluates to a single value that is returned.

> Expressions do not include ending semicolons.
> If you add a semicolon to the end of an expression, you turn it into a statement, which will then not return a value

The last line in that block does not end in a semicolon.
The last line in that block is an expression, it gets returned as value for the block.

So the codeblock is equivalent to:

```rust
fn main() {
    let x = 5;

    let y = 4;

    println!("The value of y is: {}", y);
}
```

#### Functions with Return Values

Functions can return values to the code that calls them.
That makes sense, calling a function is an expression, it returns a value.

Syntax to declare the type of the returned value in a function: `->`

The value of the last expression in a codeblock is the return value (implicit return).

You can return from a function early/explicitly by using the `return` keyword followed by a value.

That means these 2 snippets are equivalent

```rust
fn five() -> i32 {
    5
}
```

```rust
fn five() -> i32 {
    return 5;
}
```

To recap, consider this snippet

```rust
// this will not compile
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1;
}
```

In working code, the call to `plus_one(5)` will return a value.
That value is then assigned to the variable `x`.

The `plus_one` function says it will return an `i32` ( `-> i32`).
But in this codesnippet the `plus_one` function doesn't return anything.
That's because the `x + 1;` line ends in a semicolon, making it a statement.
A statement returns nothing, expressed in Rust by the empty tuple `()`, causing an error.

```err
error[E0308]: mismatched types
 --> src/main.rs:7:24
  |
7 | fn plus_one(x: i32) -> i32 {
  |    --------            ^^^ expected `i32`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
8 |     x + 1;
  |          - help: consider removing this semicolon
```

The error tells us exactly what to do.
Removing that semicolon turns the statement into an expression.
That expression is the last one in the function and returns a value.
That value is an integer and fulfills the type declared in the function definition `i32`.

## 3.4. Comments

syntax:

- single line comments: `//`
- block comment start: `/*`
- block comment end: `*/`

A line comment starts at the `//`, and continues until the end of the line.
They can be placed inline.
By convention, they have their own lines, often directly above the relevant line of code.

So, technically, I could do this:

```rust
fn main() {
    let num = /* ooh, I bet this is a number,
    exciting,
    let's see */ 42;
}
```

I solemnly swear I won't do that (probably).

## 3.5. Control Flow

### if Expressions

`if` is an expression.
That means it returns a value.

An `if` allows you to branch code based on a condition.
If it's met, run this block of code, if it's not, don't.

syntax: `if` keyword followed by a condition, optionally surrounded by parentheses, generally not.
Followed by a block of code, marked by curly braces `{}`.

There can be multiple codeblocks.
Because there can be an `else` block, and `else if` blocks.
Blocks associated with an `if` are called _arms_.

> If you don’t provide an `else` expression and the condition is false, the program will just skip the `if` block and move on to the next bit of code.

A condition _must_ be a boolean.

### Handling Multiple Conditions with else if

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

This snippet checks each `if` (or `else if`) individually and only executes the first branch that satisfies its condidion.
Eventhough `6` is divisible by `2`, the line `"number is divisible by 2"` will never be printed.
Only the first branch that satisfies its condition (`else if number % 3 == 0`) will,
so we see `"number is divisible by 3"`.

> That’s because Rust only executes the block for the first true condition, and once it finds one, it doesn’t even check the rest.

If there are a bunch of `else if` branches, it might be cleaner to use a different construct called `match`.
From what I can gather right now, that'll be like a JavaScript `switch` statement.

### Using if in a let Statement

Because `if` is an expression, it returns a value.
We can use it to assign a value to a variable by using it as the right side of a `let` statement.

The value of the entire `if` depends on which block of code executes.
That means the returned result from each arm of the `if` expression must be of the same type.

```rust
// this code will not compile
fn main() {
    let condition = true;

    let number = if condition { 5 } else { "six" };

    println!("The value of number is: {}", number);
}
```

The types of `5` and `"six"` don't match.

> This won’t work because variables must have a single type.

That means the code doesn't compile and displays an error.

```err
error[E0308]: if and else have incompatible types
 --> src/main.rs:4:44
  |
4 |     let number = if condition { 5 } else { "six" };
  |                                 -          ^^^^^ expected integer, found `&str`
  |                                 |
  |                                 expected because of this
```

The reason the type of a variable has to be known at compile time:
It allows the compiler to do a bunch of checks and keeps the complexity of the compiler down.

### Repetition with Loops

A loop executes the code inside its body more than once.
(At least, it typically does, else, why use a loop amirite?)

> Rust has three kinds of loops: `loop`, `while`, and `for`.

#### Repeating Code with loop

Syntax: the `loop` keyword followed by a codeblock.

The codeblock will execute over and over until you explicitly tell it to stop.
In other words: `loop` creates an infinite loop.

The stopping is done with the `break` keyword.
You `break` out of the `loop`.

You can use the `continue` keyword to skip the rest of the iteration and start a new one.

#### Returning Values from Loops

To return a value from a loop, put the expression you want to return after the `break` keyword.

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}
```

The body of the loop will continue to execute as long as it is not broken out of with `break`.
When `counter` has the value of `10`, the `if` block is entered.
Inside that block, the loop is broken with `break`.
Because it has `counter * 2` after it. The value that expression evaluates to will be returned from the `loop`.
That evaluates to `20`, which is then assigned to the variable `result`.
Note the loop body ends with a semicolon.
That is the semicolon to end the variable assignment statement.

#### Conditional Loops with while

A `while` loop has a condition.
The loop body will execute if that condition evaluates to `true`.
When the end of the body is reached, it will evaluate that condition again.
If it passes again, the cycle continues.

> When the condition ceases to be true, the program calls break, stopping the loop.

This line in the book is kinda misleading, I tested if making the condition true in the middle of the `while` body breaks the loop.
It does not, so I'm sticking with my explanation above.

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;

         println!("number at end of loop: {}", number);
    }

    println!("LIFTOFF!!!");
}
```

Prints

```
3!
number at end of loop: 2
2!
number at end of loop: 1
1!
number at end of loop: 0
LIFTOFF!!!
```

Since the `number at end of loop: 0` got printed, that means the loop didn't immediately `break` as soon as the condition became `true`.
The loop executed to the end of the body first.

> While a condition holds true, the code runs; otherwise, it exits the loop.

#### Looping Through a Collection with for

While you can (hehe, while) loop over the elements of a collection with `while`, often `for` loops are preferred.

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
```

Is errorprone, because of the check with the magic number. What if the length ever changes?
(I'd check with `a.len()` instead, but 🤷‍♂️)
Is slow, because the check has to be evaluated at every step in the loop.

with a `for` loop.

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
```

`for` is the most used loop.

`for in` can be used to iterate through an `Iterator`.

Like in python, you can iterate over a range (which is an iterator).
The Rust standard library provides a `Range` type for that.

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
```

`1..4` is a `Range` that goes from `1` to `3` in steps of one.
The first number in the range is inclusive, the last number is exclusive.
If you want the last number to be included, add the `=`.
So `1..4` would be the same as `1..=3`.
`rev` is a method to reverse it, so the loop goes from `3` to `1`.
