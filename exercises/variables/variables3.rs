// variables3.rs
// Make me compile! Execute the command `rustlings hint variables3` if you want a hint :)


fn main() {
    // variables are immutable by default,  but you can make them mutable 
    // by adding mut in front of their name. Adding mut also conveys the 
    // intention to readers of the code that this value will change.
    let mut x = 3;
    println!("Number {}", x);
    x = 5; // don't change this line
    println!("Number {}", x);
}
