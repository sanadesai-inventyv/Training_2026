 struct Address {
    city: String,
    state: String,
    pincode: u32,
}

struct User {
    id: u32,
    name: String,
    email: String,
    phone: String,
    active: bool,
    address: Address,
}

impl User {
    fn new(
        id: u32,
        name: String,
        email: String,
        phone: String,
        address: Address,
    ) -> User {
        User {
            id,
            name,
            email,
            phone,
            active: true,
            address,
        }
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn set_email(&mut self, email: String) {
        self.email = email;
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn set_city(&mut self, city: String) {
        self.address.city = city;
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_email(&self) -> String {
        self.email.clone()
    }

    fn is_active(&self) -> bool {
        self.active
    }
    fn get_address(&self) -> String {
        format!(
            "{},{},{}",
        self.address.city,
        self.address.state,
        self.address.pincode
        )
    }

    // full info
    fn get_user_info(&self) -> String {
        format!(
            "Id: {}\nName: {}\nEmail: {}\nPhone: {}\nActive: {}\nCity: {}\nState: {}\nPincode: {}",
            self.id,
            self.name,
            self.email,
            self.phone,
            self.active,
            self.address.city,
            self.address.state,
            self.address.pincode
        )
    }
}

fn main() {

     let mut user = User::new(
        11234,
        String::from("Anjali"),
        String::from("anjali@gmail.com"),
        String::from("9998887770"),
        Address {
            city: String::from("Ahmedabad"),
            state: String::from("Gujarat"),
            pincode: 380001,
        },
    );

    user.set_name(String::from("Anjali Shah"));
    user.set_city(String::from("Ahmedabad"));
    user.set_email(String::from("anjali112@gmail.com"));
    user.set_active(true);

    println!("Name: {}", user.get_name());
    println!("Email: {}", user.get_email());
    println!("Active: {}", user.is_active());
    println!("Address: {}",user.get_address());

    println!("\n{}", user.get_user_info());
}

