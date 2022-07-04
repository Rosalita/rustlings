// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` for hints :)

fn main() {
    // in Rust a value can not be owned by more than one variable
    // consider
    // let mut x = MyStruct{ s: 5u32 };
    // let y = x;
    // the data gets moved out of x when y is set.
    // Using a reference, can let a function borrow a value
    // without changing it's ownership.
    // this seems to be done by putting a & infront of the value
    // borrowing should be used in three cases:
    // 1. when some value should be passed in then out
    // 2. when some value can't be cloned
    // 3. when cloning is too slow

    let vec0 = Vec::new();

    // vec0 is being moved into the function fill_vec
    // this means it is dropped at the end of fill_vec
    // so it can't be used anywhere after the call to fill_vec
    // unless a clone is passed in
    let mut vec1 = fill_vec(vec0.clone());

    // in Go the default behaviour is to copy values which are passed into functions
    // so under the hood, Go clones silently.
    // it looks like borrowing is the same as passing by reference in Go e.g. &value

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
