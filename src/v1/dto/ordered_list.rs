use serde::{Serialize, Deserialize};
use crate::error::Error;
use crate::v1::{model};
use crate::v1::model::AllowedTypes;

#[derive(Deserialize)]
pub struct ExpensesIngoing {
    title: String,
    min_length: i32,
    max_length: i32,
    allowed_types: AllowedTypes
}

#[derive(Serialize)]
pub struct ExpensesOutgoing {
    id: i64,
    title: String,
    min_length: i32,
    max_length: i32,
    allowed_types: AllowedTypes
}

impl ExpensesOutgoing {
    pub fn from_model(value: model::OrderedList) -> Self {
        let model::OrderedList {
            id,
            title,
            min_length,
            max_length,
            allowed_types,
            // elements,

        } = value;
        Self{
            id,
            title,
            min_length,
            max_length,
            allowed_types
        }
    }
}
