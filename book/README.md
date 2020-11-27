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

## 2. Programming a Guessing Game

Doing the chapter 2 exercise after reading chapter 3 that introduces a bunch of concepts.

> We’ll implement a classic beginner programming problem: a guessing game.
> Here’s how it works: the program will generate a random integer between 1 and 100.
> It will then prompt the player to enter a guess. After a guess is entered,
> the program will indicate whether the guess is too low or too high.
> If the guess is correct,
> the game will print a congratulatory message and exit.

### Setting Up a New Project

```
cargo new guessing_game
```

Cargo takes a bunch of defaults.
https://github.com/rust-lang/cargo/blob/0.27.0/src/cargo/ops/cargo_new.rs#L587-L633
For example: my email is taken from the one set in `git`.

### Processing a Guess

To take user input, use the `io` library on the standard library.

```rust
use std::io;
```

While it is in the standard library, it's not imported by default.
Only a few things are included in the [prelude](https://doc.rust-lang.org/std/prelude/index.html).

### Storing Values with Variables

```rust
let mut guess = String::new();
```

`new` is an associated function of the `String` type.
It returns a `String` of whatever is the argument of `new`.
Here, nothing is given as argument, so `guess` becomes an empty `String`.

```rust
io::stdin()
    .read_line(&mut guess)
```

This stops execution and lets the user enter something in the terminal (to stdin).
When they press enter, the resulting string (including the newline, `\n`!) is appended to the variable passed as argument to `read_line`.

(more on that `&` in chapter 3)

### Handling Potential Failure with the Result Type

That line of code can fail. It return a `Result` type. (more specifically [`std::io::Result`](https://doc.rust-lang.org/std/io/type.Result.html)).
The `Result` type is an [enumeration](https://doc.rust-lang.org/book/ch06-00-enums.html) that contains 2 variants, it can be one of these 2.

1. `Ok` - indicates success
2. `Err` - indicates error

Quite straightforward.
The cool thing is, those variants _contain values_.

- The `Ok` variant will contain the result if the operation that returned a `Result` was successful.
- The `Err` variant will contain the error if the operation that returned a `Result` failed.

```rust
io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
```

We chain [`.expect`](https://doc.rust-lang.org/std/result/enum.Result.html#method.expect) to the operation that returned a `Result`.
That will either return the value contained in the `Ok` _or_ panick (stop the program) with the provided message if it finds an `Err`.

### Printing Values with println! Placeholders

```rust
println!("You guessed: {}", guess);
```

> The set of curly brackets, {}, is a placeholder: think of {} as little crab pincers that hold a value in place.

You can use multiple placeholders. The values passed to the `println` macro populate them in order.
You can name the placeholders.

```rust
println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");
```

### Generating a Secret Number

To generate a random number (between 1 and 100 for the exercise), use a Rust package, called a crate.
More specifically, the [`rand` crate](https://crates.io/crates/rand).

### Using a Crate to Get More Functionality

2 types of crates:

1. binary crate - is an executable
2. library crate - contains code to be used in other programs

The `rand` crate is a library crate.
Add it to the dependencies in `cargo.toml`.

```toml
[dependencies]
rand = "0.5.5"
```

The `"0.5.5"` is the version.
Rust follows [SemVer](http://semver.org/).

Building the code with `cargo build` now will first download and compile the code of the crates in dependencies.

It also creates a `cargo.lock` file, similar to the `package-lock.json` or `yarn.lock` in JavaScript.
The package registry, or crate registry, is [crates.io/](https://crates.io/). Similar to npm in JavaScript.

### Generating a Random Number

They first import a trait.
What a trait is, I don't know yet, but they say it'll be covered in chapter 10.
That trait has a method attached to it we'll use in the program.

> The Rng trait defines methods that random number generators implement,
> and this trait must be in scope for us to use those methods. Chapter 10 will cover traits in detail.

```rust
use rand::Rng;
```

The random number is stored in the `secret_number` variable.

```rust
let secret_number = rand::thread_rng().gen_range(1, 101);
```

`thread_rng` returns a thread-local random number generator.
`gen_range(1, 101)` uses that random number generator to return a number between 1 (inclusive) and 101 (exclusive). So between 1 and 100.
`gen_range` is a method defined by the `Rng` trait.

### Comparing the Guess to the Secret Number

To compare the guess to the random number, we'll use `cmp`.
`cmp` is a method that returns an `Ordering` enum (like `Result` is an enum).

So `Ordering` needs to be imported.

```rust
use std::cmp::Ordering;
```

The `Ordering` enum has 3 variants.

1. `Less` - a compared value is less than another.
2. `Equal` - a compared value is equal to another.
3. `Greater` - a compared value is greater than another.

We use a `match` expression to execute a codeblock based on what variant we get back.

```rust
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => println!("You win!"),
}
```

`match` is exhaustive and the compiler will force you to cover every possible case.
In this example, all 3 of the possible variants for `Ordering`.
As soon as Rust find a matching, it will execute the associated arm.
It won't check other arms after that, the `match` ends after one arm is executed.

However, this code won't compile yet, because `guess` and `secret_number` are of different types.
The `guess` comes in as input from the terminal, and is a `String`. `secret_number` is an integer.
Let's convert `guess` to also be an integer.

This is done by calling this line after the user input is received:

```rust
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

From chapter 3, the `guess` variable is shadowed.
A new variable is declared that just so happens to have the same name as the existing `guess` variable.
This lets us change the type of the thing stored in `guess`. Here, from `String` to `u32`.

`trim` is called on the old `guess`, eliminating any whitespace.
Remember the newline (`\n`) was stored when the user entered the input, that's gone now.
Then `parse` gets called on the result, parsing it into a new type (the one we declared, `u32`).
This return a `Result`, so we call `except` on it to either return the successfully parsed integer, or panic.

### Allowing Multiple Guesses with Looping

The program exists after one guess now.
We'll move all the code after generating the secret number inside a `loop`.

### Quitting After a Correct Guess

Next, handle the case where the user guesses correctly and exit the program.
Modify the `match` arm for the `Equal` case and break the loop, allowing the program to come to an end.

```rust
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => {
        println!("You win!");
        break;
    }
}
```

That case is now longer than one line, so we're putting curly braces around the arms.
Use the `break` keyword to break out of the `loop`.

### Handling Invalid Input

Right now, the program panics if the user supplies an invalid guess (thanks to the `except` after the `parse`).

`match` is an expression, it returns a value.

Change the line to handle a possible `Err` returned from the `Result` that `parse` returns instead of panicing with `except`.

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```

- If `parse` returns the `Ok` variant of `Result`, return the number it holds and store it in `guess`.
- If `parse` returns the `Err` variant of `Result`, exit this iteration of the loop and start a new one with `continue`. (starts from top of `loop`)

> The underscore, `_`, is a catchall value; in this example, we’re saying we want to match all `Err` values, no matter what information they have inside them.

## 4. Understanding Ownership

Ownership enables Rust to make guarantees about memory safety etc.
It's the reason the language doesn't have a garbage collector and also doesn't require you to manually manage memory.

## 4.1. What is Ownership?

Ownership is a central feature of Rust and has deep implications.
It's the way Rust manages the memory it uses.
It does not have a garbage collector.
It does not require you to allocate and free memory explicitly.

Rust manages memory through the system of ownership and checks if code adheres to a set of ownership rules at compile time.

### The Stack and the Heap

The stack and the heap are 2 places in memory where Rust stores information.
A [stack](<https://en.wikipedia.org/wiki/Stack_(abstract_data_type)>) is a LIFO (last in, first out) data structure.

- _push_ to the stack. You add things by "putting them on top" of the stack.
- _pop_ off the stack. You remove things by "taking them from the top" of the stack.

**Every piece of data stored on the stack must have a known, fixed size.**
Data with unknown size at compile time, or a size that can change, must be stored on the heap.

The heap is less organized.
When you put data on the heap, you request some space from the memory allocator (piece of logic that manages data stored in the heap).
The allocator then marks a part of the heap as in use and returns a pointer to that location in the heap (the address).
This process is called allocating on the heap, or allocating for short.

So: pushing values onto the stack is not allocating!

The pointer has a known, fixed size and is then pushed to the stack.
For things on the heap: the data is located on the heap, a pointer to that location in the heap is stored on the stack.

- Pushing to the stack is faster than allocating on the heap.
- Accessing data on the heap is slower than accessing data on the stack (you have to follow a pointer to get to the right address in the heap).

> When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap)
> and the function’s local variables get pushed onto the stack.
> When the function is over, those values get popped off the stack.

Ownership addresses many of the problems that come with managing memory.
Like keeping duplication low, removing unused data, ...

### Ownership rules

- Each value in Rust has a variable that’s called its _owner_.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

### Variable Scope

> A scope is the range within a program for which an item is valid

A variable is valid from the point it's declared until the end of the scope it's declared in. (a scope is often barriered by curcly braces `{}`)

### The String Type

The data types described before (in chapter 3) store data on the stack and were popped off when their scope is over.
To illustrate ownership rules, we'll use a type that's stored on the heap, a `String`.

You can create a `String` from a string literal (the string in double quotes).

```rust
let s = String::from("hello");
```

The double colon `::` indicates the `from` is namespaced under the `String` type. Aka: We're using the `from` that's specific to `String`.

### Memory and Allocation

A string literal is fixed/known at compile time so the text is stored hardcoded in the final executable.
A `String` can be unknown at compile time and change during execution.

> This means:
>
> - The memory must be requested from the memory allocator at runtime.
> - We need a way of returning this memory to the allocator when we’re done with our String.

The requesting part, we do, by calling `String::from`.
The returning of that memory is handled by Rust's ownership system: the allocator frees the memory when the owner goes out of scope.

```rust
{
    let s = String::from("hello"); // s is valid from this point forward

    // do stuff with s
}                                  // this scope is now over, and s is no
                                    // longer valid
```

While these rules will hold up, it's straightforward to understand now, but things get more complicated when we want to have multiple variables access the data we've allocated on the heap.

### Ways Variables and Data Interact: Move

An example with data that is pushed onto the _stack_.

```rust
let x = 5;
let y = x;
```

`x` gets set to `5`.
A copy of the value stored in `x` is made and that gets stored in `y`.
Both variables are accessible and are set to `5` (these two `5`s are pushed onto the stack).

An example with data that is allocated on the _heap_.

```rust
let s1 = String::from("hello");
let s2 = s1;
```

This does not work the same way!

Under the hood, a `String` has 2 parts.
One part is stored on the stack and one part is stored in the heap.

The part on the stack contains 3 pieces of information:

1. A pointer to the location of the data in the heap
2. A length
3. A capacity

> The length is how much memory, in bytes, the contents of the String is currently using.
> The capacity is the total amount of memory, in bytes, that the String has received from the allocator.
> The difference between length and capacity matters, but not in this context, so for now, it’s fine to ignore the capacity.

![](trpl04-01.svg)

When we assign `s1` to `s2`, the data on the stack is copied, while the data on the heap is not.
So: We copy the pointer, the length, and the capacity. Not the data on the heap.

If Rust copied the data on the heap too, that assignment could become very computationally expensive if the data stored on the heap was large.

The image shows two pointers that point to the same location on the heap. This is a problem.
Any one of those variables going out of scope would cause Rust to try and free that location (by calling `drop`).
They will try to free the same memory, this is known as a double free error.

To prevent this and ensure memory safety, Rust considers `s1` to no longer be valid.
Therefore, Rust doesn't need to free anything when `s1` goes out of scope.
Trying to use `s1` after `s2` is created will throw an error.
Rust prevents you from using the invalidated reference.

![](trpl04-04.svg)

That's why the data stored on the stack in `s1` in the image is grayed-out.
When `s2` was assigned, `s1` got invalidated.
This is called _moving_. In this example, we _moved_ `s1` into `s2`.

Only `s2` going out of scope will free the memory.

As a result of this pattern: any automatic data copying done can be assumed to be fast.

### Ways Variables and Data Interact: Clone

If we do want to copy the data stored on the heap, we can use a common method: `clone`.

```rust
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
```

Because `s1` was not _moved_ into `s2`, rather the data on the heap was copied, `s1` stays valid after the declaration of `s2` and can still be used later.

### Stack-Only Data: Copy

```rust
let x = 5;
let y = x;
println!("x = {}, y = {}", x, y);
```

Works without calling `clone` on `x`, because integers are stored on the stack, so that would do the same thing.
`x` is copied and that copied data is stored in `y`.

Rust has a special `Copy` trait that can be placed on variables with data stored on the stack.
Rust won't let us have a type with the `Copy` if the type or any of its parts have the `Drop` trait.

As a general rule, any group of scalar values can be `Copy`, and nothing that needs allocation or is a resource is `Copy`.

example: tuples that only contain types that are `Copy` are also `Copy`.
As soon as the tuple contains one item that is not, the tuple isn't either.

- `(i32, bool, char)` is `Copy`
- `(i32, bool, String)` is not

### Ownership and Functions

> The semantics for passing a value to a function are similar to those for assigning a value to a variable.
> Passing a variable to a function will move or copy, just as assignment does.

```rust
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```

### Return Values and Scope

Returning values can also transfer ownership.

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
```

Unless the data has been moved into a new variable, the data on the heap will be freed when the variable that holds that data goes out of scope.
Functions return a value, that value can be moved into a new variable.
For instance in `let s1 = gives_ownership();`, `s1` takes ownership of the `String` returned by `gives_ownership`.

Functions taking ownership, only to give it back when they return that value again are quite tedious to write, especially if that function return something else too.

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
```

Rust has a feature for this, called _references_.

## 4.2. References and Borrowing

```rust
// this code does not compile
fn main() {
    let s1 = String::from("hello");
    let s2 = hello(s1);
    println!("{}", s1);
}

fn hello(s: String) -> String {
    println!("Inside hello: {}", s);
    s
}
```

Let's follow what happens here.
A `String` is created and stored in the variable `s1`.
The `hello` function is called with `s1` as argument, and the body of the function takes ownership. At this point `s1` is invalidated. We _move_ the `String`.
The `hello` function returns a `String` which is stored in the `s2` variable.

After that we try to access what's in `s1` again by printing it, but `s1` is no longer the owner of the data stored on the heap, `s1` is invalidated.
Rust will show a compilation error.
That error: `value borrowed after move`

A workaround was not trying to print `s1`, but `s2` at the end, since that now has the ownership of the `String`.
This works, but is inconvenient.

Another method is to clone the data as we pass `s1` to `hello`, duplicating the data stored on the heap.
This works, but is possibly very expensive and causes data duplication.

What we want to do, is pass `s1` to that function, without the function taking ownership.
That way, we can use `s1` again later.

We can do that by passing a _reference_ to `s1` into the function.
Syntax: `&s1`

> These ampersands are references, and they allow you to refer to some value without taking ownership of it.

The code below passes a reference of `s1` to `hello` and compiles (without passing ownership back, or cloning the value).

```rust
fn main() {
    let s1 = String::from("hello");
    hello(&s1);
    println!("{}", s1);
}

fn hello(s: &String) {
    println!("Inside hello: {}", s);
}
```

We pass `&s1`, a reference to `s1` into the function.
That means the function signature must now expects a `&String` to be passed as argument,
a reference to a `String`.

Under the hood, it works like this:

![](trpl04-05.svg)

`s` is a reference to `s1`.
`s` has a fixed size and is stored on the stack.
It contains a pointer to `s1`. (More specifically, to the part of `s1` that's stored on the stack.)

The `s1` it points to is the value for `s1` that is stored on the stack.
Remember from the previous chapter, it contains a pointer to the location in the heap, a size, and a capacity.

As a result, when `s` goes out of scope (ergo: the function ends), it is simply popped off the stack.
The allocated memory on the heap is not freed yet, it will only be freed when the owner `s1` goes out of its scope.

https://www.youtube.com/watch?v=wZv62ShoStY

To get to the underlying data while in the function:
Mr C The Slide Man would say: "2 hops this time"

1. `s` points to `s1`.
2. `s1` points to the data on the heap.

> We call having references as function parameters borrowing.

### Mutable References

Remember how variables are immutable by default?
References are too.

In the following example, we mutate the underlying `String`, by appending `, world` to it inside the `hello` function.

If we want a reference to be mutable, we have to explicitly declare it: `&mut s1`.

```rust
fn main() {
    let mut s1 = String::from("hello");
    hello(&mut s1);
    println!("{}", s1);
}

fn hello(s: &mut String) {
    println!("Inside hello: {}", s);
    s.push_str(", world");
}
```

Notice adding `mut` to the reference passed into `hello` was not enough.

- We had to first declare `s1` to be mutable by changing the declaration to `let mut s1`.
- Then we passed a mutable reference to `hello`: `hello(&mut s1)`.
- We also had to change the function type signature to accept a mutable reference to a string: `s: &mut String`.

> But mutable references have one big restriction: you can have only one mutable reference to a particular piece of data in a particular scope.

This restriction prevents data races at compile time.
If we only have one writer at a time, then we can not have a concurrency problem.
Multiple writers at the same time (without synchronization between those 2) can cause data races.

We can only have 1 mutable reference in a scope at a the same time.
OR
We can have unlimited immutable references in a scope at the same time.

```rust
// does not compile
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
}
```

Creating a new scope, and having one of those mutable references there will let the code compile.
Because the scope ends, the mutable reference inside that scope is popped off the stack.
As a result, there is only one mutable reference per scope, this is allowed.

> we can use curly brackets to create a new scope, allowing for multiple mutable references, just not simultaneous ones

```rust
fn main() {
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        println!("{}", r1);
    }
    let r2 = &mut s;
    println!("{}", r2);
}
```

Multiple immutable references at the same time are fine, because no one can change the data.
But we can't add a mutable reference into the mix, it might change the data the immutable references point to, which is unexpected.

> Users of an immutable reference don’t expect the values to suddenly change out from under them!

> Note that a reference’s scope starts from where it is introduced and continues through the last time that reference is used.

That means our broken code from before will work with a small change: we access te value in `r1` before we create `r2`.

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    println!("{}", r1);
    // r1 is no longer used after this point

    let r2 = &mut s;
    println!("{}", r2);
}
```

The scopes don't overlap.
We can add multiple immutable references to our example to illustrate the point further.

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    println!("{}", r1);
    // r1 is no longer used after this point

    let r2 = &s;
    let r3 = &s;
    println!("{} and {}", r2, r3);
    // r2 and r3 are no longer used after this point

    let r4 = &mut s;
    println!("{}", r4);
}
```

The scopes of references end after the last time they are used. (that means they end within the same codeblock!)
In that example there is either 1 mutable reference OR there are multiple immutable references at the same time.

### Dangling References

tl;dr: Rust doesn't allow dangling references.

Dangling references are when a reference points to a place in the heap which has been freed, thus the reference no longer points to the data, the data is gone.
The compiler guarantees that references will never be dangling references.
The owner of the data can not go out of scope before a reference to that data.

To illustrate, this code will not compile:

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
```

When `s` goes out of scope, the data on the heap it points to will be deallocated.
The reference to `s` that's returned from the function and stored in `reference_to_nothing` would point to that deallocated position in the heap.
That's a bug, so Rust doesn't even compile.

### The Rules of References

> - At any given time, you can have either one mutable reference or any number of immutable references.
> - References must always be valid.

## 4.3. The Slice Type

> Another data type that does not have ownership is the slice.
> Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.

Slices are references to a part of another value.

If we created a method to find the first word of a `string` and called it `first_word`.
It could return an integer, that integer would then be the index of the first space in the `String`.

But that's an awkward solution, when the original `String` goes out of scope and the integer doesn't, that integer loses its meaning.
When that original string changes, the integer can lose its meaning too. (for example, setting the string to `""`.)
Sure, the integer still exists, but _what does it mean?_

Worrying about the integer and the string going out of sync is tedious and error prone.
Rust has a solution for this problem: string slices.

### String Slices

> A string slice is a reference to part of a `String`

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

String slices look similar to regular references, with the addition of the `[starting_index..ending_index]` part.

Like with ranges, `starting_index` is inclusive. And `0` is equivalent to not specifying it at all.
`ending_index` is exclusive. And the index of last byte + 1 in the string (`len()`) is equivalent to not specifying it at all.

Under the hood, the slice data structure stores the starting position and the length of the slice, which corresponds to `ending_index` minus `starting_index`.

![](trpl04-06.svg)

In this example, both instances of `slice1` are identical.
Both instances of `slice2` are identical.
Both instances of `slice3` are identical.

```rust
let s = String::from("hello");
let len = s.len();

let slice1 = &s[0..2];
let slice1 = &s[..2];

let slice2 = &s[3..len];
let slice2 = &s[3..];

let slice3 = &s[0..len];
let slice3 = &s[..];
```

With that information in mind, it's more logical for `first_word` to return a string slice instead of an integer.

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

We return a string slice of the start, up to (not including) the first space in the string parameter.
If no space is found, we return a string slice of the entire string parameter.

Now, when `first_word` returns a value, it is tied to the underlying data.

> The value is made up of a reference to the starting point of the slice and the number of elements in the slice.

Remember, the compiler guarantees references will be valid.
So string slices pointing to a part of a `String` will be valid, if you try to use a string slice whe it's not valid, the compiler will prevent that.

```rust
// this does not compile
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // error!

    println!("the first word is: {}", word);
}
```

```error
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
  --> src/main.rs:18:5
   |
16 |     let word = first_word(&s);
   |                           -- immutable borrow occurs here
17 |
18 |     s.clear(); // error!
   |     ^^^^^^^^^ mutable borrow occurs here
19 |
20 |     println!("the first word is: {}", word);
   |                                       ---- immutable borrow later used here

error: aborting due to previous error

For more information about this error, try `rustc --explain E0502`.
error: could not compile `ownership`.

To learn more, run the command again with --verbose.
```

We had a mutable reference and an immutable reference in scope at the same time, that's an error!
(the mutable reference occured when we tried to call `clear`)
The compiler keeps track of this for us an lets us know.

### String Literals Are Slices

That's it. That's the tweet.
It would be funny if the book said that wouldn't it?

The book previously stated that string literals are stored in the binary, we can now understand why.
String literals are of type `&str`.
String literals are slices pointing to a specific point in the binary.

### String Slices as Parameters

We can tweak the `first_word` function from taking a reference to a string as parameter `&String`,
to taking a string slice as parameter `&str`.

This makes it more general without losing functionality.

1. If we have a string slice, we can pass it directly.
2. If we have a `String`, we can pass a slice of the entire string.

Because string literals _are_ string slices, the `&variable_name[..]` syntax is not needed.

```rust
fn main() {
    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}
```

### Other Slices

String slices are not the only type of slices.
For instance, there are also slices of arrays.

> It works the same way as string slices do, by storing a reference to the first element and a length

### Summary

> The concepts of ownership, borrowing, and slices ensure memory safety in Rust programs at compile time.

They make memory management fast, without the bugs that are associated with manual memory management.

## 5. Using Structs to Structure Related Data

Paraphrased this chapter is called "so you want to group related data"

You can create a structure and name it.
Every field inside the structure also has a name and a type the data it holds can be.
You can create new instances of that structure which has to adhere to those types you declared.
There are also methods and associated functiond, making a struct sound kinda like a class from other languages.

## 5.1. Defining and Instantiating Structs

As a result of naming each piece of data, structs are more flexible than tuples because you no longer rely on the position, but on the name.

By convention structs are named with PascalCase.
The fields a struct contain are usually variable, so snake_case for those.

To define a struct, use the `struct` keyword, followed by the name of the struct and open curly braces.
Inside, specify the names of the fields, followed by a colon and the type it will contain when you create an instance of that struct.
Each of these is seperated with a comma, ending commas are fine.

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

To create an instance of a struct, use the struct's name, open curly braces and fill in the fields with values that have the correct types.
Fields don't have to be in order as they're named.

```rust
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```

To access a field on an instance of a struct use `.` notation, followed by the name of the field.
The email address would be `user1.email`.

If the instance is mutable, all data in fields are too.

> Rust doesn’t allow us to mark only certain fields as mutable.

### Using the Field Init Shorthand when Variables and Fields Have the Same Name

Similar to the JavaScript shorthand.
If the name of the field and the name of the variable you are storing in it are identical, the repetition is not needed.

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

### Creating Instances From Other Instances With Struct Update Syntax

When creating an instance of a struct, you can use another instance to "fill in the gaps".

If you wrote it out manually

```rust
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    active: user1.active,
    sign_in_count: user1.sign_in_count,
};
```

With struct update syntax that becomes

```rust
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};
```

The `..user1` will **not override** the fields that were already specified.
It will only fill in the fields that are not specified yet, and leave the ones that were alone.
The `..` update syntax always comes at the end of a struct instantiation, not the middle or top.

### Using Tuple Structs without Named Fields to Create Different Types

AKA tuple structs are named tuples, change my mind.

To define a tuple struct, use `struct` followed by a name, open parentheses and provide a comma seperated list of the types it can hold.

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

`black` and `origin` have different types since they are instances of different structs.

### Unit-Like Structs Without Any Fields

As the title suggest a struct witout fields.
A unitstruct has a name. That's it.
Unit because they behave similarly to `()`, the unit (the "nothing" value in Rust, the thing statements evaluate to).

To define a unit struct, use `struct` followed by a name.

```rust
struct UnitStruct;
```

### Ownership of Struct Data

We previously instantiated structs with the owned `String` type instead of the `&str` type to own everything contained in the struct instance.
If we tried to use `&str` the compiler would complain.

> It’s possible for structs to store references to data owned by something else, but to do so requires the use of lifetimes

Lifetimes are a tool to ensure the data referenced inside a struct stays valid for the lifetime of that struct.

## 5.2. An Example Program Using Structs

Going from unrelated variables holding the height and the width.
To grouping them in a tuple.
To grouping them in a tuple struct.
To grouping them in a regular struct.

The tuple version is better because `height` and `width` are now clearly grouped together.
It is worse because they don't have names anymore, only types.
It's not clear what the values in the tuple represents.
It's not clear what the entire tuple represents.

Using a tuple struct makes that clear, it's a `Rectangle` tuple.
Still, the values it contains are not clear and accessing it via indices (`rect1.0` for the width and `rect1.1` for height) is unclear.

### Refactoring with Structs: Adding More Meaning

```rust
struct Rectangle {
    width: u32,
    height: u32,
}
```

Now the entire struct is named and all values it contains are named, eliminating confusion.
Accessing the height and with is also clearer with `rect1.height` and `rect1.width`.

### Adding Useful Functionality with Derived Traits

We can't print our instance of a struct with `println!("{}", rect1)`

```error
error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
```

The curly brackets use `Display` formatting by default.
Output designed directly for users, for many primmitive types there's only 1 way to display them.
For a struct: does it include new lines? Commas? Leave the braces off? Start with the name?

So structs don't have `Display` implemented.
If we want to print it, we can put `:?` inside the brackets.
That will tell the compiler to use `Debug` for the output instead of the standard `Display`.

This also fails

```error
error[E0277]: `Rectangle` doesn't implement `std::fmt::Debug`
```

To use it, we have to explicitly declare we want to use it, or it won't be there.
To do that add this hashtag thingy in front of the struct definition.
(I hope the book explains what those hashtag things are)

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
}
```

`{:?}` prints without newlines after the fields.
`{:#?}` prints with newlines after the fields.

`Display` and `Debug` are traits.
We told our code to derive a trait.
We'll see what traits are and how to use them later.

## 5.3. Method Syntax

It would make sense to tie our implementation of `area` directly to the `Rectangle` struct,
since the function can only be used on instances of `Rectangle`.

A function tied to a definition of a struct is a method.
They're defined similarly, but methods receive `self` as first parameter.
`self` represents the instance of the struct that is calling the method.

### Defining Methods

To define our `area` function in the contect of our `Rectangle` struct we'll make it into a method.

start with the `impl` keyword and open curly braces.
Inside, define the method. The first parameter is `self`.

We know `self` will be an instance of the struct inside the `impl` block.
We use `&self` instead of `self` because we don't want to take ownership of the instance that calls the method.
If we wanted a mutable reference, it'd be `&mut self`.

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

Syntax for calling a method: `struct_instance.method()`, here `rect1.area()`.

### Where’s the -> Operator?

My reaction: the whatnow?
Oh, it's a C thing.

> Rust has a feature called automatic referencing and dereferencing.
> Calling methods is one of the few places in Rust that has this behavior.

> Here’s how it works: when you call a method with object.something(),
> Rust automatically adds in &, &mut, or \* so object matches the signature of the method.

This explains why we could do `rect1.area()` and didn't have to do `(&rect1).area()`
while the `area` method required a borrowed `self`.

### Methods with More Parameters

We'll add another method to the `Rectangle` struct, one that takes an argument.
That means our method will have 2 parameters since the first is always `self`.

The method `can_hold` will take an immutable borrow of a `Rectangle` and return a boolean.
If the instance the method is called on can hold the instance passed as argument
the method will return `true`, else `false`.

### Associated Functions

We can define functions inside the `impl` block that _don't_ have `self` as parameter.
Those are called _associated functions_.

They are associated with that struct, but not called on instances of that struct.
They're _not_ methods.

> Associated functions are often used for constructors that will return a new instance of the struct.

```rust
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
```

To call an associated function, type the name of the struct, go into that namespace (`::`),
and call the name of the associated function.

Example: `Rectangle::square(50)`

### Multiple impl Blocks

A struct can have multiple `impl` blocks, with different methods and associated functions living in each one.

## 6. Enums and Pattern Matching

Enumerations (_enums_) have possible _variants_.
Rust's enums are not made up of a bunch of variants that are flat values, a variant can hold some data too.

## 6.1. Defining an Enum

Consider an IP address. It's either v4 of v6, those are the possible _enumerations_.
Syntax: the `enum` keyword, followed by the enum name (PascalCase). Open brackets and list the variants (PascalCase) seperated by a comma.

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

### Enum Values

We can create instances of an enum variant by first going into the namespace of the enum (syntax `EnumName::`) and then naming the variant.

```rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

Variants of an enum have the same type. (the enum name).

We can change the enum variants so they hold data inside.

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));
```

The types of data it holds can vary per variant.

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```

> you can put any kind of data inside an enum variant: strings, numeric types, or structs, for example.
> You can even include another enum!

example:

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

> - Quit has no data associated with it at all.
> - Move includes an anonymous struct inside it.
> - Write includes a single String.
> - ChangeColor includes three i32 values.

Enums can have methods.
Similar to structs, put those inside an `impl` block.

```rust
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

### The Option Enum and Its Advantages Over Null Values

An enum in the standard library, and is so good it's in the prelude, thus included in every Rust file is `Option`.
That means you don't have to access variant under the namespace `Option::`, but you can refer to them by name directly.

> The `Option` type is used in many places because it encodes the very common scenario in which a value could be something or it could be nothing.

That's why Rust doesn't include the `null` value.
Null leads to so many bugs, especially when you incorrectly assume something is there but it's not, it's `null`.

The same concept of something not being there is with the `None` variant of the `Option` enum.

```rust
enum Option<T> {
    Some(T),
    None,
}
```

An `Option` either holds some data, inside the `Some` variant, or it hold nothing, expressed by the `None` variant.
The `T` is a placeholder for a type, that means an `Option<i32>` can have either a `Some` that holds an `i32`, or a `None`.

```rust
let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;
```

Notice the type annotations, the compiler knows the types for both `some_number` and `some_string` because of the values they hold.
For `absent_number`, we have to tell it, since the value it holds is the `None` variant of a certain `Option` enum.

The compiler will not let us use `Option<T>` as if it's a `T` value.
We have to specifically handle the case where the variant might be `None`, preventing loads of bugs.
As a result, when we are dealing with a value that's not an `Option`, we don't have to check for null like in many other languages.

We have to turn the `Option<T>` into a `T` before we can do operations on that `T`.

The `Option` enum has [many ways to work with it](https://doc.rust-lang.org/std/option/enum.Option.html).

### 6.2. The match Control Flow Operator

The `match` control flow operator allows you to compare a value against some patterns and execute code based on which pattern matches first.

`match` requires the patterns that are listed to match on to cover every possible case for the value you are matching on.
The compiler enforces this exhaustiveness.

While a bunch of values can be `matched`ed on, the most popular value is going to be a variant of an enum.

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

`coin` is one of the possible variants of the `Coin` enum.
The compiler enforces all possible values for `coin` to be handled.

The first pattern that matches the value being `matched` on get to execute its code.
Nothing else, even if there are more matching patterns below that one.
The checking for a matching pattern goes from top to bottom.

syntax: `match` keyword followed by a value, then open curly braces `{}`.
Next are _arms_, each one has a pattern the value is checked against.
the arrow `=>` points to the code that will be executed if it's the first matching pattern.
If that code is small, it typically doesn't have curly braces and ends in a comma.
If that code has multiple lines, wrap it in curly braces and omit the comma `=> { // code }`.

A `match` is an expression.
It evaluates to the value of the codeblock that was executed for the matching case.

### Patterns that Bind to Values

You can refer to parts of a pattern with a name.
For example if the `Quarter` variant held some information, you could match on a pattern that matched every `Quarter` and named the value it held inside.
If the `match` matched the pattern (eg. `Quarter(state)`), the associated codeblock can then refer to that value you named in the pattern (eg. `state`).

```rust
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
```

> If we were to call `value_in_cents(Coin::Quarter(UsState::Alaska))`, `coin` would be `Coin::Quarter(UsState::Alaska)`.
> When we compare that value with each of the match arms, none of them match until we reach `Coin::Quarter(state)`.
> At that point, the binding for `state` will be the value `UsState::Alaska`.

### Matching with Option<T>

Using `match` with the `Option` enum to either do something if there is a `T`, or nothing.

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

The first call to `plus_one` is with a `Some(5)` value.
It enters the `match`, doesn't match the `None` case so it goes to the next one.
It matched the `Some(i)` case and the `5` is bound to the name `i`.
That name can be used inside the executed block.
The executed block returns a `Some` again, inside is `i+1`, making it return `Some(6)`.
The returned value is the returned value for the `match` and since that's the last expression in the function, also the returned value of the function.
That means `let six = plus_one(five)` is equivalent to `let six = Some(6)`.

The second call to `plus_one` is with a `None` value.
It enters the `match`, matches the `None` case so it executes the associated codeblock.
The executed block returns a `None` value and the function returns `None`.
That means `let none = plus_one(None)` is equivalent to `let none = None`.

### Matches Are Exhaustive

This time I'll quote myself instead of the book

> `match` requires the patterns that are listed to match on to cover every possible case for the value you are matching on.
> The compiler enforces this exhaustiveness.

That means when matching on an enum variant, all possible variants of that enum have to be handled.
If you are matching on something that is an `Option<T>`, you will have to deal with both the `None` and the `Some` variant of that enum.

### The \_ Placeholder

The underscore is the placeholder for "a value here", it can be used to cover every other possible value as a patterns itself, or as part of a pattern.

This `match` has cases for 1,3,5, and 7 where a printline macro is called.
That doesn't cover every possibility, it's not exhaustive!
the `_` pattern matches everything, if a value being `match`ed on didn't already match one of the patterns listed above, it will match this one.

```rust
let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),
}
```

The `()` is the unit value, the thing that means "nothing" in Rust, it's the same value statements have.
(not to be confused with `None`, that is a variant of the `Option` type and absolutely is a value)

## 6.3. Concise Control Flow with if let

`if let` is a less verbose way to handle pattern matching,
if you only care about one pattern being matched, ignoring the rest of the patterns a `match` would have.

Conside this `match` where we only do something if the value being matched on matches the `Some(3)` pattern.

```rust
let some_u8_value = Some(0u8);
match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
}
```

Oldschool Robin would say: holy boilerplates, Batman!

An equivalent to the `match`, this time written as `if let`:

```rust
let some_u8_value = Some(0u8);
if let Some(3) = some_u8_value {
    println!("three");
}
```

It works the same way as a `match`.
It looks if the expression that is given (here: `some_u8_value`) matches the pattern (here: `Some(3)`).
If it matches the pattern, the codeblock (`{}`) is executed.

That's less typing, however, **you lose the exhaustive checking from `match`**.

`if let` can also have an `else` arm.
It is equivalent to the `_` patterns in a `match` that only checks 2 patterns.

Example with `match` and 2 patterns: `Coin::Quarter(state)` and everything else `_`:

```rust
let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    _ => count += 1,
}
```

Equivalent code written as `if let` with an `else`:

```rust
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
```

## 7. Managing Growing Projects with Packages, Crates, and Modules

Breaking a huge single file into multiple smaller files is done via modules.

> A package can contain multiple binary crates and optionally one library crate.

What does that quote mean? No idea of the difference between a binary crate and a library crate.
Also, is a crate like an npm package, or is it different?

You have to mark pieces of code as "public" for other pieces of code to be able to use them,
this hides implementation details.

> - Packages: A Cargo feature that lets you build, test, and share crates
> - Crates: A tree of modules that produces a library or executable
> - Modules and use: Let you control the organization, scope, and privacy of paths
> - Paths: A way of naming an item, such as a struct, function, or module

Note to self: so crates aren't like npm packages after all, Rust has packages too?

## 7.1. Packages and Crates

A crate is a binary or a library.
The "crate root" is the file the compiler starts from.
It is also the root module of that crate.
Comparing to JavaScript, it's the entrypoint.

A package is one or more crates.
A package has a `cargo.toml` file.

> A package must contain zero or one library crates, and no more.
> It can contain as many binary crates as you’d like, but it must contain at least one crate (either library or binary).

When you run `cargo new my-project` you create a new binary package.
It's in a folder called `my-project` that has a `cargo.toml`.
The entrypoint is in `src/main.rs`.
There is no mention of that in the `cargo.toml` file because it's a convention that file will be the crate root of a binary crate with the same name as the package.

If there is a `src/lib.rs`, cargo knows the package contains a library with the same name as the package and that file is treated as the crate root.

If a package contains both files (`src/main.rs` and `src/lib.rs`), cargo knows it's dealing with a package that has 2 crates with the same name as the package.

- One is a library crate (the one with `src/lib.rs` as crate root).
- One is a binary crate (the one with `src/main.rs` as crate root).

A package can have more than one binary crate.
This is done by placing files in the `src/bin/` folder.
Each file will be the crate root of a seperate binary crate.

By default, everything in a crate is namespaced under that crate's name in your code.
For example, if you use the `rand` crate, the trait it has named `Rng` will be available via `rand::Rng`.
That way it doesn't clash with the name `Rng` if that's a name that already exists in your own code.

## 7.2. Defining Modules to Control Scope and Privacy

Modules let you organize code within a crate.
Modules control privacy (if external code can access pieces of code from that module).

create a new library crate with `cargo new --lib restaurant`.
The package is named `restaurant` and has a library crate of the same name, with it's root at `src/lib.rs`.

add to that file:

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

fn main() {}
```

That defined a module called `front_of_house`.

Syntax for defining modules: The `mod` keyword, followed by the name of the module and curly braces for it's body.

The `front_of_house` module contains 2 other modules, `hosting`, and `serving`.
Each with some functions.

The crate roots (`src/main.rs` for binary crates, and `src/lib.rs` for library crates) are a module.
That module is named `crate`.

The module tree for the `restaurant` crate:

```
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

## 7.3. Paths for Referring to an Item in the Module Tree

If we want to use something in a module tree, we have to refer to it by it's _path_.
2 options to write down a path: absolute or relative.

- _absolute_ paths start from the crate root.
- _relative_ paths start from the current module.

Both options use the double colon `::` to separate parts of the path (like the slash in filepaths).

A small change to `src/lib.rs` of our `restaurant` package.

Add a function called `eat_at_restaurant` to it.
It'll be part of the public API of this library, so mark it as public by prefixing it with the `pub` keyword.

```rust
// this will not compile
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
    //
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

Inside `eat_at_restaurant`, refer to the `add_to_waitlist` function twice, once with an absolute path and once with a relative path.
The absolute path starts from the crate root and uses the keyword `crate` to indicate it does.
The relative path starts from our current module, since we're in the crate root, the rest of the path is identical for both paths.

> Starting with a name means that the path is relative.

The reason this didn't compile is because we tried to access a module that isn't available to us.
The error message reads: `` module `hosting` is private ``

(`hosting` is a child and isn't accessible. Why not `front_of_house`? That's on the same level as the module we tried to access from and is a sibling, thus, accessible.)
Modules can see private parts in their ancestor modules, but not their children modules. (levels above, or below in that module tree)

> Modules aren’t useful only for organizing your code. They also define Rust’s privacy boundary:
> the line that encapsulates the implementation details external code isn’t allowed to know about, call, or rely on.
> So, if you want to make an item like a function or struct private, you put it in a module.

### Exposing Paths with the pub Keyword

We tried to access a child module. It's private by default, but we can make it public (accessible to ancestor modules) by marking it with the `pub` keyword.

`pub hosting { // ... }`

Now ancestor modules of `hosting` can see it.
We called a function within that module, that's private by default too.
The contents of a module are private by default.
Marking the module as public doesn't mark the contents as public.

> The `pub` keyword on a module only lets code in its ancestor modules refer to it.

The error changed to `` function `add_to_waitlist` is private ``

Mark the function as public by adding the `pub` keyword in front of the function declaration.
`pub fn eat_at_restaurant() { // ... }`

Now the code will compile.
The place that calls the `eat_at_restaurant` function is allowed to access every part that's used in the relative, and in the absolute path that was used to call the function.

### Starting Relative Paths with super

Like `crate` is a keyword used in absolute paths to refer to the crate root.
`super` is a keyword used in relative paths to refer to the parent module.

`super` is used at the beginning of a path.
It goes up one level (similar to a filepath that starts with `..`).

Adding to our example:

```rust
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}
```

We refer to the `serve_order` function, by going up one level in the module tree (up to the crate root) with `super`.

### Making Structs and Enums Public

When we mark a struct as public, the struct itself is, but the fields will still be private.
We can make each field public on a case by case basis.

Adding to our example:

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}
```

The `toast` field of the `Breakfast` struct is public.
We can read it, and write to it in our instance of `Breakfast` in the `eat_at_restaurant` function.

The `seasonal_fruit` field is private and we're not allowed to access it.

This means we can't create an instance of `Breakfast`!
Because a field is private, the struct must provide a publically accessible associated function to create an instance: `pub fn summer`.

In contrast, a public enum means all the variants of that enum are also public.

```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```

## 7.4. Bringing Paths Into Scope with the use Keyword

Always writing out an entire path to use something in it is [whack](https://www.youtube.com/watch?v=ApjG_5o9BFw).
We can bring a path into the scope of the current module with the `use` keyword.

By adding `use crate::front_of_house::hosting;` to `src/lib.rs`, we bring the `hosting` module into scope and can call it as if it's local.
Making the call to `add_to_waitlist` now `hosting::add_to_waitlist()`.

Paths after the `use` keyword can also be absolute paths. eg. `use crate::front_of_house::hosting`

`self` is a keyword you can use in paths to refer to the current module. (make it very clear you are using a relative path)

### Creating Idiomatic use Paths

The idomatic way to bring something (like functions) into scope with `use` is to bring the parent of whatever you want to use into scope.
That way, you still have to namespace usages of that thing under the parent eg. `parent::thing`.

On the other hand, for structs, enums, and other items. It's ideomatic to bring the full path in.
Note to self: define what "other items" are here. Seems kinda "eh, we did it this way, no real reason why"

> There’s no strong reason behind this idiom: it’s just the convention that has emerged, and folks have gotten used to reading and writing Rust code this way.

### Providing New Names with the as Keyword

You can rename things you bring in with `use` by using the `as` keyword.

```rust
use std::fmt::Result;
use std::io::Result as IoResult;
```

Will bring in 2 `Result` types.
2 identical names aren't allowed, so one is renamed to `IoResult`.

### Re-exporting Names with pub use

To let other code refer to something you brought into scope with `use` as if it were inside that code's scope, mark the `use` as public with `pub`.

> This technique is called re-exporting because we’re bringing an item into scope but also making that item available for others to bring into their scope.

example: `pub use crate::front_of_house::hosting;`

Reexporting lets your internal structure of code be different from the structure you have to use in (absolute and relative) paths.

### Using External Packages

Use external packages in your code by adding them to `[dependencies]` in `cargo.toml`.

```toml
[dependencies]
rand = "0.5.5"
```

> Adding rand as a dependency in Cargo.toml tells Cargo to download the rand package and any dependencies from crates.io and make rand available to our project.

Then, to bring parts of `rand` into scope, we add a `use` statement starting with the name of the crate, `rand`.

A special case is the standard library, `std`. It's already shipped with Rust and doesn't need to be listed inside `[dependencies]`.
Things in it _do_ need to be brought into scope before you can use them.

### Using Nested Paths to Clean Up Large use Lists

Using multiple items from the same crate can take up a lot of lines.

```rust
// --snip--
use std::cmp::Ordering;
use std::io;
// --snip--
```

By using nested paths, we don't have to duplicate identical parts of each path.

We do this by specifying the common part of the path, followed by two colons, and curly braces hold the parts that differ.
Those parts are separated from eachother with a comma.

```rust
// --snip--
use std::{cmp::Ordering, io};
// --snip--
```

What if the common part of the path is the entire path?

```rust
use std::io;
use std::io::Write;
```

The `self` keyword can be used in a nested path in that case.

```rust
use std::io::{self, Write};
```

### The Glob Operator

> If we want to bring all public items defined in a path into scope, we can specify that path followed by `*,` the glob operator:

```rust
use std::collections::*;
```

This brings all public items in `std::collections` into scope.
Be careful, since you don't name them there, it might be strange/frustrating to figure out where things are coming from when you use them!

The glob operator is often used while writing tests.

## 7.5. Separating Modules into Different Files

We can separate modules into different files.

By declaring a module in a file with `mod` and not opening curly braces `{}`, but ending with a semicolon `;` instead.
We tell Rust to load the contents of the module from another file with the same name.

in `src/lib.rs` we move the `front_of_house` module to its own file in `src/front_of_house.rs` and refer to that module.

```rust
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

To continue, we can extract the `hosting` module to its own file too.
To do so, declare the module inside `src/front_of_house.rs` via `pub mod hosting;` (instead of the version with curly braces).
And create an identically names file inside the `src/front_of_house` folder.
ie. `src/front_of_house/hosting.rs`.

To the compiler, nothing changed.
To use, we declare a module, and instead of opening a body `{}`, we put a semicolon `;` and place what would have been inside that body in another file.
Those files have identical names to the name of the module.
If those modules live inside another module, they live inside a folder with an identical name to that module.

## 8. Common Collections

The standard library provides types that are built upon other types.
This chapter talk about collections of values that are stored on the heap, these collections can change size at runtime.

- a vector is like an array with a variable length
- a string is a vector under the hood (well, `String` is.)
- a hash map allows association of keys with values, it's an implementation of the general data structure called _map_

## 8.1. Storing Lists of Values with Vectors

Type: `Vec<T>`
Store more than one variable of a single type.
Can change in length.

To create an empty new one:

```rust
let v: Vec<i32> = Vec::new();
```

Annotate the type for the compiler to know what type the values in the vector will be.

There is a macro that will create a new vector.
If you give it an array, the compiler will know the type of the vector

```rust
let v = vec![1, 2, 3];
```

### Updating a Vector

Adding elements via the `push` method

```rust
let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
v.push(8);
```

`mut` because pushing changes the variable, it mutates it.
No type annotation during creation, the compiler infers the type by the value that is first pushed into the vector.

Similarly: removing elements via the `pop` method.

### Dropping a Vector Drops Its Elements

Like a struct, a vector is freed when it goes out of scope.
That means the elements it contains are freed when the vector is.
This gets tricky when you start referring to elements inside that vector.

### Reading Elements of Vectors

Either with an index directly, or with the `get` method.
Indexes start at 0, because _computers_.
`get` takes an index and returns an `Option`

```rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {}", third);

match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}
```

Note the `&` when reading an element via index directly.
We get a reference to that element.
The `Some` that `.get` can return also returns a reference, the owner is the vector itself.

When trying to get an index that is out of bounds: the direct method with `&[5]` will panic.
The `get(5)` will return a `None`.

Remember the borrowing rule of: many immutable references OR one mutable reference.

This won't compile, as it tries to access the immutable reference `first`, after a mutable borrow when using `push`.

```rust
// does not compile
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0]; // immutable borrow

v.push(6); // mutable borrow

println!("The first element is: {}", first); // error: tried to use immutable borrow
```

> might look like it should work: why should a reference to the first element care about what changes at the end of the vector?
> This error is due to the way vectors work:
> adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space,
> if there isn’t enough room to put all the elements next to each other where the vector currently is.
> In that case, the reference to the first element would be pointing to deallocated memory.
> The borrowing rules prevent programs from ending up in that situation.

### Iterating over the Values in a Vector

Iterating over a vector with a `for` loop will give you an immutable reference to each element.

```rust
let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}
```

If we want to mutate: first mark the vector with `mut`.
Inside the `for` loop, dereference the current element with `*`

```rust
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}
```

At this point, I don't know what dereferencing means and what that star does, but the book says it'll explain later, so I'm going with it.
My current working understanding is: the vector is mutable but we mutate an element, so we follow the reference to the location of that element in the heap with that star somehow.

### Using an Enum to Store Multiple Types

A vector can only store elements of one type.
All variants of an enum have the same type and can hold other values of differing types, _taps-temple.jpg_

> When you’re writing a program, if you don’t know the exhaustive set of types the program will get at runtime to store in a vector,
> the enum technique won’t work.
> Instead, you can use a trait object, which we’ll cover in Chapter 17.

## 8.2. Storing UTF-8 Encoded Text with Strings

### What Is a String?

The one string in the core language is `str`, usually seen as borrowed `&str`.

A `String` is a type provided by the standard library, not the core language.

> is a growable, mutable, owned, UTF-8 encoded string type

Both `String` and `str` are UTF-8 encoded.

What does that mean? 🤷‍♂️, UTF-8 is a standard for encoding text information and they both adhere to that standard.
Apparently characters in UTF-8 can take up different amounts of bytes (1 to 4).

### Creating a New String

Creating an empty `String`:

```rust
let mut s = String::new();
```

Creating a `String`, starting with a string literal:

```rust
let data = "initial contents";

let s = data.to_string();

// the method also works on a literal directly:
let s = "initial contents".to_string();
```

`to_string` is available on every type that implements the `Display` trait.
Makes sense, if it can be printed to the console, if can be converted to a string (in fact, it has to do that first!).

An other way to create a `String` from a string literal is with the `from` method on the `String` type.

```rust
let s = String::from("initial contents");
```

Because strings are UTF-8 encoded, they can hold a bunch of language text with different characters/alphabets (and emojis).

### Updating a String

You can concatenate strings with the `+`, of with the `format!` macro.

#### Appending to a String with push_str and push

`push_str` adds a string slice to the end of a `String`, so it doesn't take ownership of the string you pass in.

```rust
let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2);
println!("s1 is {}", s1); // the String "foobar"
println!("s2 is {}", s2); // the &str "bar"
```

`push` takes a `char` type and appends it to the `String`

```rust
let mut s = String::from("lo");
s.push('l'); // notice the single brackets, indicating a char
```

#### Concatenation with the + Operator or the format! Macro

The `+` is a string concatenation operator.

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
```

Under the hood, a function call happens.
That's why if you don't pass a reference, you will give ownership to variable you are assigning to.

> we can only add a `&str` to a `String`; we can’t add two `String` values together.

The type in the example is `&String` and not `&str` so why does it compile?
The compiler can coerce `&String` to `&str`.
Something called a deref coercion does this.
In that example it would turn `&s2`, which is a `&String`, into `&s2[..]` (slice of the entire thing), which is a `&str`.

For long/many concatenations, using `+` becomes unwieldy.

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = s1 + "-" + &s2 + "-" + &s3;
```

The `format!` macro makes that more readable.
It returns the `String` it created.
It doesn't take ownership of any of its parameters.

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{}-{}-{}", s1, s2, s3);
```

### Indexing into Strings

In Rust, you can't index into a string and access a part of it that way.

The reason? The internal respresentation.

#### Internal Representation

A `String` is a wrapper over a `Vec<u8>`.
For many characters that take 1 byte of space, indexing into a string would be a solution, but not every UTF-8 character takes one byte, it can take up to 4.

A `String` made from a string literal in another language, using another alphabet. Each letter takes up 2 bytes.

> Note that this string begins with the capital Cyrillic letter Ze, not the Arabic number 3.

```rust
let hello = String::from("Здравствуйте");
```

> Asked how long the string is, you might say 12.
> However, Rust’s answer is 24: that’s the number of bytes it takes to encode “Здравствуйте” in UTF-8,
> because each Unicode scalar value in that string takes 2 bytes of storage.
> Therefore, an index into the string’s bytes will not always correlate to a valid Unicode scalar value.

#### Bytes and Scalar Values and Grapheme Clusters! Oh My!

Example: “नमस्ते” written in the Devanagari script.
Three relevant ways to look at that data from Rust's perspective.

It is stored as a `Vec<u8>`.
As bytes:

```rust
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
224, 165, 135]
```

As unicode scalar values, what the Rust `char` type is:

```rust
['न', 'म', 'स', '्', 'त', 'े']
```

The fourth and sixth are not letters, they are diacritics (accents, like accent circomflexe).

As grapheme clusters, what a person would call letters:

```rust
["न", "म", "स्", "ते"]
```

### Slicing Strings

Indexing into a string with a single index is weird because a single character can take up to 4 bytes.
If you index using a single number, you can index in the middle of a character.

> Indexing into a string is often a bad idea because it’s not clear what the return type of the string-indexing operation should be:
> a byte value, a character, a grapheme cluster, or a string slice.

As a result, Rust will not let you index into a string with a single index, but with a range.
That range has to be valid, if it's not the program will panic at runtime.

```rust
let hello = "Здравствуйте";
let answer = &hello[0..6];
println!("{}", answer); // Здр
```

Each entity humans think of as a character is two bytes for that string, so the range from index 0 to index 6 contains those first 3 character.
Note it doesn't return the byte values (`u8` numbers), but the UTF-8 encoded values.

### Methods for Iterating Over Strings

Luckily more methods exist to get at the things inside a string.

by using a `for` loop, we can loop over every `char` if we call the `.chars()` method

```rust
for c in "नमस्ते".chars() {
    println!("{}", c);
}
```

Remember from before, that string has 6 chars (unicode scalar values), 2 of them are diacritics.

Similarly, looping over every `byte` by calling the `.bytes()` method

```rust
for b in "नमस्ते".bytes() {
    println!("{}", b);
}
```

This code will print the 18 bytes that make up the internal `Vec<u8>`:

```
224
164
// --snip--
165
135
```

For getting grapheme clusters: use a crate, that functionality is not in the standard library.

## 8.3. Storing Keys with Associated Values in Hash Maps

`HashMap<K,V>` stores a mapping of keys of type `K` to values of type `V`.

A hashing function determines how it does that.

### Creating a New Hash Map

Create an empty one with `::new()`, and insert things into it with `insert()`.
Notice this codesnippet has no type annotations for `scores`, as the compiler infers them from the first usage of `.insert()`:

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

Like vectors, hashmap data is stored on the heap, it can grow/shrink.

Another method to create them is by using iterators (tuple, vector,...) combined with the `collect` method to group some data into a hashmap.
`collect` gathers data into a collection, that collection can be a hashmap, but also a vector or an other type.

This piece of code combines 2 vectors (turned into iterators) into a single iterator of pairs (tuples of the form `(team, score)`).
That single iterator is a vector of tuples.
It then collects that iterator into a `HashMap` with keys of type `String` (for team) and values of type `i32` (for score).

```rust
use std::collections::HashMap;

let teams = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];

let mut scores: HashMap<_, _> =
    teams.into_iter().zip(initial_scores.into_iter()).collect();
```

### Hash Maps and Ownership

Similar to the ownership rules of a struct:
Types that implement the `Copy` trait are copied into the hashmap.
Owned values like `String` are moved and the hashmap will be the owner of those values.

### Accessing Values in a Hash Map

Access a value in a hashmap by providing the corresponding key to its `get` method.

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score = scores.get(&team_name);
```

Like with vectors, `get` returns an `Option<&V>`.
It might hold a reference to the value for that key in a `Some`, it might return `None`.

You can iterate over every entry in a hashmap with a `for` loop

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

for (key, value) in &scores {
    println!("{}: {}", key, value);
}
```

The order you get the key/value pairs in is arbitrary!
If you need guaranteed ordering, use a vector.

### Updating a Hash Map

Each key can only have one value.
_but that value can be a vector, taps-temple.jpg_

#### Overwriting a Value

Replacing an existing value for a key with a new value.

With `.insert`:
Only the last value for the same key will be used, that means when the second `insert` in this snippet fires,
the existing value for the key is discarded and the new one replaces it.

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);

println!("{:?}", scores); // {"Blue": 25}
```

#### Only Inserting a Value If the Key Has No Value

Checking if a value exists and only inserting if there is no value yet.

With `.entry`:
`.entry` checks if a value is already present for a given key and returns an `Entry` enum.
`.or_insert` checks that enum and will insert into the hashmap at that key if there is no value already there.
The value will only be inserted for that key if no value was already associated with that key.
That means that when the

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

println!("{:?}", scores); // {"Yellow": 50, "Blue": 10}
```

### Updating a Value Based on the Old Value

Updating a value based on the value that's already there.

Like counting the amount of occurences of a word in a string:

```rust
use std::collections::HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{:?}", map); // {"world": 2, "hello": 1, "wonderful": 1}
```

The first time it comes across a word, it inserts `0` as a value for that key.
If there is a value already, it doesn't do this step.
Every iteration adds `1` to the current value.
(so for a first occurrance of a word it sets it to `0`, then adds `1`)
The `.or_insert` returns a mutable reference to the value for that key: `&mut V`.
So in order to assign to `count`, we first dereference it with the `*`.

### Hashing Functions

The default hashing function `HashMap` uses is [_pretty good_](https://www.131002.net/siphash/siphash.pdf) (I have no idea, I'm taking the word of smart people.)

There are tradeoffs made, so if you ever want to change it, you can.
You can change the _hasher_ by implementing the `BuildHasher` trait.

## 9. Error Handling

Errors, they happen.
In many cases Rust will force you to deal with them before your code will compile.
2 kinds:
1. recoverable
2. unrecoverable

Rust doesn't have exceptions.
It has the `Result<T, E>` enum, which has an `Err(E)` variant for recoverable errors.
For unrecoverable errors there is `panic!`, ~~a peculiar call to action~~ which will stop execution.

## 9.1. Unrecoverable Errors with panic!

When the `panic!` macro exucutes, the program will stop and print a failure.
It will unwind and clean up the stack before quitting.

### Unwinding the Stack or Aborting in Response to a Panic

By default, when a panic occurs, the program starts unwinding the stack.
Rust pops things off the stack and cleans them up, that takes time.
The alternative is to immediately abort upon a panic, and leave the cleanup (of memory) to the operating system.

This is done sometimes when the resulting binary needs to be as small as possible.
You can change the behaviour upon panic in `Cargo.toml`.

```toml
[profile.release]
panic = 'abort'
```

---

You can call the `panic!` in your code directly.

```rust
fn main() {
    panic!("crash and burn");
}
```

When the macro gets executed, it will stop the program and produce some output with the message you included among some extra information.

```
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished dev [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/panic`
thread 'main' panicked at 'crash and burn', src/main.rs:2:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
```

That message also shows the location at which the panic happened: the file at `src/main.rs` at line 2, column 5.

### Using a panic! Backtrace

The `panic!` isn't always called in code you wrote, causing the displayed error to point at the location `panic!` was called in someone else's code.
To figure out which line of code you wrote that caused that panic, you can use the backtrace (I guess this is Rust's name for stacktrace?).

The following code tries to access a vector at an index that is out of bounds:

```rust
fn main() {
    let v = vec![1, 2, 3];

    v[99];
}
```

This won't cause Rust to try and read the memory at that location, it doesn't belong to the vector.
That would be a _buffer overread_ and would cause security issues.
Instead, it panics.

```
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/panic`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', /rustc/5e1a799842ba6ed4a57e91f7ab9435947482f7d8/src/libcore/slice/mod.rs:2806:10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
```

The panic happened at a place that isn't in the code we wrote.
It happened at `libcore/slice/mod.rs`. (That's the implementation of `slice`, the code that ran when we tried to into the vector with `[]`)

Running the code with the `RUST_BACKTRACE` environment variable set will cause the backtrace to be included in that error.

A backtrace is a list of all functions that have been called up to that point.
The line in the code you wrote that triggered the panic will be in there.
Everything above that in the backtrace is code your code called.
Everything below that it are functions that called your function.
Those might still be lines of code you wrote, but also include lines of code that call the code you wrote.

> These lines might include core Rust code, standard library code, or crates that you’re using.

```
$ RUST_BACKTRACE=1 cargo run
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', /rustc/5e1a799842ba6ed4a57e91f7ab9435947482f7d8/src/libcore/slice/mod.rs:2806:10
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /Users/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /Users/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:84
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:61
   4: core::fmt::ArgumentV1::show_usize
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1426
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:65
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:50
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:193
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:210
  10: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:471
  11: rust_begin_unwind
             at src/libstd/panicking.rs:375
  12: core::panicking::panic_fmt
             at src/libcore/panicking.rs:84
  13: core::panicking::panic_bounds_check
             at src/libcore/panicking.rs:62
  14: <usize as core::slice::SliceIndex<[T]>>::index
             at /rustc/5e1a799842ba6ed4a57e91f7ab9435947482f7d8/src/libcore/slice/mod.rs:2806
  15: core::slice::<impl core::ops::index::Index<I> for [T]>::index
             at /rustc/5e1a799842ba6ed4a57e91f7ab9435947482f7d8/src/libcore/slice/mod.rs:2657
  16: <alloc::vec::Vec<T> as core::ops::index::Index<I>>::index
             at /rustc/5e1a799842ba6ed4a57e91f7ab9435947482f7d8/src/liballoc/vec.rs:1871
  17: panic::main
             at src/main.rs:4
  18: std::rt::lang_start::{{closure}}
             at /rustc/5e1a799842ba6ed4a57e91f7ab9435947482f7d8/src/libstd/rt.rs:67
  19: std::rt::lang_start_internal::{{closure}}
             at src/libstd/rt.rs:52
  20: std::panicking::try::do_call
             at src/libstd/panicking.rs:292
  21: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:78
  22: std::panicking::try
             at src/libstd/panicking.rs:270
  23: std::panic::catch_unwind
             at src/libstd/panic.rs:394
  24: std::rt::lang_start_internal
             at src/libstd/rt.rs:51
  25: std::rt::lang_start
             at /rustc/5e1a799842ba6ed4a57e91f7ab9435947482f7d8/src/libstd/rt.rs:67
  26: panic::main
```

So, the way to read it is from top to bottom (most recent to longest ago) until you arrive at code you wrote.
Doing that, this trace points us to `src/main.rs:4`.

> When your code panics in the future, you’ll need to figure out what action the code is taking with what values to cause the panic and what the code should do instead

To get that output, debug symbols have to be enabled.
Those are pieces of information that is included in the binary you run to help figure out where the instructions to the CPU the binary contains came from.
That information is ignored by the CPU, but used by debugging tools.

By default, debug symbols are only enabled in development builds and omitted in release builds.

## 9.2. Recoverable Errors with Result

Most errors don't require the program to stop entirely.

> For example, if you try to open a file and that operation fails because the file doesn’t exist, you might want to create the file instead of terminating the process.

Those are good cases for a function to return a `Result<T, E>` type. (an enum with 2 variants)
- The generic `T` represents the type the `Ok` variant will hold.
- The generic `E` represents the type the `Err` variant will hold.

Code that calls it can then determine how to handle a possible success or error case based on the returned variant:
- Handle the error if there was one. (handling the `Err(E)` variant)
- If everything went well, an `Ok(T)` will be returned.

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
}
```

`File::open` returns a `Result<std::fs::File, std::io::Error>`.
To confirm, visit the documentation, or induce an error by wrongly annotating its type, the compiler will tell you what it found instead.

That means the function was successful, it will return an `Ok()` and the type of the thing inside will be `std::fs::File`.
If that function fails (eg. the file doesn't exist), it will return an `Err()` and the type of the thing inside will be `std::io::Error`.

A `match` can be used to execute code based on the variant of the enum that was returned.

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}
```

If the returned value is an `Ok`, return the file handler that's inside the `Ok` variant.
If the returned value is an `Err`, panic. Show the error contained in the `Err` variant in the panic message.

### Matching on Different Errors

That snippet would panic regardless of the type of error that was returned.

The code below distinguishes between the returned errors.
If the error was because the file was not found, it tries to create the file, and returns the file handler to the newly created file.
If it was any other error, it panics.

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}
```

It does this by running a `match` on the error inside the `Err` variant.
From the type (`std::io::Error)`, we know it has a `.kind()` method that will return an `io::ErrorKind` value.

> the enum io::ErrorKind is provided by the standard library and has variants representing the different kinds of errors that might result from an io operation. 

If the `ErrorKind` was an `ErrorKind::NotFound`, indicating the file we're trying to open doesn't exist yet, the code tries to create it.
Trying to create it with `File::create` also returns a `Result`, so the code has another `match` to handle the possible `Err` and `Ok` from that function.

> That’s a lot of `match`! The `match` expression is very useful but also very much a primitive.

`Result` types have lots of useful methods that can shorten the code, like the `unwrap_or_else` method.

### Shortcuts for Panic on Error: unwrap and expect

`unwrap` is another method available on a `Result`.
It will either return the value in the `Ok` or panic if the value was an `Err`.

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
}
```

if it panics:
```
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error {
repr: Os { code: 2, message: "No such file or directory" } }',
src/libcore/result.rs:906:4
```

The `expect` method is very similar, but will also let you choose the message shown in case of a panic.

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
```

if it panics:
```
thread 'main' panicked at 'Failed to open hello.txt: Error { repr: Os { code:
2, message: "No such file or directory" } }', src/libcore/result.rs:906:4
```

### Propagating Errors

If a function whose implementation calls something that might fail, you can return that error to the calling code so it can handle it.

This code reads a username from a file, it returns a `Result`.
If the `File::open` call fails, the function returns that error immediately.
If it succeeds, it will call `read_to_string` and return the string it read to in an `Ok`, or the error that produced in an `Err`.

```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

In other words, it fails early.
If an operation that can fail returns an `Err`, the whole function returns that `Err`.
Else, it continues on until the end of the function body, making sure the eventual return value from the function is a `Result` variant that matches the function signature.
The last `match` statement doesn't need a `return` keyword, because it's the ending expression of the function.

Based on the function signature, the `Err` holds an `io::Error` and the `Ok` holds a `String`.
That means the possible error returned from `File::open` and `read_to_string` are both instances of `io::Error`.

Because we returned a `Result`, the calling code has more flexibility.
It will either receive an `Ok` with a `String` in it, or an error.
It can choose what to do with that error.
It could `panic!`, but it could also use a default username, or use an other method to get a username.

This can be written much shorter.
This pattern of propagating errors is so common, Rust has a special piece of syntax for it, the questionmark `?`.

#### A Shortcut for Propagating Errors: the ? Operator

The same functionality, written with the `?` operator:

```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

When placing a `?` after a `Result` value:
If the value is an `Ok`, the value inside the `Ok` will be returned and the function continues execution.
If the value is an `Err`, the function will stop execution and return the entire `Err`.

There is a difference between the longer code using `match` and the code using the `?` operator.
Error values that have the `?` operator called on them go through the `from` function.

The `from` function is defined in the `From` trait in the standard library.
It converts error messages from one type into another.
When `?` calls `from`, the received error type is converted into the error type that is defined in the signature of the function the `?` was used in.

> This is useful when a function returns one error type to represent all the ways a function might fail, even if parts might fail for many different reasons
> As long as each error type implements the `from` function to define how to convert itself to the returned error type, the `?` operator takes care of the conversion automatically.

You can use the `?` and chain methods.
The `?` will return from the function early if there is an `Err`, else, the entire thing acts as an `Ok` (and can have methods chained on it).

```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
```

Turns out this example is a common enough usecase to have its own method

```rust
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```

#### The ? Operator Can Be Used in Functions That Return Result

As a result of early returning an `Err`, the `?` can only be used in functions that have a return type of `Result`.

note: `?` can also be used with `Option` types, where it early returns the `None`.
note2: or with types that implement the `std::ops::Try` trait.

By default, the `main` function has a return type of `()`, so the `?` cannot be used there.

Trying to use it will result in the following compiler error:

```
$ cargo run
   Compiling error-handling v0.1.0 (file:///projects/error-handling)
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `std::ops::Try`)
 --> src/main.rs:4:13
  |
3 | / fn main() {
4 | |     let f = File::open("hello.txt")?;
  | |             ^^^^^^^^^^^^^^^^^^^^^^^^ cannot use the `?` operator in a function that returns `()`
5 | | }
  | |_- this function should return `Result` or `Option` to accept `?`
  |
  = help: the trait `std::ops::Try` is not implemented for `()`
  = note: required by `std::ops::Try::from_error`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
error: could not compile `error-handling`.

To learn more, run the command again with --verbose.
```

The `main` function has a few restrictions on what types it can return.
The default is `()`, and an other valid return type is `Result<T,E>`

```rust
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}
```

The `Box<dyn Error>` type is called a trait object, the book will explain what those are later.
For now, treat it as "pick an error, any error"

Because that function returns a `Result`, using the `?` operator is allowed.

## 9.3. To panic! or Not To panic!

A `panic!` is final, a `Result` it not.
If you call `panic!`, you are making a decision on behalf of the code that called your code.
If you return a `Result`, the code that calls your may still decide to call `panic!` on error.
Usually, returning a `Result` from a function that might fail is a good default choice as it gives more control to the caller.
In other situations, it is more appropriate to panic.

### Examples, Prototype Code, and Tests

In example code, `panic!` is usually preferred, since it is clearer.
Similarly `unwrap` and `expect` are handy when prototyping.
They're clear markers that say "this code might panic"

In tests, when a method call fails, it's appropriate to panic (in your code, lol).
You'd want the whole test to fail.
`panic!` is how a test is marked as a failure, making `unwrap` and `expect` ideal parts of testing code.

### Cases in Which You Have More Information Than the Compiler

It's also fine to call `unwrap` in regular code when you don't want to panic sometimes.
Maybe some preceding logic dictates it will always succeed.
You know this, the compiler doesn't.

For example: by looking at the hardcoded string literal, you can guarantee the call to `parse` will succeed:

```rust
use std::net::IpAddr;

let home: IpAddr = "127.0.0.1".parse().unwrap();
```

### Guidelines for Error Handling

The book advises to panic when your code ends up in a "bad state".
That means: when an assumption, guarantee, contract, invariant is broken.
When there are invalid, contradictory, or missing values.
That, combined with one or more of these:
> - The bad state is not something that’s expected to happen occasionally.
> - Your code after this point needs to rely on not being in this bad state.
> - There’s not a good way to encode this information in the types you use.

Basically when stuff happens that shouldn't be possible or is impossible to recover from.

Your code can panic if someone calls it with invalid values.
This lets the user of the library they are passing wrong values and alerts them to a possible bug in their code.

When calling external code, it might make sense to panic if it returns an invalid state that you have no way of fixing.

If the failure is expected, a `Result` makes more sense to indicate a failure.
Examples are a rate limit code on a HTTP request, or a parser returning an error on invalid date.

The Rust type system is a way define a type of contract for a function.
It expects specific types of input and won't even compile when that contract is broken.
If a function operates on an `i32`, giving it an `Option<i32>` is invalid.
Your function expects an `i32`, not a `Some` that holds an `i32`, and also not a `None`.

### Creating Custom Types for Validation

You can turn Rust's type system up to 11 by creating your own custom error types.

The guessing game dealt with a secret number between 1 and 100, yet the guess was never validated.

We could parse the guess as `i32` (and allow potentially negative guesses) and print a specific line of text if the guess wasn't between 1-100.
If it wasn't, call `continue` to restart the `loop` and ask for a new guess.

```rust
loop {
    // --snip--

    let guess: i32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    if guess < 1 || guess > 100 {
        println!("The secret number will be between 1 and 100.");
        continue;
    }

    match guess.cmp(&secret_number) {
        // --snip--
    }
}
```

> However, this is not an ideal solution: if it was absolutely critical that the program only operated on values between 1 and 100,
> and it had many functions with this requirement,
> having a check like this in every function would be tedious (and might impact performance).

We can create a new type and put the validation logic on that type.

```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
```

To create an instance of `Guess`, we have to call `Guess::new()` since the `value` key for that struct is private and we can't set it directly.
`new` is an associated function that will create a new instance of `Guess`.
It will first validate the argument passed to it, if that argument isn't a valid integer between 1 and 100, it will panic.
Trying to create a `Guess` with a `value` outside of that range would violate the contract `Guess::new` relies on.
That panic will alert the programmer who is writing the calling code that they have a bug that needs fixing.

**The conditions in which `Guess::new` panics should be available in documentation.**

Every instance of `Guess` will hold an `i32` in the `value` key that is between 1 and 100.
Code that uses it can operate on that assumption.

The `value` method is a _getting_, it's necessary because `value` is private.
It's important it's private so every instance of `Guess` has to be created through `Guess::new` and is guaranteed to hold a `value` between 1 and 100.
It being private also prevents code using the `Guess` struct from manipulating it directly.

A function could then take a `Guess` instead of an `i32`.
At that point, you wouldn't need any additional checks, but can safely assume the `value` to be between 1 and 100.

## 10. Generic Types, Traits, and Lifetimes

What function parameter types are to concrete values, generics are to types.
Kinda.

Generics are a tool to handle the duplication of concepts. They are abstract stand-ins for types or properties.
Generic types are typically presented with a capital letter, that has no real meaning. A capital letter is just short.
The popular first choice is `T`, which stands for type (aren't programmers so smart).

The `T` in `Option<T>` is a generic. It's a stand-in for a type.
It says that whatever type is entered in the place of that `T` will be the type of the concrete value `Some` holds.

```rust
enum Option<T> {
    Some(T)
    None
}
```

Same thing with `Result<T,E>`.
The `T` and `E` are generics, they are stand-ins for concrete types.
Whatever type `T` gets will be the type of the value `Ok` holds.  
Whatever type `E` gets will be the type of the value `Err` holds.

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

A `Result<String, std::io::Error>` has 2 variants, an `Ok` with a value of type `String` inside, or an `Err` with a value of type `std::io::Error` inside.

First, the book will extract duplicated logic to a function.
Then it'll make the function generic to replace 2 functions that differ only in the types of data they deal with.

Traits define behaviour in a generic way.
Traits can be combined with generic types to constrain those generic types to only those that have a specific behaviour.

Lifetimes are a form of generics that give the compiler information about how references relate to each other.
They allow us to borrow values while the compiler ensures the data behind that borrow is still there in memory.

### Removing Duplication by Extracting a Function

Let's start by removing duplication without generics, by moving instructions to a function.

```rust
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}
```

This code repeats the same logic twice.
It finds the largest integer in a vector and stores it in a variable.

A function can abstract this logic, which is then repeated by giving the function different inputs when we call it.

```rust
fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(result, &100);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(result, &6000);
}
```

> In sum, here are the steps we took to change the code from Listing 10-2 to Listing 10-3:
> 
> 1. Identify duplicate code.
> 1. Extract the duplicate code into the body of the function and specify the inputs and return values of that code in the function signature.
> 1. Update the two instances of duplicated code to call the function instead.

Next, we'll repeat those steps.
In the same way the function can operate on an abstract vector by using a type, it can operate on an abstract type by using a generic.

## 10.1. Generic Data Types

Generics can be used in function signatures, structs, enums, ...

### In Function Definitions

Generics in a function signature are where the types would usually be.

Let's replace 2 functions that deal with concrete types with one that deals with a generic type:

```rust
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(result, &100);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
    assert_eq!(result, &'y');
}
```

Before using a generic type, we must first declare we are going to use a generic type. That way, the compiler doesn't confuse a generic type for a concrete one.

Generic types are represented with CamelCase, often abbreviated to one letter. `T` is the most popular one, for "type".

Declaring a generic type is done with angle brackets `<>`, between the function name and the parameter list.

Using a generic type to rewrite the function above:

```rust
// COMPILER ERROR
fn largest<T>(list: &[T]) -> &T {
    let mut largest = list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```

This doesn't compile yet.

```error
$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0369]: binary operation `>` cannot be applied to type `T`
 --> src/main.rs:5:17
  |
5 |         if item > largest {
  |            ---- ^ ------- T
  |            |
  |            T
  |
  = note: `T` might need a bound for `std::cmp::PartialOrd`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0369`.
error: could not compile `chapter10`.

To learn more, run the command again with --verbose.
```

Right now, the generic `T` represents _any_ type.
Inside the function, we compare 2 values of that type with the `<` operator, not all types can do that.

That's where constraining the generic to types that allow comparison comes in handy.
In Rust, types can have certain behaviours when they implement _traits_.

The standard library has a trait that says: types that have this trait (`std::cmp::PartialOrd`) can be ordered. 
That means that comparison will work.
So the solution is: constraining the generic type to only allow types that implement that trait.
More about this error in the very next chapter, the one on traits.

### In Struct Definitions

An example of a struct using a generic type:
The generic type is declared after the name in angle brackets again.
The generic is used in place of where concrete types would be inside the definition.
Like with functions, if more than one generic parameter is used, they are seperated with a comma inside the angle brackets.

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
```

In this case, both `x` and `y` must have the same type.
If they need to be able to have different types, we can explicitly define that possibility.

```rust
struct Point<T, U> {
    x: T,
    y: U,
}
```

### In Enum Definitions

Very similar to structs: 
- generic declaration after the name
- in angle brackets
- multiple generic declarations seperated by a comma
- used in place of concrete types.

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

> When you recognize situations in your code with multiple struct or enum definitions that differ only in the types of the values they hold,
> you can avoid duplication by using generic types instead.

### In Method Definitions

The `impl` block can use generics too.
As with functions, usage of a generic inside an `impl` block needs to be declared first, giving that information to the compiler.
After declaring the block uses a generc, you can use it within the block but also on the `impl` line, as you can use it to implement for a struct that uses a generic itself. _woah_

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```

The code above implemented a struct `Point` that's generic over a type `T`, both the `x` and the `y` fields will have values of that type.

Later, we write an implementation for that generic type.
We have to declare that generic before we can use it, so we tell the compiler with `<T>`.

By declaring the generic type right after the `impl`, Rust can identify it's a generic when it's used again, not a concrete type (like `i32`).
The fact this generic type is also `T` is a coincidence, it could also be `Boop`.

In fact, let's make the implementation use a generic named `Boop`:

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<Boop> Point<Boop> {
    fn x(&self) -> &Boop {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```

The method `x` returns a reference to the data in the field `x` of the `Point` struct.

We could implement `Point<T>` for a specific type:

```rust
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

Only instances of `Point<T>` that use `f32` as values for `T` will then have the `distance_from_origin` method defined.

> we use the concrete type `f32`, meaning we don’t declare any types after `impl`.

Other generics can be used within an `impl` block that uses generics itself, they don't need to all be declared right after `impl`.
Generics that are specific to a method can be defined in their usual position: between the name and the parameter list:

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```

> The method takes another `Point` as a parameter, which might have different types from the `self` `Point` we’re calling `mixup` on.
> The method creates a new `Point` instance with the `x` value from the `self` `Point` (of type `T`) and the `y` value from the passed-in `Point` (of type `W`).

> The `println!` macro call will print `p3.x = 5, p3.y = c`

### Performance of Code Using Generics

Rust implements generics in such a way that your code using generics is just as fast as code with concrete types.

That is because Rust uses Performance of Code Using Generics.
Under the hood Rust literally treats instances using a specific type as different types.
It turns generic code into concrete code based on the usage in your code.
You can use generics while writing Rust, but when compiling, they're replaced with concrete types.

> Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.

That means this code using an enum that uses a generic:

```rust
enum Option<T> {
    Some(T),
    None,
}

let integer = Some(5);
let float = Some(5.0);
```

Will turn into this code that does not use generics, only concrete types.
The types are filled in during compilation.

```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

## 10.2. Traits: Defining Shared Behavior

A trait tells the compiler about the functionality of a particular type.
Different types can implement the same trait and therefor share the same behaviour.
We can use trait bounds to restrict a generic to only be a type that has certain behaviour.

### Defining a Trait

Trait definitions are a way to group method signatures together to define a set of behaviours necessary to accomplish some purpose.
Types that implement the same trait will have that same behaviour/the same methods.

If we have 2 structs: `NewsArticle` and `Tweet` and we want them both to have a method called `summarize`.
We can choose to create a `Summary` trait and have both types implement that trait.

syntax: `trait` keyword followed by a PalcalCase name and a block of code surrounded by `{}`.
Inside the curly brackets are the signatures for the methods of types that implement this trait.

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

They don't need to be full functions with implementations, the signature is enough.
The type implementing that trait must then provide implementations of methods that match that signature.
Each line is terminated with a semicolon `;`.

> The compiler will enforce that any type that has the `Summary` trait will have the method `summarize` defined with this signature exactly.

### Implementing a Trait on a Type

Below are implementations of the `Summary` trait.
`NewsArticle` and `Tweet` both implement it, each providing an implementation for the required `summarize` method.

syntax: similar to a regular implementation for a struct.
The `impl` keyword, followed by the name of the trait, the `for` keyword, followed by the name of the struct.
Then the codeblock with the implementation opens and closes `{}`.

syntax: `impl TraitName for TypeName {}`

```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

> After implementing the trait, we can call the methods on instances of `NewsArticle` and `Tweet` in the same way we call regular methods

If we published this as a crate called `aggregator`, others can bring the trait into scope and implement it on their types.
They'd bring it into scope with: `use agrregator::Summary;`
In order for them to do that, the trait needs to be marked as public with `pub`.

A restriction: we can implement a trait on a type only if at least one of those is local to our crate.
Implementing a third-party trait on a third-party type is not possible.
For example, we can't implement the `Display` trait on a `Vec<T>`, as both the trait and the type are external to our crate.
they are both part of the `std` library in this example, but the same rule applies if they were from seperate external crates.

> This restriction is part of a property of programs called coherence,
> and more specifically the orphan rule, so named because the parent type is not present.
> This rule ensures that other people’s code can’t break your code and vice versa.
> Without the rule, two crates could implement the same trait for the same type, and Rust wouldn’t know which implementation to use.

### Default Implementations

That trait declaration with the signatures in it?
Those method signatures can have implementations too, those are default implementations.

Types implementing this trait will use the default implementation if they don't override it in their own implementation.

> instead of requiring implementations for all methods on every type.
> Then, as we implement the trait on a particular type, we can keep or override each method’s default behavior.

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```

To use this default implementation for the `NewsArticle` type, we specify an empty `impl` block with `impl Summary for NewsArticle {}`.

Because we implemented the trait without an implementation for `summarize`, the default implementation will be used.

```rust
let article = NewsArticle {
    headline: String::from("Penguins win the Stanley Cup Championship!"),
    location: String::from("Pittsburgh, PA, USA"),
    author: String::from("Iceburgh"),
    content: String::from(
        "The Pittsburgh Penguins once again are the best \
            hockey team in the NHL.",
    ),
};

println!("New article available! {}", article.summarize());
// New article available! (Read more...).
```

A specific implementation will be preferred to the default one.
If we had left the implementation for `summarize` on `NewsArticle` there,
the default implementation defined during the trait declaration would not be used.

Default implementations can call other methods in the same trait, even if those other methods don't have a default implementation.
This way, traits can provide lots of functionality while require implementors to only specify a small part of the entire trait.

When adding a `summarize_author` method to the trait and providing a default implementation of `summarize`:
types that implement the trait only have to implement a `summarize_author` method.
They can still call `summarize`, which will in turn call their implementation of `summarize_author`.

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
```

```rust
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
```

```rust
let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from(
        "of course, as you probably already know, people",
    ),
    reply: false,
    retweet: false,
};

println!("1 new tweet: {}", tweet.summarize());
// 1 new tweet: (Read more from @horse_ebooks...)
```

It isn't possible to call the default implementation from an overriding implementation of that same method.
I think it isn't possible to call default implentations if they are overridden at all.

### Traits as Parameters

Traits can be used in generic function parameters.

> we implemented the Summary trait on the NewsArticle and Tweet types.
> We can define a notify function that calls the summarize method on its item parameter,
> which is of some type that implements the Summary trait. To do this, we can use the impl Trait syntax, like this:

```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

syntax: in the place where you would normally specify a type: `impl` followed by the name of the trait.
That parameter then accepts any value of a type that implements that trait.

#### Trait Bound Syntax

The `impl Trait` syntax is syntax sugar for a longer form syntax.

The longer syntax is called a _trait bound_.

Declare a generic, inside the angle brackets,
follow the name of the generic with a colon `:`, followed by the name of the trait that generic type has to implement.

```rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

This syntax makes it shorter to have multiple parameters that have the same generic types.
In the following example, `item1` and `item2` can have different types:

```rust
pub fn notify(item1: &impl Summary, item2: &impl Summary) {}
```

Using the trait bound syntax, we can force both parameters to have the same type:

```rust
pub fn notify<T: Summary>(item1: &T, item2: &T) {}
```

#### Specifying Multiple Trait Bounds with the + Syntax

It also allows for a single generic type to be constrained by multiple traits.

Syntax: a `+` between trait names the generic type must implement.

with the `impl Trait` syntax:

```rust
pub fn notify(item: &(impl Summary + Display)) {}
```

with the trait bound syntax:

```rust
pub fn notify<T: Summary + Display>(item: &T) {}
```

#### Clearer Trait Bounds with where Clauses

This can get long pretty fast.
The `where` syntax places that information at the end of a function signature.

syntax:
- declare the names of generic types used at their regular location, inside angle brackets `<>` 
- at the end of the function signature, between the return type and the function body place the `where` keyword.
type what would be between the angle brackets `<>` in the normal trait bound syntax there.

without the syntax:

```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
```

using the `where` syntax:

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{}
```

#### Returning Types that Implement Traits

The `impl Trait` syntax can be used for return values too.

> The ability to return a type that is only specified by the trait it implements is especially useful in the context of closures and iterators,
> which we cover in Chapter 13.
> Closures and iterators create types that only the compiler knows or types that are very long to specify.
> The `impl Trait` syntax lets you concisely specify that a function returns some type that implements the `Iterator` trait without needing to write out a very long type.

You can only use the `impl Trait` syntax if you're returning a single type.

The following code returns either a `Tweet` or a `NewsArticle`.
Both implement the `Summary` trait.

```rust
// DOES NOT COMPILE
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}
```

> Returning either a `NewsArticle` or a `Tweet` isn’t allowed due to restrictions around how the `impl Trait` syntax is implemented in the compiler
> We’ll cover how to write a function with this behavior in the “Using Trait Objects That Allow for Values of Different Types” section of Chapter 17.

### Fixing the largest Function with Trait Bounds

Remember that `largest` function that wouldn't compile with generics from the previous chapter?

In the body of the function, we used `>` to compare two values.

Whichever values we compare should be able to do that.
With the `std::cmp::PartialOrd` trait, they are!

Change the type signature to include a trait bound:

```rust
fn largest<T: PartialOrd>(list: &[T]) -> T {}
```

This results in a new error

```error
$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0508]: cannot move out of type `[T]`, a non-copy slice
 --> src/main.rs:2:23
  |
2 |     let mut largest = list[0];
  |                       ^^^^^^^
  |                       |
  |                       cannot move out of here
  |                       move occurs because `list[_]` has type `T`, which does not implement the `Copy` trait
  |                       help: consider borrowing here: `&list[0]`

error[E0507]: cannot move out of a shared reference
 --> src/main.rs:4:18
  |
4 |     for &item in list {
  |         -----    ^^^^
  |         ||
  |         |data moved here
  |         |move occurs because `item` has type `T`, which does not implement the `Copy` trait
  |         help: consider removing the `&`: `item`

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0507, E0508.
For more information about an error, try `rustc --explain E0507`.
error: could not compile `chapter10`.

To learn more, run the command again with --verbose.
```

The code tries to _move_ values into the `largest` variable.

With out non-generic functions we only operated on lists of `i32` or `char`.
Both types are able to be stored on the stack and implement the `Copy` trait.
As a result, that line doesn't try to move the values into `largest`, it copies them.

To enforce this we can add another trait bound and require the generic type `T` to implement the `Copy` trait.

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```

disclaimer: I don't know if this is correct, but it compiles, and the output makes sense:
If we didn't want to do that, we could slightly change the function to return a reference to the item instead of the item

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = &item;
        }
    }

    largest
}
```

> If we don’t want to restrict the `largest` function to the types that implement the `Copy` trait,
> we could specify that T has the trait bound `Clone` instead of `Copy`.
> Then we could clone each value in the slice when we want the `largest` function to have ownership.
> Using the clone function means we’re potentially making more heap allocations in the case of types that own heap data like String,
> and heap allocations can be slow if we’re working with large amounts of data.

> Another way we could implement `largest` is for the function to return a reference to a `T` value in the slice.
> If we change the return type to `&T` instead of `T`, thereby changing the body of the function to return a reference,
> we wouldn’t need the `Clone` or `Copy` trait bounds and we could avoid heap allocations.


### Using Trait Bounds to Conditionally Implement Methods

We can implement methods conditionally for types that implement specified traits.

In the example below, the type `Pair<T>` always implements a `new` function.
Instances of `Pair<T>` with an inner type `T` that implements both the `Display` and `ParialOrd` trait also implements the `cmp_display` method.

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

We can also conditionally implement a trait for any type that implements another trait.

> Implementations of a trait on any type that satisfies the trait bounds are called blanket implementations and are extensively used in the Rust standard library.

> For example, the standard library implements the `ToString` trait on any type that implements the `Display` trait.
> The `impl` block in the standard library looks similar to this code:

```rust
impl<T: Display> ToString for T {
    // --snip--
}
```

> Because the standard library has this blanket implementation,
> we can call the `to_string` method defined by the `ToString` trait on any type that implements the `Display` trait.

### 10.3. Validating References with Lifetimes

Another kind of generic are lifetimes.
Lifetimes ensure that references are valid as long as we need them to be.
In other words: they're a hint the compiler uses to make sure the backing memory behind a reference is there,
the compiler ensures references are always valid.

Every reference in Rust has a lifetime, which is the scope for which that reference is valid.
Most of the time, lifetimes are implicit and inferred.
Just like we must annotate types when the compiler can't infer them,
we must annotate lifetimes when the lifetime of references could be related in a few different ways.

They are a tool to remove ambiguity and enable the compiler to make guarantees about references being valid.

Rust requires us to annotate the relationships sometimes by using generic lifetime parameters.

### Preventing Dangling References with Lifetimes

The main aim of lifetimes is to prevent dangling references.

This piece of code tries to print a value where the backing memory is already released, it won't compile.

```rust
// DOES NOT COMPILE
{
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}
```

The outer scope declares the variable `r` without an initial value.
The inner scope declares the variable `x` and sets the value of `r` to be a reference to `x`.
The inner scope ends, and we attempt to print the value in `r`.

This won't compile because the value `r` is referring to has gone out of scope and no longer exists.

```error
$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0597]: `x` does not live long enough
  --> src/main.rs:7:17
   |
7  |             r = &x;
   |                 ^^ borrowed value does not live long enough
8  |         }
   |         - `x` dropped here while still borrowed
9  | 
10 |         println!("r: {}", r);
   |                           - borrow later used here

error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.
error: could not compile `chapter10`.

To learn more, run the command again with --verbose.
```

### The Borrow Checker

The compiler has a _borrow checker_ that does what it advertises.
It checks borrows (surprising, I know).
It determines if those borrows (references) are valid.

This piece of code has the lifetimes of variables annotated
as `'a` for the variable `r`,
and `'b` for the variable `x`:

```rust
{
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+
```

We try to access something with lifetime `'b` during lifetime `'a`.

> The program is rejected because 'b is shorter than 'a: the subject of the reference doesn’t live as long as the reference.

This piece of code does not have a dangling reference, and compiles:

```rust
{
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}                         // ----------+
```

We try to access something with lifetime `'b` during lifetime `'a` again.
Now `'b` is larger than `'a`.
The compiler knows the reference in `r` will always be valid.

### Generic Lifetimes in Functions

> Let’s write a function that returns the longer of two string slices.
> This function will take two string slices and return a string slice.
> After we’ve implemented the longest function, the code in Listing 10-20 should print `The longest string is abcd`.

```rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
```

We don't want the function to take ownership of the parameters it gets.
We want the function to take string slices, those are references.

An implementation that doesn't compile:

```rust
// DOES NOT COMPILE
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

The compiler error tells us why:

```error
$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0106]: missing lifetime specifier
 --> src/main.rs:9:33
  |
9 | fn longest(x: &str, y: &str) -> &str {
  |                                 ^ expected lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0106`.
error: could not compile `chapter10`.

To learn more, run the command again with --verbose.
```

Rust can't tell if the returned value will be `x` or `y`.
It doesn't know the lifetime of the returned value (we don't either).

We also don't know anything about the lifetimes of the references that are passed as parameters when this function is used.
So we can't look at the scopes like above and determine if the reference it returns will be valid.
The borrow checker can't determine this either, it doesn't know how the lifetime of `x` and `y` relate to the lifetime of the returned value.

> To fix this error, we’ll add generic lifetime parameters that define the relationship between the references so the borrow checker can perform its analysis.

### Lifetime Annotation Syntax

Lifetime annotations don't change how long any reference lives.

Like generic type annotations can represent any type, generic lifetime annotations can represent any lifetime.
Lifetime annotations describe the relationships of the lifetimes of multiple references to eachother.

syntax: start with an apostrophe `'`, followed by a name in all lowercase.
The first lifetime is usually `'a` (presumably for "a lifetime"? I don't know, programmers ran out of the imagination that made them use `T` for generic types.)
Lifetime parameter annotations are places after the `&` of a reference, with a space to seperate them from the reference's type.

```rust
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

One lifetime annotation doesn't mean much, as the compiler uses them to determine the relationships between the lifetimes of multiple references.

> let’s say we have a function with the parameter first that is a reference to an `i32` with lifetime `'a`.
> The function also has another parameter named second that is another reference to an `i32` that also has the lifetime `'a`.
> The lifetime annotations indicate that the references first and second must both live as long as that generic lifetime.

### Lifetime Annotations in Function Signatures

Back to our `longest` function.
Let's annotate a lifetime to tell the compiler that both `x` and `y` live exactly as long as eachother.
In practice, one of them can live longer than the other, and the compiler will pick the shortest lifetime.

> The constraint we want to express in this signature is that all the references in the parameters and the return value must have the same lifetime. 

As with generic type parameters, lifetimes have to be declared before they can be used.
syntax: inside angle brackets `<>`. After the function name and before the list of parameters.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

This code compiles and works.
This function signature tells the compiler the function take 2 parameters that are a reference.
For some lifetime `'a`, both parameters live at least as long as the lifetime `'a`.
The string slice that is returned will live at least as long as the lifetime `'a`.
In practice, that means the lifetime of the reference that is returned is the same as the smaller of the lifetimes that are passed in.
(It might be the longer one, but the compiler doesn't know this and assumes it's the shortest one in order to make guarantees about references being valid)

> we’re not changing the lifetimes of any values passed in or returned.
> Rather, we’re specifying that the borrow checker should reject any values that don’t adhere to these constraints.

The `longest` function doesn't need to know exactly how long `x` and `y` will life, only that some scope can be substituted for `'a` that will satisfy the signature.

The lifetime annotation are in the function signature.
They give hints about how lifetimes of things from the outside relate to eachother.
Rust can analyze the code within the function without help.
However for a reference to or from code outside that function, it needs help.
The lifetimes of parameters or a return value might be different each time the function is called, that's why we have to annotate them.

When we call `longest`, the concrete references that are passed in determine the concrete lifetime of the generic `'a` annotation.
The concrete lifetime that is substituted for `'a` is the part of the scope of `x` that overlaps with the scope of `y`.
In other words, the concrete lifetime that is used for the generic `'a` is equal to the smaller of the lifetimes of `x` and `y`.
Because we've annotated the returned reference with the same lifetime parameter `'a`, it will be valid equally as long as the smaller of the lifetimes for `x` or `y` is.

a practical example of this in action:

```rust
fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

Here, `string1` is valid until the end of the outer scope.
`string2` is valid until the end of the inner scope.
The `result` references something that is valid until the end of the inner scope.
It compiles and prints `longest string is long string is long`.

In the next example, a small change prevents this code from compiling.
The lifetime of the reference in `result` must be the smaller lifetime of the two arguments.
By moving the declaration of `result` to the outer scope while leaving the assignment of the reference in the inner scope,
we'll then move the `println!` that uses `result` outside of the inner scope.

The program tries to access something with a lifetime that has already passes, the compiler won't allow this.

```rust
// DOES NOT COMPILE
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

The compiler error alerts us to this problem:

```error
$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0597]: `string2` does not live long enough
 --> src/main.rs:6:44
  |
6 |         result = longest(string1.as_str(), string2.as_str());
  |                                            ^^^^^^^ borrowed value does not live long enough
7 |     }
  |     - `string2` dropped here while still borrowed
8 |     println!("The longest string is {}", result);
  |                                          ------ borrow later used here

error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.
error: could not compile `chapter10`.

To learn more, run the command again with --verbose.
```

For `result` to be valid until the `println!`, `string2` has to be valid until the end of the outer scope.
The compilers knows this because we annotated the lifetimes of the function parameters and the return value.

But wait a minute, as humans we can see the lifetime of `string2` doesn't matter.
The reference stored in `result` is a reference to the value of `string1` and that clearly lives long enough.
The compiler doesn't know this.
It provides guarantees that the thing referenced will be valid.
Our annotations mean that it treats the lifetime of the returned value as the shorter of the passed in parameters.
Then it can guarantee that the reference will be valid during that lifetime.
Any longer and it can't make that guarantee anymore.
If we didn't do that and ran it without those lifetime annotations:
Things _might_ work, they might also blow up.
Rust, instead of picking the potential blowup, doesn't even compile.

> We’ve told Rust that the lifetime of the reference returned by the longest function
> is the same as the smaller of the lifetimes of the references passed in.
> Therefore, the borrow checker disallows the code in Listing 10-24 as possibly having an invalid reference.

### Thinking in Terms of Lifetimes

When returning a reference from a function, the lifetime parameter for the return type needs to match
the lifetime parameter for one of the function parameters.
If the returned reference does not refer to one of the parameters, it must refer to a value created within the function.
That would be a dangling reference, as the value it is referring to goes out of scope when the function ends.

```rust
// DOES NOT COMPILE
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
```

Eventhough we specified a lifetime parameter `'a` for the return type.
It does not relate to the lifetimes of the function parameters, causing a compiler error:

```error
$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0515]: cannot return value referencing local variable `result`
  --> src/main.rs:11:5
   |
11 |     result.as_str()
   |     ------^^^^^^^^^
   |     |
   |     returns a value referencing data owned by the current function
   |     `result` is borrowed here

error: aborting due to previous error

For more information about this error, try `rustc --explain E0515`.
error: could not compile `chapter10`.

To learn more, run the command again with --verbose.
```

`result` goes out of scope at the end of the function and gets cleaned up.
The returned value is trying to refer to `result`, that doesn't exist anymore.

The fix is: not returning a reference to a value, but an owned data type.

### Lifetime Annotations in Struct Definitions

It's possible for structs to hold references.
If it does, the reference needs to be valid _at least_ as long as the struct lives.

If a struct holds references, it needs a lifetime annotation on every reference in the struct's definition.

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
```

As with function generic types, the generic lifetime annotations need to be declared before they can be used.
syntax: Declaring annotations after the struct name, inside angle brackets.
Using annotations in the places where types are normally annotated.

This annotation means the struct can't outlive the reference it holds.

> The `main` function here creates an instance of the `ImportantExcerpt` struct that holds a reference
> to the first sentence of the `String` owned by the variable `novel`.
> The data in `novel` exists before the `ImportantExcerpt` instance is created. In addition,
> `novel` doesn’t go out of scope until after the `ImportantExcerpt` goes out of scope, so the reference in the `ImportantExcerpt` instance is valid.

### Lifetime Elision

Every reference has a lifetime, so we learned to specify lifetime parameters for functions and structs that use references.
Time for exceptions to the rule (unconvincing "yeey").

I'm groaning, but this make code easier and less cluttered to write and understand.

For instance, an example from a while ago:

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

In the olden days of Rust, that wouldn't even compile as those exceptions weren't in place yet.
The equivalent annotated signature:

```rust
fn first_word<'a>(s: &'a str) -> &'a str {}
```

Some patterns (like the one above) were so common they got coded into the compiler, making explicit lifetime annotations unnecessary.
Who knows, the future might bring even more of these, making explicit lifetime annotations even less necessary.

The patterns programmed into Rust's analysis of references are called the _lifetime elision rules_.
They're a set of rules the compiler considers.
If your code fits these cases, you don't need to annotate lifetimes explicitly.

If Rust applies the rules and there is still ambiguity as to what lifetimes references have, the compiler won't guess.
In that case, it will give a compilation error you can fix by annotating how lifetimes relate to each other.

> Lifetimes on function or method parameters are called _input lifetimes_, and lifetimes on return values are called _output lifetimes_.

The compiler uses three of these rules to figure out information about lifetimes that aren't annotated.

> The first rule applies to input lifetimes, and the second and third rules apply to output lifetimes

If the compiler gets to the end of these three rules and there are still references for which it can't figure out lifetimes, it will error.
These rules apply to `fn` definitions and `impl` blocks.

The first rule is that each parameter that is a reference gets its own lifetime parameter.
For a function with 3 parameters that are references there will be 3 seperate lifetime parameters.
eg. `fn foo(x: &i32, y: &i32) {}` will turn into `fn foo<'a, 'b>(x: &'a i32, y: &'b i32) {}`

The second rule is if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
eg. `fn foo<'a>(x: &'a i32) -> &'a i32`

The third rule is if there are multiple input parameters, but one of them is `&self` or `&mut self` (ergo: in methods).
The lifetime of `self` is assigned to all output lifetime parameters.

Time to pretend we're the compiler.
Let's figure out what the lifetimes of references in a function signature are.
"beep, boop" ok, I'm in character.

```rust
fn first_word(s: &str) -> &str {}
```

First rule, each function parameter gets its own lifetime

```rust
fn first_word<'a>(s: &'a str) -> &str {}
```

The second rule only applies on signatures with one input lifetime parameter.
That's the case here, so the lifetime of all outputs gets the same lifetime annotation.

```rust
fn first_word<'a>(s: &'a str) -> &'a str {}
```

All references now have a lifetime annotation and the compiler can continue its analysis without the programmer needing to annotate any lifetimes.

Another example:

```rust
fn longest(x: &str, y: &str) -> &str {}
```

First rule:

```rust
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {}
```

The second rule doesn't apply because there is more than one input lifetime.
The third rule doesn't apply because this is not a method, no `&self`.
The returned reference still doesn't have a lifetime, so the compiler shows an error.

### Lifetime Annotations in Method Definitions

When implementing methods on a struct with lifetimes, we use the same syntax as that of generic type parameters.
Where we declare and use lifetime parameters depends on whether they're related to the struct fields or the method parameters and return values.

Lifetime names for struct fields always need to be declared after the `impl` keyword and then used after the struct's name.
Those lifetimes are part of the struct's type.

In the `impl` block, references might be tied to the lifetime of references in the struct's fields, or they might be independent.
The third lifetime elision rule makes it so annotations are often not needed for methods.

```rust
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
```

The lifetime parameter declaration after the `impl` and the usage after the type name are required.
The first elision rule makes it so we don't need to annotate `level`.

an example of the third elision rule in action:

```rust
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

### The Static Lifetime

A special lifetime is `'static`.
That means a reference _can_ live for the entire duration of the program.
All string literals have the `'static` lifetime, which means we can annotate them as such:

```rust
let s: &'static str = "I have a static lifetime.";
```

The text of this string is stored directly in the binary of the program.
That memory is always there while the program is running, as it contains the program itself.

The compiler might sometimes suggest adding a `'static` lifetime.
The cause of that problem is often trying to create a dangling reference.
While adding `'static` can "solve" that problem,
the best solution is not always best solved by making something live the entire duration of the program.

### Generic Type Parameters, Trait Bounds, and Lifetimes Together

Bringing generic types, trait bounds, and lifetime annotations together in one function:

```rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest_with_an_announcement(
        string1.as_str(),
        string2,
        "Today is someone's birthday!",
    );
    println!("The longest string is {}", result);
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

The lifetime annotation and generic type parameter are next to each other in the angle brackets `<>`, seperated by a comma.

## 11. Writing Automated Tests

Rust is designed with a high degree of concern about correctness.
The compiler will catch many issues and even provides some guarantees about the corectness of your code.
The type system is responsible for a large part of this.

However, it can't read your mind, and logical correctness is up to you, the programmer.
Like a function called `add_2`, adding 2 to a number and not subtracting 2, or any other behavior for that matter.
They might all be technically correct [(the best kind of correct)](https://youtu.be/hou0lU8WMgo), but logically flawed.

Rust supports automated software tests.

If we write an `add_2` function that takes an integer, and returns an integer, Rust will check that.
For instance, trying to pass a `String` as parameter won't work.
The compiler will prevent that.
The correctness of the logic within that function is up to us.

We can write tests that assert the function returns the expected values when given a few inputs.
We can run these tests whenever we make changes to our code to make sure any existing correct behavior doesn't change.

## 11.1. How to Write Tests

Tests are functions that verify the non-test code is functioning correctly.

> The bodies of test functions typically perform these three actions:
> 
> 1. Set up any needed data or state.
> 1. Run the code you want to test.
> 1. Assert the results are what you expect.

### The Anatomy of a Test Function

A test is a function that is annotated with the `test` attribute.
Attributes are a piece of syntax we haven't seen before.
They're metadata about pieces of Rust code.

An example is the `derive` attribute we previously used with structs (`#[derive(Debug)]`).
To change a function into a test, add `#[test]` on the line before `fn`.

Functions marked like that will be treated as tests.
When you run `cargo test`, cargo builds a test binary runner and executes it.
It reports passes or failures from the functions that were marked with `#[test]`.

When you create a new library project with Cargo, a test module with a test function inside is automatically generated.
The module isn't strictly necessary, but has a few advantages and is considered best practice.

You can add as many test modules, or functions, as you wish.

Let's create a new library project

```sh
cargo new adder --lib
```

In `src/lib.rs` there is a `tests` module marked with the `cfg` attribute.
More specifically, `#[cgf(tests)]`.

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

The `it_works` function is marked with `#[test]`,
this attribute indicates it's a test function and the success/failure should be reported to the user.
We could have non-test functions in the `tests` module (helper functions), that's why we need to indicate which functions are tests with `#[test]`.

The function body uses the `assert_eq!` macro in the body.
That macro calls `panic!` internally is the 2 arguments it gets aren't equal.

Running `cargo test` will execute all tests in the project and output the results:

```sh
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.57s
     Running target/debug/deps/adder-92948b65e88960b4

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

It shows `running 1 test`.
Underneath it is a list of all tests it ran (a single one in this case).
The line below shows the path to the test functions it ran:
`tests::it_works` (a module called `tests` and a function called `it_works`),
and the result of that test: `ok`.

The overall summary of all the tests is underneath that list.
`test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`
It mentions the overall result of the tests: `ok`, followed by a summary.

Tests can be marked as ignored, those won't be ran unless specifically instructed.
Specific tests can be run using a filter, which we didn't do, so the summary showed 0 tests being filtered out.

The `0 measured` is for benchmark tests.

That was one section of the testing output.
The next section starts with `Doc-tests adder`, and follows a similar structure.

This section is for documentation tests, which we don't have yet.
Rust can compile code examples in documentation.
This is very useful to make sure comments stay current to the code they use.

Let's add a failing test.
Tests fail when something in the test function panics.
Each test is ran in a new thread, when the main thread sees that a thread died, that test is marked as failed.

the added test is a function marked with the `test` attribute, inside the module named `tests`:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("Oh, oh.");
    }
}
```

The output:

```sh
❯ cargo test
   Compiling adder v0.1.0 (/home/nicky/projects/scrapyard/book/11_writing_automated_tests/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.41s
     Running target/debug/deps/adder-dae29a78559bde73

running 2 tests
test tests::it_works ... ok
test tests::another ... FAILED

failures:

---- tests::another stdout ----
thread 'tests::another' panicked at 'Oh, oh.', src/lib.rs:10:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::another

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out

error: test failed, to rerun pass '--lib'
```

The list of tests now includes a second line for the test we added: `test tests::another ... FAILED`
The status of the test is not `ok`, it's `FAILED`.

2 new sections appear between that list and the summary.

1. The first section displays a detailed reason for each test failure.
Including the output that caused the panic.
That output mentions the panic happened on line 10 of `src/lib.rs`.
1. The second section lists the names of all the failing tests.
If there are a lot of them, that's useful to have, instead of having to scroll to each output section individually.
We can use the name of a failing test to ran it more easily to debug by running tests with a filter.

The overall result changed from `result: ok` to `result: FAILED`.

### Checking Results with the assert! Macro

The `assert!` macro is provided by the standard library and assures the value passed to it evaluates to `true`.
We pass it an argument that evaluates to a boolean.
If that boolean is `true`, `assert!` does nothing and the test passes.
If that boolean is `false`, `assert!` panics and the test fails.

Earlier, this book had us write some code for a `Rectangle` struct, and a `can_hold` method.

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

`can_hold` returns a boolean, perfect to test the `assert!` macro in a test.

Change the file to add that code and write a test for it:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }
}
```

The `use super::*` is because the test is in a child module to the code it's testing.
This is very common in a testing module, bringing in everything from the parent module so it can be used.

The test creates two instances of `Rectangle` and asserts that the larger one can hold the smaller one.
Checked via the method on `Rectangle` named `can_hold`.
Because the definition for that method requires a reference to a `Rectangle` to be passed as argument,
we pass `&smaller`.
The returned value should be `true`, the `assert!` should do nothing, and the test passes.

```sh
running 3 tests
test tests::it_works ... ok
test tests::another ... FAILED
test tests::larger_can_hold_smaller ... ok
```

Adding another test that asserts a smaller rectangle cannot hold a larger one:

```rust
#[test]
fn smaller_cannot_hold_larger() {
    let larger = Rectangle {
        width: 8,
        height: 7,
    };
    let smaller = Rectangle {
        width: 5,
        height: 1,
    };

    assert!(!smaller.can_hold(&larger));
}
```

Very similar in structure, almost identical.
The logic is flipped, so we need to flip the boolean that returns from `can_hold`.
Now, if the method returns `false`, the `assert!` macro will see `!false`, which evaluates to `true`,
not making the `assert!` panic, and passing the test.

Time to see what happens when we introduce a bug, making our tests fail.
Flipping a signle sign from `>` to `<` will do the trick:

```rust
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width < other.width && self.height > other.height
    }
}
```

Now, the list of tests when you run `cargo run` includes a failure:

```sh
test tests::smaller_cannot_hold_larger ... ok
test tests::larger_can_hold_smaller ... FAILED
```

The output for that failure:
```sh
---- tests::larger_can_hold_smaller stdout ----
thread 'tests::larger_can_hold_smaller' panicked at 'assertion failed: larger.can_hold(&smaller)', src/lib.rs:39:9
```

### Testing Equality with the assert_eq! and assert_ne! Macros

It is very common to compare the result of your code to an expected value.
This is possible with `assert!` and adding an equality expression with the `==` operator.
It's so common the standard library provides two other macros for it:
- `assert_eq!` checks for equality
- `assert_ne!` expects inequality

They will also print the two values given as arguments if they fail, making it more convenient to see whhy they failed.
The `assert!` macro with a `==`  or `!=` comparison would not do that and only mention the check failed.

We add a function to test and use `assert_eq!` to verify the return value for the given argument is what we expect:

```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
}
```

The test verifies the first argument `4` i s equal to the second argument `add_two(2)`.
As expected, it passes and the relevant line is added to the `cargo test` output:
`test tests::it_adds_two ... ok`

Time to introduce a bug to see the output again!

```rust
pub fn add_two(a: i32) -> i32 {
    a + 3
}
```

The item in the list changes to `test tests::it_adds_two ... FAILED`.
It has a section with output from that test:

```sh
---- tests::it_adds_two stdout ----
thread 'tests::it_adds_two' panicked at 'assertion failed: `(left == right)`
  left: `4`,
 right: `5`', src/lib.rs:61:9
```

The parameters are not named `expexted` and `received` like in [Jest](https://jestjs.io/en/).
So the order the arguments are supplied to the macro have no special semantical meaning.
They will be called `left` for the first argument, and `right` for the second argument.

`assert_ne!` will fail if two values are equal.
It's useful for cases we're not sure what a value will be, but certain about what it won't be.
For example: a function that is guaranteed to change the input and return the changed value.
The change is unknown as it's _flaky_ (based on the current time, or random, ...)

To get `assert_eq!` and `assert_ne!` to print the results of comparison with `==` or `!=`,
arguments need to implement the `PartialEq` and `Debug` traits.
You'll sometimes need to implement these on types you create.
It's common that possible by annotating them with the derive attribute.
`#[derive(PartialEq, Debug)]`.

### Adding Custom Failure Messages

You can add a custom message to the `assert!`, `assert_eq!`, and `assert_ne!` macros.
Any argument after the two required ones are passed to the `format!` macro.
This allows you to construct a custom string that will be output upon failure.
In practice that means a format string with `{}` as placeholder, and values that should fill that placeholder.

> Custom messages are useful to document what an assertion means; when a test fails, you’ll have a better idea of what the problem is with the code.

Let's add a greeting function and a test:

```rust
pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }
}
```

The requirements for the function aren't agreed yet and may change.
We decide to not check for exact equality, but we'll check the output contains the input parameter.

Purposeful bug introduction time!

```rust
pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}
```

The output in `cargo test` for this failing test:

```sh
---- tests::greeting_contains_name stdout ----
thread 'tests::greeting_contains_name' panicked at 'assertion failed: result.contains("Carol")', src/lib.rs:71:9
```

This just alerts us that the test failed,
a more useful message would also print the value we got from the `greeting` function.

Giving the `assert!` a custom message:

```rust
#[test]
fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{}`",
        result
    );
}
```

The resulting output from the failure is now more descriptive:

```sh
---- tests::greeting_contains_name stdout ----
thread 'tests::greeting_contains_name' panicked at 'Greeting did not contain name, value was `Hello!`', src/lib.rs:71:9
```

### Checking for Panics with should_panic

Apart from checking correct returned values, it's important to verify errors are handled correctly.
_sooooo, testing the not-so-happy-path_

The book created a `Guess` type earlier.
Other code depends on the guarantee `Guess` provides: instances will be integers between 1 and 100.
We can write a test that verifies that trying to create an instance of `Guess` with a value outside of that range causes a panic.

We do this by adding another attribute to our function that has the `test` attribute.
The `should_panic` attribute (written as `#[should_panic]`) makes tests pass if the code inside the function panics.
It doesn't care how or where, only that it does.

`#[should_panic]` is written after the `#[test]` attribute and before the test function it applies to.
Applied to the `Guess` code:

```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

Running `cargo test` causes a panic in the `greater_than_100` function and the test passes.

Time to, you guessed it (pun definitely intended), introducing a bug and looking at the output!

Deleting the `|| value > 100` check in the `new` associated function should do it.

The output:

```sh
---- tests::greater_than_100 stdout ----
note: test did not panic as expected
```

It's correct, just not very helpful.
We can add an optional `expected` parameter to the `should_panic` attribute.
The tests will make sure the panic message contains the provided text.

syntax: `#[should_panic(expected = "boop")]`

We modified the `Guess` struct to provide more detailed messages on panic.
In the test, we expect the function to panic with a specific (part of) a message.

```rust
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

This test will pass because the `expected` parameter to the `should_panic` attribute matches the message that was output when the function panicked.

The `expected` parameter is a substring of the panic message that caused the `greater_than_100` function to panic.

> What you choose to specify in the expected parameter for `should_panic` depends on how much of the panic message is unique or dynamic and how precise you want your test to be. 

The substring is sufficient to ensure one arm of the `if` condition was executed.

Time to cause a failing test again! Swapping the `<` to a `>` in the `value < 100` check this time.

~~Ok. This is a part where my experience is different to the book.
I get no output, the section of output for failure.~~
I am a doofus, to cause the bug, swap the bodies of the `if` and `else if` conditions.

The complete output from `cargo test` I should have seen, per the book:

```sh
$ cargo test
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished test [unoptimized + debuginfo] target(s) in 0.66s
     Running target/debug/deps/guessing_game-57d70c3acb738f4d

running 1 test
test tests::greater_than_100 ... FAILED

failures:

---- tests::greater_than_100 stdout ----
thread 'main' panicked at 'Guess value must be greater than or equal to 1, got 200.', src/lib.rs:13:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
note: panic did not contain expected string
      panic message: `"Guess value must be greater than or equal to 1, got 200."`,
 expected substring: `"Guess value must be less than or equal to 100"`

failures:
    tests::greater_than_100

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out

error: test failed, to rerun pass '--lib'
```

That message indicates the test paniced as expected, but the panic message did not include the string we passed to `expected`.

### Using Result<T, E> in Tests

So far, test functions panicked to fail.
Test functions can also return a `Result`.
Either an `Ok` to pass, or an `Err` to fail.

Here is that first basic test rewritten to return a `Result`

```rust
#[test]
fn it_works() -> Result<(), String> {
    if 2 + 2 == 4 {
        Ok(())
    } else {
        Err(String::from("two plus two does not equal four"))
    }
}
```

We return an `Ok(())` if the test passes.
An `Ok` that contains the unit value `()`.
(the unit type is a type with one single possible value, an empty tuple)

We return an `Err(String)` if the test fails.

Returning a `Result` allows you to use the questionmark operator ([the Annie operator](https://nickymeuleman.netlify.app/garden/rust-syntax-questionmark)) in your test function.
Convenient to write tests that should fail if an operation return an `Err` variant.

You can't use the `#[should_panic]` annotation on tests that return a `Result`, only on ones that return the unit type.
Instead, you should return an `Err` value directly (or wrapped in `Ok`, I guess) if the test should fail.

## 11.2. Controlling How Tests Are Run

`cargo run` compiles your code and runs the resulting binary.
`cargo test` also compiles your code and runs the resulting binary.

We can specify command line arguments to either affect `cargo test`, or the resulting binary from `cargo test`.

The default behavior of the binary `cargo test` produces is to run all tests in parallel and capture output generated by tests.
This prevents output from being displayed while the tests are ran.

Use the `--` to seperate the arguments to `cargo test`, and the binary it creates.

- `cargo test --help` displays help for options on `cargo test`.
- `cargo test -- --help` displays help for options on the resulting binary.

### Running Tests in Parallel or Consecutively

Because tests run in parallel by default,
you have to make sure they don't depend on each other, or any shared state, current working directory, or environment variables.

If for example each test runs some code that creates a file on disk and writes to it.
Then, because tests run at the same time, if the file is created with the same name in the same directory, they might fight each other.
One test might create a file, write to it, but before it tests the results an other test overwrites that file.
This makes the tests fail, resulting in sad tests...

One solution is to make each test create a different file so they don't interfere with each other.
Another is to run the tests in series, not parallel.

You can control the number of threads that are used to run tests:

```sh
cargo test -- --test-threads=1
```

One thread, gone parallellism, _taps temple_.

It'll take longer to finish the suite of tests, but they won't interfere with each other anymore, tradeoffs.

### Showing Function Output

By default, Rust captures output, and if a test passes, it's not shown at all.
As we saw, failure messages are captured too, but shown in the final output of `cargo test`.

That means, no `println!` for passing tests!
Failing tests show that output, it's included in the failure message.

Messages to stdout can be helpful to debug (I agree, I `console.log` in JS all the time).
To show them for passing tests as well, pass a command line flag to the testig binary.

```sh
cargo test -- --show-output
```

With that flag, passing tests that output to stdout get their own section.
for example:

```rust
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }
}
```

```sh
successes:

---- tests::this_test_will_pass stdout ----
I got the value 4
```

and later:

```sh
failures:

---- tests::this_test_will_fail stdout ----
I got the value 8
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `5`,
 right: `10`', src/lib.rs:19:9
```

### Running a Subset of Tests by Name

Running a whole testsuite can take a long time, or the resulting output makes it hard to find the output relevant to the feature you're currently working on.
You can choose which tests to run by passing the names of the test(s) you want to run to `cargo test` as an argument.

add three tests for the `add_two` function

```rust
#[test]
fn add_two_and_two() {
    assert_eq!(4, add_two(2));
}

#[test]
fn add_three_and_two() {
    assert_eq!(5, add_two(3));
}

#[test]
fn one_hundred() {
    assert_eq!(102, add_two(100));
}
```

### Running Single Tests

Passing the name of a test function as argument:

```sh
cargo test one_hundred
```

causes only that test to run and output results:

```sh
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running target/debug/deps/adder-dae29a78559bde73

running 1 test
test tests::one_hundred ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out
```

Only the test with the name `one_hundred` ran.
The summary at the end tells us: `running 1 test`.
Lower it tells us 1 test passed and 8 were filtered out (the remaining ones I didn't clear out from `src/lib.rs`).

Only the first value given to `cargo test` will be used.
So we can't run multiple tests by providing multiple arguments.

### Filtering to Run Multiple Tests

We can specify a part of a test name.
Doing that will run all tests that have a name that matches that value.

```sh
cargo test add
```

This will run all tests with `add` in the name.

```sh
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running target/debug/deps/adder-dae29a78559bde73

running 2 tests
test tests::add_two_and_two ... ok
test tests::add_three_and_two ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out
```

We can run all tests within a module by filtering on the name of that module.
That's a further usecase for having tests in (multiple) modules.

Our module is names `tests`, so the command would be `cargo test tests`.

### Ignoring Some Tests Unless Specifically Requested

A single test can take a very long time, so we might want to exclude that one from running during normal runs of `cargo test`.

You can annotate those tests with the `ignore` attribute, beneath the `test` attribute.

```rust
#[test]
#[ignore]
fn expensive_test() {
    // code that takes an hour to run
}
```

Now tests annotated with `#[ignore]` won't execute during regular `cargo test` runs.
They will be counted as `ignored` in the summary.

If we want to run only ignored tests:

```sh
cargo test -- --ignored
```

## 11.3. Test Organization

Rust has 2 main categories: unit, and integration tests.

Unit tests are small and focussed.
Testing one module in isolation at a time.
They can use private interfaces, as they're typically located in the same file as the code they are testing.
(because of the privacy rules we covered when talking about the module system).

Integration tests are external to your library and use only public parts of your code.
They can test multiple modules of your code per test.

### Unit Tests

As the name suggest, these test a unit of code.
Usually small, and in isolation from the rest of the code.
They test if that unit of code is working as expected.

The convention is te create a module that houses all your test functions.
That module is typically named `tests` and is annotated with the `cfg()` attribute that takes `test` as a parameter.

Syntax:
```rust
#[cfg(test)]
mod tests {
    // tests
}
```

### The Tests Module and #[cfg(test)]

The `#[cfg(test)]` tells the compiler what follows is code specific to tests.
It will only compile it during `cargo test`, not during `cargo build`.

Because integration tests go in a different directtory, they don't need that annotation.

The `cfg` attribute stands for configuration and tells the compiler the following item should only be included in a specific configuration.
In this case, that option is `test`, so the complete attribute is `cfg(test)`.
`cargo test` tells the compiler to use the `test` configuration option, and bingo, presto, TESTS.
That's useful when this module includes a bunch of code that's only used as a helper during testing.
It also prevents that code from compiling to production binaries, not just functions that are annotate with `#[test]`.

### Testing Private Functions

Unit tests are usually in a module named `tests` inside of the code module they are testing.
The code adheres to the privacy rules of the module system,
so units of code can be accessed even if they aren't marked with `pub` to make them explicitly public.

### Integration Tests

In Rust, integration tests are external to your library.
You use your library in the same way as other people would.
You adhere to the privacy rules of the module system and only public units of code can be used.

Their purpose is to test if many parts of your library work as intended when used together.

To create integration tests you need a `tests` directory.
As mentioned before, this directory is only compiled during `cargo test`, not `cargo build`.

#### The tests Directory

We create a `/tests` directory at the top level of our project directory, at the same level as `/src`.
Cargo knows to look for integration tests there.
We can make as many files as we want there, and cargo will compile each of them as an individual crate.

Create a new file at `tests/integration_test.rs`

```rust
use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
```

We import our crate by its name at the top: `use adder`.
Each file in the `tests` directory is a seperate crate, we need to bring our library into each crate's scope.

Right now, `add_two` isn't marked as `pub` yet, so when we run `cargo test`, the testing binary won't even compile.
After fixing that, this is the output:

```sh
 Finished test [unoptimized + debuginfo] target(s) in 0.94s
     Running target/debug/deps/adder-dae29a78559bde73

running 10 tests
test tests::add_three_and_two ... ok
test tests::add_two_and_two ... ok
test tests::greater_than_100 ... FAILED
test tests::it_works ... ok
test tests::it_boops ... FAILED
test tests::greeting_contains_name ... FAILED
test tests::smaller_cannot_hold_larger ... ok
test tests::larger_can_hold_smaller ... FAILED
test tests::one_hundred ... ok
test tests::another ... FAILED

failures:

-- shortened for notes --


failures:
    tests::another
    tests::greater_than_100
    tests::greeting_contains_name
    tests::it_boops
    tests::larger_can_hold_smaller

test result: FAILED. 5 passed; 5 failed; 0 ignored; 0 measured; 0 filtered out

error: test failed, to rerun pass '--lib'
```

Soooooooooo, those are only the unit tests.
After commenting out all failing unit tests:

```sh
    Finished test [unoptimized + debuginfo] target(s) in 0.54s
     Running target/debug/deps/adder-dae29a78559bde73

running 5 tests
test tests::add_two_and_two ... ok
test tests::add_three_and_two ... ok
test tests::one_hundred ... ok
test tests::smaller_cannot_hold_larger ... ok
test tests::it_works ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/integration_test-7842cb402bfb7433

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Much better.
It seems the integration (and doc?) tests don't run if the unit test fail.
TODO: investigate

The output contains 3 big sections:
1. unit tests
2. integration tests
3. doc tests

As with unit tests, each test has a different line and outputs conditionally (output from passing tests stays captured).
Similar to unit tests, adding more `#[test]` functions adds another line.

Each integration test file has its own section.
Adding another file will add another integration test section to the 3 that are output now.

Add another file at `tests/second_integration_file.rs`.

```rust
#[test]
fn learning_testing() {
    assert!(true);
}
```

Similar to unit test filtering, we can run particular integration tests by specifying the test name as argument to `cargo test`.
To run all tests in an integration test file, use the `--test` argument followed by the name of the file.

`cargo test --test second_integration_file`

```sh
   Compiling adder v0.1.0 (/home/nicky/projects/scrapyard/book/11_writing_automated_tests/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.32s
     Running target/debug/deps/second_integration_file-8e9ac4fba2a98530

running 1 test
test learning_testing ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

#### Submodules in Integration Tests

As integration tests get larger, you might want to break the testing files up into modules that house (helper) code.
Each file in the `tests` directory gets compiled as an individual crate.

This means the files in the `tests` directory don't behave like the files in the `src` directory regarding usage of modules.

Imagine a function that's used across our tests and we want to extract it to a module.

```rust
pub fn setup() {
    // setup code specific to your library's tests would go here
}
```

If we place that in `tests/common.rs`, as described above,
it's now its own crate that gets its own section in the testing output section.

The relevant output when we run `cargo test`:

```sh
    Running target/debug/deps/common-7064e1b6d2e271be

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

We don't want that, it contains no test functions and should not get an output section when we run `cargo test`.

We'll use the other method to create a module.
We'll make a file at: `tests/common/mod.rs`.
`mod.rs` is the generic name for a module named after its parent folder.
This reminds me of an `index.js` in javascript.

This will create a module called `common`.
Since it's not a top level file, it's not handled as a seperate integration test file with an own section.

This naming for modules is also understood in `src`, you can use it if you don't like the top level modules.
Especially useful if that module has nested modules of its own.

The code in the `common` module can then be used as expected in tests.

```rust
use adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
```

### Integration Tests for Binary Crates

If the project is a binary crate and only contains `src/main.rs`, and no `src/lib.rs`,
we can't create integration tests in the `tests` directory and bring functions from `src/main.rs` into scope.
Only library crates expose functions that other crates can use.
Integration tests are crates.

Binary crates are meant to be run on their own.

Many projects that provide a binary have a straightforward `src/main.rs` and house most of the logic under `src/lib.rs`.
That logic can then be tested with integration tests.
The `main.rs` then uses the logic from `lib.rs`.

## 12. An I/O Project: Building a Command Line Program

Project time! This chapter of the book combines what we learned and guides us through building a CLI.

We're going to be building a miniature version of `grep`, the real one uses regular expressions, this one won't.
It takes as arguments a filename and a string.
It reads that file, searches it for that string, and prints those lines.

We'll start off by creating a new binary crate:

```sh
cargo new minigrep
```

## 12.1. Accepting Command Line Arguments

We want the project to accept 2 command line arguments, a file, and a string to search for.
Then we would be able to execute the resulting binary like so:

```sh
cargo run searchstring example-filename.txt
```

### Reading the Argument Values

Rust's standard library has a function to read command line arguments: `std::env::args()`.
It returns an iterator, so we `collect()` it into a vector.

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
```

The book specifically notes `std::env::args` will panic if an argument contains invalid Unicode.
You should use `std::env::args_os` then instead.
The resulting iterator for that one produces `OsString` types instead of `String` types.

The first item in our vector is `"target/debug/minigrep"`, that's where the binary ran from.

> This matches the behavior of the arguments list in C, letting programs use the name by which they were invoked in their execution.
> It’s often convenient to have access to the program name in case you want to print it in messages or change behavior of the program based on what command line alias was used to invoke the program.

The next items are aguments we passed to it when executing the binary.

For our example above, it would log: `["target/debug/minigrep", "searchstring", "example-filename.txt"]`.

### Saving the Argument Values in Variables

Adding the following line to store references to items in the vector in variables:

```rust
let query = &args[1];
let filename = &args[2];
```

We decided that the first argument will be the string to search for.
The second argument will be the file to seach in.

Printing them out to see our progress:

```rust
println!("Searching for {}", query);
println!("In file {}", filename);
```

```sh
cargo run searchstring example-filename.txt
   Compiling minigrep v0.1.0 (/home/nicky/projects/scrapyard/book/12_an_io_project/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33s
     Running `target/debug/minigrep searchstring example-filename.txt`
Searching for searchstring
In file example-filename.txt
```