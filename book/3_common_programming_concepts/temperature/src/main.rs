fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Please pass in 2 arguments: \n 1. A number. \n 2. C or F for Celcius or Fahrenheit respectively.");
        // Panic early if the length of the args is wrong and display a helpful message.
        panic!("Invalid amount of command line arguments.");
    }
    let num: f64 = args[1].parse().expect("First argument must be a number.");
    let scale: String = args[2].parse().expect("Second argument must be a string.");
    let is_c: bool = if scale == "C" {
        true
    } else if scale == "F" {
        false
    } else {
        panic!("Second arg must be C for Celsius, or F for Fahrenheit.");
    };
    let result = if is_c { c_to_f(num) } else { f_to_c(num) };
    println!(
        "{}°{} is {}°{}.",
        num,
        if is_c { "C" } else { "F" },
        result,
        if is_c { "F" } else { "C" },
    );
}

fn f_to_c(f: f64) -> f64 {
    (f - 32.0) * (5.0 / 9.0)
}

fn c_to_f(c: f64) -> f64 {
    c * (9.0 / 5.0) + 32.0
}
