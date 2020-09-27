// strings1.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings1` for hints ;)

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

fn current_favorite_color() -> String {
    // Use any of these, the goal is to convert the &str to a String
    // "blue".to_string()
    // String::from("blue")
    // "blue".into()
    "blue".to_owned()
}
