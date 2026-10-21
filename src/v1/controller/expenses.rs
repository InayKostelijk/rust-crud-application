use crate::{
    error::Error,
    v1::{
        dto::{ExpensesOutgoing, ExpensesIngoing},
        repository::ExpensesRepository,
        model::Expenses,
    }
};
pub struct ExpenseController<Repo> {
    expense_repository: Repo,
}
impl <Repo: ExpensesRepository> ExpenseController<Repo> {
    pub fn new(expense_repository: Repo) -> Self {
        Self {
            expense_repository
        }
    }
    pub async fn get_all_expenses(&self) ->  Result<Vec<ExpensesOutgoing>, Error>   {
        let expenses = self.expense_repository.get_all_expenses().await?;
        Ok(ExpensesOutgoing::from_models(expenses))
    }


}
impl ExpensesOutgoing {
    pub fn from_models(expenses: Vec<Expenses>) -> Vec<Self> {
        expenses.into_iter().map(Self::from_model).collect()
    }
}
