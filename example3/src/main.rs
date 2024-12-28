use std::io;

fn main() {
    println!("Select an index from the array");

    let array = [3, 4, 5, 8, 9, 10];

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("couldn´t read the input");

    let index: usize = index.trim().parse().expect("The index could not be parsed");

    let selected_element = array[index];

    println!(
        "el numero seleccionado del array es el: {}",
        selected_element,
    );
}
