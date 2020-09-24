// functions3.rs
// Make me compile! Execute `rustlings hint functions3` for hints :)

fn main() {
    // the type of 10 will automatically be inferred to be i8 because the function definition
    call_me(10);
}

fn call_me(num: i8) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
