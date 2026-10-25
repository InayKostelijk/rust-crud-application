use axum::{Json, extract::State, routing:: {get,post,put,delete},Router};
use axum::extract::Path;
use serde_json::json;
use crate::error::{Error, Result};
use crate::v1::{repository::ExpensesRepository};
use super::{dto::ExpensesOutgoing,controller };


    pub fn router() -> Router<u32> {
        Router::new().route("/ordered-lists", get(get_all_ordered_list())).with_state(5)
    }
    async fn get_all_ordered_list(state: State<u32>) -> Result<Json<Vec<ExpensesOutgoing>>, Error> {
        let expenses= controller::get_all_expenses().await?;
        Ok(Json(expenses))
    }