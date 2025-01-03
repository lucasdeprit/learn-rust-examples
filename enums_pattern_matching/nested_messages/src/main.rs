/* [Hard] Decode Nested Messages
Create an enum Message with the following variants:

Text(String)
List(Vec<Message>)
Write a function that takes a Message and extracts all the Text messages, flattening them into a single vector of strings.

Input:
let msg = Message::List(vec![
    Message::Text("Hello".to_string()),
    Message::List(vec![
        Message::Text("Nested".to_string()),
        Message::Text("World".to_string()),
    ]),
]);
extract_texts(msg)

Output:
vec!["Hello", "Nested", "World"]

*/

enum Message {
    Text(String),
    List(Vec<Message>),
}

fn extract_texts(messages: Message) -> Vec<String> {
    match messages {
        Message::Text(text) => vec![text],
        Message::List(messages) => messages.into_iter().flat_map(extract_texts).collect(),
    }
}

fn main() {
    let msg = Message::List(vec![
        Message::Text("Hello".to_string()),
        Message::List(vec![
            Message::Text("Nested".to_string()),
            Message::List(vec![Message::Text("Double Nested".to_string())]),
        ]),
    ]);

    println!("{:?}", extract_texts(msg));
}
