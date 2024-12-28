/*
Move Zeros to the End
Given a vector of integers, move all zeros to the end while keeping the relative order of other elements.

Example:
Input: vec = [0, 1, 0, 3, 12]
Output: [1, 3, 12, 0, 0]
*/

fn main() {
    let vec: Vec<i32> = [0, 1, 0, 3, 12].to_vec();
    print_response(zeros_to_end(&vec));
    print_response(zeros_to_end_better(vec));
}

fn zeros_to_end_better(vec: Vec<i32>) -> Vec<i32> {
    let mut non_zero: Vec<i32> = vec.iter().cloned().filter(|&x| x != 0).collect();
    non_zero.extend(
        vec.iter()
            .cloned()
            .filter(|&x| x == 0)
            .collect::<Vec<i32>>(),
    );
    non_zero
}
// super unoptimized
fn zeros_to_end(vec: &Vec<i32>) -> Vec<i32> {
    let mut zeros: i32 = 0;
    let mut response: Vec<i32> = [].to_vec();

    for i in 0..vec.len() {
        if vec[i] == 0 {
            zeros += 1
        } else {
            response.push(vec[i]);
        }
    }

    while zeros > 0 {
        response.push(0);
        zeros -= 1
    }
    response
}

fn print_response(vec: Vec<i32>) {
    for i in vec {
        print!("{}, ", i);
    }
    println!()
}
