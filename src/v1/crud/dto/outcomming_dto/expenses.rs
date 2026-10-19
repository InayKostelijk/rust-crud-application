use serde:: Serialize;

#[derive(Serialize)]
pub struct expenses {
    id: i64,
    description: String,
    amount: i32
}

