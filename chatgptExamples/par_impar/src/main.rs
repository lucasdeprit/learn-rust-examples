/* 2. Números pares e impares
Crea un programa que imprima si los números del 1 al 20 son pares o impares.

Usa un bucle for para recorrer los números.
Usa un condicional if para determinar si un número es par o impar.
Salida esperada:

python
Copy code
1 es impar
2 es par
3 es impar
...
20 es par */

use std::io;

fn main() {
    println!("Pick a number:");

    let mut resp = String::new();

    io::stdin()
        .read_line(&mut resp)
        .expect("Failed reading the number");

    let number = match resp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error parsing the number");
            0
        }
    };

    println!("{}", get_is_par(number));
}

fn get_is_par(num: i32) -> bool {
    num % 2 == 0
}
