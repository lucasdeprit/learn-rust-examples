/*
JSON Parser (Advanced)
Description:
Write a function that takes a string containing JSON data and attempts to parse it into a Rust HashMap. Handle parsing errors gracefully and print an appropriate message.

Example Input/Output:

For the input:
json_string = r#"{"name": "Alice", "age": 30}"#

The output should be:
Parsed data: {"name": "Alice", "age": 30}

For the input:
json_string = "invalid json"

The output should be:
Error: Could not parse the JSON string.
*/

use std::collections::HashMap;

fn main() {
    let data = r#"{"name": "Alice", "age": "30"}"#;
    let map: HashMap<String, String> = serde_json::from_str(data).unwrap();
    /* {
        Ok(value) => value,
        Err(_) => "Error: Could not parse the JSON string.".to_string(),
    }; */
    println!("HashMap: {:?}", map);
}
