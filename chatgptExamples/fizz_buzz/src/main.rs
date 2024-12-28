fn main() {
    for i in 1..50 {
        let fizz: bool = i % 3 == 0;
        let buzz: bool = i % 5 == 0;

        if fizz && buzz {
            println!("fizzbuzz")
        } else if fizz {
            println!("fizz")
        } else if buzz {
            println!("buzz")
        } else {
            println!("{}", i)
        }
    }
}
