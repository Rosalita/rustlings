// clippy2.rs
// Make me compile! Execute `rustlings hint clippy2` for hints :)

fn main() {
    let mut res = 42;
    let option = Some(12);
    // for x in option {
    //     res += x;
    // }
    // for loops over type option are more readably written as if let statements

    if let Some(x) = option {
        res += x
    };
    println!("{}", res);
}
