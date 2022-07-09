// primitive_types5.rs
// Destructure the `cat` tuple so that the println will work.
// Execute `rustlings hint primitive_types5` for hints!

fn main() {
    // cat is a tuple, a comma separated list of values inside parentheses
    let cat = ("Furry McFurson", 3.5);
    // This is an example of using pattern matching to destructure a tuple.
    let (name, age) = cat;

    println!("{} is {} years old.", name, age);
}
