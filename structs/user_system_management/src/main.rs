/*
User System
Create a struct User with properties like name, email, and age. Then, implement a system to add users and check if a specific email is already registered.

Input Example:

rust
Copy code
let mut system = UserSystem::new();
system.add_user("Alice", "alice@example.com", 25);
system.add_user("Bob", "bob@example.com", 30);
println!("{}", system.email_exists("alice@example.com")); // true
println!("{}", system.email_exists("charlie@example.com")); // false
*/

struct User {
    name: String,
    email: String,
    age: u32,
}

struct System {
    users: Vec<User>,
}

impl System {
    fn new() -> Self {
        Self { users: Vec::new() }
    }

    fn add_user(&mut self, name: &str, email: &str, age: u32) {
        self.users.push(User {
            name: name.to_string(),
            email: email.to_string(),
            age,
        });
    }

    fn email_exists(&self, email: &str) -> bool {
        for user in self.users.iter() {
            if user.email == email {
                true;
            }
        }
        false
    }
}

fn main() {
    let mut system = System::new();
    system.add_user("Alice", "alice@example.com", 25);
    system.add_user("Bob", "bob@example.com", 30);
    println!("{}", system.email_exists("alice@example.com")); // true
    println!("{}", system.email_exists("charlie@example.com")); // false
}
