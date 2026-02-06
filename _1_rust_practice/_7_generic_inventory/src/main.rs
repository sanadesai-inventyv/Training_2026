use std::collections::HashMap;

trait DisplayItem{
    
    fn display(&self) -> String;
}

 #[derive(Clone)]
 struct Product{
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

struct Inventory<T>
where T: DisplayItem + Clone {

    items: HashMap<String, T>,
}

impl<T> Inventory<T>
where T: DisplayItem + Clone {
    fn new() -> Self {
        Inventory {

            items: HashMap::new(),
        }
    }

    fn add_item(&mut self, id: String, item: T) -> Result<(), InventoryError> {
        if id.is_empty() {

            return Err(InventoryError::InvalidId);
        }
        if self.items.contains_key(&id) {

            return Err(InventoryError::DuplicateId);
        }
        self.items.insert(id, item);
        Ok(())
    }

    fn get_item(&self, id: &str) -> Result<&T, InventoryError> {
        self.items.get(id).ok_or(InventoryError::ItemNotFound)
    }

    fn display_item(&self, id: &str) -> Result<String, InventoryError> { 
        let item = self.get_item(id)?; 
        Ok(item.display()) 
    }

    fn display_all(&self) -> String {
    let mut result = String::new();

    for (id, item) in &self.items {
        result.push_str(&format!("ID: {} -> {}\n", id, item.display()));
    }

    result
    
}

}

fn main(){
    let mut inventory = Inventory::<Product>::new();

    let item1 = Product {
        name: "shirt".to_string(),
        value: 900,
    };

    let item2 = Product {
        name: "jeans".to_string(),
        value: 1200,
    };

    inventory.add_item("shirt".to_string(), item1).unwrap();
    inventory.add_item("jeans".to_string(), item2).unwrap();

    match inventory.display_item("jeans") { 
        Ok(display) => println!("Display: {}", display),
        Err(e) => println!("Error: {:?}", e), 
    }

    println!("All items: {}", inventory.display_all());
}