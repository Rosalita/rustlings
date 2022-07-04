// variables5.rs
// Make me compile! Execute the command `rustlings hint variables5` if you want a hint :)

fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    // this is called shadowing in Rust
    // it creates a shadow of the variable with the same name and a new value
    let number = 3;
    println!("Number plus two is : {}", number + 2);
}
