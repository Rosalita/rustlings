// vec2.rs
// A Vec of even numbers is given. Your task is to complete the loop
// so that each number in the Vec is multiplied by 2.
//
// Make me pass the test!
//
// Execute the command `rustlings hint vec2` if you need
// hints.

fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for i in v.iter_mut() {
        // TODO: Fill this up so that each element in the Vec `v` is
        // multiplied by 2.
        *i *= 2;
    }

    // At this point, `v` should be equal to [4, 8, 12, 16, 20].
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        // the following line was hard to read its a test input that was calculated
        // let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        // for the sake of readability, this line can simply be expressed as...
        let v = vec![2, 4, 6, 8, 10];
        let ans = vec_loop(v.clone());

        // this assertion statement too also has calculated values
        // assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
        // it could be expressed simply as...
        assert_eq!(ans, vec![4, 8, 12, 16, 20])
    }
}
