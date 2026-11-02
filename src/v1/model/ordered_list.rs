use crate::v1::model::AllowedTypes;


pub struct OrderedList {
    pub id: i64,
    pub title: String,
    pub min_length: i32,
    pub max_length: i32,
    pub allowed_types: AllowedTypes,
    //  pub elements: Vec

}