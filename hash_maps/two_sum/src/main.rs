/*
 Two Sum (Medium)
Given a vector of integers and a target number, find two numbers in the vector that add up to the target. Return their indices as a tuple (i, j). Use a HashMap to optimize the search.

Input:
nums = [2, 7, 11, 15], target = 9

Output:
(0, 1)
*/

use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Option<(usize, usize)> {
    let mut map = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&j) = map.get(&complement) {
            return Some((j, i));
        }
        map.insert(num, i);
    }
    None
}

fn main() {
    let nums = [2, 7, 11, 15].to_vec();
    let target = 9;

    print!("{:?}", two_sum(nums, target));
}
