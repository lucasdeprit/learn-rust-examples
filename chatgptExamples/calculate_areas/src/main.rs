/* 1. Cálculo de áreas
Escribe un programa que calcule el área de un círculo, un cuadrado y un rectángulo.

Solicita al usuario que elija la figura.
Según su elección, pide los datos necesarios (radio, lado, base y altura).
Usa if, else if, o match para realizar los cálculos.
Ejemplo de entrada/salida:

Elige la figura:
1. Círculo
2. Cuadrado
3. Rectángulo
> 2
Introduce el lado del cuadrado:
> 5
El área del cuadrado es 25. */

use std::io;

fn main() {
    let mut repeat: bool = true;

    while repeat {
        println!("Let's calculate the area of a figure!");
        println!("1. Circle");
        println!("2. Square");
        println!("3. Rectangle");

        let selection = get_number_from_user("Selection:");

        match selection {
            1 => {
                let radius = get_number_from_user("What is the radius?");
                println!("The area is: {}", calculate_circle_area(radius as f64));
            }
            2 => {
                let width = get_number_from_user("What is the width?");
                println!("The area is: {}", calculate_square_area(width, width));
            }
            3 => {
                let width = get_number_from_user("What is the width?");
                let height = get_number_from_user("What is the height?");
                println!("The area is: {}", calculate_square_area(width, height));
            }
            _ => {
                println!("Invalid option");
            }
        }

        repeat = get_continuation_from_user();
    }
}

fn get_continuation_from_user() -> bool {
    let mut repeat: bool = true;
    let mut response: bool = false;

    while repeat {
        println!("Do you want to calculate another area? (y/n)");

        let mut resp = String::new();
        io::stdin()
            .read_line(&mut resp)
            .expect("Error reading the response");

        match resp.trim().chars().next() {
            Some('y' | 'Y') => {
                response = true;
                repeat = false
            }
            Some('n' | 'N') => {
                response = false;
                repeat = false
            }
            _ => {
                println!("Write y or n");
                repeat = true
            }
        };
    }
    response
}

fn get_number_from_user(question: &str) -> i32 {
    let mut str_num = String::new();
    let mut num: i32 = -1;

    while num < 0 {
        println!("{}", question);
        io::stdin()
            .read_line(&mut str_num)
            .expect("Failed to read the number");

        num = match str_num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number, please introduce a correct one");
                -1
            }
        };
    }
    num
}

fn calculate_circle_area(radius: f64) -> f64 {
    radius * 3.14
}
fn calculate_square_area(width: i32, height: i32) -> i32 {
    width * height
}
