// iterators4.rs

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.

    // using =num means that num is included in the range
    // if the range was (1..4), it would be 1,2,3 and not include 4.
    // range (1..=4) would be 1,2,3,4
    // fold solds every element into an accumulator by applying an operation and returning the final result.
    // In other languages this is called reduce.
    // Folding always produces a single value from a collection of values.
    (1..=num).fold(1, |acc, v| acc * v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
