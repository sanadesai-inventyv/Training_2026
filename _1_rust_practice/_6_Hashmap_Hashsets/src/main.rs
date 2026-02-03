use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, Default, Debug)]
struct Student {
    id_name: HashMap<u32, String>,
    groups: HashSet<String>,
}

fn handle_student(mut s: Student) {

    match s.id_name.try_reserve(2) {
        Ok(_) => println!("Reserved"),
        Err(_) => println!("Reserve failed"),
    }

    s.id_name.insert(1, "Ram".to_string());
    s.id_name.insert(2, "Shyam".to_string());

    s.groups.insert("GroupA".to_string());
    s.groups.insert("GroupB".to_string());

    
    let mut s1 = s.clone();

    
    let mut taken = std::mem::take(&mut s1.id_name);

    taken.retain(|_, v| v == "Ram");

    taken.extend([(3, "Sita".to_string())]);

    println!("Final: {taken:?}");
    println!("Groups: {:?}", s.groups);
}

fn main() {

    let s2 = Student::default();

    handle_student(s2);
}
