use serde_json;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Address {
    city: String,
    state: String,
    pincode: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u32,
    name: String,
    email: String,
    active: bool,
    address: Address,
}

fn main() {
    let user = User {
        id: 1123,
        name: "Anjali".to_string(),
        email: "anjali@gmail.com".to_string(),
        active: true,
        address: Address {
            city: "Ahmedabad".to_string(),
            state: "Gujarat".to_string(),
            pincode: 380001,
        },
    };

    let json = serde_json::to_string(&user).unwrap();

    println!("Serialized User:\n{}", json);

    let user_back: User = serde_json::from_str(&json).unwrap();

    println!("\nDeserialized Struct:");
    println!("{:?}", user_back);

    let raw_json = r#"
    {
        "id": 2,
        "name": "Rahul",
        "email": "rahul@gmail.com",
        "active": false,
        "address": {
            "city": "Mumbai",
            "state": "Maharashtra",
            "pincode": 400001
        }
    }
    "#;

    let raw_user: User = serde_json::from_str(raw_json).unwrap();

    println!("id: {}", raw_user.id);
	println!("name: {}", raw_user.name);
	println!("email: {}", raw_user.email);
	println!("active: {}", raw_user.active);
    println!("address: {}",raw_user.address.city);
    println!("address: {}",raw_user.address.state);
    println!("address: {}",raw_user.address.pincode);
}
