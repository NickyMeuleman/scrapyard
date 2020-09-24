// structs1.rs
// Address all the TODOs to make the tests pass!

// Needing to introduce lifetimes is superconfusing, what do those mean, why?
// Easier to change the things inside the struct to String instead of &str

struct ColorClassicStruct<'a> {
    name: &'a str,
    hex: String,
}

struct ColorTupleStruct<'a>(&'a str, &'a str);

#[derive(Debug)]
struct UnitStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        let green = ColorClassicStruct {
            name: "green",
            hex: String::from("#00FF00"),
        };
        // Why does this pass? A String and a &str, aren't those different?
        // https://doc.rust-lang.org/book/ch08-02-strings.html#what-is-a-string
        assert_eq!(String::from("#00FF00"), "#00FF00");
        assert_eq!(green.name, "green");
        assert_eq!(green.hex, "#00FF00");
    }

    #[test]
    fn tuple_structs() {
        let green = ColorTupleStruct("green", "#00FF00");

        assert_eq!(green.0, "green");
        assert_eq!(green.1, "#00FF00");
    }

    #[test]
    fn unit_structs() {
        let unit_struct = UnitStruct;
        let message = format!("{:?}s are fun!", unit_struct);

        assert_eq!(message, "UnitStructs are fun!");
    }
}
