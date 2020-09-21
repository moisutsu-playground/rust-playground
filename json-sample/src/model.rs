use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonSample {
    pub name: String,
    pub age: i32,
    pub address: Address,
    pub phones: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Address {
    pub street: String,
    pub city: String,
}
