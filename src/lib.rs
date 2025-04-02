pub mod solutions;
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use solutions::square_sum::square_sum_fn;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn square_test() {
        let result = square_sum_fn(vec![1, 2, 2]);
        assert_eq!(result, 9);
    }
}
