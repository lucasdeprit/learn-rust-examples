/*
Reverse an Array
Given an array of integers, reverse it in place.

Example:
Input: arr = [1, 2, 3, 4, 5]
Output: [5, 4, 3, 2, 1]
*/

fn main() {
    let mut arr = [1, 2, 3, 4, 5];
    arr.reverse();
    print_response(&arr);
}

fn print_response(array: &[i32]) {
    for i in array {
        print!("{},", i)
    }
    println!()
}
