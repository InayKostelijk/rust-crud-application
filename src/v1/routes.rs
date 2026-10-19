use axum::{Json, extract::State, routing:: {get,post,put,delete},Router};
use crate::error::Result;


pub fn router() -> Router{
    Router::new().route("/expenses", get())
    

}


async fn get_all_expenses() -> Json<Vec<Expense {

}