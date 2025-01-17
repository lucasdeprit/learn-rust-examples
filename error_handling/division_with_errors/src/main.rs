/*
Division with Error Handling (Intermediate)
Description:
Write a function that takes two numbers, a and b, and returns the result of a / b. Handle cases where b is zero by returning a custom error.

Example Input/Output:

For the input:
a = 10, b = 2
The output should be:
Result: 5

For the input:
a = 10, b = 0
The output should be:
Error: Division by zero is not allowed.
*/

fn main() {
    let (a, ref mut b) = (10, 2);

    println!("{:?}", divide(&a, &b));

    *b = 0;

    println!("{:?}", divide(&a, &b));
}

fn divide(a: &u32, b: &u32) -> Result<u32, String> {
    if *b == 0 {
        return Err("Error".to_string());
    }
    let result: u32 = a / b;
    Ok(result)
}
