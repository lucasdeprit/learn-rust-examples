/*
[Medium] Validate HTTP Response
Create an enum HttpResponse with the following variants:

Success(u16, String) where u16 is the status code and String is the message.
Error(u16, String) where u16 is the error code and String is the error description.
Write a function that takes an HttpResponse and:

Returns "Success: <message>" if it's a Success.
Returns "Error [<code>]: <description>" if it's an Error.

Input:
handle_response(HttpResponse::Error(404, "Not Found".to_string()))

Output:
"Error [404]: Not Found"
*/
enum HttpResponse {
    Success(u16, String),
    Error(u16, String),
}

fn handle_response(response: HttpResponse) {
    match response {
        HttpResponse::Success(_, message) => println!("Success: {}", message),
        HttpResponse::Error(code, message) => println!("Error [{}]: {}", code, message),
    }

    println!("")
}

fn main() {
    handle_response(HttpResponse::Error(404, "Not Found".to_string()));
    handle_response(HttpResponse::Success(200, "Found".to_string()))
}
