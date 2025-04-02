//TODO
//Complete the square sum function so that it squares each number passed into it and then sums the results together.

pub fn square_sum_fn(vec: Vec<i32>) -> i32 {
    let answer = vec.iter().map(|x| x * x).sum();
    answer
}
