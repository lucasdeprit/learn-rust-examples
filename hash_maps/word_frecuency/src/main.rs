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

// optimized
fn word_frequency(text: &str) -> HashMap<&str, usize> {
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        // if it doesnt contain the key inserts a default value
        *map.entry(word).or_insert(0) += 1;
    }
    map
}

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

    println!(
        "{:?}",
        word_frequency("hello world hello hello hello hello")
    );
}
