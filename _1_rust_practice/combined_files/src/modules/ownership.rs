#[derive(Debug, Clone)]
struct Address {
    city: String,
    state: String,
    pincode: u32,
}

#[derive(Debug, Clone)]
struct User {
    id: u32,
    name: String,
    email: String,
    active: bool,
    address: Address,
}

impl User {
    fn s2_user(&mut self) {
        self.id=18;
        self.name = "Alina".to_string();
        self.email = "alina@gmail.com".to_string();
        self.active = false;

        self.address.city = "Mumbai".to_string();
        self.address.state = "Maharashtra".to_string();
        self.address.pincode = 400001;
    }
}

pub fn run() {

    let mut user = User {
        id: 12,
        name: "Anjali".to_string(),
        email: "anjali@gmail.com".to_string(),
        active: true,
        address: Address {
            city: "Ahmedabad".to_string(),
            state: "Gujarat".to_string(),
            pincode: 380001,
        },
    };

    let s1_user = user.clone();

        let user_ref = &mut user;
        user_ref.s2_user();
    
    println!("S1 original: {:?}\nS2 Update: {:?}", s1_user, user);
}