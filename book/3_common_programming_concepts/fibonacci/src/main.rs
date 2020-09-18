// Generate the nth Fibonacci number.

const FIRST_FIB: u8 = 0;
const SECOND_FIB: u8 = 1;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Please provide one argument: \n 1. The position in the Fibonacci sequence. (one based positive integer)");
        panic!("Invalid arguments");
    }
    let pos: u32 = args[1]
        .parse()
        .expect("Argument must be a positive integer.");
    println!("{}", fib(pos));
}

fn fib(pos: u32) -> u128 {
    if pos == 0 {
        panic!("Position is one based. The first Fibonacci number is located at position 1, not 0.")
    } else if pos == 1 {
        FIRST_FIB.into()
    } else if pos == 2 {
        SECOND_FIB.into()
    } else {
        let mut last: u128 = FIRST_FIB.into();
        let mut current: u128 = SECOND_FIB.into();
        // range starts at 3, because the first two iterations are handled as constant returns in the if-else-if
        // The position is a one based number, not zero based
        for _ in 3..=pos {
            let new = last + current;
            last = current;
            current = new;
        }
        current
    }
}
