use crate::{
    error::Error,
    v1::{
        dto::{ExpensesOutgoing, ExpensesIngoing},
        repository::OrderedListRepository,
        model::OrderedList,
    }
};

    pub async fn get_all_ordered_list(expense_repository: impl OrderedListRepository) -> Result<Vec<ExpensesOutgoing>, Error>   {
        let expenses = expense_repository.get_all_ordered_lists().await?;
        Ok(ExpensesOutgoing::from_models(expenses))
    }


impl ExpensesOutgoing {
    pub fn from_models(expenses: Vec<OrderedList>) -> Vec<Self> {
        expenses.into_iter().map(Self::from_model).collect()
    }
}
