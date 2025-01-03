/*
2. [Easy-Medium] Perform Math Operations
Create an enum Operation with the following variants:

Add(i32, i32)
Subtract(i32, i32)
Multiply(i32, i32)
Divide(i32, i32)
Write a function that takes an Operation and returns the result of the operation.

Input:
perform_operation(Operation::Add(5, 3))

Output:
8

*/

enum Operation {
    Add(i32, i32),
    Subtract(i32, i32),
    Multiply(i32, i32),
    Divide(i32, i32),
}

fn perform_operation(operation: Operation) -> i32 {
    match operation {
        Operation::Add(x, y) => x + y,
        Operation::Subtract(x, y) => x - y,
        Operation::Multiply(x, y) => x * y,
        Operation::Divide(x, y) => x / y,
    }
}

fn main() {
    println!("the values are 5 & 3");
    println!("Add {}", perform_operation(Operation::Add(5, 3)));
    println!("Substract {}", perform_operation(Operation::Subtract(5, 3)));
    println!("Multiply {}", perform_operation(Operation::Multiply(5, 3)));
    println!("Divide {}", perform_operation(Operation::Divide(5, 3)));
}
