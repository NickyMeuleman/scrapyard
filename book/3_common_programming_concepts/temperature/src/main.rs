fn main() {
    let args: Vec<String> = std::env::args().collect();
    let num: f64 = args[1].parse().expect("First argument must be a number");
    let scale: String = args[2]
        .parse()
        .expect("Second arg must be c for Celsius, or f for Fahrenheit.");
    let is_c: Option<bool> = if scale == "C" {
        Some(true)
    } else if scale == "F" {
        Some(false)
    } else {
        None
    };
    let result = if is_c.unwrap() {
        c_to_f(num)
    } else {
        f_to_c(num)
    };
    println!(
        "{}°{} is {}°{}",
        num,
        if is_c.unwrap() { "C" } else { "F" },
        result,
        if is_c.unwrap() { "F" } else { "C" },
    );
}

fn f_to_c(f: f64) -> f64 {
    f - 32.0 * (5.0 / 9.0)
}

fn c_to_f(c: f64) -> f64 {
    c * (9.0 / 5.0) + 32.0
}
