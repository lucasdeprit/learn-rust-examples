fn main() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward
    let y = give_ownership();
    {
        // todo esto no afecta al y de arriba ya que esta en otro scope
        let y = String::from("hello");
        println!("{}", y);
        takes_ownership(y);
        // si yo tratase de utilizar aqui la variable y estaria fuera de mi scope, ya que se la ha
        // quedado la funcion anterior con el take ownership
    }
    println!("{}", y)
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.
  //

fn give_ownership() -> String {
    String::from("Hello World!")
}
