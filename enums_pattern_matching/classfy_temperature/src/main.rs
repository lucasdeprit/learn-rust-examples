/*
1. [Easy] Classify Temperature
Create an enum Temperature with variants Hot, Cold, and Mild. Write a function that classifies an integer temperature as follows:

Hot if it's greater than 30.
Cold if it's less than 15.
Mild otherwise.

Input:
classify_temperature(20)

Output:
Temperature::Mild
*/

enum Temperature {
    Cold,
    Mild,
    Hot,
}

impl Temperature {
    fn to_string(&self) -> String {
        match self {
            Temperature::Hot => String::from("Hot"),
            Temperature::Mild => String::from("Mild"),
            Temperature::Cold => String::from("Cold"),
        }
    }
}

fn classify_temperature(temperature: i32) -> Temperature {
    match temperature {
        t if t > 30 => Temperature::Hot,
        t if t < 15 => Temperature::Cold,
        _ => Temperature::Mild,
    }
}

fn main() {
    let hot = 32;
    let mild = 17;
    let cold = 0;

    println!("{}", classify_temperature(hot).to_string());
    println!("{}", classify_temperature(mild).to_string());
    println!("{}", classify_temperature(cold).to_string());
}
