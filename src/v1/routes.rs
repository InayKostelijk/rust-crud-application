use axum::{Json, extract::State, routing::{get, post, put, delete}, Router, debug_handler};
use axum::extract::Path;
use serde_json::json;
use crate::error::{Error, Result};
use crate::state::State as AppState;
use crate::v1::{repository::OrderedListRepository};
use super::{dto::ExpensesOutgoing,controller };
    pub fn router() -> Router<crate::state::State> {
        Router::new().route("/ordered-lists", get(get_all_ordered_lists))
    }
    async fn get_all_ordered_lists(State(state) : State<AppState>) -> Result<Json<Vec<ExpensesOutgoing>>, Error> {
        let expenses = controller::get_all_ordered_list(state.ordered_list_repository.as_ref()).await?;
        Ok(Json(expenses))
    }