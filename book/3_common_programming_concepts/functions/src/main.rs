fn main() {
    println!("Hello, world!");

    another_function(42, "boop");
    fn name() -> i32 {
        1
    };
    println!("{}", name())
}

fn another_function(x: i32, y: &str) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
