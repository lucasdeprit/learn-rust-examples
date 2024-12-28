/*
Rotate a Vector
Given a vector vec and an integer k, rotate the vector k positions to the right.

Example:
Input: vec = [1, 2, 3, 4, 5], k = 2
Output: [4, 5, 1, 2, 3]
*/

fn main() {
    let mut arr = [1, 2, 3, 4, 5];
    let k = 2;

    let size = &arr.len();
    arr.rotate_right(k % size);

    for i in arr {
        print!("{}", i);
    }
}
