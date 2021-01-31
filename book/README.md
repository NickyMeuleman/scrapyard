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
where `<name>` is whatever is in the `name` field of `Cargo.toml`.

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
Add it to the dependencies in `Cargo.toml`.

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
A package has a `Cargo.toml` file.

> A package must contain zero or one library crates, and no more.
> It can contain as many binary crates as you’d like, but it must contain at least one crate (either library or binary).

When you run `cargo new my-project` you create a new binary package.
It's in a folder called `my-project` that has a `Cargo.toml`.
The entrypoint is in `src/main.rs`.
There is no mention of that in the `Cargo.toml` file because it's a convention that file will be the crate root of a binary crate with the same name as the package.

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

Use external packages in your code by adding them to `[dependencies]` in `Cargo.toml`.

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
## 12.2. Reading a File

Time to read the file we want to search.
Same example as the book, a `poem.txt` file located at the root of the project.

```
I’m nobody! Who are you?
Are you nobody, too?
Then there’s a pair of us - don’t tell!
They’d banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
```

Read the contents of the opened file to a string by first importing `fs` from the standard library,
and then calling the `read_to_string` method with the filename as argument to it.
By editing the `main.rs` file, and print the contents like so:

```rust
use std::env;
use std::fs;

fn main() {
    // --snip--
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}
```

`read_to_string` returns a `Result<String>`, the `Ok` variant contains the `String` with the contents of the file.
We call `expect` to either get that data, or panic with the error if the variant was an `Err`.

```sh
cargo run searchstring poem.txt
```

Prints the query, the filename, and the contents of that file.
Running the program with an invalid file, results in an error at the call to `expect`:

```sh
cargo run searchstring does-not-exist.txt
```

```
thread 'main' panicked at 'Something went wrong reading the file: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:9:49
```

## 12.3. Refactoring to Improve Modularity and Error Handling

The `main` function performs two tasks.
It parses arguments and it reads files.
The function is still small, so it's not an issue yet, but splitting up those pieces of logic can be useful.
Seperating functionality makes it quicker to change/add/remove functionality at a later date.

Another point of improvement is the error handling.
Regardless of the way in which reading the file failed, the error states `Something went wrong reading the file`.
That's not very specific!

Other types of errors aren't handled gracefully too.
Running the program without arguments would result in an `index out of bounds` message.
We wrote the program, so we know we can't access spots in that `args` vector that are not there. But still, that's a cryptic error.
An end user will have no idea what that means.

### Separation of Concerns for Binary Projects

The Rust community developed a process to seperate concerns of a binary program:

> - Split your program into a main.rs and a lib.rs and move your program’s logic to lib.rs.
> - As long as your command line parsing logic is small, it can remain in main.rs.
> - When the command line parsing logic starts getting complicated, extract it from main.rs and move it to lib.rs.

> The responsibilities that remain in the main function after this process should be limited to the following:

> - Calling the command line parsing logic with the argument values
> - Setting up any other configuration
> - Calling a run function in lib.rs
> - Handling the error if run returns an error

After following those steps `main.rs` handles running the program and `lib.rs` handles all the logic in that program.
You can't test the `main` function directly, so this structure has the added benefit of being able to tests more pieces of code.
The fraction of code that remains in `main.rs` should be small enough to verify the correctness by reading it.

### Extracting the Argument Parser

We'll extract the parsing functionality into its own function.

```rust
fn main() {
    let args: Vec<String> = env::args().collect();
    let (query, filename) = parse_config(&args);
    // --snip--
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];
    (query, filename)
}
```

We do the same thing, but by passing the entire `arg` vector to a function,
`main` is no longer responsible for figuring out how command line arguments and variables relate to each other.

#### Grouping Configuration Values

Those 2 values in the tuple `parse_config` returns are closely related, they're both part of one configuration value.
Grouping them in a struct would make sense.

> Note: Using primitive values when a complex type would be more appropriate is an anti-pattern known as primitive obsession.

In `main.rs`, make a `Config` struct and return them as fields of that struct.
Later, use those fields to run the other logic.

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    // --snip--
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}
```

The `Config` struct contains owned `String` types, not slices.
The only instance of that struct is the `config` variable, which owns those values.
Since those values come from the `args` vector, and `parse_config` takes a reference to them, 
they are cloned and the owned clones are put into the `Config` instance.
That instance is then returned from the `parse_config` function.


Cloning that data is the easiest, though somewhat inefficient, way of handling those values.
It makes a full copy, taking up more time and memory.
The trade-off is that owned data is easier to work with (no lifetimes) and more flexible.

#### Creating a Constructor for Config

`parse_config` returns a new instance of `Config`.
A useful next step is creating an associated function
so calling `Config::new` with the proper arguments returns the appropriate `Config` instance.

Changing the code in `main.rs` to implement this change:

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    // --snip--
}

impl Config {
    fn new(args: &[String]) -> Self {
        let query = args[1].to_owned();
        let filename = args[2].to_owned();

        Self { query, filename }
    }
}
```

The `Self` in config is equivalent to writing `Config`, since it's in the `impl` block of `Config`.

### Fixing the Error Handling

Running the program without any arguments still results in the same error

```sh
cargo run              
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep`
thread 'main' panicked at 'index out of bounds: the len is 1 but the index is 1', src/main.rs:22:21
note: run with `RUST_BACKTRACE=1` environment variable to display a backtraces
```

#### Improving the Error Message

Adding an `if` condition inside the call to `Config::new` makes that error clearer.
Make sure to add that check before you access anything in that array,
not doing that would still result in that out of bounds error.

```rust
fn new(args: &[String]) -> Config {
    if args.len() < 3 {
        panic!("not enough arguments");
    }
    // --snip--
```

Running the program with any arguments now will show that error instead of the out of bounds one.
That's better, but the call to `panic!` still causes all the other information (like the line and column number) to be shown.
Useful for the programmer, confusing for the end-user.

#### Returning a Result from new Instead of Calling panic!

We can return a `Result` from that function.
The `Ok` will contain a `Config` struct, and the `Err` an appropriate error.
That error can then be used to, for instance, show a clean error message to the user.

Luckily, the refactor to the `new` function is rather small

```rust
impl Config {
    fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].to_owned();
        let filename = args[2].to_owned();
    
        Ok(Self { query, filename })
    }
}
```

The `Err` value is a `&'static str`. A string literal that has a static lifetime.
The function now doesn't stop execution of the entire program,
it returns a `Result` and lets whoever called it decide what to do if it failed.

#### Calling Config::new and Handling Errors

If the call to `Config::new` returns an `Ok` variant, we will continue as usual with the value inside that variant.
If the returned variant is an `Err`, we will print a custom message to the terminal and exit with a non zero error code.
That non zero code is a convention, a signal that the program exited with an error state.

```rust
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // --snip--
```

To do that we used a method called `unwrap_or_else`.
A method on `Result` that either unwraps the value in `Ok`,
or passes the value in `Err` to a closure and calls that function.

In this case, the value inside the `Err` is the static string `not enough arguments`.

`process::exit` is a method in the standard library that will immediately stop execution of the program.
We bring it in with `use` first, call it, and pass `1` to it as our non zero error code.

Now, running the program without any command line arguments result in one line being printed to the console.

```sh
cargo run
   Compiling minigrep v0.1.0 (/home/nicky/projects/scrapyard/book/12_an_io_project/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.36s
     Running `target/debug/minigrep`
Problem parsing arguments: not enough arguments
```

### Extracting Logic from main

Time to extract a function named `run` that will hold all of the logic like stated above.
It won't be involved in setting up configuration or handling errors.
When we're done, `main` will be short and sweet.
We'll be able to write tests for all the logic that isn't in the `main` function.

For now, extract the function but keep it in the `main.rs` file:

```rust
fn main() {
    // --snip--

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    run(config);
}

fn run(config: Config) {
    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

// --snip--
```

> The `run` function now contains all the remaining logic from `main`, starting from reading the file.
> The `run` function takes the `Config` instance as an argument.

#### Returning Errors from the run Function

Returning a `Result` from the `run` function, we can improve error handling further.
That will let us further consolidate logic around handling errors in a user-friendly way.
As a callback to the method mention above,
a bullet point about responsibilities that remained in `main` was "Handling the error if `run` returns an error".

The required changes to `main.rs`:

```rust
use std::error::Error;

// --snip--

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}
```

The returned type of the `run` function is now `Result`.
Previously, the function returned nothing (the unit type `()`).
That logic was kept: an `Ok` wraps the unit and is now what the function returns in case it completed successfully.
An `Ok` that contains a unit `()` might look funky: `Ok(())`.
But a unit is the idiomatic way to signal we are only callinga function for its side effects, not for a returned value.

The error type is a trait object `Box<dyn Error>`.
We've brought in `Error` with `use std::error::Error` before using it.
The book will cover trait objects later, for now it's enough to know `Box<dyn Error>` will returns a type that implements the `Error` trait.

The call to `expect` would cause the program to panic.
Instead, we return the error the `read_to_string` method returned as the returned value from the `run` function if there is one.
This is done by using the questionmark operator `?`.
Or, how I like to call it: [the Annie operator](https://nickymeuleman.netlify.app/garden/rust-syntax-questionmark).

The compiler is smart enough to know the program doesn't handle the possibility where `run` returns an `Err`.
Running it will print the following warning:

```
warning: unused `std::result::Result` that must be used
  --> src/main.rs:21:5
   |
21 |     run(config);
   |     ^^^^^^^^^^^^
   |
   = note: `#[warn(unused_must_use)]` on by default
   = note: this `Result` may be an `Err` variant, which should be handled
```

#### Handling Errors Returned from run in main

When calling `run`, if it completes succesfully that's fine.
The program will end normally by reaching the end of the `main` function.
We care about the case where `run` returns an error (the `Err` variant).

```rust
fn main() {
    // --snip--

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
```

We use `if let` to call the `run` function and destructure the possible value inside `Err`.
If that function returns an `Err`, the codeblock will be entered.
If the function returns an `Ok`, the conditional block is skipped and the program ends normally.

self-plug: https://nickymeuleman.netlify.app/garden/rust-if-let-while-let

### Splitting Code into a Library Crate

Time to move the code that's not in the `main` function to `src/lib.rs`.

Remember to move over the `use` statements, and to make the extracted functions public so they can be used in `main.rs`.

```rust
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        // --snip--
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // --snip--
}
```

`pub` was used liberally.
Not only on the `run` function, but even on the `Config` struct and the indivual fields it contains.

We now have a library crate with a public API, one we can use (and test!) individually.

Once the code is in `lib.rs`, bring the needed parts into scope in `main.rs` with the `use` keyword.

```rust
use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // --snip--
    if let Err(e) = minigrep::run(config) {
        // --snip--
    }
}
```

We bring in `minigrep::Config` to be able to use `Config` directly.
Another possibility would be to use `minigrep::Config` instead, similar to how we call `minigrep::run`.

The `minigrep` part comes from the `Cargo.toml` where the name of the crate is set to `minigrep`.

> Let’s take advantage of this newfound modularity by doing something that would have been difficult with the old code but is easy with the new code: we’ll write some tests!

## 12.4. Developing the Library’s Functionality with Test Driven Development

It's much easier to write tests for the core functionality of our code now that most of our logic is in `lib.rs`.
This chapter does some test-driven development and writes a test, then some code that makes that test pass over and over.
It will implement searching functionality and create a function named `search` using the TDD method.

### Writing a Failing Test

First, send those `println!`s to the shadow realm, they are no longer needed.
Then, add a `tests` module with a test function to `lib.rs`.
The test function specifies the behavior we want the `search` function to have:
>  it will take a query and the text to search for the query in, and it will return only the lines from the text that contain the query.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
```

The test expects the `search` function to return a vector with as only item, the line from the `contents` that included the `query`.
This tests doesn't fail, it doesn't even compile yet, as `search` doesn't exist.

Fair enough, adding a function definition to `lib.rs` fixes that.

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}
```

The function uses string slices.
We need an explicit lifetime `'a`, and tell the compiler the returned vector lives as long as the passed in `contents`.

> This is important! The data referenced by a slice needs to be valid for the reference to be valid;
> if the compiler assumes we’re making string slices of query rather than contents, it will do its safety checking incorrectly.

Trying to write the function without lifetime parameters would cause a compiler error telling us to add one.
Rust can't possibly know which of the two arguments we want, so we need to tell it.
The data returned is (part of) the data we passed into the function via the `contents` parameter, thus, it has the same lifetime.

Trying to run the tests again with `cargo test`

```
    Finished test [unoptimized + debuginfo] target(s) in 0.62s
     Running target/debug/deps/minigrep-0e7f461b6f24dfc4

running 1 test
test tests::one_result ... FAILED

failures:

---- tests::one_result stdout ----
thread 'tests::one_result' panicked at 'assertion failed: `(left == right)`
  left: `["safe, fast, productive."]`,
 right: `[]`', src/lib.rs:44:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::one_result

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out

error: test failed, to rerun pass '--lib'
```

Success, a failing test!

### Writing Code to Pass the Test

Currently, the function always returns an empty vector, to implement `search` we need to follow a few steps:

> - Iterate through each line of the contents.
> - Check whether the line contains our query string.
> - If it does, add it to the list of values we’re returning.
> - If it doesn’t, do nothing.
> - Return the list of results that match.

#### Iterating Through Lines with the lines Method

Rust has a handy method for this, `lines`.
It creates an iterator where every item is the contents of a line.

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    for line in contents.lines() {
        // do something with line
    }
}
```

#### Searching Each Line for the Query

An `if` statement is the ideal candidate for conditionally doing something.
`contains` is a method we can call on a string to check if that string contains a substring.

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    for line in contents.lines() {
        if line.contains(query) {
            // do something with line
        }
    }
}
```

### Storing Matching Lines

The logic to fit inside that `if` statement:
adding the line to a mutable vector called `results`.
That variable is then returned from the function.

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
```

And, _boom_, just like that, a passing test.
Now, we can refactor that code with the assurance of a test and be fairly certain our refactor works if the test keeps passing.
The book hints at doing exactly that later on, in the chapter that deals with iterators.

### Using the search Function in the run Function

Use the `search` function and print every matched line to the console.
This is done by looping over the results of `search` and printing them.
In `lib.rs`:

```rust
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}
```

> Now the entire program should work! Let’s try it out, first with a word that should return exactly one line from the Emily Dickinson poem, “frog”:
> 
> ```
> > $ cargo run frog poem.txt
>    Compiling minigrep v0.1.0 (file:///projects/minigrep)
>     Finished dev [unoptimized + debuginfo] target(s) in 0.38s
>      Running `target/debug/minigrep frog poem.txt`
> How public, like a frog
> ```
> Cool! Now let’s try a word that will match multiple lines, like “body”:
> ```
> $ cargo run body poem.txt
>    Compiling minigrep v0.1.0 (file:///projects/minigrep)
>     Finished dev [unoptimized + debuginfo] target(s) in 0.0s
>      Running `target/debug/minigrep body poem.txt`
> I’m nobody! Who are you?
> Are you nobody, too?
> How dreary to be somebody!
> ```
> And finally, let’s make sure that we don’t get any lines when we search for a word that isn’t anywhere in the poem, such as “monomorphization”:
> 
> ```
> $ cargo run monomorphization poem.txt
>    Compiling minigrep v0.1.0 (file:///projects/minigrep)
>     Finished dev [unoptimized + debuginfo] target(s) in 0.0s
>      Running `target/debug/minigrep monomorphization poem.txt`
> ```

Excellent, we manually tested the program in a couple scenarios and it performs as expected!

## 12.5. Working with Environment Variables

Let's add the option to search case-insensitive that can be turned on via an environment variable.
The book is choosing this method over a command line option.
The reason being, an environment variable can be set once,
and then not typed again during the same terminal session.

### Writing a Failing Test for the Case-Insensitive search Function

The goal is a `search_case_insensitive` function that we can call if the environment variable is on.
We'll rename the existing `one_result` test to `case_sensitive`, and add another test named `case_insensitive`.

```rust
#[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
```

Again, after adding the test, the code doesn't compile because the `search_case_insensitive` doesn't exist.

The line `Duct tape.` was added to the end of the `content` in the `case_sensitive` test.
This ensures the case sensitive version, with query `duct` does not pick up `Duct`.

The `case_insensitive` test searches for `rUsT` and should pick up 2 lines: `Rust:`, and `Trust me.`.

### Implementing the search_case_insensitive Function

The `search_case_insensitive` function can be written very similar to the case sensitive `seach` function.
But, [holy duplication Batman](https://youtu.be/e5hRPy-pDsY)!

```rust
pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
```

The function first shadows the `query` variable with a version that's lowercase.
When checking if a line contains `query`, it first lowercases that line.
Note this doesn't cover all usecases, `to_lowercase` will work for Unicode, so for this piece of the book it's fine.

The shadowed `query` is a `String` now, not a string slice.
That's why there is an ampersand `&` in front of that variable being passed to `contains`.

With this change, the tests pass.
Time to call it from the `run` function.

The first step is adding a boolean to the `Config` struct that signals if the search should be case sensitive or not.

```rust
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
```

In `run`, instead of looping over the returned vector from `search` directly.
Based on the `case_sensitive` boolean, store the results in a variable that is the returned value from the appropriate function.
Loop over those results instead.

```rust
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}
```

Then, decide what the value of that boolean is based on an environment variable and set it in the `new` associated function.
Note: if you were coding along, the compiler notices that boolean is missing from the struct.

```rust
use std::env;
// --snip--

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}
```

The standard library has an `env` section that works with environment variables.
We create a `case_sensitive` boolean and set it as field in the `Config` struct instance returned from `new`.
To set it, we call `env::var` and pass in the name of an environment variable (`CASE_INSENSITIVE`).
As you might notice, that means the exact opposite as the thing in the code.
Why did the book do this? Probably to show the method that makes this logic work again: `is_err`.

The `env::var` function returns a `Result`.
If that environment variable is set, that result will be `Ok`.
So that line with `is_err` will return `true` if the variable is not set. 
Confusing, I know.

Fun side effect: setting the `CASE_INSENSITIVE` environment variable to `false` will still cause our code to search case insensitive.
Because then that variable is set, the line checks if it is, returns `true`, and voila: case insensitive searching.

Testing it out without setting the variable, so searching case sensitive:

```sh
cargo run to poem.txt  
   Compiling minigrep v0.1.0 (/home/nicky/projects/scrapyard/book/12_an_io_project/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.62s
     Running `target/debug/minigrep to poem.txt`
Are you nobody, too?
How dreary to be somebody!
```

Setting the environment variable and searching for the same thing case insensitive:

```sh
CASE_INSENSITIVE=1 cargo run to poem.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep to poem.txt`
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```

Some programs allow configuration via environment variables and command line arguments.
Which take precedency depends on the program.

## 12.6. Writing Error Messages to Standard Error Instead of Standard Output

Most terminals provide two kinds of output:
1. standard output `stdout`
2. standard error `stderr`

As the names suggest, the first for general usage, and the second for error messages.
Right now, we're only using `stdout` with our calls to `println!`.

### Checking Where Errors Are Written

We'll redirect the `stdout` output to a file.
That means we won't see it anymore, the `stderr` will continue to be printed to the console.
This is a frequent usecase, especially for command line programs.

A way to see how our program is not well behaved in this regard yet is to send the output to a file:

```sh
cargo run > output.txt
```

This completes successfully and results in a file with the contents of `Problem parsing arguments: not enough arguments`.
Not exactly expected behavior.


### Printing Errors to Standard Error

The standard library has a macro that prints to the `stderr` output: `eprintln!`.
Very similar to the `println!`s that are already in our code, so refactoring is very minor.

in `src/main.rs`:

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
```

Running the program again without arguments, and writing the result to a file now shows the error in the console:

```sh
cargo run > output.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep`
Problem parsing arguments: not enough arguments
```

`output.txt` contains nothing after this, because nothing was sent to `stdout`

Running the program again with a valid command:

```sh
cargo run to poem.txt > output.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep to poem.txt`
```

Causes 2 lines to be inside `output.txt`, the ones written to `stdout`:

```
Are you nobody, too?
How dreary to be somebody!
```

Note because we send `stdout` to that file, we no longer see those lines being printed in the terminal.

To also have the output to `stdout` printed in the console, you can use `tee`

```sh
cargo run to poem.txt | tee  output.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep to poem.txt`
Are you nobody, too?
How dreary to be somebody!
```

## 13. Functional Language Features: Iterators and Closures

Rust has lots of influences, including functional programming.
Among other things, that means storing functions in variables to pass them around as values.

> More specifically, we’ll cover:
>
> - Closures, a function-like construct you can store in a variable
> - Iterators, a way of processing a series of elements
> - How to use these two features to improve the I/O project in Chapter 12
> - The performance of these two features (Spoiler alert: they’re faster than you might think!)

## 13.1. Closures: Anonymous Functions that Can Capture Their Environment

Closures in Rust are anonymous functions that can be stored in a variable (and passed as argument to other functions).
Unlike functions, closures capture values from the scope in which they're defined.
That's fancy speak for "you can access variables inside that closure from the place where you define the closure".

### Creating an Abstraction of Behavior with Closures

This chapter of the book explains closures and their features (like syntax, type information, and traits) through an example.
In this hypothetical situation you are writing an app that generates a custom workout plan.
The algorithm to do that, and the many factors it takes into account are not important, the use of closures is.
We want to call the algorithm only when needed, because it takes a while to complete.

The simulated algorithm lives in the function called `simulated_expensive_calculation`.
It takes in the desired intensity of the workout, does some calculations simulated by waiting 2 seconds, and returns that intensity.
In `src/main.rs`:

```rust
use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
```

In `main`, a function gets called that takes some inputs from the frontend.
How they got there is irrelevant to this explanation of closures, so, hardcoding those values it is.
In `src/main.rs`:

```rust
fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}
```

The `generate_workout` function does some logic during which the `simulated_expensive_calculation` is called.

```rust
fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}
```

There are multiple calls to that function, and depending on the logic that gets executed, it gets called 0-2 times.
We want to refactor this so the `simulated_expensive_calculation` only gets called once.

### Refactoring Using Functions

By extracting the result of `simulated_expensive_calculation(intensity)` to a variable it only gets called once.
The trade-off is it also gets called even if it's not needed (in the case the function advises you to rest and stay hydrated).

```rust
fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_result = simulated_expensive_calculation(intensity);

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result);
        println!("Next, do {} situps!", expensive_result);
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result);
        }
    }
}
```

### Refactoring with Closures to Store Code

We'll keep the expensive logic in a closure that's stored in a variable inside the `generate_workout` function.
We can take the entire body of the `simulated_expensive_calculation` function and move it into that closure.

```rust
let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
```

The closure is assigned to the variable called `expensive_closure`.
a pair of vertical pipes `|` enclose the parameters of that closure.
Just like how a pair of parentheses would in a function.
Multiple parameters are also seperated by a comma.
A closure with 2 parameter would have a list like `|param1, param2|`.
After the list of parameters is a body enclosed by curly brackets `{}`, just like in regular functions.
If the closure body only contains a single expression, those brackets are optional.
For example: `|x| x + 1`.

What is stored in `expensive_closure` is the definition of an anonymous function, not the result, it still has to be called.
Calling that logic is done in the same way as calling functions, by appending parentheses with (possible) arguments.

Nearly identical to our first piece of code, but now the expensive logic is inside the `generate_workout` function, in a closure:

```rust
fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure(intensity)
            );
        }
    }
}
```

### Closure Type Inference and Annotation

There aren't any type annotations on that closure, but there were in our first example when it was still a function.
Closures don't require type annotations like `fn` functions do.

This next explanation confused me, so I'm copying it here verbatim:
> Defining this interface rigidly is important for ensuring that everyone agrees on what types of values a function uses and returns. But closures aren’t used in > an exposed interface like this: they’re stored in variables and used without naming them and exposing them to users of our library.

That seems like a complicated way of saying they're used locally, so the types are probably inferred from how the closure is used.

And the next paragraph confirms this: closures are usually short and relevant within a narrow context.
Within this limited context, the compiler is reliably able to inter the types of the parameters and the returned type.

We can annotate types if we want to.

```rust
let expensive_closure = |num: u32| -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};
```

An example of a function, and the same logic in closures:

```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

That last closure look very similar to the fat arrow, implicit return syntax from JavaScript.
Calling the closure is required for `add_one_v3` and `add_one_v4` to compile.
The types will then be inferred from their usage.

Closures have one concrete type inferred for each parameter and the return value.
So the following code, where we first pass a `String`, and then an `i32` will not compile:

```rust
// DOES NOT COMPILE
let example_closure = |x| x;

let s = example_closure(String::from("hello"));
let n = example_closure(5);
```

The resulting compiler error tells you the type of parameter it received the second time the closure was called (`i32`),
did not match the type that was inferred from the first usage (`String`):

```
$ cargo run
   Compiling closure-example v0.1.0 (file:///projects/closure-example)
error[E0308]: mismatched types
 --> src/main.rs:5:29
  |
5 |     let n = example_closure(5);
  |                             ^
  |                             |
  |                             expected struct `std::string::String`, found integer
  |                             help: try using a conversion method: `5.to_string()`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
error: could not compile `closure-example`.

To learn more, run the command again with --verbose.
```

### Storing Closures Using Generic Parameters and the Fn Traits

Back to the workout app.
We can create a struct that holds the closure and the result of calling that closure.
The struct will only execute that close if the resulting value doesn't exist yet.
This pattern is also known as memoization of lazy evaluation.

A struct needs a type for its fields, so that closure has to have a type.
Each close has its own unique anonymous type.
Even two closures with the same signature will have different types.
To define pieces of code like structs, enums, or function parameters that use closures,
we use generics and trait bounds.

There are 3 `Fn` traits, all closures implement at least one of them:
1. `Fn`
2. `FnMut`
3. `FnOnce`

Like the definition of a `Some(T)`,
we add types to the `Fn` trait bound to represent the types of the parameters and the returned value 
that are needed in order for a closure to match this trait bound.
In this case, the single parameter is a `u32`, and so is the return value.
The closure has a trait bound of: `Fn(u32) -> u32`.

```rust
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}
```

The `Cacher` struct holds a `calculation` that satisfies the trait bound `Fn(u32) -> u32`.
That trait bound syntax is fairly similar to function definition syntax, the types of the parameters are in between the parentheses.
The type of the returned value is after the skinny arrow `->`

Functions can implements all three of the `Fn` traits too.
If what we want to accomplish doesn't require using a value from the environment we can use a function rather than a closure.

The `value` field of the struct is an `Option<u32>`.
Before we execute the closure, it will be `None`, only to be replaced by the result of that closure wrapped in a `Some`.
If the code asks for the `result` again, instead of executing the closure, it will return the result that was already there.

The logic to make that behavior happen:

```rust
impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
```

The fields of the `Cacher` struct are kept private, we want it to manage them and only let calling code access the `value()` method.
Fun bonus: when doing this, the compiler warns you about needing a `new` method, otherwise there would be no way to instantiate an instance of `Cacher`.

The first time we call the `value()` method, nothing is there.
The closure in `calculation` gets called, and the resulting value is stored in `value` and returned.
Every subsequent time we call the `value()` method, a `value` already exists and we get that directly.
As a result, the closure is called a maximum of once. Exactly what we wanted.
But there's a downside to this pattern too, only the first invocation matters now, regardless of the passed arguments in subsequent invocations.
In our piece of code, this luckily doesn't matter.

Using this, we can refactor the `generate_workout` function:

```rust
fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}
```

Instead of storing the closure in a variable directly, we use it to instantiate an instance of `Cacher`.
We then call `expensive_result.value(intensity)`.
If we never call it, the closure never gets executed.
If we do call it, it gets executed once, every subsequent call to `.value()` will directly return the cached value.

### Limitations of the Cacher Implementation

There are 2 problems with `Cacher`.
The `Cacher` assumes it always gets the same value for the parameter of `arg` (and the closure is a pure function, meaning `value` would be identical then).

This test would fail:

```rust
// FAILING TEST
#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}
```

`v1` will be `1`, but `v2` will also be `1`.
That is because only the first call to `.value()` invokes the closure.
So only the first invocation matters, the second call to `.value()` could have had a different value and `v2`` would still be `1`.

A possible solution is modifying the `Cacher` to hold a hash map rather than a single value.
The keys of the hash map would correspond to the arguments that are passed into `.value()`.

The second problem is that `Cacher` only accepts closures with one parameter of type `u32`, and have a returned value of type `u32`.
We might want to use the same logic for differently typed closures.
To fix this, we could introduce genericy types parameters.

### Capturing the Environment with Closures

Closures can access variable from the scope they're defined in.

```rust
fn main() {
    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));
}
```

The `equal_to_x` closure uses a variable from outside that closure's body, from the scope that closure is defined in.
We can't do the same thing with functions, they'd refuse to compile.
If we try to access a variable from the outside scope in a regular function,
the compiler gives a nice hint about using closures instead.

When a closure captures a value from its environment, it uses memory to store the values.
In many cases, this is overhead cost we don't want to pay if we only want to execute code that doesn't capture its environment.
Because functions are never allowed to capture their envorinment, using a function in those cases makes sure you never incur that overhead.

Closures can capture values from their environment in three ways, those directly map to the three ways a function can take a parameter.
It can take ownership, borrow mutably, or borrow immutably.
That relates to the three existing `Fn` traits:

`FnOnce` consumes the variables it captures.
To consume the variables, it takes ownership of them, it "moves" them into the closure when it is defined,
and they can no longer be used in the scope they came from.
The "once" part of the name represents the fact represents the fact the closure can't take ownership more than once.

`FnMut` can change the captured values because it mutably borrows them.

`Fn` borrows the values immutably.

The compiler infers which trait to use based on how the closure uses those values.
All closures implement `FnOnce` because they can all be called at least once.
Closures that don't move the captured values also implement `FnMut`.
Closure that don't need mutable access to those values also implement `Fn`.

That means a closure that immutably uses a value from the environment it's defined in will implement all three of those trait bounds,
meaning it can be called almost everywhere.
Something (struct, enum, ...) that requires a `FnOnce` trait bound means closures are limited to one invocation.
A closure that immutably uses a value from its environment satisfies that trait bound and can be used there.

If you want to force the closure to take ownership of the values it uses from its environment, you can use the `move` keyword.
That keyword is placed before the parameter list.
That technique is used a lot when working with threads to move data to a new thread.

The following code example uses the `move` syntax, and does not compile:

```rust
// DOES NOT COMPILE
fn main() {
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}
```

The `x` value moved into the closure when the closure was defined.
The closure now has ownership of `x`

We try to access it again afterwards in the call to `println!`.
We tried to access something that's no longer there, that's a compiler error.
Removing that line makes this example compile.

Most of the time when specifying `Fn` trait bounds,
you can start with `Fn` and the compiler will tell you
when you are doing something inside that closure that requires you to use `FnMut` or `FnOnce` instead.

## 13.2. Processing a Series of Items with Iterators

Iterators let you perform tasks on a sequence of items.
In Rust, iterators are lazy, they have no effect until you consume them.

This snippet creates an iterator of a vector, but doesn't do anthing with it:

```rust
let v1 = vec![1, 2, 3];
let v1_iter = v1.iter();
```

The compiler warns you about this, you should use your iterators.
If you don't, you should intentionally do so by prefixing the variable name with an underscore.
The warning can be turned off by disabling the feature: `#[warn(unused_variables)]`.

The most common way to use an iterator might be the `for` loop.
In contrast to other languages where you have to keep track of the iterating logic yourself,
often by declaring an index, accessing elements in the sequence via that index and incrementing it at the end of an iteration.
Rust iterators handle that logic.
They can be used for more than data structures you can index into.

In the following snipper, the created iterator isn't used until the code hits the `for` loop.
One element of the iterator is used in each iteration of the loop:

```rust
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();

for val in v1_iter {
    println!("Got: {}", val);
}
```

### The Iterator Trait and the next Method

All iterators implement the `Iterator` trait.
The implementation looks like this:

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations omitted
}
```

`type Item` and `Self::Item` are new syntax.
They are defining an associated type, a concept the book will explain deeper in a later chapter.
It means that a piece of code that wants to implement the `Iterator` trait is required to define an `Item` type.
That `Item` type is then returned from a call to the `next` method, wrapped in an `Option`.

The `next` method is the only one that is required to be implemented if you want to implement the `Iterator` trait.
It returns the type of an item in the iterator wrapped in a `Some`.
When there are no more items left, it returns `None`:

```rust
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
```

Note we needed to make the variable that holds iterator mutable.
Each call to `next` changes some internal state to keep track of where the iterator is in the sequence.
A call to `next` _consumes`, or uses up, an item from the iterator.
We didn't need to make the iterator mutable when we used it in a `for` loop before.
The loop took ownership of the iterator, also making it mutable.

The values we got back were immutable references wrapped in a `Some`.
The `iter` method produces an iterator over immutable references (the `Item` type is an immutable reference).
If we want the iterator to take ownership of the items in it and return those owned values, we can call `into_iter` instead.
Similarly, to iterate over mutable references, create the iterator with `iter_mut`.

### Methods that Consume the Iterator

A lot of methods provided by the standard library call `next` internally.

Those methods are called _consuming adaptors_.
Calling them uses up the iterator.
An example is the `sum` method.
It takes ownership of the iterator, 
iterates through each item (by calling `next`),
and adds each item to a running total that it returns when the iterator is used up.

```rust
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}
```

In the snippet, we can no longer use `v1_iter` after the call to `sum`.
`sum` took ownership of the iterator (and used it up) when that _consuming adaptor_ was called on it.

### Methods that Produce Other Iterators

Other methods able to be used on an iterator are known as _iterator adaptors_.
They allow you to change the iterator into a different iterator.
Iterators are lazy, so after that, you still have to consume the resulting iterator.
For example by calling a consuming adaptor on that new iterator.

An example is `map`.
`map` takes a closure with one parameter.
The current item in the iterator takes the place of that parameter.
The result of that closure is then the item in the iterator that is returned from `map`.

```rust
let v1: Vec<i32> = vec![1, 2, 3];

v1.iter().map(|x| x + 1);
```

When we try to compile that snipper,
we get the same warning as before about an unused iterator: `unused std::iter::Map that must be used`.
We need to use, to consume that iterator.
For example, by calling the `collect` method on it and storing the result in a variable.
`collect` consumes an iterator and stores the items of the iterator into a data structure.
Because the compiler can't read your mind, you have to specify a type to `collect` the items of the iterator into.

```rust
let v1: Vec<i32> = vec![1, 2, 3];

let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

assert_eq!(v2, vec![2, 3, 4]);
```

We collect the result of calling `map` on `v1.iter()` into a vector.
The type the vector holds can be inferred, so it's replaced with the catch-all underscore `_`.
Equivalent would by to type `v2` as `Vec<i32>`.

### Using Closures that Capture Their Environment

The `filter` method on iterators is an iterator adaptor.
It returns a different iterator.
`filter` takes a closure, if that closure returns `true`, the current item passed in as parameter is kept in the new iterator.
If that closure returns `false`, the item is not included in the new iterator.

That closure often uses one or more values from its environment.
In the following snippet `shoe_size` is used inside the closure to compare against the size of the current item.

```rust
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_my_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}
```

The `shoes_in_my_size` function takes ownership of a vector of `Shoe` structs.
It also received a `u32` as parameter.
That vector is then turned into an iterator that owns every shoe with `into_iter`.
Next, `filter` is called on that iterator and the closure it receives uses the `u32` that was passed as the function parameter.
The resulting iterator is then turned into a vector with `collect` and is returned from the function.

### Creating Our Own Iterators with the Iterator Trait

You can create iterators from many different collection types from the standard library with `iter`, `into_iter`, and `iter_mut`.
Some examples are: an array, a vector, a hashmap, a hashset, ...
You can also implement the `Iterator` trait yourself, and create your own iterators.
The only required method in that implementation is `next`.
The default implementations for a lot of other methods on an iterator you don't have to implement those yourself.

Let's create our own custom iterator that iterates from 1 to 5.
Starting with a struct called `Counter` we will then implement the `Iterator` trait for.

```rust
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
```

The `count` field will hold a `u32`, which will be used to keep track where we are in the iteration from 1 to 5.
It's a private field, because we only want that field to be indirectly modified.
The `new` function is the only way to create an instance of `Counter`, and it initialized the `count` field to 0.

The implementation of the `Iterator` trait for `Counter` is fairly short:

```rust
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
```

The associated `Item` type is set to `u32`, as the items of this iterator will be `i32`s.
The `next` method adds 1 to the `count` field and returns that.
Because a new `Counter` has `count` initialized to `0`, the first item the `next` method will produce is `Some(1)`.
It returns `None` when the `count` field is `5`.

### Using Our Counter Iterator’s next Method

That's it, we created an iterator and can call `next` on an instance of `Counter`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calling_next_directly() {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }
}
```

### Using Other Iterator Trait Methods

Because the other methods on the `Iterator` trait have default implementations that use the `next` we implemented, they can be used.

The following snippet combines an instance of our `Counter` iterator with another instance of `Counter` that skipped the first item the iterator returned.
It then uses the `map` iterator adaptor to return a different iterator where each item is the product of each pair.
That iterator is then adapted again to a new iterator that only contains items that are cleanly divisable by 3.
Finally, that iterator is consumed by calling the consuming adaptor `sum`.
It takes each item in the iterator, sums them up and returns the final tally:

```rust
#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}
```

Because we skipped one item in the second iterator, `zip` only produces 4 pairs of items.
The hypothetical fifth pair is never included in the iterator `zip` returns.
It would be `(5, None)`, but `zip` returns `None` if either of the items in a pair is `None`.

## 13.3. Improving Our I/O Project

Let's use more features of iterators in the `minigrep` project from chapter 12.

### Removing a clone Using an Iterator

Currently, the `new` associated function on the `Config` struct calls `clone` on the items in the borrowed `args` slice.
That way the `Config` instance owns those values.

```rust
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}
```

We can change the function parameters to take ownership of an iterator containing the arguments instead of borrowing a slice.
We'll use the iterator's `next` function to get at the values instead of indexing into a slice.

#### Using the Returned Iterator Directly

src/main.rs looks like this:

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // --snip--
}
```

We'll directly pass the iterator `env::args()` returns into the `Config::new` function instead of collecting it into a vector first.

```rust
fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // --snip--
}
```

Changing the signature of `new` to match this change:

```rust
 pub fn new(mut args: env::Args) -> Result<Config, &'static str>
```

The documentation for the `env::args` function states it returns an iterator with the type `std::env::Args`.
That's the type of our `args` parameter now.
Note it is marked as mutable, since we'll be mutating it by iterating over it.

#### Using Iterator Trait Methods Instead of Indexing

Next up: Replacing the spots where we indexed into the slice.
By using the `next` method on the iterator.
As a bonus: we replace the logic where we checked the length of the slice before.
The errors are now returned from the function when a call to `next` returns `None` when there should be something there.

```rust
impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}
```

The first value in `env::args` is the name of the program.
We don't need it, so we call `args.next()` to advance the iterator without using the value it returns.

### Making Code Clearer with Iterator Adaptors

As a reminder, the `search` function looks like this:

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
```

Rewriting it to use iterator adaptors, and finally a consuming adaptor,
allows us to avoid the creation of the intermediary `result` variable.

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
```

We create an iterator over all lines in `contents`,
only keep the ones that contain the `query` by calling `filter`.
Finally, we `collect` that iterator into a vector, the returned value for this function.

## 13.4. Comparing Performance: Loops vs. Iterators

Which version of `search` is faster?
The explicit `for` loop, or the one with iterators?

A benchmark shows they are very close, the iterators version is even a tiny bit faster.

```
test bench_search_for  ... bench:  19,620,300 ns/iter (+/- 915,700)
test bench_search_iter ... bench:  19,234,900 ns/iter (+/- 657,200)
```

Iterators, although an abstraction, get compiler down to rougly the same code as if you'd write the lower-level code yourself.
They are on of Rust's _zero-cost abstractions_.
The language allows you to use that abstraction, without paying a performance penalty for using it.

## 14. More about Cargo and Crates.io

`cargo` can do a bunch more than what we've used so far.
This chapter tells us a bit more.
The [documentation](https://doc.rust-lang.org/cargo/) explains all features fully.
It works together with [crates.io](https://crates.io/),
which is similar to JavaScript's npm, and hosts libraries and binaries.

## 14.1. Customizing Builds with Release Profiles

Release profiles tell cargo how to compile your code.
Cargo has two main profiles: `dev`, and `release`.

As the name suggests, `dev` is used while developping,
it's used when you run `cargo build`.
The `release` profile is used when you run `cargo build --release`.

Cargo has some default settings for each, `dev` putting a bigger emphasis on faster builds,
`release` optimizing the runtime performance of the code more, leading to longer build times.

A corresponding `[profile.*]` section in `Cargo.toml` is used to modify these defaults.
If that section is missing completely, all default options will be used.

For example, the following snipper sets the `opt-level` explicitly.
The values specified in `Cargo.toml` will take precedence over the defaults.
Here, the values happen to be the same ones as the default settings for this option.

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

The `opt-level` controls how much optimization will be applied to your code, with a range from 0 to 3.
If for example we would like our code when we run `cargo run` to execute a bit faster,
we could set the `opt-level` of the `dev` profile a bit higher.
This would also lead to increased compile times, that's the trade-off:
In our `Cargo.toml`:

```toml
[profile.dev]
opt-level = 1
```

## 14.2. Publishing a Crate to Crates.io

We've used a package in our code before (by adding it to the `[dependencies]` section in `Cargo.toml`).
We can also publish a package for others to use.
The crate registry at [crates.io](https://crates.io/) distributes the source code of packages.
As a result, it primarily hosts open source code.

### Making Useful Documentation Comments

A comment typically uses two slashes, `//`.
There is a particular kind of comment for documentation, known as a _documentation comment_ that uses three slashes `///`.

Those comments are mostly intended for programmers interested in knowing how to _use_ your crate.
Rust has a tool that will generate HTML documentation.
That tool uses these comments, they will show up on the relevant pages of that generated website.

Documentation comments support markdown notation.
Place them just before the item they document.

For example, the documentation for a function called `add_one` that's part of the public API of the `my_crate` crate:

```rust
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

It describes what the function does,
then the comment starts a section with the heading `Examples`.
That section contains an example usage of the function.

That documentation website that uses these comments can be generated by running `cargo doc`.
It runs the `cargodoc` tool and generates the HTML in the `target/doc` directory.

`cargo doc --open` is a convenient method to immediately open that website in a browser.
It also includes all the documentation for your crate's dependencies.

#### Commonly Used Sections

We used `# Examples` heading to create a section in the docs, these are some other common headings and how they are used:

> - Panics: The scenarios in which the function being documented could panic. Callers of the function who don’t want their programs to panic should make sure they don’t call the function in these situations.
> - Errors: If the function returns a Result, describing the kinds of errors that might occur and what conditions might cause those errors to be returned can be helpful to callers so they can write code to handle the different kinds of errors in different ways.
> - Safety: If the function is unsafe to call (we discuss unsafety in Chapter 19), there should be a section explaining why the function is unsafe and covering the invariants that the function expects callers to uphold.

These sections are not required, use them when it's relevant to the piece of code the documentation comment applies to.

#### Documentation Comments as Tests

THIS FEATURE RULES!

`cargo test` will run the code examples in your documentation as tests.
This guarantees code examples in the documentation continue to work.

Examples in documentation are great.
Examples that no longer work because the code they use change are frustrating.
This ensures the code and the documentation stay in sync.

#### Commenting Contained Items

Another style of doc comment is `//!`.
It contains umbrella documentation about the items that have individual doc comments.
These comments are typically used inside a crate root file (`src/lib.rs`),
or at the top of a module to document the entire module.

To describe the `my_crate` crate that contains the `add_one` function.
A section of `//!` comments would be added at the top of `src/lib.rs`:

```rust
//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
// --snip--
```

Notice there isn't any code between the `//!` lines, and the `///` lines.
The first part describes the entire crate,
the second part is the start of the documentation comments for `add_one`.

When you run `cargo doc --open`,
these `//!` comments in `src/lib.rs` will display on the front page of the documentation for `my_crate`.
Underneath it is the list of public items in the crate.

### Exporting a Convenient Public API with pub use

The structure you decide on to organize your code into modules might not be very convenient for users.
If you organize structs into multiple levels of modules, users would have to bring them into scope with `use` by using that same structure.
They might have to `use my_crate::some_module::another_module::UsefulType;` rather than `use my_crate::UsefulType;`.

Users of your crate are less familiar with the structure of the code, what might make very much sense to you can be confusing to them.

The good new is you don't have to change your internal organization, just to get shorter imports.
You can re-export items to make a public structure that's different from your private structure by using `pub use`.
Re-exporting takes a public item in one location, and makes it public in another location as if it were defined there.
Allowing users of your code to bring that piece into scope via that location.

As an example take this library named `art`.
It has a `kinds`, and a `utils` module on the same level:

```rust
//! # Art
//!
//! A library for modeling artistic concepts.

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --snip--
    }
}
```

The front page for `cargo doc` would list these top-level modules, not the items within those modules.

Another crate that uses this library would need specific `use` statements,
each one containing the modules that hold the pieces of code it uses.

```rust
use art::kinds::PrimaryColor;
use art::utils::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
```

While that's fine, the author of that code had to figure out that `PrimaryColor` is in the `kinds` module,
and `mix` is in the `utils` module.
The module structure is more relevant to the developers working on the `art` crate, than to developers using the `art` crate.

To remove this internal organization from the public API, the `art` crate could re-export these parts at the top level with `pub use`.

```rust
//! # Art
//!
//! A library for modeling artistic concepts.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    // --snip--
}

pub mod utils {
    // --snip--
}
```

The `cargo doc` homepage now has a heading of `Re-exports` that lists all re-exports,
making the `PrimaryColor`, `SecondaryColor`, and `mix` easier to find.

The `art` crate can still use the internal structure.
Users of the `art` crate can now bring these items into scope from the re-exported location:

```rust
use art::mix;
use art::PrimaryColor;

fn main() {
    // --snip--
}
```

### Setting Up a Crates.io Account

[crates.io](https://crates.io/) uses a login via GitHub.
Registering allows you to retreive your API key.
That key is a secret, so keep it, y'know, _secret_.

That API key can be used to login via the `cargo` CLI:

```
cargo login abcdefghijklmnopqrstuvwxyz012345
```

The API token will be stored locally in `~/.cargo/credentials`.

### Adding Metadata to a New Crate

Before publishing a crate, it needs some metadata in the `[package]` section of its `Cargo.toml`.

Your crate needs a unique name, one that's not taken already:

```toml
[package]
name = "guessing_game"
```

A name isn't the only requirement to publish a crate.
Running `cargo publish` will error, and tell you about missing fields.

```
cargo publish
    Updating crates.io index
warning: manifest has no description, license, license-file, documentation, homepage or repository.
See https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata for more info.
--snip--
error: api errors (status 200 OK): missing or empty metadata fields: description, license. Please see https://doc.rust-lang.org/cargo/reference/manifest.html for how to upload metadata
```

A `description` is a sentency or two.
It will appear with the name of your crate in search results.
The `license` takes a license identifier value.
The [Linux Foundation’s Software Package Data Exchange (SPDX)](http://spdx.org/licenses/) has a list of them.
A popular one is: `MIT`.

Using a license that doesn't appear in the SPDX is possible.
You need to place the text of that license in a file, include that file in your project,
then use the `license-file` key to specify the name of that file instead of using the `license` key.

You can specify multiple license identifiers seperated by `OR`.
Many people in the Rust community use a dual license of `MIT OR Apache-2.0`.

A crate ready to publish might have a `Cargo.toml` file like this:

```toml
[package]
name = "guessing_game"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
edition = "2018"
description = "A fun game where you guess what number the computer has chosen."
license = "MIT OR Apache-2.0"

[dependencies]
```

### Publishing to Crates.io

`cargo publish` will work now if the `name` is unique (this example name isn't anymore).

A crate publish is permanent, you can't delete it or overwrite it.
You can, however, publish a new version.

```
cargo publish
    Updating crates.io index
   Packaging guessing_game v0.1.0 (file:///projects/guessing_game)
   Verifying guessing_game v0.1.0 (file:///projects/guessing_game)
   Compiling guessing_game v0.1.0
(file:///projects/guessing_game/target/package/guessing_game-0.1.0)
    Finished dev [unoptimized + debuginfo] target(s) in 0.19s
   Uploading guessing_game v0.1.0 (file:///projects/guessing_game)
```

### Publishing a New Version of an Existing Crate

If you made changes and want to release a new version, change the `version` in `Cargo.toml`.
It is recommended you use [Semantic Versioning rules](http://semver.org/) to determine what that version number should be.
Then run `cargo publish`.

### Removing Versions from Crates.io with cargo yank

You can't remove or overwrite versions of a crate, but you can prevent future projects from adding them as a new dependency.
This is useful when a crate version is broken.
In such scenario's you can _yank_ that version.

It prevents new projects from starting to depend on the yanked version while allowing existing projects to continue using it.
What that means is, yanking doesn't break a `cargo.lock`.
If that version is already there, it can still be used.
Any future generated `cargo.lock` files will not use the yanked version.

To yank a specific version of a crate:

```sh
cargo yank --vers 1.0.1
```

To undo the yank:

```sh
cargo yank --vers 1.0.1 --undo
```

Remember, a yank doesn't delete code.
If you accidentally uploaded secrets, reset them immediately.

## 14.3. Cargo Workspaces

To split a package into multiple crates, cargo has the _workspaces_ feature.
(_in the distance, you hear [@dayhaysoos](https://twitter.com/Dayhaysoos) yelling M O N O R E P O_)

### Creating a Workspace

A workspace is a set of packages that share the same `Cargo.lock` and output directory.

This chapter will guide you through setting up a workspace with one binary and two libraries.
The binary will depend on the libraries.
Create a new folder called `add` and put a `Cargo.toml` file there with these contents:

```toml
[workspace]

members = [
    "adder",
]
```

This file won't have a `[package]` section.
Instead, it has a section called `[workspace]`.
There, the `name` field lists all members of this workspace.
We can add a member to the workspace by listing the path to the package there.
In the example, we added the path to our soon-to-be binary crate called `adder` which will be a direct child in the workspace root, in the `add` directory.

```sh
cargo new adder
```

That package can be built with `cargo build` from the workspace root, the `add` directory.
The workspace has a single `target` directory at the top level, `adder` doesn't have its own target directory.
Even if we ran `cargo build` from inside the `adder` directory, the artefacts would end up in that top level directory.
The crates in a workspace are meant to depend on each other.
Only a single version of a dependency can be used across the different members of a workspace.
They can all use the same dependency, but they should use the same _version_ of that dependency.
Sharing a `target` directory prevents duplicate work when compiling packages.

### Creating the Second Package in the Workspace

Creating the first library crate called `add-one` as direct child of the `add` directory:

```sh
cargo new add-one --lib
```

Doing this produces a helpful message of what to do next:

```sh
cargo new add-one --lib
warning: compiling this new crate may not work due to invalid workspace configuration

current package believes it's in a workspace when it's not:
current:   /home/nicky/projects/scrapyard/book/14_more_about_cargo/add/add-one/Cargo.toml
workspace: /home/nicky/projects/scrapyard/book/14_more_about_cargo/add/Cargo.toml

this may be fixable by adding `add-one` to the `workspace.members` array of the manifest located at: /home/nicky/projects/scrapyard/book/14_more_about_cargo/add/Cargo.toml
Alternatively, to keep it out of the workspace, add the package to the `workspace.exclude` array, or add an empty `[workspace]` table to the package's manifest.s
```

Adding it as member in the workspace root `Cargo.toml`:

```toml
[workspace]

members = [
    "adder",
    "add-one",
]
```

The folder structure now looks like this:

```
├── Cargo.lock
├── Cargo.toml
├── add-one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

Adding a public function to `add/add-one/src/lib.rs`:

```rust
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

To use that function in the binary `adder` crate, we need to add a path depencency to `adder/Cargo.toml`:

```toml
[dependencies]

add-one = { path = "../add-one" }
```

While `add-one` is in the same workspace as `adder`, dependencies need to be made explicit.
Instead of the familiar version number when adding a dependency from crates.io,
the field in `[dependencies]` takes a relative path to the location of the crate in the workspace.

We can now use the public API of `add-one` in `adder`.
In `adder/src/main.rs`:

```rust
use add_one;

fn main() {
    let num = 10;
    println!(
        "Hello, world! {} plus one is {}!",
        num,
        add_one::add_one(num)
    );
}
```

Note, while the crate is named `add-one`, it is imported with `add_one`.

To run the binary crate from the `add` directory, we can specify which package we want to run with the `-p` argument to `cargo run`:

```sh
cargo run -p adder
```

#### Depending on an External Package in a Workspace

If we add an external package like `rand` to both `adder` and `add-one`,
cargo will resolve both of those to one version and add a single version to the top-level `Cargo.lock`.

In `add-one/Cargo.toml`:

```toml
[dependencies]
rand = "0.5.5"
```

The `rand` crate can now be used in the `add-one` crate.
Building the workspace in the `add` directory will download and compile the crate.

If we also want to use `rand` in `adder`, we need to add it to the `[dependencies]` section of its `Cargo.toml`.
No additional copies will be downloaded to the workspace, since it's the same version, and lives in the workspace root `target` directory.

### Adding a Test to a Workspace

Let's add a test in the `add-one` crate:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}
```

Running `cargo test` in the root of the workspace will run all tests in that workspace.
We can run tests for a specific crate by passing the `-p` option to the `cargo test` command:

```sh
cargo test -p add-one
```

If you publish crates in the workspace to crates.io, each crate will need to be published seperately, as if it were a stand-alone crate.

## 14.4. Installing Binaries from Crates.io with cargo install

With `cargo install`, you can install binary crates locally.
You can only install packages that have binary targets this way.
In other words, runnable programs that have a `main.rs` that you can execute.

The binaries installed this way are stored in the installation root's `bin` folder.
If you installed Rust with rustup and didn't customize it, that directory will be `$HOME/.cargo/bin`.
You'll be able to run programs you've installed directly via their names if they are in your `$PATH`, which should happen automatically.

For example, there's a Rust based tool cool `ripgrep` that's an implementation of `grep`:

```sh
$ cargo install ripgrep
    Updating crates.io index
  Downloaded ripgrep v11.0.2
  Downloaded 1 crate (243.3 KB) in 0.88s
  Installing ripgrep v11.0.2
--snip--
   Compiling ripgrep v11.0.2
    Finished release [optimized] target(s) in 3m 10s
  Installing ~/.cargo/bin/rg
   Installed package `ripgrep v11.0.2` (executable `rg`)
```

After installing it, it can be used with the name: `rg`.

## 14.5. Extending Cargo with Custom Commands

If a binary in your `$PATH` is names `cargo-something`, you can run it as if it was a cargo subcommand,
with `cargo something`.

All cargo subcommands, including these custom ones,
are listen when you execute `cargo --list`.


## 15. Smart Pointers

I read the chapters once before I take notes on them, and this one is a big spike in difficulty for me.
Especially since it talks about a totally new concept for me.
That means, if the book repeats itself at points, I'll happily do so too.

A pointer is a concept for a variable that contains an address in memory.
It points to, or refers to, some other data.
In rust, the most used pointer is a reference.
They are indicated by the ampersand symbol `&`, and borrow the value they point to.
They have no special capabilities, they point to a piece of data.

Smart pointers can have those special capabilities.
They are data structures that not only act like a pointer, but also have additional metadata.
They exist in other languages too, originating in C++.

In Rust, an additional difference between references and smart pointers is that while references only borrow data;
smart pointers often own the data they point to.

We've already used some smart pointers without knowing they were smart pointers, such as the `String` and `Vec<T>`.
They both own some memory, and allow you to manipulate it.
They have metadata (like their capacity, their length) and extra capabilities.

Previously, the book explained how the `String` looked like under the hood.
On the stack, it stored a small data structure with a `len`, a `capacity`, and a `ptr`.
That pointer points to a memory address on the heap that holds the data, in the example: `hello`.
![String representation](trpl04-01.svg)

Smart pointers are usually implemented using structs.
They implement the `Deref` and `Drop` traits.
`Deref` allows an instance of the struct (in other words, a smart pointer) to behave like a reference.
That way you can write code that works for references, and smart pointers will work with it.
`Drop` allows you to customize the code that runs when an instance of that struct (a smart pointer) goes out of scope.

Many smart pointer exist, this chapter covers a few important ones:
- `Box<T>` to allocate values on the heap
- `Rc<T>`, a reference counting type that allows for a piece of data to have multiple owners.
- `Ref<T>` and `RefMut<T>`, the wrapper types that are accessed through `RefCell<T>` (a type that enforces borrow rules at runtime instead of compile time)

## 15.1. Using Box to Point to Data on the Heap

The most straightforward smart pointer is a `Box<T>`.
It allows you to store the `Box` on the stack, and the data it is pointing to on the heap.
That's like the explanation of `String` above, minus those extra properties like length and capacity.

Boxes are useful in several different situations:

- When you have a type whose size can't be know at compile time and want to use it in a context that requires an exact size (eg. recursive data types).
- When you have a large amount of data and want to transfer ownership without that entire piece of data being copied when you do (eg. only copying the part on the stack while leaving the data the pointer leads to on the heap intact).
- When you want to own a value and only care that value implements a certain trait (trait objects, like that boxed dynamic error we say before).

### Using a Box<T> to Store Data on the Heap

The `Box<T>` is one of the types that are included in the prelude, you don't have to bring it into scope explicitly.
A value in a box is created by calling the `new` method on `Box`, and passing the value you want to store on the heap:

```rust
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}
```

We defined the variable named `b` to have a value of `Box`.
That `Box` points to the `i32` `5`, that is stored on the heap.
The `println!` prints `5` for the variable `b`, so the data it points to, not the memory address of the pointer.
When the variable `b` goes out of scope, the `Box` is not only popped off the stack, but like any owned variable, the memory it points to on the heap is deallocated.

Putting single values in a box is not that useful.
Often having single values (like that `i32` in the example) stored on the stack is fine.

### Enabling Recursive Types with Boxes

Rust needs to know how much space a type takes up at compile time.
For recursive types (types where a part of that type, is itself), this is a problem.
That nesting could theoretically continue infinitely.
Rust doesn't know how much space a value of that recursive type needs.
Boxes have a known size (the pointer they hold).
The data that has an unknown size at compile time is stored on the heap, but the part on the stack has a known size.
By inserting a box into a recursive type definition, that size is no longer unknown.

A data type known as the cons list is very popular in functional programming languages.
It's an example of a recursive type.

#### More Information About the Cons List

The `cons` function (short for construct function), constructs a new pair from its two arguments,
usually those are: a value, and another pair.

"to cons x onto y` means to create a new pair where x is the value, and y is the pair.

Each item in a cons list contains two elements: the value of the current item, and the next item.
Usually, that next item is another pair with the same structure,
but at the very end of the cons list, that next item is `Nil`.

`Nil` is the canonical name to denote the end of the recursion.
It's not the same as null in other languages, it's still a value!

The cons list isn't commonly used in Rust.
Most of the time, a `Vec<T>` is the better choice if you have a list of values.

To me, eventhough I have no experience in Lisp, or other functional languages, this seems very familiar.
It's a linked list.

A cons list that holds `i32` values could be represented as an enum like the snippet below.
That code won't compile yet, because the `List` contains itself and as a result can have an infinite size:

```rust
// DOES NOT COMPILE
enum List {
    Cons(i32, List),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
```

A variant of the `List` enum is either:
1. A pair, where the first item is an `i32`, and the second item is another `List`.
2. `Nil`

The list we create stores `1, 2, 3`.

The first `Cons` holds `1`, and another `Cons`.
That second `Cons` holds `2`, and a next `Cons`.
That third `Cons` holds `3`, and `Nil`.

Trying to compile that code, we get the following errors:

```text
cargo run
   Compiling code_examples v0.1.0 (/home/nicky/projects/scrapyard/book/15_smart_pointers/code_examples)
error[E0072]: recursive type `List` has infinite size
 --> src/main.rs:3:1
  |
3 | enum List {
  | ^^^^^^^^^ recursive type has infinite size
4 |     Cons(i32, List),
  |               ---- recursive without indirection
  |
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to make `List` representable
  |
4 |     Cons(i32, Box<List>),
  |               ^^^^    ^

error[E0391]: cycle detected when computing drop-check constraints for `List`
 --> src/main.rs:3:1
  |
3 | enum List {
  | ^^^^^^^^^
  |
  = note: ...which again requires computing drop-check constraints for `List`, completing the cycle
  = note: cycle used when computing dropck types for `Canonical { max_universe: U0, variables: [], value: ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: UserFacing }, value: List } }`

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0072, E0391.
For more information about an error, try `rustc --explain E0072`.
error: could not compile `code_examples`

To learn more, run the command again with --verbose.
```

The compiler tells us the enum `List` is a recursive type that has an infinite size.
A variant of that enum can contain that same enum.
Rust can't figure out how much space it needs to store a `List` value, because it may store another `List` value, which may contain another `List` value, and the loop goes on.

#### Computing the Size of a Non-Recursive Type

We previously used a `Message` enum:

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

Rust goes through each variant to determine which variant needs the most space.
Here, `Quit` is a unit struct, it doesn't need any space.
`Move` stores two `i32` values, and so forth.
The most space a `Message` variant will need is the space required to store the largest variant.

When Rust tries to determine how much space a recursive type like `List` needs, it can't.
A `Cons` needs as much space as an `i32` needs, plus the space a `List` needs.
If that last `List` is the `Nil` variant, the size is known, but if it is the `Cons` variant, the cycle continues, over and over.

![the size required for `Cons` that holds a `List` directly](trpl15-01.svg)

#### Using Box<T> to Get a Recursive Type with a Known Size

That error included a useful hint:

```
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to make `List` representable
```

Indirection is a weird, foreign word to me.
What they mean is instead of storing the value directly, store the value indirectly by storing a pointer to that value instead.
That value will still be unknown in size, but the pointer is known in size (and corresponds to the type of machine, `64` bits for a `64`bit machine).

A `Box<T>` is a pointer, so Rust knows exactly how much space is needed for a `Box<T>`, the size of the pointer.
The size of the data that pointer points to can be unknown, since it is stored on the heap.

This means we can put a `Box<T>` inside of the `Cons` variant.
The `Box` will point to the next `List` value.
The size of that `Box` will always be the same, regardless of the size of the data it's pointing to.

Changing our definition of `List` to make the `Cons` variant hold an `i32` and a boxed `List`:

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
```

When Rust tries to figure out the maximum size a `List` can hold by going over all variants now, it can.
- The `Nil` variant stores no values.
- The `Cons` variant stores an `i32` and a `Box` (a pointer of size `usize`).

![the size required for `Cons` that holds a boxed `List`](trpl15-01.svg)

Note that `Nil` also needs a `Box` around it!
It is inside a `Cons(i32, Box<List>)`.
The `Nil` is a variant of `List`, the `Cons` it is in stores a boxed variant of `List`.

Boxes don't do anything other than provide that indirection by storing a pointer (of a known size) to some data (of possibly unknown size).
They don't have extra capabilities like other smart pointers.
It is a smart pointer because it implements the `Deref`, and `Drop` traits.
`Deref` allows boxes to be treated like references.
When a `Box<T>` value goes out of scope, the data on the heap it points to is deallocated too because of the `Drop` implementation.

## 15.2. Treating Smart Pointers Like Regular References with the Deref Trait

The `Deref` trait allows you to customize the behavior of the dereference operator, `*`.
You can implement the `Deref` trait so that you can treat the smart pointer (usually a struct) as a regular reference,
not as a struct with a reference in a field.
Then you can use code that works with regular references with those smart pointers.

### Following the Pointer to the Value with the Dereference Operator

A regular reference is a pointer.
It _points_ to, or _refers_ to, a value that is stored somewhere else.
You can think of it like an arrow to that value.

The dereference operator follows that arrow to the value it is pointing to.

```rust
fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

In this example, `x` holds the number `5` (an `i32` value) directly.
`y` hold a reference to `x`.
The first `assert_eq` confirms that `5` is equal to the value stored in `x`.
In the second `assert_eq`, we dereference `y` first, we follow the arrow to the value it is pointing to.
That value is equal to `5`.
We couldn't directly compare `5` to `y` since they are different types.
Trying to do that would result in a compiler error:

```
error[E0277]: can't compare `{integer}` with `&{integer}`
  --> src/main.rs:19:5
   |
19 |     assert_eq!(5, y);
   |     ^^^^^^^^^^^^^^^^^ no implementation for `{integer} == &{integer}`
   |
```
The value `y` is pointing to and `5` are the same type, those can be compared.
To get to the value `y` is pointing at, we dereference it with the star operator, `*`.

### Using Box<T> Like a Reference

We can put the value of `x` in a `Box` and store that in `y`.
`y` can still be dereferenced with the `*`.

```rust
fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

`y` is now a `Box` pointing to the copied value of `x`.
Because the `Box` implements the `Deref` trait, we can dereference the `Box(x)` to follow the `Box`'s pointer to the value it points to.

### Defining Our Own Smart Pointer

We'll build our own smart pointer that's similar to the `Box<T>` the standard library provides.

The `Box<T>` type is a tuple struct with one element.
We'll define our own `MyBox<T>` in the same way:

```rust
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
```

The `<T>` is a generic parameter, because we want our `MyBox` type to be able to hold multiple types,
those will then take the place of `T` when it is used.
The `MyBox::new` associated function returns an instance of the `MyBox` tuple struct that holds the value that was passed to that function.

If we try to use the code snippet where we used a `Box` above, it will fail.
Rust doesn't know how to dereference a `MyBox`:

```rust
// DOES NOT COMPILE
fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

When we try to compile that code, Rust lets us know it does not know how to dereference the `MyBox`.
The compiler shows us the exact location we tried to dereference it,
at the point we tried to use the `*` operator.

```
error[E0614]: type `MyBox<{integer}>` cannot be dereferenced
  --> src/main.rs:33:19
   |
33 |     assert_eq!(5, *y);
   |                   ^^
```

### Treating a Type Like a Reference by Implementing the Deref Trait

To implement a trait, we need to provide implementations for that trait's required methods.
For `Deref`, that means implementing the `deref` method.
It take a shared (so immutable) reference to `self`, and returns a shared reference to `T`.
Translated to our `MyBox`: it takes a reference to the `MyBox` instance, and returns a reference to the thing that `MyBox` points to.
So it turns a reference into another reference.

```rust
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
```

Like when we talked about the `Iterator` trait, that `type Target` syntax is an associated type.
It is used in the `Deref` trait implementation.

The body of the method returns a reference to the first element of that tuple struct: `&self.0`.
At that point, the compiler knows how to dereference that regular reference by following that pointer to a value.

With that addition, tha piece of code where we tried to dereference a `MyBox` with the star operator, `*`, works!

The compiler can only dereference `&` references.
The `Deref` trait has a `deref` method.
That gives the compiler the ability to turn a value that implements `Deref` into a regular reference by calling `deref` on it.
Then, the compiler can dereference that `&` referene.

When we wrote `*y` in our code, what happened under the hood was:
```rust
*(y.deref())
```

The compiler first turned the `MyBox<i32>` into an `&i32` by calling `deref` on `y`.
Then, the `*` operator makes the compiler follow that reference to the value it is pointing at, which is an `i32`.

This behavior happens automatically and is the reason we can use smart pointers in code written for regular pointers.

`deref` returns a reference to `T`, instead of `T` itself because of the ownership system.
If it returned `T`, the value would be moved out of `self` (here: out of `MyBox`).
We don't want to take ownership of the value in most cases where we use the dereference operator.

In my opinion, the `deref` method is incredibly confusing naming.
`deref` doesn't dereference at all, it returns a reference.
The compiler knows how to dereference _that_ reference.

### Implicit Deref Coercions with Functions and Methods

Deref coercion is a convenience Rust performs on arguments to functions.
It works on types that implement the `Deref` trait.
Deref coercion converts a type that implements `Deref` into a reference to another type.
(it does this by calling the `deref` method)

For example, deref conversion can convert `&String` to `&str`.
`String` implements the `Deref` trait, their `Target` type is `str`.
Deref coercion happens automatically when we pass a reference to a type into a function that doesn't match that type in its signature.
A sequence of calls to `deref` converts the type that was passed in, to the type the function signature requires.
If after that, the type still doesn't match you'll get a compiler error.

Using deref conversion with our `MyBox<T>` type:

```rust
// --snip--
fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
```

This call to `hello` succeeds.
We pass a reference to a `MyBox<String>` to the `hello` function,
eventhough its signature expects a `&str`.

- Because we implemented `Deref` for `MyBox<T>`, Rust calls the `deref` method.
  - This turns the `&MyBox<String>` into a `&String`.
- This still isn't the type the function signature expects.
- Because `Deref` is implemented for `String`, Rust calls the `deref` method.
  - This turns the `&String` into a `&str`
- This is the type the function signature expects.

This process happens automatically.
If it didn't we'd have to convert our `MyBox<String>` to an `&str` manually.
This would be the code we would write then:

```rust
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
}
```

The `(*m)` calls the `deref` method on `MyBox<T>`, and then the compilers follows the reference that method returns,
turning the `MyBox<String>` into a `String`.
We then take a slice of the full string with `&` and `[..]` to turn that `String` into an `&str`.
So this code is equivalent, but much harder to read.

The number of times `Deref::deref` needs to be called is resolved at compile time.
There is no runtime cost for using deref conversion (since you'd manually have to convert it otherwise).

### How Deref Coercion Interacts with Mutability

- The `Deref` trait overrides the behavior of the `*` operator on immutable references.
- The `DerefMut` trait overrides the behavior of the `*` operator on mutable references.

> Rust does deref coercion when it finds types and trait implementations in three cases:

> - From &T to &U when T: Deref<Target=U>
> - From &mut T to &mut U when T: DerefMut<Target=U>
> - From &mut T to &U when T: Deref<Target=U>

- Rust can coerce an immutable reference to a type that implements `Deref` to an immutable reference of the `Target` type.
- Rust can coerce a mutable reference to a type that implements `DerefMut` to a mutable reference of the `Target` type.
- Rust can coerce a mutable reference to a type that implements `Deref` to an immutable reference of the `Target` type.

The reverse of that last one is not possible.
An immutable reference will never coerce to a mutable reference.
This is because of the borrowing rules.
A mutable reference must be the only reference to that data.

Converting a mutable reference to an other mutable reference is always ok.
Converting a mutable reference to an immutable reference is always ok.
Converting an immutable reference to an other immutable reference is always ok.
Converting an immutable reference to a mutable reference is _not_,
that would require the initial immutable reference to be the only one that exist, and that can not be guaranteed by the compiler.

## 15.3. Running Code on Cleanup with the Drop Trait

The `Drop` trait is almost always implemented on smart pointers.
It is included in the prelude, so you don't have to bring it into scope first.
It lets you customize the code that executes when an owner of a value goes out of scope.
It can be implemented on any type, and is used to release resources like network connections, files, and memory usage.

An example usage: when the owner of a `Box<T>` goes out of scope,
not only is the `Box` popped off the stack, the `T` that uses memory on the heap is deallocated.

In some languages, the programmer must call code to free memory or resources. (eg. `free` in C)
In Rust, the compiler will automatically insert the bit of code that needs to run when a value goes out of scope.
You as the programmer can customize that bit of code by implementing the `Drop` trait.

The `Drop` trait requires you to implement a method called `drop`.
That method takes a mutable reference to `self` and returns the unit type, the empty tuple, `()`.

Let's implement this trait for our own struct, `CustomSmartPointer`:

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}
```

Executing this file allows us to see when the code in the `drop` function is called.
The `drop` function prints a line to the console when the owner (`self`) goes out of scope.
In this example the following lines are printed, in this order:

```
CustomSmartPointers created.
Dropping CustomSmartPointer with data `other stuff`!
Dropping CustomSmartPointer with data `my stuff`!
```

First, the instances of `CustomSmartPointer` are created, this prints nothing to the console.
The line `println!("CustomSmartPointers created.");` is reached and prints to the console.
As the `main` function ends, the `d`, and `c` variables go out of scope and Rust automatically cals the `drop` method.
Variables are dropped in reverse order of their creation,
`d` is dropped before `c`.

### Dropping a Value Early with std::mem::drop

It's not straightforward to disable the automatic calling of `drop`.
Occasionally, when you want to clean up a value early you might want to do that.
Calling `drop` again automatically after manually calling `drop` could cause unwanted situations. (eg. you try to free the same memory twice.)
This is known as -real bad-, alternatively it's called a -big no no-.

An example where you might want to call `drop` early is when using smart pointers that manage locks.

Sidenote: I didn't know what a lock was in this context.
It's a think that manages exclusive access to a resource,
if one variable holds the lock, no other variables can manipulate that resource.
The variable that holds the lock can release that lock.
Then an other variable can acquire that lock,
and manipulate that resource.

If the `drop` method releases the lock so other code in the same scope can acquire that lock.
You might want to force the `drop` method to be called before that scope ends.
Rust doesn't let you call the `Drop` trait's `drop` method manually.
Trying to do so causes a compiler error: `explicit use of destructor method`.
You have to call `std::mem::drop` from the standard library.

To call `std::mem::drop`, pass the value you want to force to be dropped as an argument.
The function is in the prelude, which means calling `drop` in your code calls `std::mem::drop`.
Calling that function will call the `drop` method you implemented in the `Drop` trait to be called at that time,
but not again when the thing you passed as an argument to `std::mem::drop` goes out of scope

```rust
fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}
```

This prints:

```
CustomSmartPointer created.
Dropping CustomSmartPointer with data `some data`!
CustomSmartPointer dropped before the end of main.
```

An instance of `CustomSmartPointer` is created and is owned by `c`.
The line `println!("CustomSmartPointer created.");` is reached and gets printed to the console.
`drop(c);` is called, this executes the `drop` method we implemented in the `Drop` trait.
That prints `Dropping CustomSmartPointer with data 'some data'!` to the console.
The `main` function continues executing and the `println!("CustomSmartPointer dropped before the end of main.");` is reached.
At the end of `main`, when `c` goes out of scope, the implementation of `drop` we specified on the `Drop` trait is not called again.

## 15.4. Rc, the Reference Counted Smart Pointer

Remember when this book told you a value can have at most one owner?
Yeah, nah.

Most of the time, you know exactly which variable owns a given value.
There are cases when a single value might have multiple owners.
For example, a node in graph data structures.
Multiple edges might point to the same node, that node is then conceptually owned by all those edges.
A node shouldn't be cleaned up unless it has no edges pointing to it.

Rust has a type called `Rc<T>` that enables multiple ownership.
Rc is an abbreviation of reference counting.
`Rc<T>` keeps track of the number of references there are to `T`.
If there are zero left, the value can be cleaned up. (if the value would be cleaned up sooner, the existing references would become invalid.)

An `Rc<T>` can be thought of as a light with a presence detector.
When someone walks in the room, the light turns on.
As long as at least one person remains in that room, the light stays on.
It doesn't matter how many people join, or leave.
Only when the last person leaves, the light turns off.

`Rc<T>` allocates data on the heap.
We use it when multiple parts of our program read that data,
but we can't determine at compile time which part will finish using the data last.
If we knew that, we would make that part the owner of the data,
and the normal ownership rules would be enforced.
`Rc<T>` is only for use in single-threaded scenarios.
(It doesn't implement the `Sync`, and `Send` traits. More about those in the next chapter)

### Using Rc<T> to Share Data

Expanding on our cons list example.
We want to create two lists that share ownership of a third list.

![two cons lists share ownership of a thrid](trpl15-03.svg)

- List `a` contains: `5, 10`.
- List `b` contains: `3, 5, 10`.
- List `c` contains: `4, 5, 10`.

List `b` and `c` share ownership of the items in list `a`.

Trying to implement this using the definition of `List` that uses `Box<T>` won't work:

```rust
// DOES NOT COMPILE
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a));
}
```

The compiler error is: `error[E0382]: use of moved value: `a``.
The `Cons` variants own the data they hold.
When we create `b`, we move ownership of `a` into `b`.
When we try to create `c`, the `a` we try to use is no longer there.

It's possible to build a recursive data type like our cons list with references.
(note that then, the data would all be stored on the stack)
To do that, we'd need to add lifetimes:

```rust
enum List<'a> {
    Cons(u32, &'a List<'a>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let list = Cons(1, &Cons(2, &Cons(3, &Nil)));

    println!("{:?}", list);
}
```

The lifetime requirements make it incredibly restrictive and hard to use.
The compiler even has to do a trick to make it compile in the first place.
By specifying lifetime parameters in this way, we specify every element in the list will live at least as long as the entire list.
That would be a problem for `Cons(3, &Nil)`.
In older version of Rust, this wouldn't compile because of that problem.
The `Nil` would be dropped before the `list` variable could be built, leading to a reference that points at nothing.
References always need to be valid, so that's not allowed, and the code would fail to compile.
However, in newer versions, that reference to the constant `Nil` value is promoted to a `'static` lifetime.
This means the value is kept in a special part of the program's memory rather than treated as a temporary value.
That reference is never invalid, so the code compiles.

If that `Nil` was a non-constant expression, like the output of a function, this wouldn't work.
The same problem happens, and we get the `temporary value dropped while borrowed` error.

Information gathered from many places, including [Prince Wilson](https://twitter.com/maxcell) and [Chris Biscardi](https://twitter.com/chrisbiscardi) on Discord, [/u/zeta12ti](https://www.reddit.com/r/learnrust/comments/hehl0q/trouble_understanding_how_the_book_explains_rct/fvrv8dr/) on Reddit, and [Brent Kerby](https://stackoverflow.com/a/62368584) on StackOverflow.

Instead of using references, we change our `List` to use `Rc<T>` in place of `Box<T>`.
It also stores `T` on the heap, only keeping a small part of the `Rc` on the stack, which has a known size at compile time.
Every instance of an `Rc` owns that data, and it is only dropped once no owners are left.

Each `Cons` variant will hold a value and an `Rc` pointing to a `List`.

Back to the code snippet that tried to share one list as a part of two other lists.
When we create those two lists (`b`, and `c`) that contain the first one (`a`).
When creating `b`, instead of transferring ownership of `a`, we clone the `Rc<List>` that `a` is holding.
That increases the number of references pointing to that data from one to two and lets `a` and `b` both own the data in that `Rc<List>`.
When creating `c`, we do the same thing, increasing the number of references from two to three.

Every call to `Rc::clone`, the reference count to the `T` of the `Rc<T>` goes up by one.
Every time an `Rc<T>` goes out of scope, that reference count goes down by one.
If the reference count reaches zero, the `T` is dropped and the memory it occupied on the heap is deallocated.

```rust
use crate::List::{Cons, Nil};
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}
```

`Rc` implements the `Clone` trait.
That trait implements the `clone` method.
Inside that method is the logic to increment that reference count.
It doesn't clone the data the `Rc` that `clone` takes as argument points to.
The returned `Rc` will also point to that same data.
It takes a reference to `self`, we when we call `Rc::clone`, we pass a reference to `a`.

Note: `Rc` also implements the `Drop` trait.
the logic to decrement that count is done in the `drop` method.

We could have called `a.clone()` instead of `Rc::clone(&a)`.
It would call the same `clone` method in the `Clone` trait, but it's convention to call it via `Rc::clone`.
That also visually distinguishes `Rc::clone` from regular calls to `clone`, which make a deep copy, and can have a much larger performance impact.

### Cloning an Rc<T> Increases the Reference Count

Let's visualize the reference count to a value by printing out the count as `Rc`s are created, cloned, and dropped.

```rust
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
```

We get the current reference count by calling the `Rc::strong_count` function.
It's named `strong_count` because `Rc` also has a `weak_count`.

The printed results:

```
count after creating a = 1
count after creating b = 2
count after creating c = 3
count after c goes out of scope = 2
```

After we create `a`, the reference count to the data that `Rc` holds is 1.
`b` is created, we call `clone` on `a` and that reference count is increased by 1.
We enter a new scope, denoted by curly bois `{}` where we call `clone` on `a` again, making the count go up to 3.
When that block ends, `drop` is called on `c`, and the reference count goes down by 1.
When the `main` function scope ends, `drop` is called on variables in the reverse order they were created.
So `drop` is called on `b` first, and the count goes down to 1.
Immediately after, `a` is dropped, the count goes to 0, and the value `a` held is dropped from the heap.

`Rc<T>` allows you to share data between multiple parts of your program for reading only.
It allows multiple immutable references.
If those references were mutable, they would break the borrow rules.
Mutable references are also called exclusive references, because if a reference is mutable, the compiler guarantees it's the only one.
The very next chapter describes a way to "break" this rule yet again, because apparently that's the overarching theme of smart pointers /s.
The next chapter talks about `RefCell<T>`, a type that can be used to mutate the data it holds, even when used together with `Rc`.

## 15.5. RefCell and the Interior Mutability Pattern

Interior mutability is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data.
That goes directly against the borrow rules.
To mutate that data, the patterns uses `unsafe` code to bend Rust's usual rules and do it anyway.
An `unsafe` block is equivalent to telling the compiler "Don't check this, I know what I'm doing, it's fine."
That block is inside a data structure.
That data structure exposes a safe API, and the outer type is still immutable.

We can use types that use the interior mutability pattern when we can ensure the borrowing rules will be followed at runtime,
even though the compiler couldn't guarantee that.

### Enforcing Borrowing Rules at Runtime with RefCell<T>

The `RefCell<T>` represents single ownership of the data it holds (the `T`).

As a reminder of the borrow rules:

- References must always be valid
- You can either have one mutable reference, or any number of immutable references

With regular references and `Box<T>`, the borrowing rules are enforced at compile time.
With `RefCell<T>`, those rules are enforced at runtime.

In the first case, at compile time, if you break the rules you get a compiler error.
With `RefCell<T>`, at runtime, if you break the rules your program will panic.

Checking those rules at compile time has several advantages.
You catch errors sooner in the development process,
and there is no need to check them during runtime, so there is no runtime performance cost.

Sometimes, it's impossible for the Rust compiler to guarantee these rules will be followed during runtime.
The compiler is inherently conservative, it might disallow scenarios that follow these rules during runtime, because it can't without a doubt confirm they will be followed.

The `RefCell<T>` type is useful when you're sure your code will follow the borrowing rules at runtime, but they can't be guaranteed at compile time.

`RefCell` is only for use in single-threaded scenarios.

> Here is a recap of the reasons to choose Box<T>, Rc<T>, or RefCell<T>:
> 
> - Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
> - Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at compile time; RefCell<T> allows immutable > or mutable borrows checked at runtime.
> - Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.

One I'd like to add to that list is: `Box` and `Rc` store the `T` on the heap, `RefCell` stores it on the stack.

### Interior Mutability: A Mutable Borrow to an Immutable Value

Because of the borrowing rules, you can not create a mutable reference to an immutable variable.
If you wish to mutate the reference, you have to mark the variable as `mut`.

```rust
// DOES NOT COMPILE
fn main() {
    let x = 5;
    let y = &mut x;
}
```

The accompanying compiler error:

```
error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
```

There are situations where it would be useful for a variable to mutate itself in methods, but appear immutable to other code.
Using `RefCell<T>` is one way to get that ability.
`RefCell` doesn't avoid the borrowing rules, instead of being checked at compile time, they are enforced at runtime.
If your code breaks them, it will panic and exit.

#### A Use Case for Interior Mutability: Mock Objects

A test double as a type used in place of another type during testing.
Mock doubles are specific types of test doubles.
They record what happens during a test.
Using the information the mock double recorded, you can then check if certain actions happened correctly.

Rust doesn't have objects in the same sense other languages do or have mock object functionality in the standard library.
You can create a struct that can serve the same purpose as a mock object.

This chapter will create (and test) a library that tracks a value against a maximum,
and sends messages based on the proximity of that value to that maximum.
The library needs something that implements a trait it defined called `Messenger`.

The library code `src/lib.rs`:

```rust
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}
```

The `Messenger` trait requires the implementation of the `send` method.
That method takes an immutable `self`, and the text of the message.
This is the trait our mock object will need to implement.

We want to test the behavior of the `set_value` method on a `LimitTracker` struct.
The method takes in a `value`, but doesn't return anything.
That's why we want a mock object, to confirm it does the correct thing when we call it.

> We want to be able to say that if we create a `LimitTracker` with something that implements the `Messenger` trait and a particular value for `max`, when we pass different numbers for `value`, the messenger is told to send the appropriate messages.

To test this behavior, we can:
- create a new instance of the mock object
- create a `LimitTracker` that uses the mock object
- call `set_value` on the `LimitTracker` instance
- check that the mock object has the message we expect

The following snippet shows an attempt to implement these steps that won't compile yet:

```rust
// DOES NOT COMPILE
#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        sent_messages: Vec<String>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: vec![],
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.len(), 1);
    }
}
```

That code has a `MockMessenger` struct with a `sent_messages` field.
That field contains a `Vec<String>` to keep track of the messages it's told to send.
We implement the `Messenger` trait for `MockMessenger`.
That way the `messenger` field on the `LimitTracker` we use in our test can contain an instance of `MockMessenger`.
Inside the `send` method, we take the `message` and `push` it into the `sent_messages` vector.
(this is already a problem since that vector isn't marked as mutable, more on that in a bit)

In the test in the snippet, we test what happens when the `LimitTracker` is told to set its `value` to something that is more than 75 percent of its `max.
The `LimitTracker` we use in the test is created with a reference to our `MockMessenger` instance and a `max` of 100.
We then call `set_value(80)` on our instance of `LimitTracker`.
What we expect is that the new instance of `MockMessenger` now contains a single message in its `sent_messages` vector.

Try to compile reveals the following error:

```
cargo test
   Compiling limit-tracker v0.1.0 (file:///projects/limit-tracker)
error[E0596]: cannot borrow `self.sent_messages` as mutable, as it is behind a `&` reference
  --> src/lib.rs:58:13
   |
57 |         fn send(&self, message: &str) {
   |                 ----- help: consider changing this to be a mutable reference: `&mut self`
58 |             self.sent_messages.push(String::from(message));
   |             ^^^^^^^^^^^^^^^^^^ `self` is a `&` reference, so the data it refers to cannot be borrowed as mutable

error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
error: could not compile `limit-tracker`.

To learn more, run the command again with --verbose.
```

We can't modify the `MockMessenger`, because the `send` method takes an immutable reference to `self`.
We can't implement the suggestion of the compiler to take a mutable reference instead,
because of the signature the `Messenger` trait expects the `send` method to have.

In this situation, interior mutability can help.
We'll store the `sent_messages` vector within a `RefCell`.
Then the `send` method will be able to modify that vector.

The changes test using `RefCell<T>`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        // --snip--

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
```

`sent_messages` is now a `RefCell<Vec<String>>` instead of a `Vec<String>`.
The signature of the `send` method doesn't change.
We call the `borrow_mut` method on the `RefCell<Vec<String>>` to get a mutable reference to the inner `Vec<String>`.
Then we call `push` on that mutable reference.

In the assertion, we now call `borrow` on the `RefCell<Vec<String>>` to get an immutable reference to the inner `Vec<String>`.
During this test, the borrow rules were still respected.
There was one mutable reference (inside the `send` method).
That reference was dropped before any other reference was taken.

#### Keeping Track of Borrows at Runtime with RefCell<T>

When creating immutable references, we use the `&` syntax.
When creating mutable references, we use the `&mut` syntax.
With `RefCell<T>`, we use the `borrow` and `borrow_mut` methods.
They are part of the safe API of `RefCell<T>`.

- The `borrow` method returns a `Ref<T>`, a smart pointer.
- The `borrow_mut` method return a `RefMut<T>`, a smart pointer.

Both types are smart pointers that implement `Deref`.
They can be treated as regular references.

The `RefCell<T>` keeps track of how many `Ref<T>` and `RefMut<T>` exist at any given point during runtime.
Parallel to the borrowing rules, there can be at most one `RefMut`, or multiple `Ref`s, not both.
A call to the `borrow` method on a `RefCell` increases the count of how many immutable borrows are active.
When the `Ref` is dropped, that count decreases by one.
The same thing happens for `borrow_mut` and the `RefMut` it returns.
If the borrow rules are broken at any point, the program panics and exits.

The following snippet creates two immutable borrows of the same value at the same time.
While this code compiles, the `RefCell` will notice this breaks the rules at runtime, panic, and exit:

```rust
// THIS CODE PANICS AT RUNTIME
impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        let mut one_borrow = self.sent_messages.borrow_mut();
        let mut two_borrow = self.sent_messages.borrow_mut();

        one_borrow.push(String::from(message));
        two_borrow.push(String::from(message));
    }
}
```

This creates on `RefMut` of `sent_messages` which is fine.
Before that mutable reference is dropped, it tries to create a second mutable reference.
This is not allowed, and the program panics.

Running `carge test` notifies us of this:

```
cargo test
   Compiling limit-tracker v0.1.0 (file:///projects/limit-tracker)
    Finished test [unoptimized + debuginfo] target(s) in 0.91s
     Running target/debug/deps/limit_tracker-d1b2637139dca6ca

running 1 test
test tests::it_sends_an_over_75_percent_warning_message ... FAILED

failures:

---- tests::it_sends_an_over_75_percent_warning_message stdout ----
thread 'main' panicked at 'already borrowed: BorrowMutError', src/libcore/result.rs:1188:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.


failures:
    tests::it_sends_an_over_75_percent_warning_message

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out

error: test failed, to rerun pass '--lib'
```

The reason the test failed is because the code panicked.
With the message: `already borrowed: BorrowMutError`.

As stated before, this runtime checking comes at a small runtime performance cost.
Violations of the borrow rules are also caught later in development, during runtime.
It is therefore advised to use the regular borrow checking at compile time whenever possible,
and only use `RefCell` when you know those rules won't be broken but the compiler can't verify that.

Using `RefCell<T>` made it possible to write to a field of a mock object while using it in a context where only immutable variables are allowed.

### Having Multiple Owners of Mutable Data by Combining Rc<T> and RefCell<T>

`RefCell<T>` is often combined with `Rc<T>`.
The `Rc` type allows a value to have multiple owners, but it only gives immutable access to that value.
Having an `Rc`, that holds a `RefCell`, that lets you have a value with multiple owners that can all mutate that data (just not at the same time).

Recall our cons list wher we used `Rc<T>`.
Because `Rc` holds immutable values, we can't change any of the values once we've created them.
Let's wrap the `i32` values in a `RefCell` to allow mutating them.

```rust
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
```

We create a value that's an `Rc<RefCell<i32>>` and store it in a variable called `value`.
Then we create the cons list that will be used by other cons lists later on.
The `a` variable holds a `Cons` variant inside of an `Rc` where an `Rc::clone` of `value` is the first item in the pair and the second value is an `Rc` with `Nil` inside.
The `b` and `c` both refer to the value in `a` by using `Rc::clone(&a)`.

After the lists are created, we add 10 to the value in `value`.
We do this by calling `borrow_mut` on the `RefCell`.
In the code, we call it directly on `value` with `value.borrow_mut()`.
That `value` isn't a `RefCell`, it's an `Rc`!
Because of the automatic dereferencing feature we discussed earlier,
`deref` is automatically called on the `Rc`, which returns the `RefCell`.
The `borrow_mut` returns a `RefMut` smart pointer.
We dereference that smart pointer with the star operator `*` to get to the underlying value,
then we mutate that value by adding 10 to it.

After that `a`, `b`, and `c` have the modified value:

```
cargo run
   Compiling cons-list v0.1.0 (file:///projects/cons-list)
    Finished dev [unoptimized + debuginfo] target(s) in 0.63s
     Running `target/debug/cons-list`
a after = Cons(RefCell { value: 15 }, Nil)
b after = Cons(RefCell { value: 3 }, Cons(RefCell { value: 15 }, Nil))
c after = Cons(RefCell { value: 4 }, Cons(RefCell { value: 15 }, Nil))
```

By using `RefCell<T>` we have an outwardly immutable `List`.
We can use methods on that `RefCell` to get a mutable pointer to that value.
The runtime checks make sure there are no data races.

## 15.6. Reference Cycles Can Leak Memory

The memory safety guarantees make it _difficult_ to use memory that is never cleaned up, not impossible.
If that happens, that's a memory leak.
Rust doesn't guarantee it doesn't leak memory in the same way it disallows data races at compile time.
That means you can write a program that has a memory leak without ever using an `unsafe` block.

We can create a memory leak by using `Rc<T>` and `RefCell<T>` together.
By creating references where items refer to each other in a cycle.
The reference count will never reach 0, and the used memory will never be deallocated.

### Creating a Reference Cycle

To demonstrate a reference cycle, let's create yet another version of our `List` enum.
This version also defines a `tail` method to access the second item in a `Cons` variant.

```rust
use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}
```

The `Cons` variant of this `List` enum now holds an `i32`, and a `RefCell<Rc<List>>`.
That means that instead of having the ability to modify the `i32` like we did before,
we want to modify the second value in the pair, the `List` that `Cons` is pointing to.

In the following snippet, in the `main` function:
This creates an `Rc` of a list in `a` and an `Rc` of a list in `b` that points to the list in `a`.

```rust
fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}
```

The variable `a` contains an `Rc<List>`, the `List` value is a `Cons` variant that holds `5, Nil`.
The variable `b` also contains an `Rc<List>`, the `List` is also a `Cons` variant.
We create it with `10` as the `i32` and `RefCell::new(Rc::clone(&a))` as the second item in the pair.
In other words, the list in `b` points to `a`.

We then call `tail` on `a`.
That uses Rust's automatic dereferencing functionality to turn the `Rc` into a reference of the `List`,
and `tail` is then called on that `List`.

Using the `if let` syntax, we take the reference to the `item` inside the `Some`.
Inside the `if let` block,
we call `borrow_mut` on that `item` (which is automatically turned into the `RefCell` it contains by Rust's dereferencing feature).
Using the star operator to follow the `RefMut` we get back to the value it point to, we change that value to be an `Rc` that points to `b`.
In other words: we changed the `Rc<List>` that holds `Nil` to the `Rc<List>` in `b`.

`a` now points to `b`, and `b` points to `a`.
And thus, a ~~merry go round~~ reference cycle is created.

The code above, with that last `println` commented out, prints this:

```
$ cargo run
   Compiling cons-list v0.1.0 (file:///projects/cons-list)
    Finished dev [unoptimized + debuginfo] target(s) in 0.53s
     Running `target/debug/cons-list`
a initial rc count = 1
a next item = Some(RefCell { value: Nil })
a rc count after b creation = 2
b initial rc count = 1
b next item = Some(RefCell { value: Cons(5, RefCell { value: Nil }) })
b rc count after changing a = 2
a rc count after changing a = 2
```

When `a` is created, it's `Rc` has a reference count of 1.
After `b` is created, the value `a` points to has a reference count of 2 and the value `b` points to has a reference count of 1.
After we make `a` point to `b`, the value `b` points to has a reference count of 2.

At the end of `main`, Rust will drop `b` first, because it was declared last (variables are dropped in the reverse order they are declared).
That will decrease the reference count by one, from 2 to 1.
Because `a` is still referencing the `Rc<List>` in `b`, after `b` is dropped, the reference count of the `Rc<List>` it pointed to is still 1, not 0.
The memory that the list in `b` occupies on the heap will sit there, forever.
well, until every reference is dropped.
In this case that's immediately because the very next thing that happens is `a` being dropped.

![a reference cycle](trpl15-04.svg)

Uncommenting the last line won't lead to a compiler error.
However, when it runs, the stack will overflow and the program will exit.
Rust will try to print the `tail` of `a`.
But that points back to `a`, which in turn points to `b`, that points to `a` again, and, here we go again -insert gta:sa meme-, an infinite loop.

The consequences of this reference cycle aren't very dire.
In a program that uses a lot of memory in a cycle, that memory never gets freed, causing the program to use more and more memory as it runs and more cycles are created.
That might overwhelm the system and cause it to run out of memory.

Creating reference cycles is not easy, but not impossible.
When using `RefCell<T>` values that contain `Rc<T>` values or similar nested combinations of types with interior mutability and reference counting,
you must ensure you don't create cycles.

A solution for avoiding reference cycles is reorganizing the data in a way that some references express ownership while others don't.
As a result, you can still have cycles made up of some ownership relationships and some non-ownership relationships.
In the cons list example, we always want `Cons` variants to own their list, so such a reorganization is not possible.
Let's look at a graph next, where those two types of relationships do make sense.

### Preventing Reference Cycles: Turning an Rc<T> into a Weak<T>

`Rc::clone` increases the `strong_count` of an `Rc<T>`.
That `T` is only dropped when the `strong_count` reaches 0.
You can also create a _weak_ reference to the value in an `Rc<T>`,
by calling `Rc::downgrade` and passing a reference to the `Rc<T>`.
Calling `Rc::downgrade` returns a smart pointer of type `Weak<T>`.
When calling `Rc::downgrade`, instead of increasing the `strong_count` in the `Rc<T>` by 1,
you increase the `weak_count` in that `Rc<T>` by 1.
The `weak_count` doesn't need to be 0 for a value to be dropped.

Strong references are how you share ownership of an `Rc<T>`.
Weak references don't express an ownership relation.
They won't cause a reference cycle because the cycles involving weak references will be broken once the strong reference count drops to 0.

Because of the possibility that the value a `Weak<T>` points to has been dropped, we can't access it directly.
We have to make sure that `T` still exists first.
This is done by calling the `upgrade` method on a `Weak<T>`.
That method returns an `Option<Rc<T>>`.
If the `T` has been dropped, you'll get a `None`.
If it's still there, you get a `Some` with an `Rc<T>` inside.

The goal is to build a graph where parent nodes own their child nodes,
while child nodes have references to their parent node that doesn't express ownership.

#### Creating a Tree Data Structure: a Node with Child Nodes

To start, we'll build a tree with nodes that know about (and own) their children.
The `Node` struct holds an `i32` as well as references to its children `Node` values:

```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}
```

We want a `Node` to own its children.
We also want to share ownership with variables so we can access the `Node` directly through that variable.
To do this, we define the items inside the `Vec` to be of type `Rc<Node>`.
We want to be able to modify which nodes are children of another node, so that vector is wrapped in a `RefCell`.

We create one instance of `Node` with a `value` of 3, and no children.
We store it in the `leaf` variable.
We create another instance of `Node` with a `value` of 5, and `leaf` as one of its children.
That instance is stored in the `branch` variable.

```rust
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
}
```

We used `Rc::clone(&leaf)` to add `leaf` as a child of `branch`.
That means that `Node` now has two owners, `leaf`, and `branch` (the `strong_count` of the `Rc` is 2).
We can get from `branch` to `leaf` through `branch.children`, but not from `leaf` to its parent, `branch`.
`leaf` has no reference to `branch`, it doesn't even know it exists.

#### Adding a Reference from a Child to Its Parent

We want to add a `parent` field to our `Node` struct that holds a reference to the parent `Node`.
We know we can't use an `Rc<T>`, that would create a reference cycle (parent <-> child).
The `leaf.parent` would refer to the `branch`, and the `branch.children` would include a reference to `leaf` again.
The `strong_count` would never reach 0 that way.

Instead, only one relationship should express ownership.
A parent should own its children.
If a parent node is dropped, all child nodes should be dropped as well.
A child node should not own its parent, dropping a child node should not also drop the parent node.
This is a case for weak references.

The type of the `parent` field on the `Node` struct will use a `Weak<Node>`.
We want to be able to mutate that field, so we wrap it in a `RefCell` and use a `RefCell<Weak<Node>>`.

```rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
```

A `Node` will be able to refer to its parent node but doesn't own its parent.
The updates `leaf` and `branch` variable creation:

```rust
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
```

We create the `leaf` node with an empty `Weak<Node>` wrapped in a `RefCell` as parent.
When we `println` what's inside that `parent` by calling `.upgrade()` on the `Weak`,
we get a `None` variant back.
Since that `Weak` doesn't point to anything yet.

After the `branch` is created, the `leaf`'s `parent` field is updated to include a `Weak<Node>` that points to `branch` inside its `RefCell`.
Now, when we `println` what's inside that `parent` by calling `.upgrade()` on the `Weak`, something is there.
We get a `Some` back that holds `branch`.

That last `println` doesn't overflow the stack like our cons example did.
Eventhough it contains a cycle, `leaf` points to `branch`, and `branch` points to `leaf`!
That last line prints:

```
leaf parent = Some(Node { value: 5, parent: RefCell { value: (Weak) }, children: RefCell { value: [Node { value: 3, parent: RefCell { value: (Weak) }, children: RefCell { value: [] } }] } })
```

This is because the `Weak<Node>` references are printed as `(Weak)` and don't kick off infinite cycles.

#### Visualizing Changes to strong_count and weak_count

Let's look at how the `strong_count` and `weak_count` of the `Rc<Node>` change.
By moving the creation of `branch` to an inner scope (denoted by curly bois `{}`), 
we can print what happens to those counts after `branch` is dropped.

```rust
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
```

After the creation of `leaf`, the `Rc<Node>` it contains has a strong count of 1 and a weak count of 0.
We open a new scope.
After the creation of `branch`, we add a `Weak` reference to it in `leaf.parent`.
The `branch`'s `Rc` now has a strong count of 1 and a weak count of 1.
The `leaf`'s `Rc` now has a strong count of 2 and a weak count of 0.
(1 for the `leaf` variable, and 1 for the reference in `branch.children`).

When the inner scope ends, `branch` goes out of scope and is dropped.
The strong count of the `Rc<Node>` `branch` pointed to decreases to 0 and the `Node` is dropped.
The weak count of 1 from `leaf.parent` doesn't influence this, it doesn't prevent the `Node` from being dropped.
When we call `upgrade` on that `Weak` now, it will return a `None` variant.
At the end of the `main` function, the `Rc` that `leaf` points to has a strong count of 1 (the dropping of `branch` made it go from 2 to 1),
and a weak count of 0.