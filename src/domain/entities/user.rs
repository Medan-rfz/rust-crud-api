use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub age: i32,
}

impl User {
    pub fn new(name: &str, age: i32) -> Self {
        User {
            id: 0,
            name: name.to_string(),
            age: age
        }
    }
}
