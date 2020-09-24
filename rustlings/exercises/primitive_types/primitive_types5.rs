// primitive_types5.rs
// Destructure the `cat` tuple so that the println will work.
// Execute `rustlings hint primitive_types5` for hints!

fn main() {
    let cat = ("Furry McFurson", 3.5);
    // let mut zero_index_item = "";
    // let mut age = 1.0;
    // (zero_index_item, age) = cat;
    // womp, womp, sad noises, because:
    // note: destructuring assignments are not currently supported

    // instead declare the variables
    let (zero_index_item, age) = cat;

    println!("{} is {} years old.", zero_index_item, age);
}
