// variables1.rs
// Make me compile! Execute the command `rustlings hint variables1` if you want a hint :)

// About this `I AM NOT DONE` thing:
// We sometimes encourage you to keep trying things on a given exercise,
// even after you already figured it out. If you got everything working and
// feel ready for the next exercise, remove the `I AM NOT DONE` comment below.

fn main() {
    // Things stored on the heap can't be a constant
    // https://github.com/rust-lang/const-eval/issues/20
    let x: String = String::from("test");
    println!("x has the value {}", x);
}
