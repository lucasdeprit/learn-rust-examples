/*
Find the Missing Element
Given an array of consecutive numbers from 1 to n where one number is missing, find the missing number.

Example:
Input: arr = [1, 2, 4, 5, 6], n = 6
Output: 3
*/

fn main() {
    // hint! if we sum all the espected values of the array and we - original array
    // we get the number that is missing
    let arr = [1, 2, 4, 5, 6];
    let n = 6;

    println!("Result (1, 2, 4, 5, 6): {}", find_expected_num(&arr, n));

    let arr = [1, 2, 4, 5, 6];
    let n = 6;

    println!("Result (1, 2, 3, 5, 6): {}", find_expected_num(&arr, n));

    let arr = [1, 3, 4, 5];
    let n = 5;

    println!("Result (1, 3, 4, 5): {}", find_expected_num(&arr, n));
}

fn find_expected_num(array: &[i32], n: i32) -> i32 {
    let exp_sum: i32 = (1..=n).sum();
    let sum: i32 = array.iter().sum();

    exp_sum - sum
}
