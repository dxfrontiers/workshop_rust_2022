use serde::{Deserialize, Serialize};

use crate::schema::people;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "people"]
pub struct Person {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub age: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "people"]
pub struct NewPerson {
    pub first_name: String,
    pub last_name: String,
    pub age: i32,
}