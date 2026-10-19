use crate::{
    error::Error,
    v1::{
        dto::Expense,
        model,
        repository::ExpenseRepository
    }
};

pub async fn get_all_expenses() -> Result <dto::Expenses> {
    let response = repo.get_all_expenses().await?;
    dto::Expenses::from_query(response).ok_or(Error::NotFound)
}