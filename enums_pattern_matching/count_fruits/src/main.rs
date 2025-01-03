/*
*
4. [Medium-Hard] Count Fruits
Create an enum Fruit with the following variants:

Apple
Banana
Cherry
Write a function that takes a vector of Fruit and counts how many of each type are present, returning the counts in a tuple (apples, bananas, cherries).

Input:
count_fruits(vec![Fruit::Apple, Fruit::Banana, Fruit::Cherry, Fruit::Apple])

Output:
(2, 1, 1)

*/

enum Fruit {
    Apple,
    Banana,
    Cherry,
}

fn count_fruits(fruits: Vec<Fruit>) -> (i32, i32, i32) {
    let mut apples = 0;
    let mut bananas = 0;
    let mut cherries = 0;

    for fruit in fruits {
        match fruit {
            Fruit::Apple => apples += 1,
            Fruit::Banana => bananas += 1,
            Fruit::Cherry => cherries += 1,
        }
    }

    (apples, bananas, cherries)
}

fn main() {
    let (x, y, z) = count_fruits(vec![
        Fruit::Apple,
        Fruit::Banana,
        Fruit::Cherry,
        Fruit::Apple,
    ]);

    println!("({}, {}, {})", x, y, z)
}
