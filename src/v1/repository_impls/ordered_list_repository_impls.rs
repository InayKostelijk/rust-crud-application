use async_trait::async_trait;
use sqlx:: {mysql::MySqlPoolOptions, MySqlPool };
use crate::error::Error;
use crate::v1::{
    model::OrderedList,

};
use crate::v1::model::AllowedTypes;
use crate::v1::repository::OrderedListRepository;

#[derive(Clone)]
pub struct OrderedListRepositoryImpls {
    pool: MySqlPool
}
impl OrderedListRepositoryImpls {
    pub fn new(pool:MySqlPool) -> Self {
        Self {
            pool
        }
    }
}
#[async_trait]
impl OrderedListRepository for OrderedListRepositoryImpls {
    async fn get_all_ordered_lists(&self) -> Result<Vec<OrderedList>, Error> {
        let ordered_lists = sqlx::query_as!( OrderedList, "SELECT * FROM ordered_list").fetch_all(&self.pool).await?;
        Ok(ordered_lists)    }

    async fn get_ordered_lists_by_id(&self, id: i64) -> Result<OrderedList, Error> {
        let ordered_lists_by_id = sqlx::query_as!(OrderedList, "SELECT * FROM ordered_list WHERE id=?", id).fetch_one(self.pool);
        Ok(ordered_lists_by_id)
    }
    // TODO
    async fn create_ordered_lists(&self, title: String, min_length: i32, max_length: i32,allowed_types: AllowedTypes) -> Result<OrderedList, Error> {
        let created_ordered_lists = sqlx::query_as!(OrderedList, "INSERT INTO ordered_list(title, min_length, max_length, allowed_types) VALUES (?,?,?,?)",
        title, min_length, max_lenght, allowed_types).execute(&self.pool).await?;


        Ok(created_ordered_lists)

    }
    // TODO
    async fn update_ordered_lists(&self, id: i64, title: String, min_length: i32, max_length: i32,allowed_types: AllowedTypes) -> Result<OrderedList, Error> {
        let updated_ordered_list = sqlx::query_as!(OrderedList,"UPDATE ");

        Ok(updated_ordered_list)
    }
    //TODO
    async fn delete_ordered_lists(&self, id: i64) -> Result<OrderedList, Error> {
        let deleted_ordered_lists = sqlx::query_as!(OrderedList, "DELETE FROM ordered_list WHERE id =?", id);

        Ok(deleted_ordered_lists)
    }
}



