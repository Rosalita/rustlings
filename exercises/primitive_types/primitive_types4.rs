// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` for hints!!

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    // &a is used to borrow so that the size of the slice is known at compile time
    // presumably this is to keep the slice off the heap.
    let nice_slice = &a[1..4];
    // 0 represents the first element
    // however if first index is 0, rust allows it to be omitted, e.g. &a[..4]
    // same with the last element, rust allows it to be omitted e.g. &a[1..]
    // to create a slice with all elements, rust allows &a[..]

    assert_eq!([2, 3, 4], nice_slice)
}
