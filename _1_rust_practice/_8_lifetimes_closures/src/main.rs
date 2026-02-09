use std::collections::HashMap;

trait DisplayItem {

    fn display(&self) -> String;
}

struct Product {
    name: String,
    value: i32,
}

impl DisplayItem for Product {
    fn display(&self) -> String {
        format!("Product: {}, Value: {}", self.name, self.value)
    }
}

#[derive(Debug)]
enum InventoryError {
    DuplicateId,
    InvalidId,
    ItemNotFound,
}

struct Inventory<'a, T>
where
    T: DisplayItem,
{
    items: HashMap<String, &'a T>,
}

impl<'a, T> Inventory<'a, T>
where
    T: DisplayItem,
{
    fn new() -> Self {
        Inventory {
            items: HashMap::new(),
        }
    }

    fn add_item(&mut self, id: String, item: &'a T) -> Result<(), InventoryError> {
        if id.is_empty() {
            return Err(InventoryError::InvalidId);
        }

        if self.items.contains_key(&id) {
            return Err(InventoryError::DuplicateId);
        }

        self.items.insert(id, item);
        Ok(())
    }

    fn display_all(&self) -> String {
        let mut result = String::new();

        let show = |item: &T| item.display();

        for (id, item) in &self.items {
            result.push_str(&format!("ID: {} -> {}\n", id, show(item)));
        }

        result
    }
}

fn main() {
    let item1 = Product {
        name: "shirt".to_string(),
        value: 900,
    };

    let item2 = Product {
        name: "jeans".to_string(),
        value: 1200,
    };

    let mut inventory = Inventory::<Product>::new();

    inventory.add_item("shirt".to_string(), &item1).unwrap();
    inventory.add_item("jeans".to_string(), &item2).unwrap();

    println!("All items:\n{}", inventory.display_all());
}

