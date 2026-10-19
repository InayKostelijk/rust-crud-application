use serde::Deserialize;


#[derive(Deserialize)]
pub struct expenses {
    description: String,
    amount: i32

}

