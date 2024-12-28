/*
Exercise 3: Intersection of Vectors
Given two vectors, find the common elements between them.

Example:
Input: vec1 = [1, 2, 3, 4], vec2 = [3, 4, 5, 6]
Output: [3, 4]
*/

fn main() {
    let vec1: Vec<i32> = [1, 2, 3, 4, 5, 6].to_vec();
    let vec2: Vec<i32> = [3, 4, 5, 6, 2].to_vec();

    print_response(get_intersected_numbers(vec1, vec2));
}

fn get_bigger_smaller_vector(vec1: Vec<i32>, vec2: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    if vec1.len() > vec2.len() {
        return (vec1, vec2);
    }
    (vec2, vec1)
}

// this function is an extra
fn get_intersected_numbers(vec1: Vec<i32>, vec2: Vec<i32>) -> Vec<i32> {
    let (big, small) = get_bigger_smaller_vector(vec1, vec2);
    big.into_iter().filter(|x| small.contains(x)).collect()
}

fn print_response(vec: Vec<i32>) {
    for i in vec {
        print!("{}", i);
    }
}
