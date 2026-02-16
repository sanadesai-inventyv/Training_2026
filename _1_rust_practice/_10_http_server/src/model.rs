use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Student {
    pub id: String,
    pub name: String,
    pub email: String,
}
