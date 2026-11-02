use async_trait::async_trait;
use crate::{v1::model};
use crate::error::Error;
use crate::v1::model::AllowedTypes;

#[async_trait]
pub trait OrderedListRepository: Send + Sync {
    async fn get_all_ordered_lists(&self) -> Result<Vec<model::OrderedList>, Error>;
    async fn get_ordered_lists_by_id(&self, id: i64) -> Result<model::OrderedList, Error>;
    async fn create_ordered_lists(&self, title: String, min_length: i32, max_length: i32,allowed_types: AllowedTypes) -> Result<model::OrderedList, Error>;
    async fn update_ordered_lists(&self, id: i64, title: String, min_length: i32, max_length: i32,allowed_types: AllowedTypes ) -> Result<model::OrderedList, Error>;
    async fn delete_ordered_lists(&self, id:i64) -> Result<model::OrderedList, Error>;



}

