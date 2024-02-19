use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]

//This allows the Ticket struct to be printed for debugging purposes
// and to be serialized and deserialized to and from JSON
pub struct Ticket {
    pub id: i32,
    pub name: String,

}