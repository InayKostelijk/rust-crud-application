use crate::{
    error::Error,
    v1::{
        dto::{ExpensesOutgoing, ExpensesIngoing},
        repository::OrderedListRepository,
        model::OrderedList,
    }
};

    pub async fn get_all_ordered_list(ordered_list_repository: &dyn OrderedListRepository,) -> Result<Vec<ExpensesOutgoing>, Error>   {
        let ordered_list = ordered_list_repository.get_all_ordered_lists().await?;
        Ok(ExpensesOutgoing::from_models(ordered_list))
    }


impl ExpensesOutgoing {
    pub fn from_models(ordered_list: Vec<OrderedList>) -> Vec<Self> {
        ordered_list.into_iter().map(Self::from_model).collect()
    }
}
