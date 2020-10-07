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
