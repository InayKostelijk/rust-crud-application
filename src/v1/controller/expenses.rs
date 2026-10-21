use crate::{
    error::Error,
    v1::{
        dto::{ExpensesOutgoing, ExpensesIngoing},
        repository::ExpensesRepository,
        model::Expenses,
    }
};

    pub async fn get_all_expenses(expense_repository: impl ExpensesRepository) -> Result<Vec<ExpensesOutgoing>, Error>   {
        let expenses = expense_repository.get_all_expenses().await?;
        Ok(ExpensesOutgoing::from_models(expenses))
    }


impl ExpensesOutgoing {
    pub fn from_models(expenses: Vec<Expenses>) -> Vec<Self> {
        expenses.into_iter().map(Self::from_model).collect()
    }
}
