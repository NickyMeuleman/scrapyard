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
