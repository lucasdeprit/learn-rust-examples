use std::fs;

fn main() {
    let contents = read_file("src/example.txt");
    println!("{contents}")
}

fn read_file(path: &str) -> String {
    match fs::read_to_string(path) {
        Ok(text) => text,
        Err(_e) => "couldnt read the file".to_string(),
    }
}
