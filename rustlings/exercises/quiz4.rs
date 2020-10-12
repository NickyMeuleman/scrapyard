// quiz4.rs
// This quiz covers the sections:
// - Modules
// - Macros

// Write a macro that passes the quiz! No hints this time, you can do it!

macro_rules! my_macro {
    ($x: expr) => {
        // format! returns a String, not a &str
        // the tests pass, because coersion
        // format!("Hello {}", $x)

        // concat! returns a &'static str
        // only literals can be passed to concat, not variables
        // $x gets replaced by the string literal, it's not a variable when used
        concat!("Hello ", $x)
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_my_macro_world() {
        assert_eq!(my_macro!("world!"), "Hello world!");
    }

    #[test]
    fn test_my_macro_goodbye() {
        assert_eq!(my_macro!("goodbye!"), "Hello goodbye!");
    }
}
