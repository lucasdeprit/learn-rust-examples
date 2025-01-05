/*
Find the Missing Number (Easy-Medium)
Given a vector of integers containing numbers from 1 to n with one number missing, write a function to find the missing number using a HashMap.

Input:
nums = [1, 2, 4, 5]

Output:
3
*/

use std::collections::HashMap;

fn find_missing_number(vec: Vec<i32>) -> i32 {
    let mut map = HashMap::new();

    for number in &vec {
        let num = *number as usize;
        map.insert(num, true);
    }

    for i in 1..vec.len() {
        if !map.contains_key(&i) {
            let i = i as i32;
            return i;
        }
    }
    -1
}

fn main() {
    let nums = [1, 2, 4, 5].to_vec();

    print!("{}", find_missing_number(nums));
}
