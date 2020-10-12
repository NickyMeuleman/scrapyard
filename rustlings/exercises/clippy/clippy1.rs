// clippy1.rs
// The Clippy tool is a collection of lints to analyze your code
// so you can catch common mistakes and improve your Rust code.
//
// For these exercises the code will fail to compile when there are clippy warnings
// check clippy's suggestions from the output to solve the exercise.
// Execute `rustlings hint clippy1` for hints :)

fn main() {
    let x = 1.2331f64;
    let y = 1.2332f64;
    // check if difference is within a margin of errors
    if (y - x).abs() < 0.0001f64 {
        // Maths tells me this shouldn't pass because it's exactly 0.0001
        // Difference is 0.00009999999999998899
        // because computers (floating point binary math is wild)
        println!("Success! Difference is {}", (y-x).abs());
    }
}
