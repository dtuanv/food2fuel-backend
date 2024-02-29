use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow,Debug, Serialize, Deserialize)]

//This allows the Item struct to be printed for debugging purposes
// and to be serialized and deserialized to and from JSON
pub struct Item{
    pub id:i64,
    pub description: String,
    pub category: String,
    pub expirationdate: String,
}

#[derive(sqlx::FromRow, Debug,Serialize, Deserialize)]
pub struct Account{
    id: i64,
    password: String,
    title: String,
    username: String
}