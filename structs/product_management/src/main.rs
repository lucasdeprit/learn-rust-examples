/*
Product Management
Create a struct Product to represent a product with a name, price, and stock quantity. Then, create a struct Inventory to manage multiple products.

Input Example:

rust
Copy code
let mut inventory = Inventory::new();
inventory.add_product("Apple", 1.0, 10);
inventory.add_product("Banana", 0.5, 20);
println!("{}", inventory.total_value()); // 20.0
inventory.update_stock("Apple", 15);
println!("{}", inventory.total_value()); // 22.5
*/

struct Product {
    name: String,
    price: u64, // cents
    stock: u64,
}

struct Inventory {
    products: Vec<Product>,
}

impl Inventory {
    fn new() -> Self {
        Inventory {
            products: Vec::new(),
        }
    }

    fn add_product(&mut self, name: &str, price: u64, stock: u64) {
        self.products.push(Product {
            name: String::from(name),
            price,
            stock,
        });
    }

    fn update_stock(&mut self, name: &str, new_stock: u64) {
        if let Some(product) = self.products.iter_mut().find(|x| x.name == name) {
            product.stock = new_stock;
        }
    }

    fn update_price(&mut self, name: &str, new_price: u64) {
        if let Some(product) = self.products.iter_mut().find(|x| x.name == name) {
            product.price = new_price;
        }
    }

    fn total_value(&self) -> u64 {
        let mut response: u64 = 0;
        for product in self.products.iter() {
            response += product.price * product.stock;
        }
        response
    }
}

fn main() {
    println!("Hello, world!");
    let mut inventory = Inventory::new();
    inventory.add_product("Apple", 100, 1000);
    inventory.add_product("Banana", 50, 2000);
    println!("{}", inventory.total_value()); // 20000
    inventory.update_stock("Apple", 15);
    inventory.update_price("Apple", 1500);
    println!("{}", inventory.total_value()); // 122500
}
