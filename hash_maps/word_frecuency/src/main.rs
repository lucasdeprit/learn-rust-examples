/*
Exercise 1: Word Frequency (Easy)

Write a function that takes a string and returns a HashMap where the keys are the
words and the values are the number of times each word appears in the text.

Input:
"hello world hello"

Output:
{"hello": 2, "world": 1}

*/

use std::collections::HashMap;

fn get_word_frecuency(text: &str) -> HashMap<&str, u32> {
    let mut map = HashMap::new();

    for split in text.split(' ') {
        if let Some(value) = map.get_mut(split) {
            *value += 1;
        } else {
            map.insert(split, 1);
        }
    }
    map
}

fn main() {
    println!(
        "{:?}",
        get_word_frecuency("hello world hello hello hello hello")
    );
}
