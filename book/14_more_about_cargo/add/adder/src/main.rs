use add_one;
use add_two;

fn main() {
    let num = 10;
    println!(
        "Hello, world! {0} plus one is {1}! {0} plus two is {2}!",
        num,
        add_one::add_one(num),
        add_two::add_two(num)
    );
}
