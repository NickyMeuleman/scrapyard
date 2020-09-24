// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` for hints!

fn main() {
    let a = [1; 60];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        // https://stackoverflow.com/questions/33036859/why-does-println-work-only-for-arrays-with-a-length-less-than-33
        // printing arrays with length over 32 is not supported (yet?)
        // workaround: convert them to a slice
        println!("{:?}", &a[..]);
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
