use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Person {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub age: u32,
}

#[derive(Serialize, Deserialize)]
pub struct NewPerson {
    pub first_name: String,
    pub last_name: String,
    pub age: u32,
}