use serde::{Serialize, Deserialize};


#[derive(Deserialize)]
pub struct expenses {
    description: String,
    amount: i32

}

#[derive(Serialize)]
pub struct expenses {
    id: i64,
    description: String,
    amount: i32
}

