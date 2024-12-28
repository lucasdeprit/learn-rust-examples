/*
Exercise 4: Sum of Even Numbers
Given a vector of integers, calculate the sum of all even numbers.

Example:
Input: vec = [1, 2, 3, 4, 5, 6]
Output: 12
*/

fn main() {
    let vec: Vec<i32> = [1, 2, 3, 4, 5, 6].to_vec();
    println!("expected: 12, returned: {}", get_sum_even_numbers(vec));

    let vec: Vec<i32> = [1, 2, 3, 4, 5].to_vec();
    println!("expected: 6, returned: {}", get_sum_even_numbers(vec));

    let vec: Vec<i32> = [8, 2, 3, 4, 5].to_vec();
    println!("expected: 14, returned: {}", get_sum_even_numbers(vec))
}

fn get_sum_even_numbers(vec: Vec<i32>) -> i32 {
    vec.into_iter()
        .filter(|x| *x % 2 == 0)
        .collect::<Vec<i32>>()
        .into_iter()
        .sum()
}
