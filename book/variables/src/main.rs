// fn main() {
//     let x = 5;
//     println!("The value of x is: {}", x);
//     x = 6;
//     println!("The value of x is: {}", x);
// }

// error[E0384]: cannot assign twice to immutable variable `x`
//  --> src/main.rs:4:5
//   |
// 2 |     let x = 5;
//   |         -
//   |         |
//   |         first assignment to `x`
//   |         help: make this binding mutable: `mut x`
// 3 |     println!("The value of x is: {}", x);
// 4 |     x = 6;
//   |     ^^^^^ cannot assign twice to immutable variable

// error: aborting due to previous error

// For more information about this error, try `rustc --explain E0384`.
// error: could not compile `variables`.

// To learn more, run the command again with --verbose.

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
